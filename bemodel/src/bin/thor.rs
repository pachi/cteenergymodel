// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

use bemodel::*;
use hulc::ctehexml;

const APP_TITLE: &str = r#"Thor"#;
const APP_DESCRIPTION: &str = r#"
Copyright (c) 2018-2022 Instituto de CC. de la Construcción Eduardo Torroja (IETcc-CSIC)

Autores: Rafael Villar Burke <pachi@ietcc.csic.es>,
         Daniel Jiménez González <danielj@ietcc.csic.es>
         Marta Sorribes Gil <msorribes@ietcc.csic.es>

Licencia: Publicado bajo licencia MIT.

"#;
const APP_ABOUT: &str =
    r#"Thor - Indicadores de eficiencia energética de la envolvente del edificio (CTE DB-HE)."#;
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

/// Crea aplicación y detecta opciones seleccionadas
fn start_app_and_get_matches() -> clap::ArgMatches<'static> {
    use clap::Arg;
    clap::App::new(APP_TITLE)
        .bin_name("frost")
        .version(env!("CARGO_PKG_VERSION"))
        .author(APP_DESCRIPTION)
        .about(APP_ABOUT)
        .setting(clap::AppSettings::NextLineHelp)
        .arg(
            Arg::with_name("ARCHIVO_HULC")
                .help("Archivo .ctehexml de HULC")
                .required(true)
                .index(1),
        )
        // Archivos de salida
        .arg(
            Arg::with_name("archivo_salida_json")
                .short("o")
                .long("output")
                .value_name("ARCHIVO_SALIDA_JSON")
                .help("Archivo de salida del modelo en formato JSON")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("archivo_salida_indicadores")
                .short("r")
                .long("res_output")
                .value_name("ARCHIVO_SALIDA_INDICADORES")
                .help("Archivo de salida de indicadores energéticos en formato JSON")
                .takes_value(true),
        )
        // Opciones estándar: licencia y nivel de detalle
        .arg(
            Arg::with_name("showlicense")
                .short("L")
                .long("licencia")
                .help("Muestra la licencia del programa (MIT)"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches()
}

// Función principal ------------------------------------------------------------------------------

fn main() {
    env_logger::init();

    let matches = start_app_and_get_matches();

    if matches.is_present("showlicense") {
        println!("{}", APP_LICENSE);
        exit(exitcode::OK);
    }

    let verbosity = matches.occurrences_of("v");

    // Componentes energéticos ---------------------------------------------------------------------
    let ctehexmldata =
        ctehexml::parse_with_catalog_from_path(matches.value_of("ARCHIVO_HULC").unwrap())
            .unwrap_or_else(|e| {
                eprintln!("ERROR: formato incorrecto del archivo .ctehexml: {}", e);
                exit(exitcode::DATAERR);
            });
    let model = Model::try_from(&ctehexmldata).unwrap_or_else(|e| {
        eprintln!("ERROR: conversión incorrecta del archivo .ctehexml: {}", e);
        exit(exitcode::DATAERR);
    });

    // Salida de resultados -----------------------------------------------------------------------

    // Modelo en formato json
    let model_json = model.as_json().unwrap_or_else(|e| {
        eprintln!("ERROR: conversión incorrecta del modelo a JSON: {}", e);
        exit(exitcode::DATAERR);
    });

    // Guardar modelo en disco
    if matches.is_present("archivo_salida_json") {
        let path = matches.value_of_os("archivo_salida_json").unwrap();
        if verbosity > 1 {
            println!("Modelo en formato JSON: {:?}", path);
        }
        writefile(&path, model_json.as_bytes());
    }

    // Indicadores en formato JSON
    let indicadores_json = model.energy_indicators().as_json().unwrap_or_else(|e| {
        eprintln!("ERROR: conversión incorrecta del modelo a JSON: {}", e);
        exit(exitcode::DATAERR);
    });

    // Guardar resultados en disco
    if matches.is_present("archivo_salida_indicadores") {
        let path = matches.value_of_os("archivo_salida_indicadores").unwrap();
        if verbosity > 1 {
            println!("Resultados de indicadores en formato JSON: {:?}", path);
        }
        writefile(&path, indicadores_json.as_bytes());
    }

    // Mostrar siempre en formato de texto plano
    if verbosity > 0 {
        println!("{}", indicadores_json);
    }
}
