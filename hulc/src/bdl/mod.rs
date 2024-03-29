// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::path::Path;

use anyhow::{bail, format_err, Error};
use log::{info, warn};

mod blocks;
mod common;
mod db;
mod envelope;
mod systems;

pub use blocks::{build_blocks, BdlBlock, BdlBlockType};
pub use common::{extract_f32vec, extract_namesvec, extract_u32vec, AttrMap};
pub use db::{Construction, Frame, Glass, Material, MaterialProperties, WallCons, WinCons, DB};
pub use envelope::{
    BoundaryType, Floor, Polygon, Shading, Space, ThermalBridge, Tilt, Wall, Window,
};
pub use systems::{DaySchedule, Schedule, WeekSchedule, YearSchedule};

// ------------------------- BDL ----------------------------

/// Datos del archivo BDL
#[derive(Debug, Clone, Default)]
pub struct Data {
    /// Metadatos: espacio de trabajo, parámetros de edificio, construcciones por defecto y datos generales
    pub meta: BTreeMap<BdlBlockType, BdlBlock>,
    /// Base de datos de materiales, productos y composiciones constructivas
    pub db: DB,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Elementos opacos de la envolvente
    pub walls: Vec<Wall>,
    /// Elementos semitransparentes de la envolvente
    pub windows: Vec<Window>,
    /// Puentes térmicos
    pub thermal_bridges: Vec<ThermalBridge>,
    /// Sombras exteriores del edificio
    pub shadings: Vec<Shading>,
    /// Condiciones de uso de los espacios
    pub space_conditions: BTreeMap<String, BdlBlock>,
    /// Consignas de los sistemas
    pub system_conditions: BTreeMap<String, BdlBlock>,
    /// Horarios
    pub schedules: Vec<Schedule>,
}

