// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Rutinas de cálculo geométrico
#![allow(non_snake_case)]

use std::convert::From;

// use log::{debug, info, warn};
use na::{
    point, vector, IsometryMatrix2, IsometryMatrix3, Point2, Point3, Rotation2, Rotation3,
    Translation2, Translation3, Vector2, Vector3,
};

use super::{utils::uuid_from_str, Geometry, Shade};

const EPSILON: f32 = 1e-5;

// -------------------------- Funciones auxiliares ---------------------------

impl Geometry {
    /// Vector unitario normal a la geometría, en coordenadas globales
    pub fn normal(&self) -> Vector3<f32> {
        let n_p = poly_normal(&self.polygon);
        let zrot = Rotation3::new(Vector3::z() * self.azimuth.to_radians());
        let xrot = Rotation3::new(Vector3::x() * self.tilt.to_radians());
        let tr = zrot * xrot;
        tr * n_p
    }

    /// Calcula la intersección entre rayo y geometría, e indica el factor t en la dirección del rayo
    ///
    /// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
    /// ray_dir: dirección del rayo en coordenadas globales (Vector3)
    ///
    /// Si es un punto interior devuelve t tal que la intersección se produce en ray_origin + t * ray_dir
    pub fn intersect(&self, ray_origin: &Point3<f32>, ray_dir: &Vector3<f32>) -> Option<f32> {
        // Matrices de transformación de geometría
        let transInv = self.local_to_global().map(|m| m.inverse());
        // Normal to the planar polygon
        let n_p = &poly_normal(&self.polygon);
        intersect_with_data(ray_origin, ray_dir, &self.polygon, transInv.as_ref(), n_p)
    }

    pub fn aabb(&self) -> AABB {
        if let Some(trans) = self.local_to_global() {
            let mut min_x = f32::INFINITY;
            let mut min_y = f32::INFINITY;
            let mut min_z = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut max_y = f32::NEG_INFINITY;
            let mut max_z = f32::NEG_INFINITY;

            for p in self.polygon.iter().map(|p| trans * point![p.x, p.y, 0.0]) {
                min_x = min_x.min(p.x);
                max_x = max_x.max(p.x);
                min_y = min_y.min(p.y);
                max_y = max_y.max(p.y);
                min_z = min_z.min(p.z);
                max_z = max_z.max(p.z);
            }

            AABB {
                min: point![min_x, min_y, min_z],
                max: point![max_x, max_y, max_z],
            }
        } else {
            Default::default()
        }
    }

    /// Matriz de transformación de coordenadas locales a coordenadas globales
    pub fn local_to_global(&self) -> Option<IsometryMatrix3<f32>> {
        local_to_global_transform(self.tilt, self.azimuth, self.position)
    }

    /// Matriz de transformación de coordenadas locales de la geometría a coordenadas de polígono interno
    pub fn local_to_polygon(&self) -> Option<IsometryMatrix2<f32>> {
        local_to_polygon_transform(&self.polygon)
    }
}

/// Calcula la intersección entre rayo y polígono aportando matrices, e indica el factor t en la dirección del rayo
///
/// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
/// ray_dir: dirección del rayo en coordenadas globales (Vector3)
/// polygon: polígono 2D (en plano XY con transformación de coordenadas y normal dadas)
/// transInv: matriz de transformación desde coordenadas globales a locales del polígono
/// n_p: normal a la geometría
///
/// - Transforma el rayo al espacio del polígono
/// - Calcula el punto de intersección del rayo transformado con el plano XY
/// - Comprueba si el punto está en el interior del polígono
/// - Si es un punto interior devuelve t tal que la intersección se produce en ray_origin + t * ray_dir
pub fn intersect_with_data(
    ray_origin: &Point3<f32>,
    ray_dir: &Vector3<f32>,
    polygon: &[Point2<f32>],
    transInv: Option<&IsometryMatrix3<f32>>,
    n_p: &Vector3<f32>,
) -> Option<f32> {
    let transInv = transInv?;
    // Inverse transform of ray (we keep the 2D polygon as is and transform the ray)
    let inv_ray_o = transInv * ray_origin;
    // En JS es transInv.extractRotation porque no diferencia Vector de Point
    let inv_ray_d = transInv * ray_dir;

    // Check if ray is parallel to the polygon
    let denominator = n_p.dot(&inv_ray_d);
    if denominator.abs() < EPSILON {
        return None;
    }

    // Find intersection of ray with XY plane
    let poly_o_to_ray = point![polygon[0].x, polygon[0].y, 0.0] - inv_ray_o;
    let t = n_p.dot(&poly_o_to_ray) / denominator;

    // We only consider positive t (it's a ray!)
    if t < 0.0 {
        return None;
    }

    // Verify that the point falls inside the polygon
    let intersection_point = inv_ray_o + t * inv_ray_d;
    let point2d = intersection_point.xy();
    // TODO: Pending optimization: check if point is in the 2D AABB
    let point_is_inside = point_in_poly(point2d, polygon);

    if point_is_inside {
        // Intersection point is at t units in the ray direction from its origin
        // let intp = trans * intersection_point;
        // let intp = ray_origin + t * ray_dir;
        Some(t)
    } else {
        None
    }
}

