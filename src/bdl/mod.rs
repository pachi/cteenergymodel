//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::HashMap;
use std::convert::TryFrom;

use failure::Error;

mod blocks;
mod common;
mod cons;
mod env;
mod geom;
mod shadings;
mod walls;
mod window;

pub use blocks::{build_blocks, BdlBlock};
pub use common::{extract_f32vec, extract_namesvec, AttrMap};
pub use cons::{BdlDB, Frame, Gap, Glass, Layers, Material, ThermalBridge};
pub use env::BdlEnvType;
pub use geom::{Construction, Floor, Polygon, Space};
pub use shadings::Shade;
pub use walls::{ExteriorWall, InteriorWall, UndergroundWall, WallExt};
pub use window::Window;

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct BdlData {
    /// Metadatos: espacio de trabajo, parámetros de edificio, construcciones por defecto y datos generales
    pub meta: HashMap<String, BdlBlock>,
    /// Base de datos de materiales, productos y composiciones constructivas
    pub db: Vec<BdlDB>,
    /// Lista de plantas
    pub floors: Vec<Floor>,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Elementos opacos de la envolvente
    pub env: Vec<BdlEnvType>,
    /// Elementos semitransparentes de la envolvente
    pub windows: Vec<Window>,
    // Sombras exteriores del edificio
    pub shadings: Vec<Shade>,
    /// Lista de polígonos
    pub polygons: HashMap<String, Polygon>,
    /// Construcciones de elementos de la envolvente
    pub constructions: HashMap<String, Construction>,
    /// Condiciones de uso de los espacios
    pub spaceconds: HashMap<String, BdlBlock>,
    /// Consignas de los sistemas
    pub systemconds: HashMap<String, BdlBlock>,
    /// Horarios
    pub schedules: HashMap<String, BdlBlock>,
}

impl BdlData {
    pub fn new(input: &str) -> Result<Self, Error> {
        let mut bdldata: Self = Default::default();
        for block in build_blocks(input)? {
            match block.btype.as_ref() {
                // Elementos generales =========================
                // Valores por defecto, Datos generales, espacio de trabajo y edificio
                "DEFECTOS" | "GENERAL-DATA" | "WORK-SPACE" | "BUILD-PARAMETERS" => {
                    bdldata.meta.insert(block.name.clone(), block);
                }
                // Horarios ----------
                "WEEK-SCHEDULE-PD" | "DAY-SCHEDULE-PD" | "SCHEDULE-PD" | "RUN-PERIOD-PD" => {
                    bdldata.schedules.insert(block.name.clone(), block);
                }
                // Condiciones de uso y ocupación ----------
                "SPACE-CONDITIONS" => {
                    bdldata.spaceconds.insert(block.name.clone(), block);
                }
                // Consignas y horarios de sistemas ----------
                "SYSTEM-CONDITIONS" => {
                    bdldata.systemconds.insert(block.name.clone(), block);
                }

                // Componentes de la envolvente ===============
                // Materiales y construcciones ----------------
                "MATERIAL" => {
                    bdldata.db.push(BdlDB::Material(Material::try_from(block)?));
                }
                "LAYERS" => {
                    bdldata.db.push(BdlDB::Layers(Layers::try_from(block)?));
                }
                "GAP" => {
                    bdldata.db.push(BdlDB::Gap(Gap::try_from(block)?));
                }
                "NAME-FRAME" => {
                    bdldata.db.push(BdlDB::Frame(Frame::try_from(block)?));
                }
                "GLASS-TYPE" => {
                    bdldata.db.push(BdlDB::Glass(Glass::try_from(block)?));
                }
                "THERMAL-BRIDGE" => {
                    bdldata
                        .db
                        .push(BdlDB::ThermalBridge(ThermalBridge::try_from(block)?));
                }

                // Elementos geométricos y espacios -----------
                // Plantas
                "FLOOR" => {
                    bdldata.floors.push(Floor::try_from(block)?);
                }
                // Espacios
                "SPACE" => {
                    bdldata.spaces.push(Space::try_from(block)?);
                }
                // Polígonos
                "POLYGON" => {
                    bdldata
                        .polygons
                        .insert(block.name.clone(), Polygon::try_from(block)?);
                }
                // Construcciones
                "CONSTRUCTION" => {
                    bdldata
                        .constructions
                        .insert(block.name.clone(), Construction::try_from(block)?);
                }

                // Elementos opacos de la envolvente -----------
                // TODO: Unificar exterior wall y roof en un solo tipo
                "EXTERIOR-WALL" => {
                    bdldata
                        .env
                        .push(BdlEnvType::ExteriorWall(ExteriorWall::try_from(block)?));
                }
                "ROOF" => {
                    bdldata
                        .env
                        .push(BdlEnvType::Roof(ExteriorWall::try_from(block)?));
                }
                "INTERIOR-WALL" => {
                    bdldata
                        .env
                        .push(BdlEnvType::InteriorWall(InteriorWall::try_from(block)?));
                }
                "UNDERGROUND-WALL" => {
                    bdldata
                        .env
                        .push(BdlEnvType::UndergroundWall(UndergroundWall::try_from(
                            block,
                        )?));
                }

                // Elementos transparentes de la envolvente -----
                // Hueco
                "WINDOW" => {
                    bdldata
                        .windows
                        .push(Window::try_from(block)?);
                }

                // Sombras --------------------------------------
                "BUILDING-SHADE" => {
                    bdldata.shadings.push(Shade::try_from(block)?);
                }

                // Elemento desconocido -------------------------
                _ => {
                    eprintln!(
                        "Tipo desconocido. bname: {}, btype: {}",
                        block.name, block.btype
                    );
                }
            };
        }

        Ok(bdldata)
    }
}
