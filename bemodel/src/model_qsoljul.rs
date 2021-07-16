// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación del cálculo de la U de una composión constructiva de opaco, según su posición
//! - UNE-EN ISO 13789:2010 transmisión general
//! - UNE-EN ISO 6946:2012 para elementos opacos
//! - UNE-EN ISO 13770:2017 para elementos en contacto con el terremo
#![allow(non_snake_case)]

use std::{collections::HashMap, convert::From};

use log::{debug, info, warn};
use na::{point, vector, Point3};

use super::{
    BoundaryType::{ADIABATIC, EXTERIOR},
    Geometry, Model, Orientation, QSolJulData, Shade, Wall, Window,
};

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
        let setback_shades = self.windows_setback_shades();
        let occluders = self.get_occluders(&setback_shades);

        let mut map: HashMap<String, f32> = HashMap::new();
        for w in &self.windows {
            let sunlit = self.sunlit_fraction(w, &occluders, sun_azimuth, sun_altitude);
            // map.insert(w.id.clone(), sunlit);
            map.insert(w.name.clone(), sunlit);
        }
        map
    }

    /// Fracción del hueco con radiación solar directa para la posición solar dada [0.0 - 1.0]
    ///
    /// Devuelve 1.0 (sin obstrucción) para definición geométrica incompleta (sin posición o hueco sin muro)
    /// Devuelve 0.0 para huecos cuya normal no mira hacia el sol (backface culling)
    ///
    /// window: window.id
    /// occluders: lista de potenciales elementos oclusores (name, id, geometry)
    /// Azimuth (S=0, E=90, W=-90)
    /// Altura solar (Horiz=0, vert=90), en grados
    pub fn sunlit_fraction(
        &self,
        window: &Window,
        occluders: &[Occluder],
        sun_azimuth: f32,
        sun_altitude: f32,
    ) -> f32 {
        let sazim = sun_azimuth.to_radians();
        let salt = sun_altitude.to_radians();
        // Direction **pointing towards the sun** in the XYZ coordinate system (Z up, +X=E, +Y=N)
        let ray_dir = vector![
            salt.cos() * sazim.sin(),
            -salt.cos() * sazim.cos(),
            salt.sin()
        ]
        .normalize();

        let window_wall = match self.walls.iter().find(|w| w.id == window.wall) {
            None => {
                warn!(
                    "Hueco {} (id: {}) sin muro asociado con id: {}. Se considera superficie soleada al 100%",
                    window.name, window.id, window.wall
                );
                return 1.0;
            }
            Some(wall) => wall,
        };

        // Elementos sin definición geométrica completa. No podemos calcular las obstrucciones
        let geometry = &window_wall.geometry;
        if geometry.position.is_none() {
            warn!(
                "Hueco {} (id: {}) sin definición geométrica completa. Se considera superficie soleada al 100%",
                window.name, window.id
            );
            return 1.0;
        };

        // Comprobamos que la normal del muro y el rayo hacia el sol no son opuestos (backface culling)
        // Si no, el rayo iría al interior del hueco, está en sombra, y devolvemos 0.0
        if window_wall.geometry.normal().dot(&ray_dir) < 0.01 {
            return 0.0;
        }

        let points: Vec<Point3<f32>> = ray_origins_for_window(&window_wall, &window);
        let num = points.len();
        let mut num_intersects = 0;
        for ray_orig in points {
            for Occluder {
                id,
                origin_id,
                geometry,
                ..
            } in occluders
            {
                // Descartamos el muro al que pertenece el hueco
                if id.as_str() == window_wall.id.as_str() {
                    continue;
                };
                // Descartamos las sombras de retranqueo que no provienen del hueco
                if let Some(id) = origin_id {
                    if *id != &window.id {
                        continue;
                    };
                }

                let intersection = geometry.intersect(ray_orig, ray_dir);
                if intersection.is_some() {
                    // debug!("La intersección del elemento oclusor {} y el rayo con origen {} y dirección {} es: t: {}, punto: {:#?}",
                    //        w.name, ray_origin, ray_dir, intersection, intersection.then(|t| Some(ray_origin + t*ray_dir)).unwrap_or_none());
                    num_intersects += 1;
                    break;
                }
            }
        }
        1.0 - num_intersects as f32 / num as f32
    }

    /// Genera lista de elementos oclusores a partir de muros, sombras y sombras de retranqueo
    /// Guarda el nombre del oclusor, su id y la geometría
    /// TODO: optimizar cálculo de colisiones
    /// TODO: - generar BVH AABB
    /// - https://gamedev.stackexchange.com/a/21030
    /// - https://tavianator.com/2011/ray_box.html
    pub fn get_occluders<'a>(&'a self, setback_shades: &'a [(String, Shade)]) -> Vec<Occluder<'a>> {
        let mut occluders: Vec<_> = self
            .walls
            .iter()
            .filter(|&e| e.bounds == ADIABATIC || e.bounds == EXTERIOR)
            .map(|e| Occluder {
                name: &e.name,
                id: &e.id,
                origin_id: None,
                geometry: &e.geometry,
            })
            .collect();
        occluders.extend(self.shades.iter().map(|e| Occluder {
            name: &e.name,
            id: &e.id,
            origin_id: None,
            geometry: &e.geometry,
        }));
        occluders.extend(setback_shades.iter().map(|(wid, e)| Occluder {
            name: &e.name,
            id: &e.id,
            origin_id: Some(wid),
            geometry: &e.geometry,
        }));
        occluders
    }
}

