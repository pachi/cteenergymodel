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
mod geometry;
mod materials;
mod meta;
mod model;
mod opaques;
mod reporting;
mod space;
mod thermalbridge;
mod window;

use crate::utils::{fround2, uuid_from_str};

pub use common::{BoundaryType, Orientation, Tilt, Uuid};
pub use constructions::{ConsDb, Layer, WallCons, WindowCons};
pub use geometry::{HasSurface, Point2, Point3, Polygon, Vector2, Vector3};
pub use materials::{Frame, Glass, MatProps, Material, MatsDb};
pub use meta::Meta;
pub use model::{ExtraData, Model};
pub use opaques::{Shade, Wall, WallGeometry};
pub use reporting::{Warning, WarningLevel};
pub use space::{Space, SpaceType};
pub use thermalbridge::{ThermalBridge, ThermalBridgeKind};
pub use window::{Window, WindowGeometry};
