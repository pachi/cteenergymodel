//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos de la envolvente térmica:

use super::{ExteriorWall, InteriorWall, UndergroundWall, Window};

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlEnvType {
    ExteriorWall(ExteriorWall),
    InteriorWall(InteriorWall),
    UndergroundWall(UndergroundWall),
    Roof(ExteriorWall),
}
