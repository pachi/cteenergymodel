// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::process::exit;

const APP_TITLE: &str = r#"ConvertDB"#;
const APP_DESCRIPTION: &str = r#"
Copyright (c) 2018-2022 Instituto de CC. de la Construcción Eduardo Torroja (IETcc-CSIC)

Autores: Rafael Villar Burke <pachi@ietcc.csic.es>,
         Daniel Jiménez González <danielj@ietcc.csic.es>
         Marta Sorribes Gil <msorribes@ietcc.csic.es>

Licencia: Publicado bajo licencia MIT.

"#;
const APP_ABOUT: &str = r#"ConvertDB - Conversión de BBDD de HULC a formato de BeModel."#;
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

/// Crea aplicación y detecta opciones seleccionadas
fn start_app_and_get_matches() -> clap::ArgMatches<'static> {
    use clap::Arg;
    clap::App::new(APP_TITLE)
        .bin_name("convertb")
        .version(env!("CARGO_PKG_VERSION"))
        .author(APP_DESCRIPTION)
        .about(APP_ABOUT)
        .setting(clap::AppSettings::NextLineHelp)
        .arg(
            Arg::with_name("ARCHIVO_HULC")
                .help("Archivo BDCatalogo.bdc.utf8.gz de HULC")
                .required(true)
                .index(1),
        )
        // Opciones estándar: licencia y nivel de detalle
        .arg(
            Arg::with_name("showlicense")
                .short("L")
                .long("licencia")
                .help("Muestra la licencia del programa (MIT)"),
        )
        .get_matches()
}

// Función principal ------------------------------------------------------------------------------

fn main() {
    let matches = start_app_and_get_matches();

    if matches.is_present("showlicense") {
        println!("{}", APP_LICENSE);
        exit(exitcode::OK);
    }

    let file_in = matches.value_of("ARCHIVO_HULC").unwrap();

    let lib = convertdb::get_library(file_in);

    let path_out = std::env::current_dir().unwrap().join("hulcdb.rs");
    let data = uneval::to_string(lib)
        .unwrap()
        .replace(".into_iter().collect()),", ".into_iter().collect()),\n")
        .replace(",WallCons", ",\nWallCons")
        .replace(",Frame", ",\nFrame")
        .replace(",Glass", ",\nGlass")
        .replace("),(", "),\n(")
        .replace("\\u{e1}", "á")
        .replace("\\u{e9}", "é")
        .replace("\\u{ed}", "í")
        .replace("\\u{f3}", "ó")
        .replace("\\u{fa}", "ú")
        .replace("\\u{f1}", "ñ")
        .replace(",1000f32", ",1000.0")
        .replace(",10f32", ",10.0")
        .replace("collect(),vec!", "collect(),\nvec!")
        .replace(
            "mats: MatsDb {materials: vec!",
            "\n\nmats:\nMatsDb {\nmaterials:\nvec!",
        )
        .replace(",glasses: vec!", ",\n\nglasses:\nvec!")
        .replace(",frames: vec!", ",\n\nframes:\nvec!")
        .replace(
            ",cons: ConsDb {wallcons: vec!",
            ",\n\ncons:\nConsDb {\nwallcons:\nvec!",
        )
        .replace(",wallcons: vec!", ",\n\nwallcons:\nvec!")
        .replace(",wincons: vec!", ",\n\nwincons:\nvec!")
        .replace(
            ",groups: Groups {materials: vec!",
            ",\n\ngroups:\nGroups {\nmaterials:\nvec!",
        );

    let contents = format!(
        r#"
    use once_cell::unsync::Lazy;

    use bemodel::{{
        ConsDb, Frame, Glass, Groups, Layer, MatProps, Material, MatsDb, Uuid, WallCons,
        WinCons,
    }};

    static LIBRARY: Lazy<Library> = Lazy::new(||
        {}
    );
    "#,
        data
    );
    std::fs::write(&path_out, contents).unwrap();
}
