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
mod constypes;
mod envtypes;
mod geomtypes;
mod types;

pub use blocks::*;
pub use constypes::*;
pub use envtypes::BdlEnvType;
pub use geomtypes::*;
pub use types::*;

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct BdlData {
    /// Base de datos de materiales, productos y composiciones constructivas
    pub db: Vec<BdlDB>,
    /// Metadatos: espacio de trabajo, parámetros de edificio, construcciones por defecto y datos generales
    pub meta: HashMap<String, BdlBlock>,
    /// Lista de plantas
    pub floors: Vec<Floor>,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Elementos de la envolvente
    pub env: Vec<BdlEnvType>,
    // Sombras exteriores del edificio
    pub shadings: Vec<BdlBlock>,
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
        // Datos
        let mut bdldata: Self = Default::default();

        // Elimina líneas en blanco y comentarios, y luego separa por bloques
        let cleanlines = input
            .replace("\r\n", "\n")
            .lines()
            .map(str::trim)
            .filter(|l| *l != "" && !l.starts_with("$"))
            // Eliminamos la línea TEMPLARY = USER que separa la parte
            // propia de LIDER del BDL "estándar"
            .filter(|l| !l.starts_with("TEMPLARY"))
            .collect::<Vec<&str>>()
            .join("\n");

        // Separamos una parte inicial de atributos sueltos, sin bloque,
        // del resto que es BDL válido:
        // CAMBIO = SI
        // CAMBIO-CALENER = NO
        // EEGeneradaAutoconsumida        = "0"
        // PANELFOTOVOLTAICOAUTOCONSUMIDO =              0
        // CONTRIBUCIONRESACS             =           1800
        // ENERGIAGT  = YES
        // TODO: guardar datos de esta parte
        let (_lider_part, bdl_part) = if let Some(pos) =
            cleanlines.find("\"DATOS GENERALES\" = GENERAL-DATA")
        {
            cleanlines.split_at(pos)
        } else {
            panic!("Error en la estructura de datos. No se han encontrado los datos de LIDER y de USARIO")
        };

        // Parsea bloques
        for block in build_blocks(bdl_part)? {
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
                "EXTERIOR-WALL" => {
                    bdldata.env.push(BdlEnvType::ExteriorWall(block));
                }
                "INTERIOR-WALL" => {
                    bdldata.env.push(BdlEnvType::InteriorWall(block));
                }
                "UNDERGROUND-WALL" => {
                    bdldata.env.push(BdlEnvType::UndergroundWall(block));
                }
                "ROOF" => {
                    bdldata.env.push(BdlEnvType::Roof(block));
                }
                "BUILDING-SHADE" => {
                    bdldata.shadings.push(block);
                }

                // Elementos transparentes de la envolvente -----
                // Hueco
                "WINDOW" => {
                    bdldata.env.push(BdlEnvType::Window(block));
                }
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
