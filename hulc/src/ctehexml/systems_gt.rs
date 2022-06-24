// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE para sistemas GT
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::BTreeMap;

use anyhow::Error;
use log::warn;

pub use crate::bdl::{build_blocks, BdlBlock};
pub use crate::bdl::{extract_f32vec, extract_namesvec, AttrMap};

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Clone, Default)]
pub struct GtSystems {
    /// Sistemas
    pub systems: BTreeMap<String, BdlBlock>,
    /// Zonas térmicas
    pub zones: BTreeMap<String, BdlBlock>,
    /// Equipos
    pub equipment: BTreeMap<String, BdlBlock>,
}

impl GtSystems {
    /// Nuevo modelo a partir de str
    pub fn new<T: AsRef<str>>(input: T) -> Result<Self, Error> {
        let blocks = build_blocks(input.as_ref())?;

        // Resto de elementos
        let mut systems: BTreeMap<String, BdlBlock> = BTreeMap::new();
        let mut zones: BTreeMap<String, BdlBlock> = BTreeMap::new();
        let mut equipment: BTreeMap<String, BdlBlock> = BTreeMap::new();

        for block in blocks {
            match block.btype.as_str() {
                // Elementos generales =========================
                // Valores por defecto, Datos generales, espacio de trabajo y edificio
                "SYSTEM" => {
                    // systems.insert(block.name.clone(), GtSystem::try_from(block)?);
                    systems.insert(block.btype.clone(), block);
                }
                // Horarios ----------
                "ZONE" => {
                    zones.insert(block.name.clone(), block);
                }
                // Condiciones de uso y ocupación ----------
                "PUMP" | "CIRCULATION-LOOP" | "CHILLER" | "DW-HEATER" => {
                    equipment.insert(block.name.clone(), block);
                }
                // Elemento desconocido -------------------------
                _ => {
                    warn!(
                        "Tipo desconocido. bname: {}, btype: {}",
                        block.name, block.btype
                    );
                }
            };
        }

        Ok(Self {
            systems,
            zones,
            equipment,
        })
    }
}
