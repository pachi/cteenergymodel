// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Información energética relativa al modelo
//!
//! Tipo para la obtención de los indicadores energéticos K, n50, qsoljul, etc

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::KData;
use super::N50Data;
use super::QSolJulData;

use crate::energy::EnergyProps;
use crate::{climatedata, Model, Warning};

/// Estructura que contiene los resultados del cálculo de indicadores y parámetros energéticos
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyIndicators {
    pub area_ref: f32,
    pub compacity: f32,
    pub vol_env_net: f32,
    pub vol_env_gross: f32,
    pub props: EnergyProps,
    pub K_data: KData,
    pub q_soljul_data: QSolJulData,
    pub n50_data: N50Data,
    pub warnings: Vec<Warning>,
}

impl EnergyIndicators {
    /// Devuelve resultados en formato JSON
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// Calcula indicadores energéticos del modelo
    pub fn compute(model: &Model) -> Self {
        let climatezone = model.meta.climate;
        let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);

        let props = EnergyProps::from(model);

        Self {
            area_ref: props.global.a_ref,
            compacity: props.global.compacity,
            vol_env_net: props.global.vol_env_net,
            vol_env_gross: props.global.vol_env_gross,

            K_data: KData::from(&props),
            q_soljul_data: QSolJulData::from(&props, &totradjul),
            n50_data: N50Data::from(&props),

            props,

            warnings: model.check(),
        }
    }
}