// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

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
use std::path::Path;

use anyhow::{bail, format_err, Error};
use log::warn;

mod blocks;
mod common;
mod db;
mod envelope;

pub use blocks::{build_blocks, BdlBlock};
pub use common::{extract_f32vec, extract_namesvec, AttrMap};
pub use db::{Construction, Frame, Glass, Material, MaterialProperties, WallCons, WindowCons, DB};
pub use envelope::{
    BoundaryType, Floor, Polygon, Shading, Space, ThermalBridge, Tilt, Wall, Window,
};

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Clone, Default)]
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
    pub shadings: Vec<Shading>,
    /// Condiciones de uso de los espacios
    pub spaceconds: HashMap<String, BdlBlock>,
    /// Consignas de los sistemas
    pub systemconds: HashMap<String, BdlBlock>,
    /// Horarios
    pub schedules: HashMap<String, BdlBlock>,
}

impl Data {
    /// Nuevo modelo a partir de path
    pub fn new_from_path<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        Self::new(super::fileutils::read_latin1_file(path)?)
    }

    /// Nuevo modelo a partir de str
    pub fn new<T: AsRef<str>>(input: T) -> Result<Self, Error> {
        let blocks = build_blocks(input.as_ref())?;

        // Primero recogemos objetos generales como POLYGON, FLOOR y CONSTRUCTION
        // que necesitamos que estén procesados previamente por aparecer en otros

        // Separa polígonos (POLYGON) -----------
        // luego los sustituiremos en los objetos que los usan
        let (poly_blocks, blocks): (Vec<BdlBlock>, Vec<BdlBlock>) =
            blocks.into_iter().partition(|b| &b.btype == "POLYGON");

        let mut polygons: HashMap<String, Polygon> = Default::default();
        for block in poly_blocks {
            polygons.insert(block.name.clone(), Polygon::try_from(block)?);
        }

        // Separa plantas (FLOOR) --------------
        // Estos objetos los eliminamos del modelo, sumando sus X,Y,Z, Azimuth a los del espacio
        // luego los sustituiremos en los objetos que los usan
        let (floor_blocks, blocks): (Vec<BdlBlock>, Vec<BdlBlock>) =
            blocks.into_iter().partition(|b| &b.btype == "FLOOR");

        let mut floors: HashMap<String, Floor> = Default::default();
        for block in floor_blocks {
            floors.insert(block.name.clone(), Floor::try_from(block)?);
        }

        // Separa construcciones (CONSTRUCTION) -------
        // luego los sustituiremos en los objetos que los usan
        let (cons_blocks, blocks): (Vec<BdlBlock>, Vec<BdlBlock>) =
            blocks.into_iter().partition(|b| &b.btype == "CONSTRUCTION");

        let mut constructions: HashMap<String, Construction> = Default::default();
        for block in cons_blocks {
            constructions.insert(block.name.clone(), Construction::try_from(block)?);
        }

        // Resto de bloques -------------------------------
        let mut bdldata: Self = Default::default();
        for block in blocks {
            match block.btype.as_str() {
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
                    bdldata.db.wincons.insert(e.name.clone(), e);
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
                    let mut space = Space::try_from(block)?;
                    // Insertamos el polígono -------
                    space.polygon = polygons
                        .get(&polygon_name)
                        .ok_or_else(|| {
                            format_err!(
                                "Polígono {} no encontrado para el espacio {}",
                                &polygon_name,
                                &space.name,
                            )
                        })?
                        .clone();

                    // Incorporamos datos de planta ----------
                    // Trasladamos la cota Z, el multiplicador de planta y la altura de planta
                    // HULC Solamente considera la altura de la planta para los espacios
                    // NOTA: los espacios con cubierta inclinada podrían llegar a tener otra altura
                    let floor = floors.get(&space.floor).ok_or_else(|| {
                        format_err!(
                            "No se ha encontrado la planta {} del espacio {}",
                            space.floor,
                            space.name
                        )
                    })?;
                    // Para la altura de espacios usamos la altura suelo-suelo de las plantas y para la neta descontamos la altura del forjado superior
                    space.height = floor.height;
                    space.z += floor.z; // Esto es siempre la z de la planta, ya que HULC no admite espacios a otro nivel distinto al de la planta
                    space.floor_multiplier = floor.multiplier;

                    bdldata.spaces.push(space);
                }
                // Construcciones -------------
                // Son elementos redundantes que se eliminan en el postproceso
                "CONSTRUCTION" => {
                    constructions.insert(block.name.clone(), Construction::try_from(block)?);
                }

                // Cerramientos opacos de la envolvente -----------
                "EXTERIOR-WALL" | "ROOF" | "INTERIOR-WALL" | "UNDERGROUND-WALL" => {
                    let maybe_polygon_name = block.attrs.get_str("POLYGON").ok();
                    let mut wall = Wall::try_from(block)?;
                    wall.polygon = if let Some(polygon_name) = maybe_polygon_name {
                        Some(polygons.remove(&polygon_name).ok_or_else(|| {
                            format_err!(
                                "Polígono {} no encontrado para definición de muro {}",
                                &polygon_name,
                                &wall.name,
                            )
                        })?)
                    } else {
                        None
                    };

                    // Sustituimos la construcción por el nombre de la composición de capas
                    // La absortividad ya está correcta en el muro y así podemos eliminar constructions
                    // XXX: El problema es que algunas construcciones comparten layers pero no absortividad
                    let cons = constructions.get(&wall.cons).ok_or_else(|| {
                        format_err!(
                            "No se ha definido la construcción {} del cerramiento {}",
                            wall.cons,
                            wall.name
                        )
                    })?;
                    // HULC: en muros exteriores el valor por defecto es 0.6 (en cubiertas 0.7 y marcos de hueco 0.9)
                    let absorptance = cons.absorptance.unwrap_or(0.6);
                    let layersname = cons.wallcons.clone();
                    // Caso en el que no se han definido las construcciones de los elementos, tienen asignado WallCons "Ninguno"
                    if &layersname == "Ninguno" && !bdldata.db.wallcons.contains_key(&layersname) {
                        bdldata.db.wallcons.insert(
                            "Ninguno".into(),
                            WallCons {
                                name: "Ninguno".into(),
                                ..Default::default()
                            },
                        );
                    };
                    let mut layers = bdldata.db.wallcons.get_mut(&layersname).ok_or_else(|| {
                        format_err!(
                            "No se ha encontrado la definición de capas {} de la construcción {}",
                            layersname,
                            cons.wallcons
                        )
                    })?;
                    layers.absorptance = absorptance;
                    wall.cons = layersname;

                    // Corregimos el ángulo con el norte para los casos con polígono o definidos por posición
                    // En el caso de elementos BOTTOM tenemos que, al exportar, alterar el polígno para que con tilt 180 se quede bien
                    wall.angle_with_space_north =
                        compute_wall_angle_with_space_north(&wall, &bdldata)?;

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
                    bdldata.shadings.push(Shading::try_from(block)?);
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

/// Ángulo del muro respecto al norte (grados sexagesimales, sentido horario, [0, 360])
///
/// Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del muro
/// Se puede indicar una desviación del norte geográfico respecto al geométrico (northangle)
///
/// Se calcula:
/// 1. Los elementos horizontales se definen con azimut igual a 0.0
/// 2. Los elementos definidos por geometría ya tiene definido su azimut
/// 3. Los elementos definidos por vértice de polígono del espacio consultan la orientación del polígono del espacio
fn compute_wall_angle_with_space_north(wall: &Wall, db: &Data) -> Result<f32, Error> {
    // Elementos horizontales (hacia arriba o hacia abajo) con tilt definido o elementos definidos por polígono
    // tilt == 0 -> azimuth 0
    // tilt == 180 -> tenemos que hacer un espejo del polígono

    // En DOE2.3 Volume 3 Topics p.153 se indica cómo obtener el AZIMUTH para superficies horizontales:
    // - se gira virtualmente el muro a una posición vertical (90º con el eje Z del espacio)
    // - sin que se mueva el origen del muro.
    // El azimuth es el ángulo entre la proyección horizontal de la normal del muro así levantado con
    // el eje Y del espacio.
    if wall.location.as_deref() == Some("BOTTOM")
        || wall.location.as_deref() == Some("TOP")
        || wall.tilt.abs() < 10.0 * std::f32::EPSILON
        || (wall.tilt.abs() - 180.0).abs() < 10.0 * std::f32::EPSILON
        || wall.polygon.is_some()
    {
        Ok(wall.angle_with_space_north)
    }
    // Elementos definidos por vértice del polígono de un espacio
    else if let Some(vertex) = wall.location.as_deref() {
        let space = db.get_space(wall.space.as_str()).ok_or_else(|| {
            format_err!(
                "Espacio {} del cerramiento {} no encontrado. No se puede calcular el azimut",
                wall.space,
                wall.name
            )
        })?;
        Ok(space.polygon.edge_normal_to_y(vertex))
    }
    // Resto de casos
    else {
        bail!("Imposible calcular azimut del muro {}", wall.name)
    }
}
