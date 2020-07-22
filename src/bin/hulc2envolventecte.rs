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

#[cfg(not(windows))]
use exitfailure::ExitFailure;
#[cfg(not(windows))]
use std::process::exit;

// TODO: investigar iui https://docs.rs/crate/iui/0.3.0
#[cfg(windows)]
use hulc2envolventecte::wingui;
#[cfg(not(windows))]
use hulc2envolventecte::{collect_hulc_data, ctehexml, get_copytxt, kyg, PROGNAME};

#[cfg(windows)]
fn main() {
    wingui::run_wingui();
}

#[cfg(not(windows))]
fn get_help() -> String {
    format!(
        "Uso: {} DIRECTORIO

Argumentos:
DIRECTORIO     Directorio del proyecto de HULC

Descripción:
Exporta al formato JSON de EnvolventeCTE los datos de un proyecto HULC.

Emite en formato JSON de EnvolventeCTE los datos de un proyecto HULC.
Puede redirigir la salida de resultados a un archivo para su uso posterior:
    {} DIRECTORIO > archivo_salida.json
",
        PROGNAME, PROGNAME
    )
}

#[cfg(not(windows))]
fn main() -> Result<(), ExitFailure> {
    eprintln!("{}\n", get_copytxt());

    let dir = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("{}", get_help());
        exit(1)
    });

    // Localiza archivos
    eprintln!("Localizando archivos de datos en '{}'", dir);
    let ctehexmlpath = ctehexml::find_ctehexml(&dir)?;
    eprintln!(
        "- {}",
        ctehexmlpath
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or("".to_string())
    );
    let kygpath = kyg::find_kyg(&dir)?;
    eprintln!(
        "- {}",
        kygpath
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or("".to_string())
    );

    // Lee datos
    let data = collect_hulc_data(ctehexmlpath, kygpath)?;

    // Convierte a JSON
    match data.as_json() {
        Ok(json) => {
            eprintln!("Salida de resultados en formato JSON de EnvolventeCTE");
            println!("{}", json);
            Ok(())
        }
        _ => {
            eprintln!("Error al guardar la información en formato JSON de EnvolventeCTE");
            exit(1)
        }
    }
}
