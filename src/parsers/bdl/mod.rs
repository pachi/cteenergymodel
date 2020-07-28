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
use log::warn;

mod blocks;
mod common;
mod db;
mod envelope;

pub use blocks::{build_blocks, BdlBlock};
pub use common::{extract_f32vec, extract_namesvec, AttrMap};
pub use db::{Construction, Frame, Glass, Material, WallCons, WindowCons, DB};
pub use envelope::{Boundaries, Floor, Polygon, Shade, Space, ThermalBridge, Tilt, Wall, Window};

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct Data {
    /// Metadatos: espacio de trabajo, parámetros de edificio, construcciones por defecto y datos generales
    pub meta: HashMap<String, BdlBlock>,
    /// Base de datos de materiales, productos y composiciones constructivas
    pub db: DB,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Elementos opacos de la envolvente
    pub walls: Vec<Wall>,
    /// Elementos semitransparentes de la envolvente
    pub windows: Vec<Window>,
    /// Puentes térmicos
    pub tbridges: Vec<ThermalBridge>,
    /// Sombras exteriores del edificio
    pub shadings: Vec<Shade>,
    /// Condiciones de uso de los espacios
    pub spaceconds: HashMap<String, BdlBlock>,
    /// Consignas de los sistemas
    pub systemconds: HashMap<String, BdlBlock>,
    /// Horarios
    pub schedules: HashMap<String, BdlBlock>,
}

