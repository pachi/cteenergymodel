// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Información energética relativa al modelo
//!
//! Cálculo de K, qsoljul, Fshobst, etc

use serde::{Deserialize, Serialize};
use anyhow::Error;

use crate::{climatedata, Model, Warning};

mod aabb;
mod bvh;
mod geometry;
mod n50;
mod occluder;
mod radiation;
mod ray;
mod transmittance;
mod utils;

pub use aabb::AABB;
pub use bvh::{Bounded, Intersectable, BVH};
pub use n50::N50Data;
pub use radiation::{ray_dir_to_sun, QSolJulData};
pub use ray::Ray;
pub use transmittance::{KData, UValues};

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

impl IndicatorsReport {
    /// Devuelve resultados en formato JSON
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }
}

/// Calcula indicadores energéticos del modelo
pub fn indicators(model: &Model) -> IndicatorsReport {
    let climatezone = model.meta.climate;
    let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
    IndicatorsReport {
        area_ref: model.a_ref(),
        compacity: model.compacity(),
        vol_env_net: model.vol_env_net(),
        vol_env_gross: model.vol_env_gross(),
        u_values: model.u_values(),
        K_data: model.K(),
        q_soljul_data: model.q_soljul(&totradjul),
        n50_data: model.n50(),
        warnings: model.check(),
    }
}
