//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos de la envolvente térmica:

use super::{Wall};

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlEnvType {
    Wall(Wall),
    InteriorWall(Wall),
    UndergroundWall(Wall),
    Roof(Wall),
}
