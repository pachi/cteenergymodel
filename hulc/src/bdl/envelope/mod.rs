// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Elementos geométricos y de la envolvente del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

mod floor;
mod geom;
mod shadings;
mod space;
mod thermalbridge;
mod walls;
mod window;

pub use floor::Floor;
pub use geom::*;
pub use shadings::Shade;
pub use space::Space;
pub use thermalbridge::ThermalBridge;
pub use walls::{BoundaryType, Tilt, Wall};
pub use window::Window;
