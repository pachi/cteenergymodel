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
mod kyg;
mod tbl;
mod utils;
#[cfg(windows)]
pub mod wingui;

#[macro_use]
extern crate failure;

use failure::Error;
use std::path::PathBuf;

use envolventetypes::{ElementosEnvolvente, EnvolventeCteData};
use tbl::Tbl;
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

    // Interpreta .tbl
    let tbl = tbl::parse(&hulcfiles.tbl)?;
    eprintln!(
        "Localizados {} espacios y {} elementos",
        tbl.spaces.len(),
        tbl.elements.len()
    );

    // Interpreta .kyg
    let elementos_envolvente = kyg::parse(&hulcfiles.kyg, Some(ctehexmldata.gglshwi))?;
    eprintln!("Localizada definición de elementos de la envolvente");

    // Calcula área útil con datos de tbl y kyg
    let area_util = compute_autil(&tbl, &elementos_envolvente);
    eprintln!("Area útil: {} m2", area_util);

    // Salida de datos
    let data = EnvolventeCteData {
        autil: area_util,
        clima: ctehexmldata.climate,
        envolvente: elementos_envolvente,
    };
    Ok(data)
}

// Calcula la superficie útil sumando la de los espacios asociados a elementos
pub fn compute_autil(tbl_data: &Tbl, elementos_envolvente: &ElementosEnvolvente) -> f32 {
    // Claves de los elementos de la envolvente
    let mut claves = Vec::new();
    for hueco in &elementos_envolvente.huecos {
        let nombre: &str = &hueco.nombre;
        claves.push(nombre);
    }
    for opaco in &elementos_envolvente.opacos {
        let nombre: &str = &opaco.nombre;
        claves.push(nombre);
    }
    for pt in &elementos_envolvente.pts {
        let nombre: &str = &pt.nombre;
        claves.push(nombre);
    }

    // Espacios asociados a esos elementos de la envolvente
    let mut spaces = Vec::new();
    for &clave in claves.iter() {
        if let Some(elem) = tbl_data.elements.iter().find(|e| e.name == clave) {
            spaces.push(elem.id_space);
        };
    }
    spaces.sort();
    spaces.dedup();

    // Suma, con multiplicador, de las áreas de los elementos
    // El multiplicador es cero para espacios no habitables
    // TODO: comprobar qué ocurre para espacios no acondicionados
    let mut a_util = 0.0_f32;
    for space_id in spaces {
        if let Some(space) = tbl_data.spaces.iter().find(|s| s.id_space == space_id) {
            a_util += space.area * (space.mult as f32);
        } else {
            println!("Espacio con id {} no encontrado!!", space_id);
        }
    }
    (a_util * 100.0).round() / 100.0
}
