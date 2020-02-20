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
pub mod envolventetypes;
pub mod kyg;
pub mod tbl;
pub mod utils;
#[cfg(windows)]
pub mod wingui;

#[macro_use]
extern crate failure;

use failure::Error;
use std::path::PathBuf;

use envolventetypes::{Boundaries, EnvolventeCteData, Space};
use utils::{find_first_file, fround2, normalize};

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

#[derive(Debug)]
pub struct HulcFiles {
    pub ctehexml: String,
    pub tbl: String,
    pub kyg: String,
}

// Localiza los archivos relevantes
pub fn find_hulc_files<T: AsRef<str>>(basedir: T) -> Result<HulcFiles, Error> {
    let basedir = basedir.as_ref();
    if !PathBuf::from(basedir).exists() {
        bail!("No se ha localizado el directorio base {}", basedir);
    }

    let ctehexmlpattern = [basedir, "*.ctehexml"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .into_owned();
    let ctehexmlpath = find_first_file(&ctehexmlpattern)?;

    let tblpattern = [basedir, "NewBDL_O.tbl"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .into_owned();
    let tblpath = find_first_file(&tblpattern)?;

    let kygpattern = [basedir, "KyGananciasSolares.txt"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .into_owned();
    let kygpath = find_first_file(&kygpattern)?;

    Ok(HulcFiles {
        ctehexml: ctehexmlpath.to_string_lossy().into_owned(),
        tbl: tblpath.to_string_lossy().into_owned(),
        kyg: kygpath.to_string_lossy().into_owned(),
    })
}

/// Construye lista de espacios a partir de datos BDL (Data)
pub fn build_spaces(bdl: &bdl::Data) -> Result<Vec<Space>, failure::Error> {
    bdl.spaces
        .iter()
        .map(|s| {
            let area = (s.area() * 100.0).round() / 100.0;
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

/// Construye elementos de la envolvente a partir de datos BDL
/// TODO: algunos datos no los podemos calcular todavía
fn envelope_from_ctehedata(
    data: &ctehexml::CtehexmlData,
) -> Result<envolventetypes::EnvelopeElements, Error> {
    let bdl = &data.bdldata;
    let mut envelope = envolventetypes::EnvelopeElements::default();

    // Walls: falta U
    for wall in &bdl.walls {
        use Boundaries::*;
        let bounds = wall.bounds.into();
        let btrx = match bounds {
            EXTERIOR | UNDERGROUND => 1.0,
            _ => 0.0,
        };
        // Actualización a criterio de la UNE-EN ISO 52016-1. S=0, E=+90, W=-90
        let orientation = normalize(180.0 - wall.azimuth(0.0, &bdl)?, -180.0, 180.0);
        let w = envolventetypes::Wall {
            name: wall.name.clone(),
            a: fround2(wall.net_area(bdl)?),
            space: wall.space.clone(),
            nextto: wall.nextto.clone(),
            bounds,
            btrx,                  // TODO: eliminar
            u: Default::default(), // TODO: por ahora completar con kyg
            absorptance: wall.absorptance.unwrap_or(0.6),
            orientation: fround2(orientation),
            tilt: fround2(wall.tilt),
        };
        envelope.walls.push(w);
    }

    // Windows: falta Fshobst, U, orientation
    for win in &bdl.windows {
        let cons = data
            .bdldata
            .db
            .windowcons
            .get(&win.construction)
            .ok_or_else(|| {
                format_err!(
                    "Construcción {} de hueco {} no encontrada",
                    win.construction,
                    win.name
                )
            })?;
        // Factor solar del hueco redondeado a dos decimales
        let glass = bdl.db.glasses.get(&cons.glass).ok_or_else(|| {
            format_err!(
                "Vidrio {} de la construcción {} del hueco {} no encontrado",
                cons.glass,
                win.construction,
                win.name
            )
        })?;
        let ff = cons.framefrac;
        let gglwi = (glass.g_gln * 0.90 * 100.0).round() / 100.0;
        let gglshwi = cons.gglshwi.unwrap_or(gglwi);
        let infcoeff_100 = cons.infcoeff;

        let w = envolventetypes::Window {
            name: win.name.clone(),
            orientation: Default::default(), // TODO: por ahora completar con kyg
            wall: win.wall.clone(),
            a: (win.area() * 100.0).round() / 100.0,
            u: Default::default(), // TODO: por ahora completar con kyg
            ff,
            gglwi,
            gglshwi,
            fshobst: Default::default(), // TODO: por ahora completar con kyg
            infcoeff_100,
        };
        envelope.windows.push(w);
    }

    // PTs
    for (_, tb) in &bdl.tbridges {
        let t = envolventetypes::ThermalBridge {
            name: tb.name.clone(),
            l: (tb.length.unwrap_or(0.0) * 100.0).round() / 100.0,
            psi: tb.psi,
        };
        envelope.thermal_bridges.push(t);
    }

    Ok(envelope)
}

pub fn collect_hulc_data(hulcfiles: &HulcFiles) -> Result<EnvolventeCteData, failure::Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmldata = ctehexml::parse_with_catalog(&hulcfiles.ctehexml)?;
    eprintln!(
        "Localizada zona climática {} y coeficientes de transmisión de energía solar g_gl;sh;wi",
        ctehexmldata.climate
    );
    let mut envelope = envelope_from_ctehedata(&ctehexmldata)?;

    // Carga datos de espacios
    let spaces = build_spaces(&ctehexmldata.bdldata)?;

    // Interpreta .kyg
    let envelopekyg = kyg::parse(&hulcfiles.kyg)?;
    eprintln!("Localizada definición KyGananciasSolares.txt");

    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------

    for wall in &mut envelope.walls {
        let kygwall = envelopekyg.walls.iter().find(|w| w.name == wall.name);
        if let Some(kw) = kygwall {
            wall.u = kw.u;
        }
    }

    for win in &mut envelope.windows {
        let kygwin = envelopekyg.windows.iter().find(|w| w.name == win.name);
        if let Some(kw) = kygwin {
            win.u = kw.u;
            win.orientation = kw.orientation.clone();
            win.fshobst = kw.fshobst;
        }
    }

    // Zona climática
    let climate = ctehexmldata.climate;

    // Salida de datos
    Ok(EnvolventeCteData {
        climate,
        envelope,
        spaces,
    })
}

// Conversiones de BDL a EnvolventeTypes -------------------

impl From<bdl::Positions> for envolventetypes::Positions {
    fn from(pos: bdl::Positions) -> Self {
        match pos {
            bdl::Positions::TOP => Self::TOP,
            bdl::Positions::BOTTOM => Self::BOTTOM,
            bdl::Positions::SIDE => Self::SIDE,
        }
    }
}

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
