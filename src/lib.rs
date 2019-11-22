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

mod ctehexml;
mod kyg;
mod tbl;
mod utils;
mod bdl;
#[cfg(windows)]
pub mod wingui;

#[macro_use]
extern crate failure;

use failure::Error;
use serde::Serialize;
use serde_json;
use std::path::PathBuf;

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

#[derive(Debug, Serialize)]
pub struct EnvolventeCteData {
    #[serde(rename(serialize = "Autil"))]
    pub autil: f32,
    pub clima: String,
    pub envolvente: kyg::ElementosEnvolvente,
}

impl EnvolventeCteData {
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }
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

pub fn collect_project_data(hulcfiles: &HulcFiles) -> Result<EnvolventeCteData, failure::Error> {
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

    // Calcula área útil
    let area_util = tbl.compute_autil(&elementos_envolvente.claves());
    eprintln!("Area útil: {} m2", area_util);

    // Salida de datos
    let data = EnvolventeCteData {
        autil: area_util,
        clima: ctehexmldata.climate,
        envolvente: elementos_envolvente,
    };
    Ok(data)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_data() {
        let hulcfiles = find_hulc_files("tests/data").unwrap();
        let data = collect_project_data(&hulcfiles).unwrap();
        assert_eq!(data.autil, 1673.92);
        assert_eq!(data.clima, "D3");
        assert_eq!(data.envolvente.huecos.len(), 92);
        assert_eq!(data.envolvente.opacos.len(), 68);
        assert_eq!(data.envolvente.pts.len(), 6);
    }

}