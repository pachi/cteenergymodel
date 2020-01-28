//! Parser del Building Description Language (BDL) de DOE
//!
//! Definición de marco de hueco (NAME-FRAME)

use failure::Error;
use std::convert::TryFrom;

use super::BdlBlock;

/// Marco de hueco (NAME-FRAME)
#[derive(Debug, Clone, Default)]
pub struct Frame {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica, U (W/m2K)
    pub conductivity: f32,
    /// Absortividad del marco, alpha (-)
    pub absorptivity: f32,
    /// Ancho del marco (m)
    pub width: f32,
}

impl TryFrom<BdlBlock> for Frame {
    type Error = Error;

    /// Conversión de bloque BDL a marco de hueco (NAME-FRAME)
    ///
    /// Conductividad en FRAME-CONDUCT (W/m2K)
    /// Absortividad(alpha) en FRAME-ABS (-)
    ///
    /// Ejemplo:
    /// ```text
    ///      "Marco PVC_1" = NAME-FRAME
    ///      GROUP         = "Marcos HULC2020"
    ///      FRAME-WIDTH   =            0.1
    ///      FRAME-CONDUCT =            1.3
    ///      FRAME-ABS     =            0.7
    ///      NAME_CALENER  = ""
    ///      LIBRARY       = NO
    ///      UTIL          =  NO
    ///      ..
    /// ```
    /// TODO: Propiedades no trasladadas: NAME-CALENER, LIBRRARY, UTIL
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let group = attrs.remove_str("GROUP")?;
        let conductivity = attrs.remove_f32("FRAME-CONDUCT")?;
        let absorptivity = attrs.remove_f32("FRAME-ABS")?;
        let width = attrs.remove_f32("FRAME-WIDTH")?;
        Ok(Self {
            name,
            group,
            conductivity,
            absorptivity,
            width,
        })
    }
}
