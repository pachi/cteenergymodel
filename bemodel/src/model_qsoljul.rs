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
}
