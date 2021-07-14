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
use na::{point, vector, Point2, Point3};

use super::{
    BoundaryType::{ADIABATIC, EXTERIOR},
    Model, Orientation, QSolJulData, Window, WindowGeometry,
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
    /// Devuelve 1.0 (sin obstrucción) para definición geométrica incompleta (sin posición o hueco sin muro)
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
        if geometry.position.is_none() {
            warn!(
                "Hueco {} (id: {}) sin definición geométrica completa. Se considera superficie soleada al 100%",
                window.name, window.id
            );
            return 1.0;
        };

        // Conversión a coordenadas globales
        let wallTransform = geometry.local_to_global().unwrap(); //transformMatrix(*tilt, *azimuth, position.unwrap());
                                                                 // Conversión de coordenadas locales de muro a coordenadas de polígono de muro
        let wallLocal2WallPolyTransform = geometry.local_to_polygon().unwrap(); //wallLocal2WallPolygon(polygon);

        // Compute ray origin points on window for shading tests
        let points: Vec<Point3<f32>> = getWindowPoints(&window)
            .iter()
            .map(|p| wallLocal2WallPolyTransform * p)
            .map(|p| wallTransform * point![p.x, p.y, 0.0])
            .collect();

        // Elementos oclusores, muros y sombras
        // - Debe excluir el muro del hueco para tener en cuenta retranqueos y evitar problemas numéricos
        // Optimizaciones implementadas:
        // - no buscar en muros interiores ni en contacto con el terreno (hecho)
        // TODO: Posibles optimizaciones
        // - agrupar elementos por zona / planta y localizar primero si se produce la intersección por zona / planta con AABB (estructura BVH)
        //      - precalcular AABB de zonas
        // - ignorar muros no enfrentados (producto escalar de normales > 0) al hueco, ya que no se ven
        //      - precalcular normales de muros?
        // - probar en primer lugar el opaco con el que se produjo la colisión en la última iteración
        let mut occluders: Vec<_> = self
            .walls
            .iter()
            // Exceptuamos el muro del hueco
            .filter(|&e| e.id != winWall.id)
            // Exceptuamos elementos interiores y en contacto con el terreno
            .filter(|&e| e.bounds == ADIABATIC || e.bounds == EXTERIOR)
            .map(|e| (&e.name, &e.geometry))
            .collect();
        let occ_shades = self.shades.iter().map(|e| (&e.name, &e.geometry));
        // TODO: añadir aletas laterales de retranqueos de ventana a las sombras.

        occluders.extend(occ_shades);

        let num = points.len();
        let mut num_intersects = 0;
        for ray_orig in points {
            for (_name, geometry) in &occluders {
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
