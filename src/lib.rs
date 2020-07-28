/* -*- coding: utf-8 -*-

Copyright (c) 2018 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

pub mod cte;
pub mod parsers;
pub mod utils;
#[cfg(windows)]
pub mod wingui;

#[macro_use]
extern crate failure;

use failure::Error;
use std::{collections::BTreeMap, path::Path};

use cte::{
    Boundaries, Constructions, Envelope, Model, Space, SpaceType, ThermalBridge, Wall, WallCons,
    Window, WindowCons,
};
use parsers::{bdl, ctehexml, kyg, tbl};
use utils::fround2;

pub const PROGNAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_copytxt() -> String {
    format!(
        "{} {} - Exportación de datos de HULC a EnvolventeCTE

Copyright (c) 2018 Rafael Villar Burke <pachi@ietcc.csic.es>
                   Daniel Jiménez González <danielj@ietcc.csic.es>
                   Marta Sorribes Gil <msorribes@ietcc.csic.es>

Publicado bajo licencia MIT
",
        PROGNAME, VERSION
    )
}

/// Construye diccionario de espacios a partir de datos BDL (Data)
pub fn spaces_from_bdl(bdl: &bdl::Data) -> Result<BTreeMap<String, Space>, failure::Error> {
    bdl.spaces
        .iter()
        .map(|s| {
            let area = fround2(s.area());
            let height_net = s.space_height(&bdl)?;
            let height_gross = s.height;
            Ok((
                s.name.clone(),
                Space {
                    name: s.name.clone(),
                    area,
                    height_net,
                    height_gross,
                    inside_tenv: s.insidete,
                    multiplier: s.multiplier,
                    space_type: match s.stype.as_ref() {
                        "CONDITIONED" => SpaceType::CONDITIONED,
                        "UNHABITED" => SpaceType::UNINHABITED,
                        _ => SpaceType::UNCONDITIONED,
                    },
                },
            ))
        })
        .collect::<Result<BTreeMap<String, Space>, Error>>()
}

/// Construye muros de la envolvente a partir de datos BDL
fn walls_from_bdl(bdl: &bdl::Data) -> Result<BTreeMap<String, Wall>, Error> {
    // Desviación general respecto al Norte (criterio BDL)
    let northangle = bdl
        .meta
        .get("BUILD-PARAMETERS")
        .unwrap()
        .attrs
        .get_f32("ANGLE")?;

    Ok(bdl
        .walls
        .iter()
        .map(|wall| -> Result<(String, Wall), Error> {
            Ok((
                wall.name.clone(),
                Wall {
                    name: wall.name.clone(),
                    cons: wall.construction.to_string(),
                    a: fround2(wall.net_area(bdl)?),
                    space: wall.space.clone(),
                    nextto: wall.nextto.clone(),
                    bounds: wall.bounds.into(),
                    azimuth: fround2(utils::orientation_bdl_to_52016(
                        wall.azimuth(northangle, &bdl)?,
                    )),
                    tilt: fround2(wall.tilt),
                },
            ))
        })
        .collect::<Result<BTreeMap<String, Wall>, _>>()?)
}

/// Construye huecos de la envolvente a partir de datos BDL
fn windows_from_bdl(bdl: &bdl::Data) -> BTreeMap<String, Window> {
    bdl.windows
        .iter()
        .map(|win| {
            (
                win.name.clone(),
                Window {
                    name: win.name.clone(),
                    cons: win.construction.to_string(),
                    wall: win.wall.clone(),
                    width: fround2(win.width),
                    height: fround2(win.height),
                    setback: win.setback,
                },
            )
        })
        .collect()
}

/// Construye puentes térmicos de la envolvente a partir de datos BDL
fn thermal_bridges_from_bdl(bdl: &bdl::Data) -> BTreeMap<String, ThermalBridge> {
    // PTs
    bdl.tbridges
        .iter()
        .map(|tb| {
            (
                tb.name.clone(),
                ThermalBridge {
                    name: tb.name.clone(),
                    l: fround2(tb.length.unwrap_or(0.0)),
                    psi: tb.psi,
                },
            )
        })
        .collect()
}

/// Construcciones de muros a partir de datos BDL
fn wallcons_from_bdl(
    walls: &BTreeMap<String, Wall>,
    bdl: &bdl::Data,
) -> Result<BTreeMap<String, WallCons>, Error> {
    let mut wcnames = walls
        .values()
        .map(|w| w.cons.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    wcnames
        .iter()
        .map(|wcons| -> Result<(String, WallCons), Error> {
            let wallcons = bdl
                .db
                .wallcons
                .get(wcons)
                .and_then(|cons| {
                    let absorptance = cons.absorptance;
                    let r_intrinsic = match cons.r_intrinsic(&bdl.db.materials) {
                        Ok(r) => r,
                        _ => return None,
                    };
                    Some(WallCons {
                        name: cons.name.clone(),
                        group: cons.group.clone(),
                        r_intrinsic,
                        absorptance,
                    })
                })
                .ok_or_else(|| format_err!("Construcción de muro no encontrada: {}", wcons))?;
            Ok((wallcons.name.clone(), wallcons))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()
}

/// Construcciones de huecos a partir de datos BDL
fn windowcons_from_bdl(bdl: &bdl::Data) -> Result<BTreeMap<String, WindowCons>, Error> {
    let mut wcnames: Vec<String> = bdl
        .windows
        .iter()
        .map(|w| w.construction.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    wcnames
        .iter()
        .map(|wcons| {
            bdl.db
                .windowcons
                .get(wcons)
                .and_then(|cons| {
                    // Vidrio del hueco (Glass)
                    let glass = match bdl
                        .db
                        .glasses
                        .get(&cons.glass)
                        .ok_or_else(|| format_err!("Vidrio no encontrado: {}", cons.glass,))
                    {
                        Ok(glass) => glass,
                        _ => return None,
                    };
                    let ff = cons.framefrac;
                    let gglwi = fround2(glass.g_gln * 0.90);
                    let gglshwi = cons.gglshwi.unwrap_or(gglwi);
                    let infcoeff_100 = cons.infcoeff;
                    let u = fround2(cons.u(&bdl.db.frames, &bdl.db.glasses).unwrap_or_default());
                    Some((
                        cons.name.clone(),
                        WindowCons {
                            name: cons.name.clone(),
                            group: cons.group.clone(),
                            u,
                            ff,
                            gglwi,
                            gglshwi,
                            infcoeff_100,
                        },
                    ))
                })
                .ok_or_else(|| {
                    format_err!(
                        "Construcción de hueco no encontrada o mal formada: {}",
                        &wcons,
                    )
                })
        })
        .collect::<Result<BTreeMap<_, _>, _>>()
}

/// Vector con nombre y U de muros, para poder comprobar diferencias en JSON
pub fn walls_u_from_data(
    walls: &BTreeMap<String, Wall>,
    wallcons: &BTreeMap<String, WallCons>,
) -> Result<Vec<(String, Boundaries, f32)>, Error> {
    walls
        .values()
        .map(|w| {
            wallcons
                .get(&w.cons)
                .and_then(|c| Some((w.name.clone(), w.bounds, w.u(c))))
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format_err!("No se han podido calcular las U de los muros"))
}

/// Vector con nombre y Fshobst de huecos, para poder comprobar diferencias en JSON
pub fn windows_fshobst_from_data(
    windows: &BTreeMap<String, Window>,
    walls: &BTreeMap<String, Wall>,
) -> Result<Vec<(String, f32)>, Error> {
    Ok(windows
        .values()
        .map(|w| {
            let wall = walls.get(&w.wall).unwrap();
            (w.name.clone(), w.fshobst(&wall))
        })
        .collect::<Vec<_>>())
}

/// Genera datos de EnvolventeCTE a partir de datos BDL en el XML
pub fn ecdata_from_xml(ctehexmldata: &ctehexml::CtehexmlData) -> Result<Model, failure::Error> {
    // Zona climática
    let climate = ctehexmldata.climate.clone();
    let walls = walls_from_bdl(&ctehexmldata.bdldata)?;
    let windows = windows_from_bdl(&ctehexmldata.bdldata);
    let thermal_bridges = thermal_bridges_from_bdl(&ctehexmldata.bdldata);
    let wallcons = wallcons_from_bdl(&walls, &ctehexmldata.bdldata)?;
    let windowcons = windowcons_from_bdl(&ctehexmldata.bdldata)?;
    let spaces = spaces_from_bdl(&ctehexmldata.bdldata)?;
    let walls_u = walls_u_from_data(&walls, &wallcons)?;
    let windows_fshobst = windows_fshobst_from_data(&windows, &walls)?;

    Ok(Model {
        climate,
        envelope: Envelope {
            walls,
            windows,
            thermal_bridges,
        },
        constructions: Constructions {
            windows: windowcons,
            walls: wallcons,
        },
        spaces,
        walls_u,
        windows_fshobst,
    })
}

/// Incluye los datos que todavía no calculamos desde el xml
pub fn fix_ecdata_from_extra<T: AsRef<Path>>(
    ecdata: &mut Model,
    kygpath: Option<T>,
    tblpath: Option<T>,
) {
    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------
    // Interpreta .kyg y añade datos que faltan
    if let Some(kygpath) = &kygpath {
        let kygdata = kyg::parse(&kygpath).unwrap();

        for tuple in &mut ecdata.walls_u {
            let wallname = tuple.0.as_str();
            let kygwall = kygdata.walls.get(wallname);
            if let Some(kw) = kygwall {
                tuple.2 = fround2(kw.u);
            }
        }

        for tuple in &mut ecdata.windows_fshobst {
            let winname = tuple.0.as_str();
            let kygwin = kygdata.windows.get(winname);
            if let Some(kw) = kygwin {
                tuple.1 = fround2(kw.fshobst);
            }
        }
    }

    // Actualizamos datos desde el archivo .tbl
    // Básicamente son U de particiones interiores
    if let Some(tblpath) = &tblpath {
        let tbldata = tbl::parse(&tblpath).unwrap();
        for (name, boundary, mut value) in &mut ecdata.walls_u {
            if *boundary != Boundaries::INTERIOR {
                continue;
            };
            let w = tbldata.elements.get(name.as_str()).unwrap();
            value = fround2(w.u);
        }
    }
}

/// Recoge datos desde archivo .ctehexml y, si se indica, del archivo KyGananciasSolares.txt
pub fn collect_hulc_data<T: AsRef<Path>>(
    ctehexmlpath: Option<T>,
    kygpath: Option<T>,
    tblpath: Option<T>,
) -> Result<Model, failure::Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmlpath = &ctehexmlpath.ok_or_else(|| {
        format_err!("No se ha podido localizar el archivo .ctehexml del proyecto")
    })?;

    // Genera Model desde BDL
    let ctehexmldata = ctehexml::parse_with_catalog(&ctehexmlpath)?;
    let mut ecdata = ecdata_from_xml(&ctehexmldata)?;

    // Interpreta .kyg y añade datos que faltan con archivos adicionales
    fix_ecdata_from_extra(&mut ecdata, kygpath, tblpath);

    Ok(ecdata)
}

// Conversiones de BDL a types -------------------

impl From<bdl::Boundaries> for Boundaries {
    fn from(boundary: bdl::Boundaries) -> Self {
        match boundary {
            bdl::Boundaries::EXTERIOR => Self::EXTERIOR,
            bdl::Boundaries::INTERIOR => Self::INTERIOR,
            bdl::Boundaries::UNDERGROUND => Self::UNDERGROUND,
            bdl::Boundaries::ADIABATIC => Self::ADIABATIC,
        }
    }
}
