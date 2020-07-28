//! Parser del Building Description Language (BDL) de DOE
//!
//! Definición de acristalamiento de hueco (GLASS-TYPE)

use failure::Error;
use std::convert::TryFrom;

use crate::bdl::BdlBlock;

/// Vidrio (GLASS-TYPE)
#[derive(Debug, Clone, Default)]
pub struct Glass {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Conductividad W/m2K (GLASS-CONDUCTANCE)
    pub conductivity: f32,
    /// Factor solar del vidrio a incidencia normal - (SHADING-COEF / 0.86)
    pub g_gln: f32,
}

impl TryFrom<BdlBlock> for Glass {
    type Error = Error;

    /// Conversión de bloque BDL a vidrio (GLASS-TYPE)
    ///
    /// Conductividad en GLASS-CONDUCTANCE (W/m2K)
    /// Factor solar (g) en SHADING-COEF * 0.85 (-)
    ///
    /// Ejemplo:
    /// ```text
    ///      "Vidrio Triple Bajo Emisivo" = GLASS-TYPE
    ///           GROUP             = "Vidrios HULC2020"
    ///           TYPE              = SHADING-COEF
    ///           SHADING-COEF      =      0.5882353
    ///           GLASS-CONDUCTANCE =           1.25
    ///           NAME_CALENER      = ""
    ///           LIBRARY       =  NO
    ///           UTIL          =  NO
    ///           ..
    ///      $ LIDER antiguo
    ///      "GT_referencia-3" = GLASS-TYPE
    ///           TYPE = SHADING-COEF
    ///           SHADING-COEF = 0
    ///           SHADING-COEF-SUMMER = 0
    ///           GLASS-CONDUCTANCE = 3.5
    ///          ..        
    /// ```
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        if attrs.remove_str("TYPE")? != "SHADING-COEF" {
            bail!(
                "Definición de vidrio por código no soportada en '{}'",
                &name
            );
        };
        // LIDER antiguo no guardaba el grupo
        let group = attrs.remove_str("GROUP").unwrap_or_else(|_| "Vidrios".to_string());
        let conductivity = attrs.remove_f32("GLASS-CONDUCTANCE")?;
        // El SHADING-COEF es SGHC/SGHC_ref donde:
        // - SGHC_ref = 0.86 (vidrio claro) (a veces se indica 0.87)
        // - SGHC es el factor solar del vidrio a incidencia normal
        // A nosotros nos interesa covertir este valor a g_gln,
        // y por tanto debemos multiplicar por 0.86
        let g_gln = attrs.remove_f32("SHADING-COEF")? * 0.86;
        Ok(Self {
            name,
            group,
            conductivity,
            g_gln,
        })
    }
}
