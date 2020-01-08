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
mod utils;
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

pub fn collect_hulc_data(hulcfiles: &HulcFiles) -> Result<EnvolventeCteData, failure::Error> {
    // Interpreta .ctehexml
    let ctehexmldata = ctehexml::parse(&hulcfiles.ctehexml)?;
    eprintln!(
        "Localizada zona climática {} y coeficientes de transmisión de energía solar g_gl;sh;wi",
        ctehexmldata.climate
    );

    let bdl = &ctehexmldata.bdldata;
    let spaces = &bdl.spaces;

    let espacios = spaces
        .iter()
        .map(|s| {
            let area = s.area(&bdl)?;
            let altura = s.height(&bdl)?;
            Ok(Space {
                nombre: s.name.clone(),
                area,
                altura,
                dentroet: s.insidete,
                mult: s.multiplier,
                tipo: match s.stype.as_ref() {
                    "CONDITIONED" => "ACONDICIONADO",
                    "UNHABITED" => "NOHABITABLE",
                    _ => "NOACONDICIONADO",
                }
                .to_string(),
            })
        })
        .collect::<Result<Vec<Space>, Error>>()?;

    // Interpreta .kyg
    let envolvente = kyg::parse(&hulcfiles.kyg, Some(ctehexmldata.gglshwi))?;
    eprintln!("Localizada definición de elementos de la envolvente");

    // Calcula área útil
    let autil = compute_autil(&espacios);
    eprintln!("Area útil: {} m2", autil);

    // Zona climática
    let clima = ctehexmldata.climate;

    // Salida de datos
    Ok(EnvolventeCteData {
        autil,
        clima,
        envolvente,
        espacios,
    })
}

/// Calcula la superficie útil de los espacios habitables interiores a la envolvente térmica
/// TODO: llevar como método de EnvolventeCteData
pub fn compute_autil(espacios: &[Space]) -> f32 {
    let a_util: f32 = espacios
        .iter()
        .map(|s| {
            if s.dentroet && s.tipo.as_str() != "NOHABITABLE" {
                s.area * s.mult
            } else {
                0.0
            }
        })
        .sum();
    (a_util * 100.0).round() / 100.0
}
