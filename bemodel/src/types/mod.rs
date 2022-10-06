// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo del edificio que comprende los elementos de la envolvente térmica, espacios, construcciones y metadatos
//!
//! Los elementos usan un identificador único con formato UUID:
//! - https://www.rfc-editor.org/rfc/rfc4122
//! - representación en cadena de 36 caracteres

pub use nalgebra::{point, vector};

mod common;
mod constructions;
mod geometry;
mod library;
mod meta;
mod model;
mod opaques;
mod overrides;
mod reporting;
mod schedules;
mod space;
mod space_loads;
mod space_sys_conditions;
mod systems;
mod thermalbridge;
mod window;

use crate::utils::{fround2, uuid_from_str};

pub use common::{BoundaryType, Orientation, Tilt, Uuid};
pub use constructions::{ConsDb, Frame, Glass, Layer, MatProps, Material, WallCons, WinCons};
pub use geometry::{HasSurface, Point2, Point3, Polygon, Vector2, Vector3};
pub use library::{ConsDbGroups, Library};
pub use meta::Meta;
pub use model::{ExtraData, Model};
pub use opaques::{Shade, Wall, WallGeom};
pub use overrides::{PropsOverrides, WallPropsOverrides, WinPropsOverrides};
pub use reporting::{Warning, WarningLevel};
pub use schedules::{Schedule, ScheduleDay, ScheduleWeek, SchedulesDb};
pub use space::{Space, SpaceType};
pub use space_loads::SpaceLoads;
pub use space_sys_conditions::SpaceSysConditions;
pub use systems::{AirFlow, ZoneSystem};
pub use thermalbridge::{ThermalBridge, ThermalBridgeKind};
pub use window::{WinGeom, Window};
