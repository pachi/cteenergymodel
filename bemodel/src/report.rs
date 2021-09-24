// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)
#![allow(clippy::upper_case_acronyms)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    climatedata,
    model_n50::N50Data,
    model_qsoljul::QSolJulData,
    model_transmittance::{KData, UValues},
    Model,
};

/// Estructura que contiene los resultados del cálculo de indicadores y parámetros energéticos
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IndicatorsReport {
    pub area_ref: f32,
    pub compacity: f32,
    pub vol_env_net: f32,
    pub vol_env_gross: f32,
    pub u_values: UValues,
    pub K_data: KData,
    pub q_soljul_data: QSolJulData,
    pub n50_data: N50Data,
    pub warnings: Vec<Warning>,
}

/// Nivel de aviso para condiciones de chequeo del modelo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WarningLevel {
    SUCCESS,
    DANGER,
    WARNING,
    INFO,
}

/// Muestra WarningLevel
impl Display for WarningLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use WarningLevel::*;
        let printable = match *self {
            SUCCESS => "SUCCESS",
            DANGER => "DANGER",
            WARNING => "WARNING",
            _ => "INFO",
        };
        write!(f, "{}", printable)
    }
}

/// Convierte str a WarningLevel
impl From<&str> for WarningLevel {
    fn from(level: &str) -> Self {
        match level.to_uppercase().as_str() {
            "SUCCESS" => Self::SUCCESS,
            "DANGER" => Self::DANGER,
            "WARNING" => Self::WARNING,
            _ => Self::INFO,
        }
    }
}

/// Reporte de avisos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    /// Nivel de afectación
    pub level: WarningLevel,
    /// Id del elemento afectado, en su caso
    pub id: Option<String>,
    /// Mensaje del aviso
    pub msg: String,
}

/// Calcula indicadores energéticos del modelo
pub fn energy_indicators(model: &Model) -> IndicatorsReport {
    let climatezone = model.meta.climate;
    let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
    IndicatorsReport {
        area_ref: model.a_ref(),
        compacity: model.compacity(),
        u_values: model.u_values(),
        K_data: model.K(),
        q_soljul_data: model.q_soljul(&totradjul),
        n50_data: model.n50(),
        vol_env_net: model.vol_env_net(),
        vol_env_gross: model.vol_env_gross(),
        warnings: model.check(),
    }
}
