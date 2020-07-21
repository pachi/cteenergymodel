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

use envolventetypes::{
    Boundaries, EnvelopeElements, EnvolventeCteData, Positions, Space, ThermalBridge, Wall, Window,
};
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

    let kygpattern = [basedir, "KyGananciasSolares.txt"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .into_owned();
    let kygpath = find_first_file(&kygpattern)?;

    Ok(HulcFiles {
        ctehexml: ctehexmlpath.to_string_lossy().into_owned(),
        kyg: kygpath.to_string_lossy().into_owned(),
    })
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

/// Construye elementos de la envolvente a partir de datos BDL
/// TODO: algunos datos no los podemos calcular todavía
fn envelope_from_bdl(bdl: &bdl::Data) -> Result<EnvelopeElements, Error> {
    let mut envelope = EnvelopeElements::default();

    // Walls: falta U
    for wall in &bdl.walls {
        let bounds = wall.bounds.into();
        // Actualización a criterio de la UNE-EN ISO 52016-1. S=0, E=+90, W=-90
        let orientation = normalize(180.0 - wall.azimuth(0.0, &bdl)?, -180.0, 180.0);
        let w = Wall {
            name: wall.name.clone(),
            a: fround2(wall.net_area(bdl)?),
            space: wall.space.clone(),
            nextto: wall.nextto.clone(),
            bounds,
            u: Default::default(), // TODO: por ahora completar con kyg
            absorptance: wall.absorptance.unwrap_or(0.6),
            orientation: fround2(orientation),
            tilt: fround2(wall.tilt),
        };
        envelope.walls.push(w);
    }

    // Windows: falta Fshobst, U, orientation
    for win in &bdl.windows {
        let cons = bdl.db.windowcons.get(&win.construction).ok_or_else(|| {
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
        let gglwi = fround2(glass.g_gln * 0.90);
        let gglshwi = cons.gglshwi.unwrap_or(gglwi);
        let infcoeff_100 = cons.infcoeff;

        let w = Window {
            name: win.name.clone(),
            orientation: Default::default(), // TODO: por ahora completar con kyg
            wall: win.wall.clone(),
            a: fround2(win.area()),
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
    for tb in bdl.tbridges.values() {
        let t = ThermalBridge {
            name: tb.name.clone(),
            l: fround2(tb.length.unwrap_or(0.0)),
            psi: tb.psi,
        };
        envelope.thermal_bridges.push(t);
    }

    Ok(envelope)
}

/// Genera datos de EnvolventeCTE a partir de datos BDL en el XML
pub fn ecdata_from_xml(
    ctehexmldata: &ctehexml::CtehexmlData,
) -> Result<EnvolventeCteData, failure::Error> {
    // Zona climática
    let climate = ctehexmldata.climate.clone();
    let envelope = envelope_from_bdl(&ctehexmldata.bdldata)?;
    let spaces = spaces_from_bdl(&ctehexmldata.bdldata)?;

    Ok(EnvolventeCteData {
        climate,
        envelope,
        spaces,
    })
}

/// Incluye los datos que todavía no calculamos desde el xml
/// TODO: ir viendo qué se podría calcular bien desde el ctehexml para ir migrando
pub fn fix_ecdata_from_kyg(ecdata: &mut EnvolventeCteData, kygdata: &kyg::KyGElements) {
    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------
    for wall in &mut ecdata.envelope.walls {
        let kygwall = kygdata.walls.iter().find(|w| w.name == wall.name);
        if let Some(kw) = kygwall {
            wall.u = kw.u;
        }
    }

    for win in &mut ecdata.envelope.windows {
        let kygwin = kygdata.windows.iter().find(|w| w.name == win.name);
        if let Some(kw) = kygwin {
            win.u = kw.u;
            win.orientation = kw.orientation.clone();
            win.fshobst = kw.fshobst;
        }
    }

    // TODO: hacer un assert de que la U del kyg y la calculada es igual
    // assert_eq!(wall.u, w.U(&data.bdldata), "Probando muro {}", wall.name);
}

pub fn collect_hulc_data(hulcfiles: &HulcFiles) -> Result<EnvolventeCteData, failure::Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmldata = ctehexml::parse_with_catalog(&hulcfiles.ctehexml)?;
    eprintln!(
        "Localizada zona climática {} y coeficientes de transmisión de energía solar g_gl;sh;wi",
        ctehexmldata.climate
    );
    // Genera EnvolventeCteData desde BDL
    let mut ecdata = ecdata_from_xml(&ctehexmldata)?;

    // Interpreta .kyg y añade datos que faltan
    let kygdata = kyg::parse(&hulcfiles.kyg)?;
    eprintln!("Localizada definición KyGananciasSolares.txt");
    fix_ecdata_from_kyg(&mut ecdata, &kygdata);

    Ok(ecdata)
}

// Conversiones de BDL a EnvolventeTypes -------------------

impl From<bdl::Positions> for Positions {
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
