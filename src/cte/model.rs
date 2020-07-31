// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo del edificio que comprende los elementos de la envolvente térmica, espacios, construcciones y  metadatos

use std::collections::BTreeMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::{
    simplemodel::SimpleModel, Boundaries, Space, SpaceType, ThermalBridge, Tilt, Wall, WallCons,
    Window, WindowCons,
};
use crate::utils::fround2;

// ---------- Estructura general de datos --------------

/// Datos adicionales para comprobación
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtraData {
    pub name: String,
    pub bounds: Boundaries,
    pub tilt: Tilt,
    pub cons: String,
    pub u: f32,
    pub computed_u: f32,
}

/// Modelo del edificio
#[serde(into = "SimpleModel", from = "SimpleModel")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Metadatos
    pub meta: Meta,
    /// Huecos
    pub windows: BTreeMap<String, Window>,
    /// Opacos
    pub walls: BTreeMap<String, Wall>,
    /// Puentes térmicos
    pub thermal_bridges: BTreeMap<String, ThermalBridge>,
    /// Espacios
    pub spaces: BTreeMap<String, Space>,
    /// Construcciones de huecos
    pub wincons: BTreeMap<String, WindowCons>,
    /// Construcciones de opacos
    pub wallcons: BTreeMap<String, WallCons>,
    // XXX: Elementos temporalmente almacenados mientras no se pueden calcular correctamente
    pub extra: Option<Vec<ExtraData>>,
}

impl Model {
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// Calcula la superficie útil [m²]
    /// Computa únicamente los espacios habitables dentro de la envolvente térmica
    pub fn a_util_ref(&self) -> f32 {
        let a_util: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv && s.space_type != SpaceType::UNINHABITED {
                    s.area * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(a_util)
    }

    /// Calcula el volumen bruto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente
    pub fn vol_env_gross(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv {
                    s.area * s.height_gross * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(v_env)
    }
    /// Calcula el volumen neto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente y
    /// descuenta los volúmenes de forjados y cubiertas
    pub fn vol_env_net(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv {
                    s.area * s.height_net * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(v_env)
    }
    /// Calcula el volumen neto de los espacios habitables de la envolvente [m³]
    /// Computa el volumen de todos los espacios (solo habitables) de la envolvente y
    /// descuenta los volúmenes de forjados y cubiertas
    pub fn vol_env_inh_net(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv && s.space_type != SpaceType::UNINHABITED {
                    s.area * s.height_net * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(v_env)
    }
}

/// Metadatos del edificio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// ¿Edificio nuevo?
    pub is_new_building: bool,
    /// ¿Es uso residencial?
    pub is_dwelling: bool,
    /// Número de viviendas
    pub num_dwellings: i32,
    /// Zona climática
    pub climate: String,
    /// Ventilación global del edificio, para los espacios habitables de uso residencial, en l/s
    /// Las zonas no habitables y todas las zonas de uso terciario tienen definida su tasa
    /// de ventilación definida (en renh)
    pub global_ventilation_l_s: Option<f32>,
    /// n50 medido mediante ensayo [renh]
    pub n50_test_ach: Option<f32>,
    /// Anchura o profundidad del aislamiento perimetral horizontal o vertical de la solera [m]
    pub d_perim_insulation: f32,
    /// Resistencia térmica del aislamiento perimetral horizontal o vertical de la solera [m2K/W]
    pub rn_perim_insulation: f32,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            is_new_building: true,
            is_dwelling: true,
            num_dwellings: 1,
            climate: "D3".to_string(),
            global_ventilation_l_s: None,
            n50_test_ach: None,
            d_perim_insulation: 0.0,
            rn_perim_insulation: 0.0,
        }
    }
}
