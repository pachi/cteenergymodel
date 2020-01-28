//! Parser del Building Description Language (BDL) de DOE
//!
//! Composiciones constructivas de cerramientos opacos (LAYERS)

use failure::Error;
use std::convert::TryFrom;

use super::{extract_f32vec, extract_namesvec, BdlBlock};

/// Definición de capas
#[derive(Debug, Clone, Default)]
pub struct Layers {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Lista de nombres de materiales de las capas
    pub material: Vec<String>,
    /// Lista de espesores de las capas ([m, m, ...])
    pub thickness: Vec<f32>,
}

impl TryFrom<BdlBlock> for Layers {
    type Error = Error;

    /// Conversión de bloque BDL a definición de capas
    ///
    /// Ejemplo:
    /// ```text
    ///     "muro_opaco" = LAYERS
    ///         GROUP        = "envolvente"
    ///         NAME_CALENER = ""
    ///         NAME         = "muro_opaco"
    ///         TYPE-DEFINITION = 1
    ///         MATERIAL     = ("Mortero de cemento o cal para albañilería y para revoco/enlucido 1000 < d < 1250","EPS Poliestireno Expandido [ 0.029 W/[mK]]","1/2 pie LP métrico o catalán 80 mm< G < 100 mm","MW Lana mineral [0.031 W/[mK]]","Placa de yeso laminado [PYL] 750 < d < 900")
    ///         THICKNESS = (          0.015,           0.06,          0.115,           0.04,           0.02)
    ///         LIBRARY       =  NO
    ///         UTIL          =  YES
    ///         IMAGE = ""
    ///         DEFAULT = NO
    ///         ..
    ///
    ///     $ LIDER antiguo
    ///     "CONST_referencia-5" = LAYERS
    ///         MATERIAL = ( "PlaquetaREF","MorteroREF","ForjadoREF" )
    ///         THICKNESS = ( 0.015, 0.020, 0.250 )
    ///         ..
    /// ```
    /// TODO: Propiedades de Layers no convertidas:
    /// TODO: IMAGE, NAME_CALENER, LIBRARY, UTIL, TYPE-DEFINITION, DEFAULT
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        // En LIDER antiguo no se guarda el grupo
        let group = attrs.remove_str("GROUP").unwrap_or("Capas".to_string());
        let material = extract_namesvec(attrs.remove_str("MATERIAL")?);
        let thickness = extract_f32vec(attrs.remove_str("THICKNESS")?)?;
        Ok(Self {
            name,
            group,
            material,
            thickness,
        })
    }
}
