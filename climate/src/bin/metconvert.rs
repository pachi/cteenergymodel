use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

use climate::{
    met::{met_monthly_data, read_metdata},
    met_july21st_radiation_data,
};

const APP_TITLE: &str = r#"MetConvert"#;
const APP_DESCRIPTION: &str = r#"
Copyright (c) 2018-2022 Instituto de CC. de la Construcción Eduardo Torroja (IETcc-CSIC)

Autores: Rafael Villar Burke <pachi@ietcc.csic.es>,

Licencia: Publicado bajo licencia MIT.

"#;
const APP_ABOUT: &str = r#"metconvert - conversión de archivos .met para uso en bemodel"#;
const APP_LICENSE: &str = r#"
Copyright (c) 2018-2022 Instituto de Ciencias de la Construcción Eduardo Torroja (IETcc-CSIC)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the 'Software'), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Author(s): Rafael Villar Burke <pachi@ietcc.csic.es>
            Daniel Jiménez González <danielj@ietcc.csic.es>
            Marta Sorribes Gil <msorribes@ietcc.csic.es>"#;

// Funciones auxiliares -----------------------------------------------------------------------

fn writefile<P: AsRef<Path>>(path: P, content: &[u8]) {
    let mut file = File::create(&path)
        .map_err(|e| {
            eprintln!(
                "ERROR: no se ha podido crear el archivo \"{}\": {}",
                path.as_ref().display(),
                e
            );
            exit(exitcode::CANTCREAT);
        })
        .unwrap();
    if let Err(e) = file.write_all(content) {
        eprintln!(
            "ERROR: no se ha podido escribir en el archivo \"{}\": {}",
            path.as_ref().display(),
            e
        );
        exit(exitcode::IOERR);
    }
}

fn main() {
    use clap::arg;

    let matches = clap::Command::new(APP_TITLE)
        .bin_name("cteepbd")
        .version(env!("CARGO_PKG_VERSION"))
        .author(APP_DESCRIPTION)
        .about(APP_ABOUT)
        .next_line_help(true)
        .args(&[
            arg!(climasdir: <CLIMASDIR> "Directorio con climas .met")
                .default_value(".")
                .index(1),
            arg!(pretty: -p --pretty "Salida en JSON embellecido"),
            arg!(showlicense: -L --licencia "Muestra la licencia del programa (MIT)"),
        ])
        .get_matches();

    if matches.get_flag("showlicense") {
        println!("{}", APP_LICENSE);
        exit(exitcode::OK);
    }

    let climasdir = matches.get_one::<&str>("climasdir").unwrap();
    let metdata = read_metdata(climasdir);

    // Datos generales de cada clima
    let metgeneraldata: Vec<_> = metdata.values().map(|v| v.meta.clone()).collect();
    let json = match matches.get_flag("pretty") {
        true => serde_json::to_string_pretty(&metgeneraldata),
        _ => serde_json::to_string(&metgeneraldata),
    }
    .unwrap_or_else(|e| {
        eprintln!(
            "ERROR: conversión incorrecta de los datos generales de climas a JSON: {}",
            e
        );
        exit(exitcode::DATAERR);
    });
    writefile("zcmetadata.json", json.as_bytes());

    // Datos mensuales de radiación
    let metmonthlydata = met_monthly_data(&metdata);
    let json = match matches.get_flag("pretty") {
        true => serde_json::to_string_pretty(&metmonthlydata),
        _ => serde_json::to_string(&metmonthlydata),
    }
    .unwrap_or_else(|e| {
        eprintln!(
            "ERROR: conversión incorrecta de los datos mensuales de radiación a JSON: {}",
            e
        );
        exit(exitcode::DATAERR);
    });
    writefile("zcraddata.json", json.as_bytes());

    // Datos de radiación para el 21 de julio
    let metjulydata = met_july21st_radiation_data(&metdata);
    let json = match matches.get_flag("pretty") {
        true => serde_json::to_string_pretty(&metjulydata),
        _ => serde_json::to_string(&metjulydata),
    }
    .unwrap_or_else(|e| {
        eprintln!(
            "ERROR: conversión incorrecta de los datos del 21 de julio a JSON: {}",
            e
        );
        exit(exitcode::DATAERR);
    });
    writefile("zcjuly21raddata.json", json.as_bytes());

    // Todos los datos climáticos en formato bincode
    let encoded: Vec<u8> = bincode::serialize(&metdata).unwrap();
    writefile("zcmetdata.bincode", &encoded);
}
