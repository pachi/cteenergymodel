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

#[macro_use]
extern crate failure;
use serde::Serialize;

mod ctehexml;
mod kyg;
mod tbl;
mod utils;

#[derive(Debug, Serialize)]
pub struct EnvolventeCteData {
    #[serde(rename(serialize = "Autil"))]
    pub autil: f32,
    pub clima: String,
    pub envolvente: kyg::ElementosEnvolvente,
}

pub fn convert_project_dir(dir: &str) -> Result<EnvolventeCteData, failure::Error> {
    // Localiza archivos
    let hulcfiles = utils::find_hulc_files(&dir)?;
    eprintln!("Localizados archivos de datos en '{}'", dir);
    eprintln!("- {}", hulcfiles.ctehexml);
    eprintln!("- {}", hulcfiles.tbl);
    eprintln!("- {}", hulcfiles.kyg);

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
