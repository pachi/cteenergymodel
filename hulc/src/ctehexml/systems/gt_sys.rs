// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

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

use super::gt_types::*;

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Clone, Default)]
pub struct GtSystems {
    /// Sistemas
    pub systems: BTreeMap<String, BdlBlock>,
    /// Zonas térmicas
    pub zones: BTreeMap<String, GtZone>,
    /// Equipos
    pub equipment: BTreeMap<String, TempEquipment>,
}

/// Datos del archivo BDL
#[derive(Debug, Clone)]
pub enum TempEquipment {
    Block(BdlBlock),
    GtPump(GtPump),
    GtCirculationLoop(GtCirculationLoop),
    GtChiller(GtChiller),
    GtBoiler(GtBoiler),
    GtDwHeater(GtDwHeater),
    GtHeatRejection(GtHeatRejection),
    GtElectricGenerator(GtElectricGenerator),
    GtGroundLoopHx(GtGroundLoopHx),
}

impl GtSystems {
    /// Nuevo modelo a partir de str
    pub fn new<T: AsRef<str>>(input: T) -> Result<Self, Error> {
        let blocks = build_blocks(input.as_ref())?;

        // Resto de elementos
        let mut zones: BTreeMap<String, GtZone> = BTreeMap::new();
        let mut systems: BTreeMap<String, BdlBlock> = BTreeMap::new();
        let mut equipment: BTreeMap<String, TempEquipment> = BTreeMap::new();

        // Para asignar a cada zona el último sistema visto
        let mut last_seen_system: Option<String> = None;

        for block in blocks {
            match block.btype.as_str() {
                // Zonas
                "ZONE" => {
                    let mut zone: GtZone = block.into();
                    zone.system = last_seen_system.clone();
                    zones.insert(zone.name.clone(), zone);
                }
                // Secundarios
                "SYSTEM" => {
                    // systems.insert(block.name.clone(), GtSystem::try_from(block)?);
                    let system = block;
                    last_seen_system = Some(system.name.clone());
                    systems.insert(system.name.clone(), system);
                }
                // Equipos
                "PUMP" => {
                    equipment.insert(block.name.clone(), TempEquipment::GtPump(block.into()));
                }
                "CIRCULATION-LOOP" => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::GtCirculationLoop(block.into()),
                    );
                }
                "CHILLER" => {
                    equipment.insert(block.name.clone(), TempEquipment::GtChiller(block.into()));
                }
                "BOILER" => {
                    equipment.insert(block.name.clone(), TempEquipment::GtBoiler(block.into()));
                }
                "DW-HEATER" => {
                    equipment.insert(block.name.clone(), TempEquipment::GtDwHeater(block.into()));
                }
                "HEAT-REJECTION" => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::GtHeatRejection(block.into()),
                    );
                }
                "ELEC-GENERATOR" => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::GtElectricGenerator(block.into()),
                    );
                }
                "GROUND-LOOP-HX" => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::GtGroundLoopHx(block.into()),
                    );
                }
                // Elemento desconocido -------------------------
                // THERMAL-STORAGE, PV-MODULE, CONDENSING-UNIT
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

pub fn parse_systems(doc: &roxmltree::Document) -> GtSystems {
    // Sistemas GT
    let gt_systems_str = doc
        .descendants()
        .find(|n| n.has_tag_name("Definicion_Sistema_CALENER_GT"))
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim();
    GtSystems::new(&gt_systems_str).unwrap()
}
