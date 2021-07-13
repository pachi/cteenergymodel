// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación del cálculo de la U de una composión constructiva de opaco, según su posición
//! - UNE-EN ISO 13789:2010 transmisión general
//! - UNE-EN ISO 6946:2012 para elementos opacos
//! - UNE-EN ISO 13770:2017 para elementos en contacto con el terremo
#![allow(non_snake_case)]

use std::{collections::HashMap, convert::From, f32::consts::PI};

use log::{debug, info, warn};
use na::{
    point, vector, Isometry, Point, Point2, Point3, Rotation, Rotation2, Rotation3, Translation2,
    Translation3, Vector2, Vector3,
};

use super::{Geometry, Model, Orientation, QSolJulData, Window, WindowGeometry};

const EPSILON: f32 = 1e-5;

impl Model {
    /// Calcula el parámetro de control solar (q_sol;jul) a partir de los datos de radiación total acumulada en julio
    /// Los huecos para los que no está definido su opaco o su construcción no se consideran en el cálculo
    pub fn q_soljul(&self, totradjul: &HashMap<Orientation, f32>) -> QSolJulData {
        let mut q_soljul_data = QSolJulData::default();

        let Q_soljul = self
            .windows_of_envelope_iter()
            .filter_map(|w| {
                let wall = self.wall_of_window(&w)?;
                let multiplier = self
                .space_of_wall(wall)
                .map(|s| s.multiplier)
                .unwrap_or(1.0);
                let wincons = self.wincons_of_window(&w)?;
                let orientation = Orientation::from(wall);
                let radjul = totradjul.get(&orientation).unwrap();
                let area = w.area * multiplier;
                let Q_soljul_orient = w.fshobst * wincons.gglshwi * (1.0 - wincons.ff) * area * radjul;
                // Datos de detalle
                let mut detail = q_soljul_data.detail.entry(orientation).or_default();
                detail.a += area;
                detail.gains += Q_soljul_orient;
                detail.irradiance = *radjul;
                detail.ff_mean += wincons.ff * area;
                detail.gglshwi_mean += wincons.gglshwi * area;
                detail.fshobst_mean += w.fshobst * area;
                // Valores medios y acumulados
                q_soljul_data.a_wp += area;
                q_soljul_data.irradiance_mean += *radjul * area;
                q_soljul_data.fshobst_mean += w.fshobst * area;
                q_soljul_data.gglshwi_mean += wincons.gglshwi * area;
                q_soljul_data.ff_mean += wincons.ff * area;
                debug!(
                    "qsoljul de {}: A {:.2}, orient {}, ff {:.2}, gglshwi {:.2}, fshobst {:.2}, H_sol;jul {:.2}",
                    w.name, area, orientation, wincons.ff, wincons.gglshwi, w.fshobst, radjul
                );
                Some(Q_soljul_orient)
            })
            .sum::<f32>();
        let a_ref = self.a_ref();
        let q_soljul = Q_soljul / a_ref;
        info!(
            "q_sol;jul={:.2} kWh/m².mes, Q_soljul={:.2} kWh/mes, A_ref={:.2}",
            q_soljul, Q_soljul, a_ref
        );

        // Guarda datos globales y corrige medias globales
        q_soljul_data.q_soljul = q_soljul;
        q_soljul_data.Q_soljul = Q_soljul;
        q_soljul_data.irradiance_mean /= q_soljul_data.a_wp;
        q_soljul_data.fshobst_mean /= q_soljul_data.a_wp;
        q_soljul_data.gglshwi_mean /= q_soljul_data.a_wp;
        q_soljul_data.ff_mean /= q_soljul_data.a_wp;

        // Completa cálcula de medias por orientación (dividiendo por area de cada orientación)
        for (_, detail) in q_soljul_data.detail.iter_mut() {
            detail.ff_mean /= detail.a;
            detail.gglshwi_mean /= detail.a;
            detail.fshobst_mean /= detail.a;
        }

        q_soljul_data
    }

    /// Calcula factor de obstáculos remotos para los huecos para la posición solar dada
    ///
    /// Considera la sombra de muros y sombras sobre el hueco
    pub fn fshobst_for_sun_pos(&self, sun_azimuth: f32, sun_altitude: f32) -> HashMap<String, f32> {
        let mut map: HashMap<String, f32> = HashMap::new();
        for w in &self.windows {
            let sunlit = self.sunlit_fraction(w, sun_azimuth, sun_altitude);
            // map.insert(w.id.clone(), sunlit);
            map.insert(w.name.clone(), sunlit);
        }
        map
    }

