// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos climáticos, modelo del edificio y rutinas para cálculo energético

mod checks;
mod purge;
mod types;

pub mod climatedata;
pub mod convert;
pub mod energy;
pub mod utils;

pub use types::{
    point, vector, BoundaryType, ConsDb, ConsDbGroups, ExtraData, Frame, Glass, Layer, Library,
    MatProps, Material, Meta, Model, Orientation, Point2, Point3, Polygon, PropsOverrides,
    Schedule, ScheduleDay, ScheduleWeek, SchedulesDb, Shade, Space, SpaceLoads, SpaceSysConditions,
    SpaceType, ThermalBridge, ThermalBridgeKind, Tilt, Uuid, Vector2, Vector3, Wall, WallCons,
    WallGeom, WallPropsOverrides, Warning, WarningLevel, WinCons, WinGeom, WinPropsOverrides,
    Window,
};

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
