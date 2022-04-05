// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación del cálculo de la U de una composión constructiva de opaco, según su posición
//! - UNE-EN ISO 13789:2010 transmisión general
//! - UNE-EN ISO 6946:2012 para elementos opacos
//! - UNE-EN ISO 13770:2017 para elementos en contacto con el terremo
#![allow(non_snake_case)]

use std::{collections::BTreeMap, collections::HashMap, convert::From};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

use climate::{nday_from_md, radiation_for_surface, SolarRadiation};

use crate::{
    climatedata::{RadData, CLIMATEMETADATA, JULYRADDATA},
    energy::{Bounded, Intersectable, Ray, BVH},
    point,
    types::HasSurface,
    utils::fround2,
    vector,
    BoundaryType::{ADIABATIC, EXTERIOR},
    Model, Orientation, Point3, Uuid, Vector3, Window,
};

use super::occluder::Occluder;

/// Reporte de cálculo del parámetro de control solar q_sol:jul (HE2019)
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QSolJulData {
    /// Parámetro de control solar q_sol:jul [kWh/m²·mes]
    pub q_soljul: f32,
    /// Ganancias para el mes de julio (Q_soljul) [kWh/mes]
    pub Q_soljul: f32,
    /// Superficie total de huecos [m²]
    pub a_wp: f32,
    /// Irradiación solar acumulada, media ponderada por superficie de huecos [kWh/m²·mes]
    pub irradiance_mean: f32,
    /// Factor de obstáculos remoto, media ponderada por superficie de huecos [-]
    pub fshobst_mean: f32,
    /// Factor solar del hueco con los elementos de sombra activados, media ponderada por superficie de huecos [-]
    pub gglshwi_mean: f32,
    /// Fracción de marco, media ponderada por superficie de huecos [-]
    pub f_f_mean: f32,
    /// Datos de ganancias solares (Q_soljul) resumidos por orientaciones
    pub detail: HashMap<Orientation, QSolJulDetail>,
}

/// Detalles del parámetro de control solar q_sol:jul (HE2019) por orientación
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct QSolJulDetail {
    /// Ganancias para el mes de julio (Q_soljul) para la orientación [kWh/mes]
    pub gains: f32,
    /// Superficie de huecos por orientación [m²]
    pub a: f32,
    /// Irradiación solar acumulada en el mes de julio (H_sol;jul) para la orientación [kWh/m²·mes]
    pub irradiance: f32,
    /// Fracción de marco media de la orientación, ponderada por superficie de huecos [-]
    pub f_f_mean: f32,
    /// Factor solar con sombras móviles activadas medio de la orientación, ponderada por superficie de huecos [-]
    pub gglshwi_mean: f32,
    /// Factor de obstáculos remotos medio de la orientación, ponderado por superficie de huecos [-]
    pub fshobst_mean: f32,
}