/// Normal al polígono plano, en coordenadas locales
pub fn poly_normal(poly: &[Point2<f32>]) -> Vector3<f32> {
    let v0 = poly[1] - poly[0];
    let v1 = poly[2] - poly[0];

    vector![v0.x, v0.y, 0.0]
        .cross(&vector![v1.x, v1.y, 0.0])
        .normalize()
}

/// Matriz de transformación de los elementos del edificio
///
/// Traslada de coordenadas de opaco / sombra a coordenadas globales (giros y desplazamientos)
fn local_to_global_transform(
    tilt: f32,
    azimuth: f32,
    position: Option<Point3<f32>>,
) -> Option<IsometryMatrix3<f32>> {
    let trans = Translation3::from(position?);
    let zrot = Rotation3::new(Vector3::z() * azimuth.to_radians());
    let xrot = Rotation3::new(Vector3::x() * tilt.to_radians());

    Some(trans * zrot * xrot)
}

/// Matriz de transformación de coordenadas locales de muro a coordenadas de su polígono 2D
/// Nos sirve para pasar de las coordenadas locales del muro a las coordenadas del polígono de muro en 2D
/// Se gira el eje X en la dirección del polígono de muro p1 - p0 y se traslada a p0 el origen
fn local_to_polygon_transform(wall_polygon: &[Point2<f32>]) -> Option<IsometryMatrix2<f32>> {
    if wall_polygon.len() <= 2 {
        return None;
    };
    let v0 = wall_polygon[0];
    let v1 = wall_polygon[1];
    let dir_x = v1 - v0;
    let rot = Rotation2::rotation_between(&Vector2::x(), &dir_x);
    let trans = Translation2::from(v0);

    Some(trans * rot)
}

