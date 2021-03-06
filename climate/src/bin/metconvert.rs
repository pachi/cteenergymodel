use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

use climate::met::met_monthly_data;

const APP_TITLE: &str = r#"CteEPBD"#;
const APP_DESCRIPTION: &str = r#"
Copyright (c) 2018-2021 Instituto de CC. de la Construcción Eduardo Torroja (IETcc-CSIC)

Autores: Rafael Villar Burke <pachi@ietcc.csic.es>,

Licencia: Publicado bajo licencia MIT.

"#;
const APP_ABOUT: &str = r#"metconvert - conversión de archivos .met para uso en bemodel"#;
const APP_LICENSE: &str = r#"
Copyright (c) 2018-2021 Instituto de Ciencias de la Construcción Eduardo Torroja (IETcc-CSIC)

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
    use clap::Arg;

    let matches = clap::App::new(APP_TITLE)
        .bin_name("cteepbd")
        .version(env!("CARGO_PKG_VERSION"))
        .author(APP_DESCRIPTION)
        .about(APP_ABOUT)
        .setting(clap::AppSettings::NextLineHelp)
        .arg(
            Arg::with_name("climasdir")
                .value_name("CLIMASDIR")
                .help("Directorio con climas .met")
                .default_value(".")
                .index(1),
        )
        .arg(
            Arg::with_name("archivo_salida_json")
                .long("json")
                .value_name("ARCHIVO_SALIDA_JSON")
                .help("Archivo de salida de resultados detallados en formato JSON")
                .takes_value(true)
                .default_value("zcraddata.json"),
        )
        .arg(
            Arg::with_name("pretty")
                .help("Salida en JSON embellecido")
                .short("-p")
                .long("pretty"),
        )
        .arg(
            Arg::with_name("showlicense")
                .short("L")
                .long("licencia")
                .help("Muestra la licencia del programa (MIT)"),
        )
        .get_matches();

    if matches.is_present("showlicense") {
        println!("{}", APP_LICENSE);
        exit(exitcode::OK);
    }

    let climasdir = matches.value_of("climasdir").unwrap();

    let metmonthlydata = met_monthly_data(&climasdir);

    let json = match matches.is_present("pretty") {
        true => serde_json::to_string_pretty(&metmonthlydata),
        _ => serde_json::to_string(&metmonthlydata),
    }
    .unwrap_or_else(|e| {
        eprintln!(
            "ERROR: conversión incorrecta del balance energético a JSON: {}",
            e
        );
        exit(exitcode::DATAERR);
    });
    let path = matches.value_of("archivo_salida_json").unwrap();
    writefile(&path, json.as_bytes());
}
