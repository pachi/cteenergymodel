// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Metadatos del modelo: Meta

use serde::{Deserialize, Serialize};

use crate::climatedata::ClimateZone;

/// Metadatos del edificio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// Nombre del proyecto
    pub name: String,
    /// ¿Edificio nuevo?
    pub is_new_building: bool,
    /// ¿Es uso residencial?
    pub is_dwelling: bool,
    /// Número de viviendas
    pub num_dwellings: i32,
    /// Zona climática
    pub climate: ClimateZone,
    /// Ventilación global del edificio, para los espacios habitables de uso residencial, en l/s
    /// Las zonas no habitables y todas las zonas de uso terciario tienen definida su tasa
    /// de ventilación definida (en renh)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub global_ventilation_l_s: Option<f32>,
    /// n50 medido mediante ensayo [renh]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub n50_test_ach: Option<f32>,
    /// Anchura o profundidad del aislamiento perimetral horizontal o vertical de la solera [m]
    pub d_perim_insulation: f32,
    /// Resistencia térmica del aislamiento perimetral horizontal o vertical de la solera [m2K/W]
    pub rn_perim_insulation: f32,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            name: "Nombre del proyecto".to_string(),
            is_new_building: true,
            is_dwelling: true,
            num_dwellings: 1,
            climate: ClimateZone::D3,
            global_ventilation_l_s: None,
            n50_test_ach: None,
            d_perim_insulation: 0.0,
            rn_perim_insulation: 0.0,
        }
    }
}
