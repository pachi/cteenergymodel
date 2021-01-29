// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos climáticos y modelo del edificio

pub mod climatedata;
pub mod common;
pub(crate) mod from_ctehexml;
mod model;
pub mod model_impl;
mod utils;

pub use climatedata::*;
pub use common::{
    BoundaryType, ClimateZone, KDetail, N50HEDetail, Orientation, SpaceType, Tilt, UValues,
    Warning, WarningLevel,
};
pub use model::*;

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
