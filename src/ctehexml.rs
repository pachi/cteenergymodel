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

// Funciones relacionadas con la interpretación de archivos .ctehexml

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use encoding::all::ISO_8859_1;
use encoding::{Encoding, DecoderTrap};
use failure::Error;
use failure::ResultExt;
use regex::Regex;

// Lee estructura de datos desde cadena con formato de archivo KyGananciasSolares.txt
pub fn findgglshwi(path: &str) -> Result<HashMap<String, f32>, Error> {
    let rg_window = Regex::new(r#".*"(.*)"\s*=\sWINDOW\s*$"#).unwrap();
    let rg_wprop = Regex::new(r#".*transmisividadJulio\s*=\s*([\d.]+)"#).unwrap();

    let buf = {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf).context("No se ha podido leer el archivo")?;
        buf
    };

    let utf8buf = match ISO_8859_1.decode(&buf, DecoderTrap::Replace) {
        Ok(utf8buf) => utf8buf,
        _ => bail!("Error de codificación del archivo {}", path)
    };
    let mut lines = utf8buf.split("\r\n")
        .filter(|l| rg_window.is_match(l) || rg_wprop.is_match(l))
        .collect::<Vec<&str>>().into_iter();

    let mut gglshwi: HashMap<String, f32> = HashMap::new();

    while let Some(line) = lines.next() {
        if rg_window.is_match(line) {
            let windowname = rg_window.captures(line)
                .unwrap().get(1).unwrap().as_str();
            let nextline = lines.next().unwrap();
            if rg_wprop.is_match(nextline) {
                let gglshwivalue: f32 = rg_wprop.captures(nextline)
                    .unwrap().get(1).unwrap().as_str().parse()?;
                gglshwi.insert(windowname.to_owned(), gglshwivalue);
            }
        }
    }
    Ok(gglshwi)
}
