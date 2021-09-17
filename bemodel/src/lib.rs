// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos climáticos y modelo del edificio

extern crate nalgebra as na;
pub use nalgebra::{point, vector};

pub mod bvh;
pub mod climatedata;
pub mod common;
pub(crate) mod from_ctehexml;
pub mod geometry;
mod model;
mod model_check;
mod model_impl;
mod model_n50;
mod model_qsoljul;
mod model_transmittance;
mod ray;
mod report;
mod utils;

pub use climatedata::*;
pub use common::{BoundaryType, ClimateZone, Orientation, SpaceType, ThermalBridgeKind, Tilt};
pub use model::*;
pub use ray::Ray;
pub use report::energy_indicators;
pub use model_qsoljul::ray_dir_to_sun;

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