/// Elemento oclusor, con información geométrica e identificación
///
/// - el id permite excluir el muro de un hueco
/// - el origin_id permite excluir las geometrías de retranqueo que no son del hueco analizado
pub struct Occluder<'a> {
    /// Nombre del elemento
    name: &'a String,
    /// Id del elemento
    id: &'a String,
    /// Id del elemento que genera este oclusor (si proviene de otro elemento, como sombras de retranqueos de huecos)
    origin_id: Option<&'a String>,
    /// Información geométrica
    geometry: &'a Geometry,
}

/// Calcula los puntos de origen en el hueco para el cálculo de fracción sombreada
///
/// Parte de una retícula de 10x10 bloques, para un 1% de cobertura por bloque
/// Potenciales mejoras:
/// - afinar el valor de N=10 según tamaño y proporción del hueco
///     - (p.e. para que sean más o menos cuadradas las celdas)
///       aunque se pierda precisión en huecos pequeños la resolución sería similar en ambas direcciones
///     - evitar bloque < 0.1 x 0.1 m
/// - en cada rectángulo el punto de muestreo podría ser aleatorio y no el punto central
fn ray_origins_for_window(wall: &Wall, window: &Window) -> Vec<Point3<f32>> {
    // Situamos NxN puntos en el plano del muro
    const N: usize = 10;
    let wg = &window.geometry;

    let (x, y) = match wg.position {
        Some(p) => (p.x, p.y),
        // Sin definición geométrica de hueco devolvemos una lista vacía de puntos
        _ => return Vec::new(),
    };

    // Conversión a coordenadas globales desde coordenadas de muro
    // Conversión de coordenadas locales de muro a coordenadas de polígono de muro
    let (to_global_tr, to_poly_tr) = match (
        wall.geometry.local_to_global(),
        wall.geometry.local_to_polygon(),
    ) {
        (Some(to_global), Some(to_poly)) => (to_global, to_poly),
        // Sin definición geométrica del hueco devolvemos una lista vacía de puntos
        _ => return Vec::new(),
    };

    // Puntos 2D del centro de cada bloque en el plano del muro
    let stepX = wg.width / N as f32;
    let stepY = wg.height / N as f32;
    let mut points = vec![];
    for j in 0..N {
        for i in 0..N {
            let px = x + (i as f32 + 0.5) * stepX;
            let py = y + (j as f32 + 0.5) * stepY;
            points.push(point![px, py]);
        }
    }

    // Puntos 3D en el plano de retranqueo
    points
        .iter()
        .map(|p| to_poly_tr * p)
        .map(|p| to_global_tr * point![p.x, p.y, -wg.setback])
        .collect()
}
