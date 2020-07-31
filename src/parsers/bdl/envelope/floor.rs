/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento de planta (FLOOR)
//!
//! Este elemento agrupa espacios del edificio,
//! define propiedades comunes a los espacios que incluye
//! y referencia la planta inferior.

use std::convert::TryFrom;

use anyhow::Error;

use crate::bdl::BdlBlock;

/// Planta (agrupación de espacios)
#[derive(Debug, Clone, Default)]
pub struct Floor {
    /// Nombre de la planta
    pub name: String,
    /// Cota de la planta en el sistema coordenado del edificio
    pub z: f32,
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
        let previous = attrs.remove_str("PREVIOUS")?;
        Ok(Self {
            name,
            z,
            height,
            previous,
        })
    }
}