impl Data {
    /// Nuevo modelo a partir de path
    pub fn new_from_path<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        Self::new(crate::utils::file::read_latin1_file(path)?)
    }

    /// Nuevo modelo a partir de str
    pub fn new<T: AsRef<str>>(input: T) -> Result<Self, Error> {
        use BdlBlockType::*;

        let blocks = build_blocks(input.as_ref())?;

        let mut db_blocks = Vec::new();
        let mut poly_blocks = Vec::new();
        let mut floor_blocks = Vec::new();
        let mut env_blocks = Vec::new();
        let mut other_blocks = Vec::new();
        let mut schedule_blocks = Vec::new();

        for block in blocks {
            match block.btype {
                Construction | Material | NameFrame | GlassType | Layers | Gap => {
                    db_blocks.push(block);
                }
                Polygon => poly_blocks.push(block),
                Floor => floor_blocks.push(block),
                Space | ExteriorWall | Roof | InteriorWall | UndergroundWall | ThermalBridge
                | Window | BuildingShade => env_blocks.push(block),
                WeekSchedulePd | DaySchedulePd | SchedulePd | RunPeriodPd => {
                    schedule_blocks.push(block);
                }
                _ => other_blocks.push(block),
            }
        }

        // Materiales y construcciones ---------------------------------------------

        let mut materials: BTreeMap<String, db::Material> = BTreeMap::new();
        let mut glasses: BTreeMap<String, db::Glass> = BTreeMap::new();
        let mut frames: BTreeMap<String, db::Frame> = BTreeMap::new();
        let mut wallcons: BTreeMap<String, db::WallCons> = BTreeMap::new();
        let mut wincons: BTreeMap<String, db::WinCons> = BTreeMap::new();

        let mut constructions: BTreeMap<String, db::Construction> = BTreeMap::new();
        let mut layers: BTreeMap<String, db::WallCons> = BTreeMap::new();
        for block in db_blocks {
            match block.btype {
                Construction => {
                    constructions.insert(block.name.clone(), db::Construction::try_from(block)?);
                }
                Material => {
                    let e = db::Material::try_from(block)?;
                    materials.insert(e.name.clone(), e);
                }
                NameFrame => {
                    let e = db::Frame::try_from(block)?;
                    frames.insert(e.name.clone(), e);
                }
                GlassType => {
                    let e = db::Glass::try_from(block)?;
                    glasses.insert(e.name.clone(), e);
                }
                Layers => {
                    let e = db::WallCons::try_from(block)?;
                    layers.insert(e.name.clone(), e);
                }
                Gap => {
                    let e = db::WinCons::try_from(block)?;
                    wincons.insert(e.name.clone(), e);
                }
                _ => unreachable!(),
            }
        }
        // Añadir, si no existe la composición e capas por defecto
        layers.entry("Ninguno".to_string()).or_insert(db::WallCons {
            name: "Ninguno".into(),
            ..Default::default()
        });

        // Generar construcciones con Constructions y Layers

        // Caso en el que no se han definido las construcciones de los elementos, tienen asignado WallCons "Ninguno"
        for cons in constructions.values() {
            let mut layers_obj = layers
                .get_mut(&cons.layers)
                .ok_or_else(|| {
                    format_err!(
                        "No se ha encontrado la definición de capas {} de la construcción {}",
                        cons.layers,
                        cons.name
                    )
                })?
                .clone();
            // Cuando TODO: esto incluye muchas veces el nombre de las capas layers_obj.name con la absortividad
            // cuando no hay más construcciones con distinta absortividad. Podríamos simplificarlo.
            if cons.name != cons.layers {
                layers_obj.name = cons.name.clone();
                layers_obj.absorptance = cons.absorptance;

                wallcons.insert(layers_obj.name.clone(), layers_obj);
            }
        }

        for mut layers_obj in layers.into_values() {
            if layers_obj.absorptance == 0.0 {
                layers_obj.absorptance = 0.6;
            };
            wallcons.insert(layers_obj.name.clone(), layers_obj);
        }

        let db = DB {
            materials,
            glasses,
            frames,
            wallcons,
            wincons,
        };

        // Plantas y polígonos -----------------------------------------------------

        // Separa polígonos (POLYGON) -----------
        // luego los sustituiremos en los objetos de opacos y SPACE que los usan
        let mut polygons: BTreeMap<String, envelope::Polygon> = BTreeMap::default();
        for block in poly_blocks {
            polygons.insert(block.name.clone(), envelope::Polygon::try_from(block)?);
        }

        // Separa plantas (FLOOR) --------------
        // Estos objetos los eliminamos finalmente del modelo, sumando sus X,Y,Z, Azimuth a los del espacio
        // luego los sustituiremos en los objetos SPACE que los usan
        let mut floors: BTreeMap<String, envelope::Floor> = BTreeMap::default();
        for block in floor_blocks {
            floors.insert(block.name.clone(), envelope::Floor::try_from(block)?);
        }

        // Horarios --------------------------------------
        let mut schedules: Vec<systems::Schedule> = Vec::new();

        for block in schedule_blocks {
            match block.btype {
                DaySchedulePd => {
                    schedules.push(systems::Schedule::Day(systems::DaySchedule::try_from(
                        block,
                    )?));
                }
                WeekSchedulePd => {
                    schedules.push(systems::Schedule::Week(systems::WeekSchedule::try_from(
                        block,
                    )?));
                }
                SchedulePd => {
                    schedules.push(systems::Schedule::Year(systems::YearSchedule::try_from(
                        block,
                    )?));
                }
                RunPeriodPd => {
                    info!("Ignorando bloque de periodo de cálculo: {}", block.name);
                }
                // Elemento desconocido -------------------------
                _ => unreachable!(),
            };
        }

        // Componentes de la envolvente ===============
        // Necesita tener los constructions, floors y polygons ya resueltos
        // También necesita resueltas las cargas (spaces_conditions) y consignas (system_conditions)
        let mut spaces: Vec<envelope::Space> = Vec::new();
        let mut walls: Vec<envelope::Wall> = Vec::new();
        let mut windows: Vec<envelope::Window> = Vec::new();
        let mut thermal_bridges: Vec<envelope::ThermalBridge> = Vec::new();
        let mut shadings: Vec<envelope::Shading> = Vec::new();
        for block in env_blocks {
            match block.btype {
                // Espacios -----------
                Space => {
                    let polygon_name = block.attrs.get_str("POLYGON")?;
                    let mut space = envelope::Space::try_from(block)?;
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

                    spaces.push(space);
                }

                // Cerramientos opacos de la envolvente -----------
                ExteriorWall | Roof | InteriorWall | UndergroundWall => {
                    let maybe_polygon_name = block.attrs.get_str("POLYGON").ok();
                    let mut wall = envelope::Wall::try_from(block)?;
                    wall.polygon = if let Some(polygon_name) = maybe_polygon_name {
                        Some(polygons.remove(&polygon_name).ok_or_else(|| {
                            format_err!(
                                "Polígono {} no encontrado para definición de opaco {}",
                                &polygon_name,
                                &wall.name,
                            )
                        })?)
                    } else {
                        None
                    };

                    if !db.wallcons.contains_key(&wall.cons) {
                        bail!(
                            "Construcción {} no encontrada para definicón de opaco {}",
                            wall.cons,
                            wall.name
                        );
                    };

                    // Corregimos el ángulo con el norte para los casos con polígono o definidos por posición
                    // En el caso de elementos BOTTOM tenemos que, al exportar, alterar el polígno para que con tilt 180 se quede bien
                    wall.angle_with_space_north =
                        compute_wall_angle_with_space_north(&wall, &spaces)?;

                    // Guardamos el opaco
                    walls.push(wall);
                }

                // Elementos transparentes de la envolvente -----
                // Hueco
                Window => {
                    windows.push(envelope::Window::try_from(block)?);
                }

                // Puentes térmicos ----------
                ThermalBridge => {
                    thermal_bridges.push(envelope::ThermalBridge::try_from(block)?);
                }

                // Sombras --------------------------------------
                BuildingShade => {
                    shadings.push(envelope::Shading::try_from(block)?);
                }

                _ => unreachable!(),
            }
        }

        // Resto de bloques ------------------------------

        // Resto de elementos
        let mut meta: BTreeMap<BdlBlockType, BdlBlock> = BTreeMap::new();
        let mut space_conditions: BTreeMap<String, BdlBlock> = BTreeMap::new();
        let mut system_conditions: BTreeMap<String, BdlBlock> = BTreeMap::new();

        for block in other_blocks {
            use BdlBlockType::*;

            match block.btype {
                // Elementos generales =========================
                // Valores por defecto, Datos generales, espacio de trabajo y edificio
                Defectos | GeneralData | WorkSpace | BuildParameters => {
                    meta.insert(block.btype, block);
                }
                // Condiciones de uso y ocupación ----------
                SpaceConditions => {
                    space_conditions.insert(block.name.clone(), block);
                }
                // Consignas y horarios de sistemas ----------
                SystemConditions => {
                    system_conditions.insert(block.name.clone(), block);
                }
                // Elementos no implementados -------------------
                // Fakes: DESCRIPTION, PARTELIDER
                AuxLine | ParteLider | DescriptionCondiction | Description => continue,

                // Elemento desconocido -------------------------
                _ => {
                    warn!(
                        "Tipo desconocido. bname: {}, btype: {:?}",
                        block.name, block.btype
                    );
                }
            };
        }

        Ok(Self {
            meta,
            db,
            spaces,
            walls,
            windows,
            thermal_bridges,
            shadings,
            space_conditions,
            system_conditions,
            schedules,
        })
    }

    /// Localiza hueco
    pub fn get_window<T: AsRef<str>>(&self, name: T) -> Option<&Window> {
        self.windows.iter().find(|w| w.name == name.as_ref())
    }

    /// Localiza opaco
    pub fn get_wall<T: AsRef<str>>(&self, name: T) -> Option<&Wall> {
        self.walls.iter().find(|w| w.name == name.as_ref())
    }

    /// Localiza espacio
    pub fn get_space<T: AsRef<str>>(&self, name: T) -> Option<&Space> {
        self.spaces.iter().find(|w| w.name == name.as_ref())
    }
}

