//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento de planta (FLOOR)
//! 
//! Este elemento agrupa espacios del edificio,
//! define propiedades comunes a los espacios que incluye
//! y referencia la planta inferior.

use std::convert::TryFrom;

use super::blocks::BdlBlock;

use failure::bail;
use failure::Error;

/// Planta (agrupación de espacios)
#[derive(Debug, Clone, Default)]
pub struct Floor {
    /// Nombre de la planta
    pub name: String,
    /// Cota de la planta en el sistema coordenado del edificio
    /// XXX: podría no aparecer y ser la de la del edificio
    pub z: f32,
    /// Polígono que define la geometría
    pub polygon: String,
    /// Altura suelo a suelo de la planta (incluye plenum y forjados)
    pub height: f32,
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
    ///     POLYGON       =  "P01_Poligono1"
    ///     FLOOR-HEIGHT  =            3.5
    ///     SPACE-HEIGHT  =            3.5
    ///     SHAPE         =  POLYGON
    ///     PREVIOUS      =  "Ninguna"
    ///     ..
    /// ```
    /// XXX: Atributos no trasladados: SPACE-HEIGHT
    /// HULC no usa esta propiedad, que permitiría definir plenum (o reducir la altura de forjados)
    /// (la altura de los espacios de tipo PLENUM es floorheight - spaceheight)

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        // De los tipos de definición de la geometría: POLYGON, BOX, NO-SHAPE
        // solamente manejamos definiciones por polígono
        if attrs.remove_str("SHAPE")? != "POLYGON" {
            bail!("Planta '{}' de tipo distinto a POLYGON, no soportado");
        };
        let z = attrs.remove_f32("Z").unwrap_or_default();
        let polygon = attrs.remove_str("POLYGON")?;
        let height = attrs.remove_f32("FLOOR-HEIGHT")?;
        let previous = attrs.remove_str("PREVIOUS")?;
        Ok(Self {
            name,
            z,
            polygon,
            height,
            previous,
        })
    }
}
