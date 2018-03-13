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

use failure::Error;

use utils::read_latin1_file;

// Lee estructura de datos desde cadena con formato de archivo KyGananciasSolares.txt
pub fn findgglshwi(path: &str) -> Result<HashMap<String, f32>, Error> {
    let utf8buf = read_latin1_file(path)?;

    // Localiza datos de huecos para extraer gglshwi
    //let rg_window = Regex::new(r#".*"(.*)"\s*=\sWINDOW\s*$"#).unwrap();
    //let rg_wprop = Regex::new(r#".*transmisividadJulio\s*=\s*([\d.]+)"#).unwrap();
    let mut window_lines = utf8buf
        .lines()
        .filter(|l| {
            (l.contains(" = WINDOW") && !l.contains("WINDOW-FRAME"))
                || l.contains("transmisividadJulio")
        })
        .collect::<Vec<&str>>()
        .into_iter();

    let mut gglshwi: HashMap<String, f32> = HashMap::new();
    while let Some(line) = window_lines.next() {
        if line.contains(" = WINDOW") {
            let windowname = line.split("=")
                .map(|e| e.trim().trim_matches('"'))
                .collect::<Vec<&str>>()[0];
            let nextline = window_lines.next().unwrap();
            if nextline.contains("transmisividadJulio") {
                let gglshwivalue: f32 =
                    nextline.split("=").map(|e| e.trim()).collect::<Vec<&str>>()[1].parse()?;
                gglshwi.insert(windowname.to_owned(), gglshwivalue);
            }
        }
    }
    Ok(gglshwi)
}