impl Data {
    pub fn new<T: AsRef<str>>(input: T) -> Result<Self, Error> {
        let blocks = build_blocks(input.as_ref())?;

        // Separa polígonos (POLYGON) -----------
        // luego los sustituiremos en los objetos que los usan
        let (poly_blocks, blocks): (Vec<BdlBlock>, Vec<BdlBlock>) =
            blocks.into_iter().partition(|b| &b.btype == "POLYGON");

        let mut polygons: HashMap<String, Polygon> = Default::default();
        for block in poly_blocks {
            // Polígonos
            polygons.insert(block.name.clone(), Polygon::try_from(block)?);
        }

        // Separa plantas (FLOOR) --------------
        // luego los sustituiremos en los objetos que los usan
        let (floor_blocks, blocks): (Vec<BdlBlock>, Vec<BdlBlock>) =
            blocks.into_iter().partition(|b| &b.btype == "FLOOR");

        let mut floors: HashMap<String, Floor> = Default::default();
        for block in floor_blocks {
            // Plantas
            floors.insert(block.name.clone(), Floor::try_from(block)?);
        }

        // Separa construcciones (CONSTRUCTION) -------
        // luego los sustituiremos en los objetos que los usan
        let (cons_blocks, blocks): (Vec<BdlBlock>, Vec<BdlBlock>) =
            blocks.into_iter().partition(|b| &b.btype == "CONSTRUCTION");

        let mut constructions: HashMap<String, Construction> = Default::default();
        for block in cons_blocks {
            // Construcciones
            constructions.insert(block.name.clone(), Construction::try_from(block)?);
        }

        // Resto de bloques -------------------------------
        let mut bdldata: Self = Default::default();
        for block in blocks {
            match block.btype.as_ref() {
                // Elementos generales =========================
                // Valores por defecto, Datos generales, espacio de trabajo y edificio
                "DEFECTOS" | "GENERAL-DATA" | "WORK-SPACE" | "BUILD-PARAMETERS" => {
                    bdldata.meta.insert(block.btype.clone(), block);
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
                    let e = WallCons::try_from(block)?;
                    bdldata.db.wallcons.insert(e.name.clone(), e);
                }
                "GAP" => {
                    let e = WindowCons::try_from(block)?;
                    bdldata.db.windowcons.insert(e.name.clone(), e);
                }
                "NAME-FRAME" => {
                    let e = Frame::try_from(block)?;
                    bdldata.db.frames.insert(e.name.clone(), e);
                }
                "GLASS-TYPE" => {
                    let e = Glass::try_from(block)?;
                    bdldata.db.glasses.insert(e.name.clone(), e);
                }

                // Elementos geométricos y espacios -----------
                // Espacios
                "SPACE" => {
                    let polygon_name = block.attrs.get_str("POLYGON")?;
                    // Se puede copiar un polígono con desplazamiento ------------
                    // Ver caso 14_BloqueH5P.CTE y espacios P1E5A-Hall, P1E5B-Hall
                    let x = block.attrs.get_f32("X");
                    let y = block.attrs.get_f32("Y");

                    // Copiamos polígono ----------
                    let mut space = Space::try_from(block)?;
                    let mut polygon = polygons
                        .get(&polygon_name)
                        .ok_or_else(|| {
                            format_err!(
                                "Polígono {} no encontrado para el espacio {}",
                                &polygon_name,
                                &space.name,
                            )
                        })?
                        .clone();
                    // Desplazamos el polígono
                    if let Ok(xval) = x {
                        polygon.vertices.iter_mut().for_each(|v| v.vector.x += xval);
                    }
                    if let Ok(yval) = y {
                        polygon.vertices.iter_mut().for_each(|v| v.vector.y += yval);
                    }
                    // Insertamos el polígono
                    space.polygon = polygon;

                    // Incorporamos datos de planta ----------
                    // Trasladamos la cota Z y la altura de planta
                    // HULC Solamente considera la altura de la planta para los espacios
                    // NOTA: los espacios con cubierta inclinada podrían llegar a tener otra altura
                    let floor = floors.get(&space.floor).ok_or_else(|| {
                        format_err!(
                            "No se ha encontrado la planta {} del espacio {}",
                            space.floor,
                            space.name
                        )
                    })?;
                    space.height = floor.height;
                    space.z = floor.z;

                    bdldata.spaces.push(space);
                }
                // Construcciones -------------
                // Son elementos redundantes que se eliminan en el postproceso
                "CONSTRUCTION" => {
                    constructions.insert(block.name.clone(), Construction::try_from(block)?);
                }

                // Cerramientos opacos de la envolvente -----------
                "EXTERIOR-WALL" | "ROOF" | "INTERIOR-WALL" | "UNDERGROUND-WALL" => {
                    let maybe_polygon_name = block.attrs.get_str("POLYGON");
                    let mut wall = Wall::try_from(block)?;

                    // Insertamos los polígonos -----------
                    if let Some(mut geom) = wall.geometry.as_mut() {
                        let wall_name = wall.name.clone();
                        let polygon_name = maybe_polygon_name.unwrap();
                        let new_polygon = polygons.remove(&polygon_name).ok_or_else(|| {
                            format_err!(
                                "Polígono {} no encontrado para definición de muro {}",
                                &polygon_name,
                                &wall_name,
                            )
                        })?;
                        geom.polygon = new_polygon;
                    };

                    // Sustituimos la construcción por el nombre de la composición de capas
                    // La absortividad ya está correcta en el muro y así podemos eliminar constructions
                    let cons = constructions.get(&wall.construction).ok_or_else(|| {
                        format_err!(
                            "No se ha definido la construcción del cerramiento {}",
                            wall.name
                        )
                    })?;
                    let absorptance = cons.absorptance.unwrap_or(0.0);
                    let layersname = cons.wallcons.clone();
                    let mut layers = bdldata.db.wallcons.get_mut(&layersname).ok_or_else(|| {
                        format_err!(
                            "No se ha encontrado la definición de capas {} de la construcción {}",
                            layersname,
                            cons.wallcons
                        )
                    })?;
                    layers.absorptance = absorptance;
                    wall.construction = layersname;

                    // Guardamos el muro
                    bdldata.walls.push(wall);
                }
                // Puentes térmicos ----------
                "THERMAL-BRIDGE" => {
                    let e = ThermalBridge::try_from(block)?;
                    bdldata.tbridges.push(e);
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
                    warn!(
                        "Tipo desconocido. bname: {}, btype: {}",
                        block.name, block.btype
                    );
                }
            };
        }

        Ok(bdldata)
    }

    /// Localiza hueco
    pub fn get_window<T: AsRef<str>>(&self, name: T) -> Option<&Window> {
        self.windows.iter().find(|w| w.name == name.as_ref())
    }

    /// Localiza muro
    pub fn get_wall<T: AsRef<str>>(&self, name: T) -> Option<&Wall> {
        self.walls.iter().find(|w| w.name == name.as_ref())
    }

    /// Localiza espacio
    pub fn get_space<T: AsRef<str>>(&self, name: T) -> Option<&Space> {
        self.spaces.iter().find(|w| w.name == name.as_ref())
    }
}
