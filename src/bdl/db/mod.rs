//! Parser del Building Description Language (BDL) de DOE
//!
//! ## Base de datos de materiales y composiciones de elementos de la envolvente
//!
//! Los materiales se organizan por familias, dentro de ellas, por grupos:
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

mod construction;
mod frame;
mod gap;
mod glass;
mod layers;
mod material;
mod thermalbridge;

pub use construction::Construction;
pub use frame::Frame;
pub use gap::Gap;
pub use glass::Glass;
pub use layers::Layers;
pub use material::Material;
pub use thermalbridge::ThermalBridge;

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
