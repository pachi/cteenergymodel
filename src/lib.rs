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

use envolventetypes::{EnvolventeCteData, Space};
use utils::find_first_file;

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
pub fn find_hulc_files(basedir: &str) -> Result<HulcFiles, Error> {
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

pub fn collect_hulc_data(hulcfiles: &HulcFiles) -> Result<EnvolventeCteData, failure::Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmldata = ctehexml::parse_with_catalog(&hulcfiles.ctehexml)?;
    eprintln!(
        "Localizada zona climática {} y coeficientes de transmisión de energía solar g_gl;sh;wi",
        ctehexmldata.climate
    );

    // Carga datos de espacios
    let spaces = build_spaces(&ctehexmldata.bdldata)?;

    // Interpreta .kyg
    let mut envelope = kyg::parse(&hulcfiles.kyg)?;
    eprintln!("Localizada definición de elementos de la envolvente");

    // Actualizaciones del kyg con datos del ctehexmldata ---------------
    // 1. Datos de huecos: gglshwi, gglwi y infcoeff
    let gglshwimap = &ctehexmldata.gglshwi;
    for mut win in &mut envelope.windows {
        // Factor solar con protecciones activadas
        if let Some(val) = gglshwimap.get(&win.name) {
            win.gglshwi = *val;
        };
        // Coeficiente de permeabilidad a 100 Pa y factor solar del hueco
        if let Some(bdlwin) = ctehexmldata
            .bdldata
            .windows
            .iter()
            .find(|w| w.name == win.name)
        {
            if let Some(cons) = ctehexmldata.bdldata.db.windows.get(&bdlwin.gap) {
                // Permeabilidad
                win.infcoeff_100 = cons.infcoeff;
                // Factor solar del hueco redondeado a dos decimales
                if let Some(glass) = ctehexmldata.bdldata.db.glasses.get(&cons.glass) {
                    win.gglwi = (glass.g_gln * 0.90 * 100.0).round() / 100.0;
                }
            }
        };
    }
    // 2. Datos de muros
    for mut wall in &mut envelope.walls {
        if let Some(w) = ctehexmldata.bdldata.walls.iter().find(|w| w.name == wall.name) {
            wall.wall_type = w.wall_type.to_string();
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
