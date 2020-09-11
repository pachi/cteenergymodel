//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento de planta (FLOOR)
//!
//! Este elemento agrupa espacios del edificio,
//! define propiedades comunes a los espacios que incluye
//! y referencia la planta inferior.

use std::convert::TryFrom;

use crate::bdl::BdlBlock;

use failure::Error;

/// Planta (agrupación de espacios)
#[derive(Debug, Clone, Default)]
pub struct Floor {
    /// Nombre de la planta
    pub name: String,
    /// Cota de la planta en el sistema coordenado del edificio
    pub z: f32,
    /// Altura suelo a suelo de la planta (incluye plenum y forjados)
    pub height: f32,
    /// Multiplicador de planta
    pub multiplier: f32,
    /// Planta anterior (inferior)
    pub previous: String,
}

impl TryFrom<BdlBlock> for Floor {
    type Error = Error;

    /// Convierte bloque BDL a planta (agrupación de espacios)
    ///
    /// Ejemplo:
    /// ```text
    ///     "P01" = FLOOR
    ///         POLYGON       =  "P01_Poligono1"
    ///         FLOOR-HEIGHT  =            3.5
    ///         SPACE-HEIGHT  =            3.5
    ///         SHAPE         =  POLYGON
    ///         PREVIOUS      =  ""
    ///         ..
    ///     "P02" = FLOOR
    ///         Z             =               3
    ///         POLYGON       =  "P02_Poligono1"
    ///         FLOOR-HEIGHT  =              3
    ///         SPACE-HEIGHT  =              3
    ///         MULTIPLIER    = 12
    ///         SHAPE         =  POLYGON
    ///         PREVIOUS      =  "P01"
    ///         ..
    /// ```
    /// XXX: Atributos no trasladados: FLOOR-HEIGHT, POLYGON, SHAPE
    /// LIDER no usa bien la propiedad SPACE-HEIGHT, que permitiría definir plenum (o reducir la altura de forjados)
    /// sino que la usa como si fuese FLOOR-HEIGHT. HULC pone igual FLOOR-HEIGHT y SPACE-HEIGHT
    /// (la altura de los espacios de tipo PLENUM es floorheight - spaceheight)
    /// XXX: SHAPE y POLYGON no tienen información relevante, solo vale para exportar a BDL
    /// XXX: HULC solo maneja plantas con SHAPE = POLYGON

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        // TODO: Si no se define Z es la del edificio. Por ahora asignamos 0.0
        let z = attrs.remove_f32("Z").unwrap_or_default();
        // Las versiones antiguas de LIDER usan SPACE-HEIGHT y dejan a cero FLOOR-HEIGHT
        // HULC escribe FLOOR-HEIGHT con el mismo valor que SPACE-HEIGHT
        let height = attrs.remove_f32("SPACE-HEIGHT")?;
        let multiplier = attrs.remove_f32("MULTIPLIER").unwrap_or(1.0);
        let previous = attrs.remove_str("PREVIOUS")?;
        Ok(Self {
            name,
            z,
            height,
            multiplier,
            previous,
        })
    }
}
