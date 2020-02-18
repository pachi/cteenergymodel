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

use failure::Error;
use std::collections::HashMap;

mod construction;
mod frame;
mod gap;
mod glass;
mod layers;
mod material;

pub use construction::Construction;
pub use frame::Frame;
pub use gap::Gap;
pub use glass::Glass;
pub use layers::Layers;
pub use material::Material;

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
}

impl DB {
    /// Espesor total de una composición de capas [m]
    pub fn get_layers_thickness(&self, name: &str) -> Option<f32> {
        self.layers
            .get(name)
            .and_then(|layers| Some(layers.thickness.iter().sum()))
    }

    /// Transmitancia térmica de una composición de capas [W/m2K]
    pub fn get_layers_transmittance(&self, name: &str) -> Result<f32, Error> {
        let layers = self
            .layers
            .get(name)
            .ok_or_else(|| format_err!("No se encuentra la composición de capas \"{}\"", name))?;

        let materials = layers
            .material
            .iter()
            .map(|m| {
                self.materials.get(m).ok_or_else(|| {
                    format_err!(
                        "No se encuentra el material \"{}\" de la composición de capas \"{}\"",
                        m,
                        name
                    )
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        materials
            .iter()
            .zip(&layers.thickness)
            // Resistencias térmicas de las capas
            .map(|(mat, thk)| match mat.properties {
                Some(props) if props.conductivity != 0.0 => Some(thk / props.conductivity),
                None => mat.resistance,
                _ => None,
            })
            // Resistencia térmica total
            .try_fold(0.0_f32, |acc, x| x.and_then(|res| Some(res + acc)))
            // Transmitancia térmica
            .and_then(|resvec| {
                if resvec != 0.0 {
                    Some(1.0 / resvec)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                format_err!(
                    "Error al calcular la transmitancia de la composición \"{}\"",
                    name
                )
            })
    }
}