    /// Fracción del hueco con radiación solar directa para la posición solar dada [0.0 - 1.0]
    ///
    /// XXX: cuando la definición geométrica es incompleta (sin posición) o no se puede localizar el muro, se devuelve 1.0 (sin obstrucción)
    ///
    /// window: window.id
    /// Azimuth (S=0, E=90, W=-90)
    /// Altura solar (Horiz=0, vert=90), en grados
    pub fn sunlit_fraction(&self, window: &Window, sun_azimuth: f32, sun_altitude: f32) -> f32 {
        let sazim = (sun_azimuth * PI) / 180.0;
        let salt = (sun_altitude * PI) / 180.0;
        // Direction pointing towards the sun in the XYZ coordinate system (Z up, +X=E, +Y=N)
        let ray_dir = vector![
            salt.cos() * sazim.sin(),
            -salt.cos() * sazim.cos(),
            salt.sin()
        ]
        .normalize();

        let winWall = self.walls.iter().find(|w| w.id == window.wall);
        if winWall.is_none() {
            warn!(
                "Hueco {} (id: {}) sin muro asociado con id: {}. Se considera superficie soleada al 100%",
                window.name, window.id, window.wall
            );
            return 1.0;
        };
        let winWall = winWall.unwrap();

        let geometry = &winWall.geometry;
        let Geometry {
            tilt,
            azimuth,
            position,
            polygon,
        } = geometry;

        if position.is_none() {
            warn!(
                "Hueco {} (id: {}) sin definición geométrica completa. Se considera superficie soleada al 100%",
                window.name, window.id
            );
            return 1.0;
        };

        // Conversión a coordenadas globales
        let wallTransform = transformMatrix(*tilt, *azimuth, position.unwrap());
        // Conversión de coordenadas locales de muro a coordenadas de polígono de muro
        let wallLocal2WallPolyTransform = wallLocal2WallPolygon(polygon);

        // Compute window for window shading tests
        let points: Vec<Point3<f32>> = getWindowPoints(&window)
            .iter()
            .map(|p| wallLocal2WallPolyTransform * p)
            .map(|p| wallTransform * point![p.x, p.y, 0.0])
            .collect();

        // Elementos oclusores, exceptuado muro del hueco
        let mut num = 0;
        let mut num_intersects = 0;
        for ray_orig in points {
            num += 1;
            let mut intersects = false;
            for w in &self.walls {
                // Exceptuamos el muro del hueco
                if w.id == winWall.id {
                    continue;
                };
                // TODO: podríamos eliminar los muros con una normal cuyo producto escalar con el rayo sea positivo (mira en la misma dirección).
                let intersection = intersectPoly2D(ray_orig, ray_dir, &w.geometry);
                if intersection.is_some() {
                    intersects = true;
                    // debug!("Intersección con muro oclusor: {}, de rayo: {}, punto 3D: {:#?}, en geometría: {:#?}", w.name, ray_dir, intersection, &w.geometry);
                    break;
                }
            }
            if !intersects {
                for s in &self.shades {
                    let intersection = intersectPoly2D(ray_orig, ray_dir, &s.geometry);
                    if intersection.is_some() {
                        intersects = true;
                        // debug!("Intersección con sombra oclusora: {}, de rayo: {}, punto 3D: {:#?}, en geometría: {:#?}", s.name, ray_dir, intersection, &s.geometry);
                        break;
                    }
                }
            }

            if intersects {
                num_intersects += 1;
            }
        }

        // debug!(
        //     "Num. intersecciones para hueco {}: {} de {}",
        //     &window.name, num_intersects, num
        // );
        1.0 - num_intersects as f32 / num as f32
    }
}

// -------------------------- Funciones auxiliares ---------------------------

/// Calcula la existencia de intersección entre rayo y geometría, e indica el punto de intersección
///
/// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
/// ray_dir: dirección del rayo en coordenadas globales (Vector3)
/// poly: polígono 2D (XY), Polygon: Vec[Point2, ...]
///
/// - Transforma el rayo al espacio del polígono
/// - Calcula el punto de intersección del rayo transformado con el plano XY
/// - Comprueba si el punto está en el interior del polígono
/// - Transforma el punto al espacio del rayo (coordenadas globales)
///
/// Si no devolvemos el punto de intersección ahorraríamos una transformación
pub fn intersectPoly2D(
    ray_origin: Point<f32, 3_usize>,
    ray_dir: Vector3<f32>,
    geom: &Geometry,
) -> Option<Point<f32, 3_usize>> {
    // Matrices de transformación de geometría
    let trans = transformMatrix(geom.tilt, geom.azimuth, geom.position.unwrap());
    let transInv = trans.inverse();

    // Inverse transform of ray (we keep the 2D polygon as is and transform the ray)
    let inv_ray_o = transInv * ray_origin;
    // En JS es transInv.extractRotation porque no diferencia Vector de Point
    let inv_ray_d = transInv * ray_dir;

    // Normal to the planar polygon
    let n_p = findPoly2DNormal(&geom.polygon);
    // Check if ray is parallel to the polygon
    let denominator = n_p.dot(&inv_ray_d);
    if denominator.abs() < EPSILON {
        return None;
    }

    // Find intersection of ray with XY plane
    let poly_o_to_ray = point![geom.polygon[0].x, geom.polygon[0].y, 0.0] - inv_ray_o;
    let t = n_p.dot(&poly_o_to_ray) / denominator;

    // We only consider positive t (it's a ray!)
    if t < 0.0 {
        return None;
    }
    let intersection_point = inv_ray_o + t * inv_ray_d;

    // Verify that the point falls inside the polygon
    let point2d = intersection_point.xy();
    // TODO: Pending optimization: check if point is in the 2D AABB
    let point_is_inside = pointInPolygon2D(point2d, &geom.polygon);

    // Return the transformed back intersection point (global coords) or None
    if point_is_inside {
        Some(trans * intersection_point)
    } else {
        None
    }
}

