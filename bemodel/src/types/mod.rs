// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo del edificio que comprende los elementos de la envolvente térmica, espacios, construcciones y metadatos
//!
//! Los elementos usan un identificador único con formato UUID:
//! - https://www.rfc-editor.org/rfc/rfc4122
//! - representación en cadena de 36 caracteres

pub use nalgebra::{point, vector};

mod common;
mod constructions;
mod meta;
mod model;
mod opaques;
mod reporting;
mod space;
mod thermalbridge;
mod window;

use crate::utils::{fround2, uuid_from_str};

pub use common::{BoundaryType, Orientation, Tilt};
pub use constructions::{WallCons, WindowCons};
pub use meta::Meta;
pub use model::{ExtraData, Model};
pub use opaques::{Geometry, Shade, Wall};
pub use reporting::{Warning, WarningLevel};
pub use space::{Space, SpaceType};
pub use thermalbridge::{ThermalBridge, ThermalBridgeKind};
pub use window::{Window, WindowGeometry};

pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