impl Model {
    /// Calcula el parámetro de control solar (q_sol;jul) a partir de los datos de radiación total acumulada en julio
    /// Los huecos para los que no está definido su opaco o su construcción no se consideran en el cálculo
    pub fn q_soljul(&self, totradjul: &HashMap<Orientation, f32>) -> QSolJulData {
        let mut q_soljul_data = QSolJulData::default();

        let Q_soljul = self
            .windows_of_envelope_iter()
            .filter_map(|w| {
                let wall = self.get_wall(w.wall)?;
                let multiplier = self
                .get_space(wall.space)
                .map(|s| s.multiplier)
                .unwrap_or(1.0);
                let wincons = self.cons.get_wincons(w.cons)?;
                let orientation = Orientation::from(wall);
                let radjul = totradjul.get(&orientation).unwrap();
                let area = w.area() * multiplier;
                let gglshwi = wincons.g_glshwi(&self.mats)?;
                let Q_soljul_orient = w.f_shobst * gglshwi * (1.0 - wincons.f_f) * area * radjul;
                // Datos de detalle
                let mut detail = q_soljul_data.detail.entry(orientation).or_default();
                detail.a += area;
                detail.gains += Q_soljul_orient;
                detail.irradiance = *radjul;
                detail.f_f_mean += wincons.f_f * area;
                detail.gglshwi_mean += gglshwi * area;
                detail.fshobst_mean += w.f_shobst * area;
                // Valores medios y acumulados
                q_soljul_data.a_wp += area;
                q_soljul_data.irradiance_mean += *radjul * area;
                q_soljul_data.fshobst_mean += w.f_shobst * area;
                q_soljul_data.gglshwi_mean += gglshwi * area;
                q_soljul_data.f_f_mean += wincons.f_f * area;
                debug!(
                    "qsoljul de {}: A {:.2}, orient {}, ff {:.2}, gglshwi {:.2}, fshobst {:.2}, H_sol;jul {:.2}",
                    w.name, area, orientation, wincons.f_f, gglshwi, w.f_shobst, radjul
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
        q_soljul_data.f_f_mean /= q_soljul_data.a_wp;

        // Completa cálcula de medias por orientación (dividiendo por area de cada orientación)
        for (_, detail) in q_soljul_data.detail.iter_mut() {
            detail.f_f_mean /= detail.a;
            detail.gglshwi_mean /= detail.a;
            detail.fshobst_mean /= detail.a;
        }

        q_soljul_data
    }

    /// Recalcula los factores de obstáculos remotos para los huecos
    ///
    /// Considera el sombreamiento de elementos de muro y sombra sobre el hueco
    /// Toma la zona climática del modelo y usa los datos del 1 de julio para los cálculos
    /// Calcula únicamente la radiación directa bloqueada, y asume factores de visibilidad fijos
    /// sin calcularlos a partir de la visión del cielo o el terreno y las reflexiones.
    /// Por esto, tiende a sobreestimar el valor respecto a un método con backwards raytracing completo.
    pub fn update_fshobst(&mut self) {
        let occluders = self.collect_occluders();

        /// Estructura interna de datos para el soporte del cálculo de fshobst de huecos
        #[derive(Default, Debug)]
        struct ObstData {
            /// Fracción de obstrucción de radiación directa (fracción soleada del hueco) para cada hora
            fshdir: Vec<f32>,
            /// Radiación directa en el plano del hueco para cada hora, W/m²
            dir: Vec<f32>,
            /// Radiación difusa en el plano del hueco para cada hora, W/m²
            dif: Vec<f32>,
            /// Factor de obstáculos remotos (sobre radiación total), ponderado por horas
            fshobst: f32,
        }
        let mut map: BTreeMap<Uuid, ObstData> = BTreeMap::new();

        let latitude = CLIMATEMETADATA
            .lock()
            .unwrap()
            .get(&self.meta.climate)
            .unwrap()
            .latitude;
        let julyraddata = JULYRADDATA.lock().unwrap();
        let raddata = match julyraddata.get(&self.meta.climate) {
            Some(data) => data,
            None => return,
        };
        for window in &self.windows {
            // if window.name != "P01_E01_PE004_V" {continue};
            let window_wall = match self.get_wall(window.wall) {
                None => continue,
                Some(wall) => wall,
            };
            let ray_origins = self.ray_origins_for_window(window);
            for d in raddata {
                let RadData {
                    month,
                    day,
                    hour,
                    azimuth,
                    altitude,
                    dir,
                    dif,
                    ..
                } = *d;
                let ray_dir = ray_dir_to_sun(azimuth, altitude);
                let nday = nday_from_md(month, day);
                let rad_on_win = radiation_for_surface(
                    nday,
                    hour,
                    SolarRadiation { dir, dif },
                    latitude,
                    window_wall.geometry.tilt,
                    window_wall.geometry.azimuth,
                    0.2,
                );
                let fshdir = self.sunlit_fraction(window, &ray_origins, &ray_dir, &occluders);
                let windata = map.entry(window.id).or_default();
                windata.fshdir.push(fshdir);
                windata.dir.push(rad_on_win.dir);
                windata.dif.push(rad_on_win.dif);
            }
        }
        map.values_mut().for_each(|d| {
            let nvalues = d.fshdir.len();
            let mut fshobst_sum = 0.0;
            for i in 0..nvalues {
                let fshobst_i = (d.fshdir[i] * d.dir[i] + d.dif[i]) / (d.dir[i] + d.dif[i]);
                fshobst_sum += fshobst_i
            }
            d.fshobst = fshobst_sum / nvalues as f32;
        });
        debug!("Fshobst map: {:#?}", map);

        for mut window in &mut self.windows {
            window.f_shobst = map
                .get(&window.id)
                .map(|v| fround2(v.fshobst))
                .unwrap_or(1.0);
        }
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
        ray_origins: &[Point3],
        ray_dir: &Vector3,
        occluders: &[Occluder],
    ) -> f32 {
        let window_wall = match self.get_wall(window.wall) {
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
        if window_wall.geometry.normal().dot(ray_dir) < 0.01 {
            return 0.0;
        }

        let candidate_occluders: Vec<_> = occluders
            .iter()
            .filter(|oc| {
                // Descartamos el muro al que pertenece el hueco
                if oc.id == window_wall.id {
                    return false;
                };
                // Descartamos las sombras de retranqueo que no provienen del hueco
                if let Some(id) = &oc.linked_to_id {
                    if *id != window.id {
                        return false;
                    };
                };
                true
            })
            .collect();

        let rays = ray_origins.iter().map(|origin| Ray::new(*origin, *ray_dir));
        let num_rays = rays.len();
        let mut num_intersects = 0;

        let bvh = BVH::build(candidate_occluders, 30);
        for ray in rays {
            if bvh.intersects(&ray).is_some() {
                num_intersects += 1;
            }
        }

        1.0 - num_intersects as f32 / num_rays as f32
    }

    /// Genera lista de elementos oclusores a partir de muros, sombras y sombras de retranqueo
    /// Guarda el nombre del oclusor, su id y la geometría
    pub fn collect_occluders(&self) -> Vec<Occluder> {
        let setback_shades = self.windows_setback_shades();
        let mut occluders: Vec<_> = self
            .walls
            .iter()
            .filter(|&e| e.bounds == ADIABATIC || e.bounds == EXTERIOR)
            .map(|e| Occluder {
                id: e.id,
                linked_to_id: None,
                normal: e.geometry.polygon.normal(),
                trans_matrix: e.geometry.to_global_coords_matrix().map(|m| m.inverse()),
                polygon: e.geometry.polygon.clone(),
                aabb: e.geometry.aabb(),
            })
            .collect();
        occluders.extend(self.shades.iter().map(|e| Occluder {
            id: e.id,
            linked_to_id: None,
            normal: e.geometry.polygon.normal(),
            trans_matrix: e.geometry.to_global_coords_matrix().map(|m| m.inverse()),
            polygon: e.geometry.polygon.clone(),
            aabb: e.geometry.aabb(),
        }));
        occluders.extend(setback_shades.iter().map(|(wid, e)| Occluder {
            id: e.id,
            linked_to_id: Some(*wid),
            normal: e.geometry.polygon.normal(),
            trans_matrix: e.geometry.to_global_coords_matrix().map(|m| m.inverse()),
            polygon: e.geometry.polygon.clone(),
            aabb: e.geometry.aabb(),
        }));
        occluders
    }

    /// Calcula los puntos de origen en el hueco para el cálculo de fracción sombreada
    ///
    /// Parte de una retícula dividida entre 5 y 10 partes por dimensión
    /// - en cada rectángulo el punto de muestreo podría ser aleatorio y no el punto central
    pub fn ray_origins_for_window(&self, window: &Window) -> Vec<Point3> {
        let wall = match self.get_wall(window.wall) {
            None => return vec![],
            Some(wall) => wall,
        };
        let wg = &window.geometry;

        // Definimos el número de puntos muestreados para que cada dimensión
        // se divida en fragmentos de 20cm aprox, sin pasar de 10 ni menos de 5
        let n_x: usize = 10.min((wg.width / 20.0).round() as usize).max(5);
        let n_y: usize = 10.min((wg.height / 20.0).round() as usize).max(5);

        let (x, y) = match wg.position {
            Some(p) => (p.x, p.y),
            // Sin definición geométrica de hueco devolvemos una lista vacía de puntos
            _ => return Vec::new(),
        };

        // Conversión a coordenadas globales desde coordenadas de muro
        // Conversión de coordenadas locales de muro a coordenadas de polígono de muro
        let (to_global_tr, to_poly_tr) = match (
            wall.geometry.to_global_coords_matrix(),
            wall.geometry.to_polygon_coords_matrix(),
        ) {
            (Some(to_global), Some(to_poly)) => (to_global, to_poly),
            // Sin definición geométrica del hueco devolvemos una lista vacía de puntos
            _ => return Vec::new(),
        };

        // Puntos 2D del centro de cada bloque en el plano del muro
        let stepX = wg.width / n_x as f32;
        let stepY = wg.height / n_y as f32;
        let mut points = vec![];
        for j in 0..n_y {
            for i in 0..n_x {
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
}

/// Vector orientado en la dirección del sol
///
/// sun_azimuth: azimuth solar [-180.0,+180.0] (E+, W-, S=0)
/// sun_altitude: altitud solar [0.0, +90] (90 es vertical)
pub fn ray_dir_to_sun(sun_azimuth: f32, sun_altitude: f32) -> Vector3 {
    let sazim = sun_azimuth.to_radians();
    let salt = sun_altitude.to_radians();
    vector![
        salt.cos() * sazim.sin(),
        -salt.cos() * sazim.cos(),
        salt.sin()
    ]
    .normalize()
}
