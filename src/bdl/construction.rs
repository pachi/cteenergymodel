//! Parser del Building Description Language (BDL) de DOE
//!
//! Composición de cerramiento (CONSTRUCTION)

use failure::Error;
use std::convert::TryFrom;

use super::BdlBlock;

/// Construcción - Remite a LAYERS (¿y otras opciones?)
#[derive(Debug, Clone, Default)]
pub struct Construction {
    /// Nombre
    pub name: String,
    /// Tipo de definición de la construcción (LAYERS o U-VALUE)
    pub ctype: String,
    /// Elemento vinculado (muro, etc)
    pub parent: String,
    /// Definición de capas, cuando ctype es LAYERS
    pub layers: Option<String>,
    /// Transmitancia, cuando ctype es U-VALUE (W/m2K)
    pub uvalue: Option<f32>,
    /// Absortividad (a la radiación solar) (-)
    pub absorptance: Option<f32>,
    /// Rugosidad (1 a 6)
    pub roughness: Option<f32>,
    /// Nombre de parámetros de muro (WALL-PARAMETERS)
    pub wallparameters: Option<String>,
}

impl TryFrom<BdlBlock> for Construction {
    type Error = Error;

    /// Convierte de bloque BDL a construcción - Remite a LAYERS (¿y otras opciones?)
    ///
    /// Ejemplo:
    /// ```text
    ///     "muro_opaco0.40" =  CONSTRUCTION
    ///     TYPE   = LAYERS  
    ///     LAYERS = "muro_opaco"
    ///     ABSORPTANCE = 0.400000
    ///     ..
    /// ```
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            mut attrs,
            parent,
            ..
        } = value;
        let ctype = attrs.remove_str("TYPE")?;
        let layers = attrs.remove_str("LAYERS").ok();
        let uvalue = attrs.remove_f32("U-VALUE").ok();
        let absorptance = attrs.remove_f32("ABSORPTANCE").ok();
        let roughness = attrs.remove_f32("ROUGHNESS").ok();
        let wallparameters = attrs.remove_str("WALL-PARAMETERS").ok();
        let parent = parent.ok_or_else(|| {
            format_err!(
                "No se encuentra la referencia al elemento en la construcción {}",
                name
            )
        })?;
        Ok(Self {
            name,
            ctype,
            parent,
            layers,
            uvalue,
            absorptance,
            roughness,
            wallparameters,
        })
    }
}
