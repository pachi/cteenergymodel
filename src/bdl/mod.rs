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
mod construction;
mod floor;
mod geom;
mod shadings;
mod space;
mod walls;
mod window;

pub use blocks::{build_blocks, BdlBlock};
pub use common::{extract_f32vec, extract_namesvec, AttrMap};
pub use cons::{Frame, Gap, Glass, Layers, Material, ThermalBridge, DB};
pub use construction::Construction;
pub use floor::Floor;
pub use geom::Polygon;
pub use shadings::Shade;
pub use space::Space;
pub use walls::Wall;
pub use window::Window;

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct Data {
    /// Metadatos: espacio de trabajo, parámetros de edificio, construcciones por defecto y datos generales
    pub meta: HashMap<String, BdlBlock>,
    /// Base de datos de materiales, productos y composiciones constructivas
    pub db: DB,
    /// Lista de plantas
    pub floors: Vec<Floor>,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Elementos opacos de la envolvente
    pub walls: Vec<Wall>,
    /// Elementos semitransparentes de la envolvente
    pub windows: Vec<Window>,
    // Sombras exteriores del edificio
    pub shadings: Vec<Shade>,
    /// Lista de polígonos
    pub polygons: HashMap<String, Polygon>,
    /// Condiciones de uso de los espacios
    pub spaceconds: HashMap<String, BdlBlock>,
    /// Consignas de los sistemas
    pub systemconds: HashMap<String, BdlBlock>,
    /// Horarios
    pub schedules: HashMap<String, BdlBlock>,
}

impl Data {
    pub fn new(input: &str) -> Result<Self, Error> {
        // Construcciones de elementos de la envolvente
        let mut constructions: HashMap<String, Construction> = HashMap::new();

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
                    let e = Material::try_from(block)?;
                    bdldata.db.materials.insert(e.name.clone(), e);
                }
                "LAYERS" => {
                    let e = Layers::try_from(block)?;
                    bdldata.db.layers.insert(e.name.clone(), e);
                }
                "GAP" => {
                    let e = Gap::try_from(block)?;
                    bdldata.db.windows.insert(e.name.clone(), e);
                }
                "NAME-FRAME" => {
                    let e = Frame::try_from(block)?;
                    bdldata.db.frames.insert(e.name.clone(), e);
                }
                "GLASS-TYPE" => {
                    let e = Glass::try_from(block)?;
                    bdldata.db.glasses.insert(e.name.clone(), e);
                }
                "THERMAL-BRIDGE" => {
                    let e = ThermalBridge::try_from(block)?;
                    bdldata.db.tbridges.insert(e.name.clone(), e);
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
                    constructions
                        .insert(block.name.clone(), Construction::try_from(block)?);
                }

                // Elementos opacos de la envolvente -----------
                "EXTERIOR-WALL" | "ROOF" | "INTERIOR-WALL" | "UNDERGROUND-WALL" => {
                    bdldata.walls.push(Wall::try_from(block)?);
                }

                // Elementos transparentes de la envolvente -----
                // Hueco
                "WINDOW" => {
                    bdldata.windows.push(Window::try_from(block)?);
                }

                // Sombras --------------------------------------
                "BUILDING-SHADE" => {
                    bdldata.shadings.push(Shade::try_from(block)?);
                }

                // Elemento desconocido -------------------------
                // No implementados: AUX-LINE
                // Fakes: DESCRIPTION, PARTELIDER
                _ => {
                    eprintln!(
                        "Tipo desconocido. bname: {}, btype: {}",
                        block.name, block.btype
                    );
                }
            };
        }

        // Postproceso para filtrar elementos redundantes (CONSTRUCTION) ============

        // 1. Traslado de datos de construcciones a muros (HULC solo las define por capas)
        // Se copia en wall.construction la composición de capas de layers de la construcción
        // la absortividad ya está bien el muro
        for s in &mut bdldata.walls {
            let cons = constructions.get(&s.construction).ok_or_else(|| format_err!("No se ha definido la construcción del cerramiento {}", s.name))?;
            s.construction = cons.layers.clone();
        };

        Ok(bdldata)
    }

    /// Localiza hueco
    pub fn get_window(&self, name: &str) -> Option<&Window> {
        self.windows.iter().find(|w| w.name == name)
    }

    /// Localiza muro
    pub fn get_wall(&self, name: &str) -> Option<&Wall> {
        self.walls.iter().find(|w| w.name == name)
    }

    /// Localiza espacio
    pub fn get_space(&self, name: &str) -> Option<&Space> {
        self.spaces.iter().find(|w| w.name == name)
    }
}
