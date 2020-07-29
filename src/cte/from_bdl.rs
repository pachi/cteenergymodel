/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

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

//! Conversión desde bdl::Data a cte::Model

use std::{collections::BTreeMap, convert::TryFrom};

use failure::Error;

use super::*;
use crate::{
    bdl::{self, Data},
    utils::{fround2, orientation_bdl_to_52016},
};
use params_compute::{fshobst_for_setback, u_for_wall};

// Conversiones de BDL a tipos CTE -------------------

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

impl TryFrom<&Data> for Model {
    type Error = Error;
    fn try_from(d: &Data) -> Result<Self, Self::Error> {
        let walls = walls_from_bdl(&d)?;
        let windows = windows_from_bdl(&walls, &d);
        let thermal_bridges = thermal_bridges_from_bdl(&d);
        let wallcons = wallcons_from_bdl(&walls, &d)?;
        let windowcons = windowcons_from_bdl(&d)?;
        let spaces = spaces_from_bdl(&d)?;
        let walls_u = walls_u_from_data(&walls, &wallcons)?;

        Ok(Model {
            meta: Default::default(),
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
        })
    }
}

/// Construye diccionario de espacios a partir de datos BDL (Data)
fn spaces_from_bdl(bdl: &Data) -> Result<BTreeMap<String, Space>, Error> {
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
                    n_v: s.airchanges_h,
                },
            ))
        })
        .collect::<Result<BTreeMap<String, Space>, Error>>()
}

/// Construye muros de la envolvente a partir de datos BDL
fn walls_from_bdl(bdl: &Data) -> Result<BTreeMap<String, Wall>, Error> {
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
                    azimuth: fround2(orientation_bdl_to_52016(wall.azimuth(northangle, &bdl)?)),
                    tilt: fround2(wall.tilt),
                },
            ))
        })
        .collect::<Result<BTreeMap<String, Wall>, _>>()?)
}

/// Construye huecos de la envolvente a partir de datos BDL
fn windows_from_bdl(walls: &BTreeMap<String, Wall>, bdl: &Data) -> BTreeMap<String, Window> {
    bdl.windows
        .iter()
        .map(|win| {
            let wall = walls.get(&win.wall).unwrap();
            let fshobst =
                fshobst_for_setback(wall.tilt, wall.azimuth, win.width, win.height, win.setback);
            (
                win.name.clone(),
                Window {
                    name: win.name.clone(),
                    cons: win.construction.to_string(),
                    wall: win.wall.clone(),
                    area: fround2(win.width * win.height),
                    fshobst,
                },
            )
        })
        .collect()
}

/// Construye puentes térmicos de la envolvente a partir de datos BDL
fn thermal_bridges_from_bdl(bdl: &Data) -> BTreeMap<String, ThermalBridge> {
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
    bdl: &Data,
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
fn windowcons_from_bdl(bdl: &Data) -> Result<BTreeMap<String, WindowCons>, Error> {
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
fn walls_u_from_data(
    walls: &BTreeMap<String, Wall>,
    wallcons: &BTreeMap<String, WallCons>,
) -> Result<Vec<(String, Boundaries, f32)>, Error> {
    walls
        .values()
        .map(|w| {
            wallcons.get(&w.cons).map(|c| {
                (
                    w.name.clone(),
                    w.bounds,
                    u_for_wall(w.tilt.into(), w.bounds.into(), c),
                )
            })
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format_err!("No se han podido calcular las U de los muros"))
}
