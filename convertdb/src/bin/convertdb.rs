// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::process::exit;

use regex::Regex;

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
fn start_app_and_get_matches() -> clap::ArgMatches {
    use clap::arg;
    clap::Command::new(APP_TITLE)
        .bin_name("convertb")
        .version(env!("CARGO_PKG_VERSION"))
        .author(APP_DESCRIPTION)
        .about(APP_ABOUT)
        .next_line_help(true)
        .args(&[
            arg!(<ARCHIVO_HULC> "Archivo BDCatalogo.bdc.utf8.gz de HULC").index(1),
            // Opciones estándar: licencia y nivel de detalle
            arg!(showlicense: -L --licencia "Muestra la licencia del programa (MIT)"),
        ])
        .get_matches()
}

// Función principal ------------------------------------------------------------------------------

fn main() {
    let matches = start_app_and_get_matches();

    if matches.get_flag("showlicense") {
        println!("{}", APP_LICENSE);
        exit(exitcode::OK);
    }

    let file_in = matches.get_one::<String>("ARCHIVO_HULC").unwrap();

    let lib = convertdb::get_library(file_in);

    let path_out = std::env::current_dir().unwrap().join("hulcdb.rs");
    let data = uneval::to_string(lib)
        .unwrap()
        .replace(".into_iter().collect()),", ".into_iter().collect()),\n")
        .replace(",WallCons", ",\nWallCons")
        .replace(",WinCons", ",\nWinCons")
        .replace(",Frame", ",\nFrame")
        .replace(",Glass", ",\nGlass")
        .replace("\\u{e1}", "á")
        .replace("\\u{e9}", "é")
        .replace("\\u{ed}", "í")
        .replace("\\u{f3}", "ó")
        .replace("\\u{fa}", "ú")
        .replace("\\u{f1}", "ñ")
        .replace(",1000f32", ",1000.0")
        .replace("(10f32)", "(10.0)")
        .replace(",10f32", ",10.0")
        .replace(" 0f32", " 0.0")
        .replace("collect(),vec!", "collect(),\nvec!")
        .replace(
            "].into_iter().collect()},cons: ConsDb {wallcons: vec![",
            "\n].into_iter().collect()\n},\n\ncons:\nConsDb {\nwallcons:\nvec![\n",
        )
        .replace(
            "},groups: Groups {wallcons: vec![",
            "},\n\ngroups:\nGroups {\nwallcons:\nvec![\n",
        )
        .replace("vec![WallCons", "vec![\nWallCons")
        .replace("vec![WinCons", "vec![\nWinCons")
        .replace("vec![vec![", "vec![\nvec![")
        .replace(
            "].into_iter().collect(),materials: vec![",
            "\n].into_iter().collect(),\n\nmaterials:\nvec![\n",
        )
        .replace(
            "].into_iter().collect(),glasses: vec![",
            "\n].into_iter().collect(),\n\nglasses:\nvec![\n",
        )
        .replace(
            "].into_iter().collect(),frames: vec![",
            "\n].into_iter().collect(),\n\nframes:\nvec![\n",
        )
        .replace(
            "].into_iter().collect(),wallcons: vec![",
            "\n].into_iter().collect(),\n\nwallcons:\nvec![\n",
        )
        .replace(
            "].into_iter().collect(),wincons: vec![",
            "\n].into_iter().collect(),\n\nwincons:\nvec![\n",
        )
        .replace(".into_iter().collect()}", ".into_iter().collect()\n}")
        .replace(
            "collect()].into_iter().collect()",
            "collect()\n].into_iter().collect()",
        )
        .replace(
            "collect())].into_iter().collect()",
            "collect())\n].into_iter().collect()",
        )
        .replace("\"id\".into()", "\"id\"")
        .replace("\"name\".into()", "\"name\"")
        .replace("\"conductivity\".into()", "\"conductivity\"")
        .replace("\"density\".into()", "\"density\"")
        .replace("\"specific_heat\".into()", "\"specific_heat\"")
        .replace("\"vapour_diff\".into()", "\"vapour_diff\"")
        .replace("\"resistance\".into()", "\"resistance\"");

    let re_mat_props = Regex::new(r#"vec!\[\("id",(?P<id>.*)\),\("name",(?P<name>.*)\),\("conductivity",(?P<conductivity>.*)\),\("density",(?P<density>.*)\),\("specific_heat",(?P<specific_heat>.*)\),\("vapour_diff",(?P<vapour_diff>.*)\)\]\.into_iter\(\)\.collect\(\),?"#).unwrap();
    let re_mat_resistance = Regex::new(r#"vec!\[\("id",(?P<id>.*)\),\("name",(?P<name>.*)\),\("resistance",(?P<resistance>.*)\)\]\.into_iter\(\)\.collect\(\),?"#).unwrap();
    let re_numbers = Regex::new(r#"(?P<sep>\s|\()(?P<number>\d+)f32"#).unwrap();
    let re_numbers_2 = Regex::new(r#" (?P<number>\d.\d+)f32"#).unwrap();
    let data = re_mat_props.replace_all(&data, "Material {id: $id, name: $name, properties: MatProps::Detailed { conductivity: $conductivity, density: $density, specific_heat: $specific_heat, vapour_diff: $vapour_diff }},");
    let data = re_mat_resistance.replace_all(&data, "Material {id: $id, name: $name, properties: MatProps::Resistance { resistance: $resistance }},");
    let data = re_numbers.replace_all(&data, "$sep$number.0");
    let data = re_numbers_2.replace_all(&data, " $number");

    let contents = format!(
        r#"
    use once_cell::unsync::Lazy;

    use bemodel::{{ ConsDb, Frame, Glass, Groups, Layer, MatProps, Material, Uuid, WallCons, WinCons }};

    pub static LIBRARY: Lazy<Library> = Lazy::new(||
        {}
    );
    "#,
        data
    );
    std::fs::write(&path_out, contents).unwrap();
}
