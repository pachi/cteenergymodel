// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos climáticos y modelo del edificio

mod checks;
mod types;
mod utils;

pub mod climatedata;
pub mod convert;
pub mod energy;

pub use types::{
    point, vector, BoundaryType, ExtraData, Geometry, Meta, Model, Orientation, Point2, Point3,
    Shade, Space, SpaceType, ThermalBridge, ThermalBridgeKind, Tilt, Vector2, Vector3, Wall,
    WallCons, Warning, WarningLevel, Window, WindowCons, WindowGeometry,
};

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