/// Test 2D de punto en polígono usando el método de Heines
/// http://erich.realtimerendering.com/ptinpoly/
/// Cuenta el número de cruces haciendo raycasting desde el punto para ver si está dentro (cruces impares) o fuera (cruces pares)
/// Evita el cálculo de las intersecciones y la división por cero viendo los cambios de signo
/// https://stackoverflow.com/questions/217578/how-can-i-determine-whether-a-2d-point-is-within-a-polygon/2922778#2922778
/// ver https://docs.rs/geo/0.2.6/src/geo/.cargo/registry/src/github.com-1ecc6299db9ec823/geo-0.2.6/src/algorithm/contains.rs.html#9-33
/// https://docs.rs/geo/0.18.0/geo/algorithm/contains/trait.Contains.html
/// Ver algunos casos límite en https://stackoverflow.com/a/63436180
/// Evita el cálculo del punto de intersección y una división localizando la condición de cruce
pub fn point_in_poly(pt: Point2<f32>, poly: &[Point2<f32>]) -> bool {
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

/// Crea elementos de sombra correpondientes el perímetro de retranqueo del hueco
pub(crate) fn shades_for_window_setback(
    wall: &super::Wall,
    win: &super::Window,
) -> Vec<(String, Shade)> {
    let wing = &win.geometry;
    // Si no hay retranqueo no se genera geometría
    if win.geometry.setback.abs() < 0.01 {
        return vec![];
    };
    let wpos = match wing.position {
        Some(pos) => pos,
        // Si no hay definición geométrica completa no se calcula geometría
        _ => return vec![],
    };

    let wall2world = wall
        .geometry
        .local_to_global()
        .expect("El muro debe tener definición geométrica completa");

    let overhang = Shade {
        id: uuid_from_str(&format!("{}-top_setback", win.id)),
        name: format!("{}_top_setback", win.name),
        geometry: Geometry {
            // inclinación: con 90º es perpendicular al hueco
            tilt: wall.geometry.tilt + 90.0,
            azimuth: wall.geometry.azimuth,
            position: Some(wall2world * point![wpos.x, wpos.y + wing.height, 0.0]),
            polygon: vec![
                point![0.0, 0.0],
                point![0.0, -wing.setback],
                point![wing.width, -wing.setback],
                point![wing.width, 0.0],
            ],
        },
    };

    let left_fin = Shade {
        id: uuid_from_str(&format!("{}-left_setback", win.id)),
        name: format!("{}_left_setback", win.name),
        geometry: Geometry {
            tilt: wall.geometry.tilt,
            azimuth: wall.geometry.azimuth + 90.0,
            position: Some(wall2world * point![wpos.x, wpos.y + wing.height, 0.0]),
            polygon: vec![
                point![0.0, 0.0],
                point![0.0, -wing.height],
                point![wing.setback, -wing.height],
                point![wing.setback, 0.0],
            ],
        },
    };

    let right_fin = Shade {
        id: uuid_from_str(&format!("{}-right_setback", win.id)),
        name: format!("{}_right_setback", win.name),
        geometry: Geometry {
            tilt: wall.geometry.tilt,
            azimuth: wall.geometry.azimuth - 90.0,
            position: Some(wall2world * point![wpos.x + wing.width, wpos.y + wing.height, 0.0]),
            polygon: vec![
                point![0.0, 0.0],
                point![-wing.setback, 0.0],
                point![-wing.setback, -wing.height],
                point![0.0, -wing.height],
            ],
        },
    };

    let sill = Shade {
        id: uuid_from_str(&format!("{}-sill_setback", win.id)),
        name: format!("{}_sill_setback", win.name),
        geometry: Geometry {
            tilt: wall.geometry.tilt - 90.0,
            azimuth: wall.geometry.azimuth,
            position: Some(wall2world * point![wpos.x, wpos.y, 0.0]),
            polygon: vec![
                point![0.0, 0.0],
                point![wing.width, 0.0],
                point![wing.width, wing.setback],
                point![0.0, wing.setback],
            ],
        },
    };

    vec![
        (win.id.clone(), overhang),
        (win.id.clone(), left_fin),
        (win.id.clone(), right_fin),
        (win.id.clone(), sill),
    ]
}

/// Axis aligned bounding box definida por puntos extremos
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl AABB {
    /// Detecta si existe intersección de AABB y rayo usando el algoritmo de Cyrus-Beck
    /// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter3/raycast_aabb.html
    /// NaN es siempre distinto, de modo que las comparaciones con NaN son correctas
    /// Las AABB deben tener ancho > 0 en todas las dimensiones
    pub fn intersects(&self, ray_origin: &Point3<f32>, ray_dir: &Vector3<f32>) -> Option<f32> {
        let idx = 1.0 / ray_dir.x;
        let idy = 1.0 / ray_dir.y;
        let idz = 1.0 / ray_dir.z;

        let t1 = (self.min.x - ray_origin.x) * idx;
        let t2 = (self.max.x - ray_origin.x) * idx;
        let t3 = (self.min.y - ray_origin.y) * idy;
        let t4 = (self.max.y - ray_origin.y) * idy;
        let t5 = (self.min.z - ray_origin.z) * idz;
        let t6 = (self.max.z - ray_origin.z) * idz;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // Si tmax < 0 la línea interseca pero el AABB está detrás
        if tmax < 0.0 {
            // t = tmax;
            return None;
        }

        // Si tmin > tmax el rayo no corta AABB
        if tmin > tmax {
            // t = tmax;
            return None;
        }
        // t = tmin;
        Some(tmin)
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: point![0.0, 0.0, 0.0],
            max: point![0.0, 0.0, 0.0],
        }
    }
}

/// Elemento oclusor, con información geométrica e identificación
///
/// - el id permite excluir el muro de un hueco
/// - el origin_id permite excluir las geometrías de retranqueo que no son del hueco analizado
/// - normal y trans_matrix permiten cachear resultados para cálculo de intersecciones con el polígono 2D transformando un rayo
#[derive(Debug)]
pub struct Occluder {
    /// Id del elemento
    pub id: String,
    /// Id del elemento que genera este oclusor (si proviene de otro elemento, como sombras de retranqueos de huecos)
    pub linked_to_id: Option<String>,
    /// normal del polígono
    pub normal: Vector3<f32>,
    /// Matriz de transformación de coordenadas globales a locales de polígono
    pub trans_matrix: Option<IsometryMatrix3<f32>>,
    /// Polígono 2D
    pub polygon: Vec<Point2<f32>>,
    /// AABB (min, max)
    pub aabb: AABB,
}

impl Occluder {
    pub fn intersect(&self, ray_origin: &Point3<f32>, ray_dir: &Vector3<f32>) -> Option<f32> {
        self.aabb.intersects(ray_origin, ray_dir)?;
        crate::geometry::intersect_with_data(
            ray_origin,
            ray_dir,
            &self.polygon,
            self.trans_matrix.as_ref(),
            &self.normal,
        )
    }
}
