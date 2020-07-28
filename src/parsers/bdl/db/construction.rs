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
//! Composición de cerramiento (CONSTRUCTION)
//!
//! Este es un elemento intermedio que se elimina en el postproceso,
//! ya que su información se almacena en los muros (ABSORPTANCE) o en
//! la construcción de muro (LAYERS).

use failure::Error;
use std::convert::TryFrom;

use crate::bdl::BdlBlock;

/// Construcción - Remite a LAYERS (¿y otras opciones?)
#[derive(Debug, Clone, Default)]
pub struct Construction {
    /// Nombre
    pub name: String,
    /// Elemento vinculado (muro, etc)
    pub parent: String,
    /// Definición de capas (HULC solo admite definición por capas)
    pub wallcons: String,
    /// Absortividad (a la radiación solar) (-)
    pub absorptance: Option<f32>,
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
    /// HULC solo usa construcciones definidas por capas (LAYERS) y únicamente permite
    /// definir la absortividad (ABSORPTANCE)
    ///
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            mut attrs,
            parent,
            ..
        } = value;
        // Tipo de definición de la construcción (LAYERS o U-VALUE)
        if attrs.remove_str("TYPE")? != "LAYERS" {
            bail!("Construcción {} no definida por capas (LAYERS)", name);
        }
        let wallcons = attrs.remove_str("LAYERS").map_err(|_| {
            format_err!(
                "No se ha definido la composición de capas de la construcción {}",
                name
            )
        })?;
        let absorptance = attrs.remove_f32("ABSORPTANCE").ok();
        let parent = parent.ok_or_else(|| {
            format_err!(
                "No se encuentra la referencia al elemento en la construcción {}",
                name
            )
        })?;
        Ok(Self {
            name,
            parent,
            wallcons,
            absorptance,
        })
    }
}
