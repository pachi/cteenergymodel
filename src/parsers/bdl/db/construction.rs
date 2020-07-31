// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Composición de cerramiento (CONSTRUCTION)
//!
//! Este es un elemento intermedio que se elimina en el postproceso,
//! ya que su información se almacena en los muros (ABSORPTANCE) o en
//! la construcción de muro (LAYERS).

use std::convert::TryFrom;

use anyhow::{bail, format_err, Error};

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
