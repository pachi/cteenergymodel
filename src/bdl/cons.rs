//! Parser del Building Description Language (BDL) de DOE
//!
//! Composiciones constructivas y de materiales
//!
//! - Acristalamiento (GLASS-TYPE)
//! - Marco (NAME-FRAME)
//!
//! Configuran la base de datos de materiales por familias y dentro de ellas, por grupos:
//!
//! BBDD
//!
//! - Opacos
//!      - Materiales y productos (MATERIAL (tipo PROPERTIES o RESISTANCE))
//!      - Cerramientos y particiones (LAYERS)
//! - Semitransparentes
//!      - Vidrios (GLASS-TYPE)
//!      - Marcos (NAME-FRAME)
//!      - Composición de huecos y lucernarios (GAP)
//! - Puentes térmicos (THERMAL-BRIDGE)?

use std::collections::HashMap;

use super::frame::Frame;
use super::gap::Gap;
use super::glass::Glass;
use super::layers::Layers;
use super::material::Material;
use super::thermalbridge::ThermalBridge;

/// Elementos constructivos y de materiales pertenecientes a la base de datos
/// Se organizan por nombre y grupo (tipo)
#[derive(Debug, Default)]
pub struct DB {
    /// Material o producto
    pub materials: HashMap<String, Material>,
    /// Composición por capas (opacos)
    pub layers: HashMap<String, Layers>,
    /// Composición por capas (huecos)
    pub windows: HashMap<String, Gap>,
    /// Vidrio
    pub glasses: HashMap<String, Glass>,
    /// Marco
    pub frames: HashMap<String, Frame>,
    /// Puente térmico
    pub tbridges: HashMap<String, ThermalBridge>,
}
