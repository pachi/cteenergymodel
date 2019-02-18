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

use serde_json;
#[macro_use]
extern crate serde_derive;

use exitfailure::ExitFailure;
mod ctehexml;
mod kyg;
mod tbl;
mod utils;

#[cfg(windows)]
mod wingui;

#[derive(Debug, Serialize)]
struct EnvolventeCteData {
    #[serde(rename(serialize = "Autil"))]
    autil: f32,
    clima: String,
    envolvente: kyg::ElementosEnvolvente,
}

const PROGNAME: &str = "hulc2envolventecte";
const VERSION: &str = "1.0";

#[cfg(windows)]
fn get_dir() -> String {
    let dir = wingui::run_wingui();
    "Dir".to_string()
}

#[cfg(not(windows))]
fn get_dir() -> String {
    String::new()
}

fn main() {
    use std::process::exit;

    let help = format!(
        "Uso: {} DIRECTORIO

Argumentos:
    DIRECTORIO     Directorio en el que se localizarán los archivos de datos de HULC

Descripción:

    Emite en formato JSON de EnvolventeCTE los datos de un proyecto HULC.
    Puede redirigir la salida de resultados a un archivo para su uso posterior:
        hulc2envolventecte DIRECTORIO > archivo_salida.json
",
        PROGNAME
    );

    let copy = format!(
        "{} {} - Exportación de datos de HULC a EnvolventeCTE

Copyright (c) 2018 Rafael Villar Burke <pachi@ietcc.csic.es>
                   Daniel Jiménez González <danielj@ietcc.csic.es>
                   Marta Sorribes Gil <msorribes@ietcc.csic.es>

Publicado bajo licencia MIT
",
        PROGNAME, VERSION
    );

    eprintln!("{}\n", copy);
    let dir = match std::env::args().nth(1) {
        Some(dir) => dir,
        None => {
            let guidir = get_dir();
            if guidir.is_empty() {
                eprintln!("{}\n", help);
                exit(1)
            } else {
                guidir
            }
        }
    };

    let hulcfiles = match utils::find_hulc_files(&dir) {
        Ok(hulcfiles) => hulcfiles,
        Err(e) => {
            eprintln!("Error: {}", e);
            for e in e.causes().skip(1) {
                eprintln!("Debido a: {}", e);
            }
            exit(1)
        }
    };
fn main() -> Result<(), ExitFailure> {

    eprintln!("Localizados archivos de datos en '{}'", dir);
    eprintln!("- {}", hulcfiles.ctehexml);
    eprintln!("- {}", hulcfiles.tbl);
    eprintln!("- {}", hulcfiles.kyg);

    let ctehexmldata = ctehexml::parse(&hulcfiles.ctehexml)?;
    eprintln!(
        "Localizada zona climática {} y coeficientes de transmisión de energía solar g_gl;sh;wi",
        ctehexmldata.climate
    );

    let tbl = match tbl::parse(&hulcfiles.tbl) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error al leer el archivo .tbl: {}", e.cause());
            exit(1);
        }
    };
    eprintln!(
        "Localizados {} espacios y {} elementos",
        tbl.spaces.len(),
        tbl.elements.len()
    );

    let elementos_envolvente = kyg::parse(&hulcfiles.kyg, Some(ctehexmldata.gglshwi))?;

    let area_util = tbl.compute_autil(&elementos_envolvente.claves());
    eprintln!("Area útil: {} m2", area_util);

    // Salida en JSON
    let envolvente_data = EnvolventeCteData {
        autil: area_util,
        clima: ctehexmldata.climate,
        envolvente: elementos_envolvente,
    };
    match serde_json::to_string_pretty(&envolvente_data) {
        Ok(json) => {
            eprintln!("Salida de resultados en formato JSON de EnvolventeCTE");
            println!("{}", json);
        }
        _ => {
            eprintln!("Error al guardar la información en formato JSON de EnvolventeCTE");
            exit(1);
        }
    }
}
