// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos climáticos y modelo del edificio

mod model;
mod checks;
mod reporting;
mod utils;

pub mod climatedata;
pub mod convert;
pub mod energy;

pub use model::*;
pub use reporting::{Warning, WarningLevel};

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
