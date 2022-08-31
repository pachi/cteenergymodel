// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Composición de cerramiento (CONSTRUCTION)
//!
//! Este es un elemento intermedio que se elimina en el postproceso,
//! ya que su información se almacena en los muros (ABSORPTANCE) o en
//! la construcción de opaco (LAYERS).

use std::convert::TryFrom;

use anyhow::{bail, format_err, Error};

use crate::bdl::BdlBlock;

/// Construcción - Remite a LAYERS (¿y otras opciones?)
#[derive(Debug, Clone, Default)]
pub struct Construction {
    /// Nombre
    pub name: String,
    /// Elemento vinculado (opaco, etc)
    pub parent: String,
    /// Definición de capas (HULC solo admite definición por capas)
    pub layers: String,
    /// Absortividad (a la radiación solar) (-)
    /// Cuando no se defina usamos como valor por defecto 0.60
    pub absorptance: f32,
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
    /// HULC: en muros exteriores el valor por defecto de absortividad es 0.6 (color medio)
    /// (aunque usa, por lo general, en cubiertas 0.7 y en marcos de hueco 0.9)
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
        let layers = attrs.remove_str("LAYERS").map_err(|_| {
            format_err!(
                "No se ha definido la composición de capas de la construcción {}",
                name
            )
        })?;
        let absorptance = attrs.remove_f32("ABSORPTANCE").unwrap_or(0.60);
        let parent = parent.ok_or_else(|| {
            format_err!(
                "No se encuentra la referencia al elemento en la construcción {}",
                name
            )
        })?;
        Ok(Self {
            name,
            parent,
            layers,
            absorptance,
        })
    }
}