// Test 2D de punto en polígono usando el método de Heines
// http://erich.realtimerendering.com/ptinpoly/
// Cuenta el número de cruces haciendo raycasting desde el punto para ver si está dentro (cruces impares) o fuera (cruces pares)
// Evita el cálculo de las intersecciones y la división por cero viendo los cambios de signo
// https://stackoverflow.com/questions/217578/how-can-i-determine-whether-a-2d-point-is-within-a-polygon/2922778#2922778
// ver https://docs.rs/geo/0.2.6/src/geo/.cargo/registry/src/github.com-1ecc6299db9ec823/geo-0.2.6/src/algorithm/contains.rs.html#9-33
// https://docs.rs/geo/0.18.0/geo/algorithm/contains/trait.Contains.html
// Ver algunos casos límite en https://stackoverflow.com/a/63436180
// Evita el cálculo del punto de intersección y una división localizando la condición de cruce
pub fn pointInPolygon2D(pt: Point2<f32>, poly: &[Point2<f32>]) -> bool {
    let x = pt.x;
    let y = pt.y;
    let mut inside = false;

    // Empezamos con el segmento que une el punto final con el inicial
    let mut v_j = poly[poly.len() - 1];
    let mut y_0 = v_j.y >= y;
    for &v_i in poly {
        let y_1 = v_i.y >= y;
        // primero se mira si el lado cruza la linea horizontal en pt.y
        // y, si es así, comprobamos si se cruza también en x para detectar que se produe el cruce
        if y_0 != y_1 && (((v_i.y - y) * (v_j.x - v_i.x) >= (v_i.x - x) * (v_j.y - v_i.y)) == y_1) {
            inside = !inside;
        }
        // Avanzamos al siguiente segmento
        y_0 = y_1;
        v_j = v_i;
    }

    inside
}

/// Matriz de transformación de los elementos del edificio
///
/// Traslada de coordenadas de opaco / sombra a coordenadas globales (giros y desplazamientos)
fn transformMatrix(
    tilt: f32,
    azimuth: f32,
    position: Point3<f32>,
) -> Isometry<f32, Rotation<f32, 3_usize>, 3_usize> {
    let trans = Translation3::<f32>::from(position);
    let zrot = Rotation3::<f32>::new(Vector3::z() * azimuth.to_radians());
    let xrot = Rotation3::<f32>::new(Vector3::x() * tilt.to_radians());

    trans * zrot * xrot
}

/// Matriz de transformación de coordenadas locales de muro a coordenadas de su polígono 2D
/// Nos sirve para pasar de las coordenadas locales del muro a las coordenadas del polígono de muro en 2D
/// Se gira el eje X en la dirección del polígono de muro p1 - p0 y se traslada a p0 el origen
fn wallLocal2WallPolygon(
    wall_polygon: &[Point2<f32>],
) -> Isometry<f32, Rotation<f32, 2_usize>, 2_usize> {
    let v0 = wall_polygon[0];
    let v1 = wall_polygon[1];
    let dir_x = v1 - v0;
    let rot = Rotation2::rotation_between(&Vector2::x(), &dir_x);
    let trans = Translation2::from(v0);

    trans * rot
}

// Normal al polígono plano
fn findPoly2DNormal(poly: &[Point2<f32>]) -> Vector3<f32> {
    let v0 = poly[1] - poly[0];
    let v1 = poly[2] - poly[0];

    vector![v0.x, v0.y, 0.0]
        .cross(&vector![v1.x, v1.y, 0.0])
        .normalize()
}

// Calcula los puntos de origen en el hueco para el cálculo de fracción sombreada
//
// Parte de una retícula de 10x10 elementos, que daría un 1% de cobertura por cada celda
// Se podría optimizar la heurística, afinando el valor de N=10 en función del
// tamaño y proporción del hueco (p.e. para que sean más o menos cuadradas las celdas)
// aunque se pierda precisión en huecos pequeños la resolución sería similar en ambas direcciones
fn getWindowPoints(window: &Window) -> Vec<Point2<f32>> {
    const N: usize = 10;
    let WindowGeometry {
        position,
        width,
        height,
        ..
    } = window.geometry;
    let stepX = width / N as f32;
    let stepY = height / N as f32;
    let mut points = vec![];
    let (x, y) = match position {
        Some(p) => (p.x, p.y),
        // devolvemos una lista vacía de puntos, no hay definición geométrica
        _ => return Vec::new(),
    };

    for j in 0..N {
        for i in 0..N {
            let px = x + (i as f32 + 0.5) * stepX;
            let py = y + (j as f32 + 0.5) * stepY;
            points.push(point![px, py]);
        }
    }

    points
}
