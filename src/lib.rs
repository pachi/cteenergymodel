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

pub mod bdl;
pub mod ctehexml;
pub mod kyg;
pub mod tbl;
pub mod types;
pub mod utils;
#[cfg(windows)]
pub mod wingui;

#[macro_use]
extern crate failure;

use failure::Error;
use std::path::Path;

use types::{
    Boundaries, ConstructionElements, EnvelopeElements, EnvolventeCteData, Space, ThermalBridge,
    Wall, WallCons, Window, WindowCons,
};
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

/// Construye lista de espacios a partir de datos BDL (Data)
pub fn spaces_from_bdl(bdl: &bdl::Data) -> Result<Vec<Space>, failure::Error> {
    bdl.spaces
        .iter()
        .map(|s| {
            let area = fround2(s.area());
            let height_net = s.space_height(&bdl)?;
            let height_gross = s.height;
            Ok(Space {
                name: s.name.clone(),
                area,
                height_net,
                height_gross,
                inside_tenv: s.insidete,
                multiplier: s.multiplier,
                space_type: match s.stype.as_ref() {
                    "CONDITIONED" => "ACONDICIONADO",
                    "UNHABITED" => "NO_HABITABLE",
                    _ => "NO_ACONDICIONADO",
                }
                .to_string(),
            })
        })
        .collect::<Result<Vec<Space>, Error>>()
}

/// Construye muros de la envolvente a partir de datos BDL
fn walls_from_bdl(bdl: &bdl::Data) -> Result<Vec<Wall>, Error> {
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
        .map(|wall| -> Result<Wall, Error> {
            Ok(Wall {
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
            })
        })
        .collect::<Result<Vec<Wall>, _>>()?)
    }

/// Construye huecos de la envolvente a partir de datos BDL
fn windows_from_bdl(bdl: &bdl::Data) -> Vec<Window> {
    bdl.windows
        .iter()
        .map(|win| Window {
            name: win.name.clone(),
            cons: win.construction.to_string(),
            wall: win.wall.clone(),
            width: fround2(win.width),
            height: fround2(win.height),
            setback: win.setback,
        })
        .collect()
    }

/// Construye puentes térmicos de la envolvente a partir de datos BDL
fn thermal_bridges_from_bdl(bdl: &bdl::Data) -> Vec<ThermalBridge> {
    // PTs
    bdl.tbridges
        .iter()
        .map(|tb| ThermalBridge {
            name: tb.name.clone(),
            l: fround2(tb.length.unwrap_or(0.0)),
            psi: tb.psi,
        })
        .collect()
}

/// Construcciones de muros a partir de datos BDL
fn wallcons_from_bdl(bdl: &bdl::Data) -> Result<Vec<WallCons>, Error> {
    let mut wcnames: Vec<String> = bdl
        .walls
        .iter()
        .map(|w| w.construction.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    wcnames
        .iter()
        .map(|wcons| {
            bdl.db
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
                .ok_or_else(|| format_err!("Construcción de muro no encontrada: {}", wcons))
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Construcciones de huecos a partir de datos BDL
fn windowcons_from_bdl(bdl: &bdl::Data) -> Result<Vec<WindowCons>, Error> {
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
                    Some(WindowCons {
                        name: cons.name.clone(),
                        group: cons.group.clone(),
                        u,
                        ff,
                        gglwi,
                        gglshwi,
                        infcoeff_100,
                    })
                })
                .ok_or_else(|| {
                    format_err!(
                        "Construcción de hueco no encontrada o mal formada: {}",
                        &wcons,
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Vector con nombre y U de muros, para poder comprobar diferencias en JSON
pub fn walls_u_from_data(
    walls: &Vec<Wall>,
    wallcons: &Vec<WallCons>,
) -> Result<Vec<(String, Boundaries, f32)>, Error> {
    walls
        .iter()
        .map(|w| {
            wallcons
                .iter()
                .find(|c| c.name == w.cons)
                .and_then(|c| Some((w.name.clone(), w.bounds, w.u(c))))
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format_err!("No se han podido calcular las U de los muros"))
}

/// Vector con nombre y Fshobst de huecos, para poder comprobar diferencias en JSON
pub fn windows_fshobst_from_data(
    windows: &[Window],
    walls: &[Wall],
) -> Result<Vec<(String, f32)>, Error> {
    Ok(windows
        .iter()
        .map(|w| {
            let wall = walls.iter().find(|wall| wall.name == w.wall).unwrap();
            (w.name.clone(), w.fshobst(&wall))
        })
        .collect::<Vec<_>>())
}

/// Genera datos de EnvolventeCTE a partir de datos BDL en el XML
pub fn ecdata_from_xml(
    ctehexmldata: &ctehexml::CtehexmlData,
) -> Result<EnvolventeCteData, failure::Error> {
    // Zona climática
    let climate = ctehexmldata.climate.clone();
    let walls = walls_from_bdl(&ctehexmldata.bdldata)?;
    let windows = windows_from_bdl(&ctehexmldata.bdldata);
    let thermal_bridges = thermal_bridges_from_bdl(&ctehexmldata.bdldata);
    let wallcons = wallcons_from_bdl(&ctehexmldata.bdldata)?;
    let windowcons = windowcons_from_bdl(&ctehexmldata.bdldata)?;
    let walls_u = walls_u_from_data(&walls, &wallcons)?;
    let windows_fshobst = windows_fshobst_from_data(&windows, &walls)?;

    Ok(EnvolventeCteData {
        climate,
        envelope: EnvelopeElements {
            walls,
            windows,
            thermal_bridges,
        },
        constructions: ConstructionElements {
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
    ecdata: &mut EnvolventeCteData,
    kygpath: Option<T>,
    tblpath: Option<T>,
) {
    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------
    // Interpreta .kyg y añade datos que faltan
    if let Some(kygpath) = &kygpath {
        let kygdata = kyg::parse(&kygpath).unwrap();

        for tuple in &mut ecdata.walls_u {
            let wallname = &tuple.0;
            let kygwall = kygdata.walls.iter().find(|w| &*w.name == wallname);
            if let Some(kw) = kygwall {
                tuple.2 = fround2(kw.u);
            }
        }

        for tuple in &mut ecdata.windows_fshobst {
            let winname = &tuple.0;
            let kygwin = kygdata.windows.iter().find(|w| &*w.name == winname);
            if let Some(kw) = kygwin {
                tuple.1 = fround2(kw.fshobst);
            }
        }
    }

    // Actualizamos datos desde el archivo .tbl
    // Básicamente son U de particiones interiores
    if let Some(tblpath) = &tblpath {
        let tbldata = tbl::parse(&tblpath).unwrap();
        for tuple in &mut ecdata.walls_u {
            if tuple.1 != Boundaries::INTERIOR {
                continue;
            };
            let w = tbldata.elements.iter().find(|w| w.name == tuple.0).unwrap();
            tuple.2 = fround2(w.u);
        }
    }
}

/// Recoge datos desde archivo .ctehexml y, si se indica, del archivo KyGananciasSolares.txt
pub fn collect_hulc_data<T: AsRef<Path>>(
    ctehexmlpath: Option<T>,
    kygpath: Option<T>,
    tblpath: Option<T>,
) -> Result<EnvolventeCteData, failure::Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmlpath = &ctehexmlpath.ok_or_else(|| {
        format_err!("No se ha podido localizar el archivo .ctehexml del proyecto")
    })?;

    // Genera EnvolventeCteData desde BDL
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
