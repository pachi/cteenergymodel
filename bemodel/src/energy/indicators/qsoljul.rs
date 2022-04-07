// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación del cálculo de la U de una composión constructiva de opaco, según su posición
//! - UNE-EN ISO 13789:2010 transmisión general
//! - UNE-EN ISO 6946:2012 para elementos opacos
//! - UNE-EN ISO 13770:2017 para elementos en contacto con el terremo
#![allow(non_snake_case)]

use std::collections::HashMap;

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

use crate::{energy::EnergyProps, Orientation};

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

impl QSolJulData {
    /// Calcula el parámetro de control solar (q_sol;jul) a partir de los datos de radiación total acumulada en julio
    /// Los huecos para los que no está definido su factor de obstáculos remotos, transmitancia total con protecciones solares
    /// activadas o fracción de marco se calculan con los valores por defecto:
    /// f_f = 0.20 (DCT), g_glshwi=g_glwi=0.90 * 0.85 = 0.77 (vidrio sencillo), f_shobst=1.0 (sin obstrucciones)
    pub fn from(props: &EnergyProps, totradjul: &HashMap<Orientation, f32>) -> Self {
        let mut q_soljul_data = QSolJulData::default();

        let mut Q_soljul = 0.0;
        for (win_id, win) in props.windows.iter().filter(|(_, w)| w.is_ext_or_gnd_tenv) {
            let orientation = win.orientation;
            let radjul = *totradjul.get(&orientation).unwrap();
            let area = win.area * win.multiplier;
            // Si no hay construcción o no está bien definida se usan valores por defecto
            // f_f = 0.20 (DCT), g_glshwi=g_glwi=0.90 * 0.85 (vidrio sencillo) = 0.77
            let (g_glshwi, f_f) = if let Some(wincons) = props.wincons.get(&win.cons) {
                (wincons.g_glshwi, wincons.f_f)
            } else {
                warn!("No se ha definido la construcción {} para el hueco {}. Se usarán valores por defecto para g_glsh, g_glshwi y F_f", win.cons, win_id);
                (0.77, 0.20)
            };
            // Si no hay definido un valor de usuario se usa el valor calculado o el valor por defecto f_shobst=1.0
            let f_shobst = win.f_shobst.or(win.f_shobst_calc).unwrap_or(1.0);

            let Q_soljul_orient = f_shobst * g_glshwi * (1.0 - f_f) * area * radjul;
            // Datos de detalle
            let mut detail = q_soljul_data.detail.entry(orientation).or_default();
            detail.a += area;
            detail.gains += Q_soljul_orient;
            detail.irradiance = radjul;
            detail.f_f_mean += f_f * area;
            detail.gglshwi_mean += g_glshwi * area;
            detail.fshobst_mean += f_shobst * area;
            // Valores medios y acumulados
            q_soljul_data.a_wp += area;
            q_soljul_data.irradiance_mean += radjul * area;
            q_soljul_data.fshobst_mean += f_shobst * area;
            q_soljul_data.gglshwi_mean += g_glshwi * area;
            q_soljul_data.f_f_mean += f_f * area;
            debug!(
                    "qsoljul de {}: A {:.2}, orient {}, ff {:.2}, gglshwi {:.2}, fshobst {:.2}, H_sol;jul {:.2}",
                    win_id, area, orientation, f_f, g_glshwi, f_shobst, radjul
                );
            Q_soljul += Q_soljul_orient
        }

        let a_ref = props.global.a_ref;
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
}