/// Ángulo del opaco respecto al norte (grados sexagesimales, sentido horario, [0, 360])
///
/// Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del opaco
/// Se puede indicar una desviación del norte geográfico respecto al geométrico (northangle)
///
/// Se calcula:
/// 1. Los elementos horizontales se definen con azimut igual a 0.0
/// 2. Los elementos definidos por geometría ya tiene definido su azimut
/// 3. Los elementos definidos por vértice de polígono del espacio consultan la orientación del polígono del espacio
fn compute_wall_angle_with_space_north(wall: &Wall, spaces: &[Space]) -> Result<f32, Error> {
    // Elementos horizontales (hacia arriba o hacia abajo) con tilt definido o elementos definidos por polígono
    // tilt == 0 -> azimuth 0
    // tilt == 180 -> tenemos que hacer un espejo del polígono

    // En DOE2.3 Volume 3 Topics p.153 se indica cómo obtener el AZIMUTH para superficies horizontales:
    // - se gira virtualmente el opaco a una posición vertical (90º con el eje Z del espacio)
    // - sin que se mueva el origen del opaco.
    // El azimuth es el ángulo entre la proyección horizontal de la normal del opaco así levantado con
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
        let space = spaces
            .iter()
            .find(|s| s.name == wall.space.as_str())
            .ok_or_else(|| {
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
        bail!("Imposible calcular azimut del opaco {}", wall.name)
    }
}
