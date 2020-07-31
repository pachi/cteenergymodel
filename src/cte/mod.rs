// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Módulo que define los tipos del modelo del edificio

pub mod common;
pub mod elements;
pub(crate) mod from_ctehexml;
pub mod impl_u_for_wall;
pub mod model;
pub(crate) mod simplemodel;

pub use common::{Boundaries, Orientation, SpaceType, Tilt};
pub use elements::{Space, ThermalBridge, Wall, WallCons, Window, WindowCons};
pub use model::{ExtraData, Meta, Model};
