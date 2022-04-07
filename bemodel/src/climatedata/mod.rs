// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos generales de zonas climáticas (latitud, longitud de referencia, nombre, etc)
//! Datos de radiación mensuales para superficies
//! Datos de radiación horaria por zona climática para el 21 de julio
//! Criterios de orientación UNE-EN ISO 52016-1, (S=0, E=+90, W=-90)
#![allow(clippy::approx_constant)]

use std::collections::HashMap;

use crate::Orientation;

mod climatezone;
mod hourlyraddata;
mod monthlyraddata;
mod zonesmeta;

pub use climatezone::ClimateZone;
pub use hourlyraddata::{RadData, JULYRADDATA};
pub use monthlyraddata::MONTHLYRADDATA;
pub use zonesmeta::CLIMATEMETADATA;

/// Diccionario con el valor de la radiación total por orientación para el mes de julio
pub fn total_radiation_in_july_by_orientation(climate: &ClimateZone) -> HashMap<Orientation, f32> {
    MONTHLYRADDATA
        .lock()
        .unwrap()
        .iter()
        .filter(|e| &e.zone == climate)
        .map(|e| (e.orientation, e.dir[6] + e.dif[6]))
        .collect()
}
