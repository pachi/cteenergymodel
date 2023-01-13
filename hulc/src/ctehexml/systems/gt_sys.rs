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

pub use crate::bdl::{build_blocks, BdlBlock, BdlBlockType};
pub use crate::bdl::{extract_f32vec, extract_namesvec, AttrMap};

use super::gt_types::*;

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Clone, Default)]
pub struct GtSystems {
    /// Sistemas
    pub systems: BTreeMap<String, GtSystem>,
    /// Zonas térmicas
    pub zones: BTreeMap<String, GtZoneSystem>,
    /// Equipos
    pub equipment: BTreeMap<String, TempEquipment>,
}

/// Datos del archivo BDL
#[derive(Debug, Clone)]
pub enum TempEquipment {
    // Block(BdlBlock),
    Pump(GtPump),
    CirculationLoop(GtCirculationLoop),
    Chiller(GtChiller),
    Boiler(GtBoiler),
    DwHeater(GtDwHeater),
    HeatRejection(GtHeatRejection),
    ElectricGenerator(GtElectricGenerator),
    GroundLoopHx(GtGroundLoopHx),
}

impl GtSystems {
    /// Nuevo modelo a partir de str
    pub fn new<T: AsRef<str>>(input: T) -> Result<Self, Error> {
        let blocks = build_blocks(input.as_ref())?;

        // Resto de elementos
        let mut zones: BTreeMap<String, GtZoneSystem> = BTreeMap::new();
        let mut systems: BTreeMap<String, GtSystem> = BTreeMap::new();
        let mut equipment: BTreeMap<String, TempEquipment> = BTreeMap::new();

        // Para asignar a cada zona el último sistema visto
        let mut last_seen_system: Option<String> = None;

        for block in blocks {
            use BdlBlockType::*;

            match block.btype {
                // Zonas
                Zone => {
                    let mut zone: GtZoneSystem = block.into();
                    zone.system = last_seen_system.clone();
                    zones.insert(zone.name.clone(), zone);
                }
                // Secundarios
                System => {
                    // systems.insert(block.name.clone(), GtSystem::try_from(block)?);
                    let system: GtSystem = block.into();
                    last_seen_system = Some(system.name.clone());
                    systems.insert(system.name.clone(), system);
                }
                // Equipos
                Pump => {
                    equipment.insert(block.name.clone(), TempEquipment::Pump(block.into()));
                }
                CirculationLoop => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::CirculationLoop(block.into()),
                    );
                }
                Chiller => {
                    equipment.insert(block.name.clone(), TempEquipment::Chiller(block.into()));
                }
                Boiler => {
                    equipment.insert(block.name.clone(), TempEquipment::Boiler(block.into()));
                }
                DwHeater => {
                    equipment.insert(block.name.clone(), TempEquipment::DwHeater(block.into()));
                }
                HeatRejection => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::HeatRejection(block.into()),
                    );
                }
                ElecGenerator => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::ElectricGenerator(block.into()),
                    );
                }
                GroundLoopHx => {
                    equipment.insert(
                        block.name.clone(),
                        TempEquipment::GroundLoopHx(block.into()),
                    );
                }
                // Elemento desconocido -------------------------
                // THERMAL-STORAGE, PV-MODULE, CONDENSING-UNIT
                _ => {
                    warn!(
                        "Tipo desconocido. bname: {}, btype: {:?}",
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
    GtSystems::new(gt_systems_str).unwrap()
}
