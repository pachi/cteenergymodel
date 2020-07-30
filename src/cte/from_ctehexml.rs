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

//! Conversión desde CtehexmlData a cte::Model

use std::{collections::BTreeMap, convert::TryFrom};

use failure::Error;

use crate::{
    bdl::{self, Data},
    parsers::ctehexml,
    utils::{fround2, orientation_bdl_to_52016},
};

pub use super::{
    Boundaries, Meta, Model, Orientation, Space, SpaceType, ThermalBridge, Tilt, Wall, WallCons,
    Window, WindowCons,
};

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

impl TryFrom<&ctehexml::CtehexmlData> for Model {
    type Error = Error;
    fn try_from(d: &ctehexml::CtehexmlData) -> Result<Self, Self::Error> {
        let bdl = &d.bdldata;

        let walls = walls_from_bdl(&bdl)?;
        let windows = windows_from_bdl(&walls, &bdl);
        let thermal_bridges = thermal_bridges_from_bdl(&bdl);
        let wallcons = wallcons_from_bdl(&walls, &bdl)?;
        let wincons = windowcons_from_bdl(&bdl)?;
        let spaces = spaces_from_bdl(&bdl)?;

        // Completa metadatos desde ctehexml y el bdl
        // Desviación general respecto al Norte (criterio BDL)
        let buildparams = bdl.meta.get("BUILD-PARAMETERS").unwrap();
        let d_perim_insulation = buildparams
            .attrs
            .get_f32("D-AISLAMIENTO-PERIMETRAL")
            .unwrap_or(0.0);
        let rn_perim_insulation = buildparams
            .attrs
            .get_f32("RA-AISLAMIENTO-PERIMETRAL")
            .unwrap_or(0.0);

        let dg = &d.datos_generales;
        let is_dwelling =
            ["Unifamiliar", "Bloque", "UnaBloque"].contains(&dg.tipo_vivienda.as_str());

        let meta = Meta {
            is_new_building: dg.tipo_definicion.as_str() == "Nuevo",
            is_dwelling,
            num_dwellings: dg.num_viviendas_bloque,
            climate: dg.archivo_climatico.clone(),
            global_ventilation_l_s: if is_dwelling {
                Some(dg.valor_impulsion_aire)
            } else {
                None
            },
            n50_test_ach: dg.valor_n50_medido,
            d_perim_insulation,
            rn_perim_insulation,
        };

        Ok(Model {
            meta,
            walls,
            windows,
            thermal_bridges,
            spaces,
            wincons,
            wallcons,
            extra: None,
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
                    area: fround2(wall.net_area(bdl)?),
                    space: wall.space.clone(),
                    nextto: wall.nextto.clone(),
                    bounds: wall.bounds.into(),
                    azimuth: fround2(orientation_bdl_to_52016(wall.azimuth(northangle, &bdl)?)),
                    tilt: fround2(wall.tilt),
                    zground: wall.zground,
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

/// Factor de obstáculos remotos (Fshobst) en función del retranqueo, orientación y geometría del hueco
/// Se calcula, para huecos verticales, de acuerdo a la tabla 17 del DA DB-HE/1 (p. 19).
/// Es un cálculo best-effort. Podríamos mejorarlo implementando la 52016-1 pero lo puede personalizar el usuario luego
pub fn fshobst_for_setback(tilt: f32, azimuth: f32, width: f32, height: f32, setback: f32) -> f32 {
    use Orientation::*;
    use Tilt::*;

    // Calcular según orientación e inclinación
    let rh = setback / height;
    let rw = setback / width;
    match tilt.into() {
        // Elementos verticales - Tabla 17 del DA DB-HE/1 (p.19)
        SIDE => {
            let range_rh = if rh < 0.05 {
                0
            } else if rh <= 0.1 {
                1
            } else if rh <= 0.2 {
                2
            } else if rh <= 0.5 {
                3
            } else {
                4
            };
            let range_rw = if rw < 0.05 {
                0
            } else if rw <= 0.1 {
                1
            } else if rw <= 0.2 {
                2
            } else if rw <= 0.5 {
                3
            } else {
                4
            };
            match azimuth.into() {
                S => match (range_rh, range_rw) {
                    (1, 1) => 0.82,
                    (1, 2) => 0.74,
                    (1, 3) => 0.62,
                    (1, 4) => 0.39,
                    (2, 1) => 0.76,
                    (2, 2) => 0.67,
                    (2, 3) => 0.56,
                    (2, 4) => 0.35,
                    (3, 1) => 0.56,
                    (3, 2) => 0.51,
                    (3, 3) => 0.39,
                    (3, 4) => 0.27,
                    (4, 1) => 0.35,
                    (4, 2) => 0.32,
                    (4, 3) => 0.27,
                    (4, 4) => 0.17,
                    _ => 1.0,
                },
                SE | SW => match (range_rh, range_rw) {
                    (1, 1) => 0.86,
                    (1, 2) => 0.81,
                    (1, 3) => 0.72,
                    (1, 4) => 0.51,
                    (2, 1) => 0.79,
                    (2, 2) => 0.74,
                    (2, 3) => 0.66,
                    (2, 4) => 0.47,
                    (3, 1) => 0.59,
                    (3, 2) => 0.56,
                    (3, 3) => 0.47,
                    (3, 4) => 0.36,
                    (4, 1) => 0.38,
                    (4, 2) => 0.36,
                    (4, 3) => 0.32,
                    (4, 4) => 0.23,
                    _ => 1.0,
                },
                E | W => match (range_rh, range_rw) {
                    (1, 1) => 0.91,
                    (1, 2) => 0.87,
                    (1, 3) => 0.81,
                    (1, 4) => 0.65,
                    (2, 1) => 0.86,
                    (2, 2) => 0.82,
                    (2, 3) => 0.76,
                    (2, 4) => 0.61,
                    (3, 1) => 0.71,
                    (3, 2) => 0.68,
                    (3, 3) => 0.61,
                    (3, 4) => 0.51,
                    (4, 1) => 0.53,
                    (4, 2) => 0.51,
                    (4, 3) => 0.48,
                    (4, 4) => 0.39,
                    _ => 1.0,
                },
                _ => 1.0,
            }
        }
        TOP => {
            // Elementos horizontales: tabla 19 DA DB-HE/1 p.19
            let range_rh = if rh <= 0.1 {
                0
            } else if rh <= 0.5 {
                1
            } else if rh <= 1.0 {
                2
            } else if rh <= 2.0 {
                3
            } else if rh <= 5.0 {
                4
            } else {
                5
            };
            let range_rw = if rw <= 0.1 {
                0
            } else if rw <= 0.5 {
                1
            } else if rw <= 1.0 {
                2
            } else if rw <= 2.0 {
                3
            } else if rw <= 5.0 {
                4
            } else {
                5
            };
            let rmin = i32::min(range_rh, range_rw);
            let rmax = i32::max(range_rh, range_rw);
            match (rmax, rmin) {
                (0, 0) => 0.42,
                (1, 0) => 0.43,
                (1, 1) => 0.46,
                (2, 0) => 0.43,
                (2, 1) => 0.48,
                (2, 2) => 0.52,
                (3, 0) => 0.43,
                (3, 1) => 0.50,
                (3, 2) => 0.55,
                (3, 3) => 0.60,
                (4, 0) => 0.44,
                (4, 1) => 0.51,
                (4, 2) => 0.58,
                (4, 3) => 0.66,
                (4, 4) => 0.75,
                (5, 0) => 0.44,
                (5, 1) => 0.52,
                (5, 2) => 0.59,
                (5, 3) => 0.68,
                (5, 4) => 0.79,
                _ => 0.85,
            }
        }
        BOTTOM => 1.0,
    }
}
