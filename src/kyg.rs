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

use std::collections::HashMap;

use failure::Error;
use uuid::Uuid;

use super::envolventetypes::{ElementosEnvolvente, Window, Opaque, TB};
use super::utils::read_latin1_file;

// Lee estructura de datos desde cadena con formato de archivo KyGananciasSolares.txt
pub fn parse(
    path: &str,
    gglshwimap: Option<HashMap<String, f32>>,
) -> Result<ElementosEnvolvente, Error> {
    let utf8buf = read_latin1_file(path)?;

    let lines = utf8buf
        .lines()
        .map(|e| e.trim())
        .collect::<Vec<&str>>()
        .into_iter();

    let mut huecos: Vec<Window> = Vec::new();
    let mut opacos: Vec<Opaque> = Vec::new();
    let mut pts: Vec<TB> = Vec::new();
    let mut qsolvalues: HashMap<String, f32> = HashMap::new();

    for line in lines {
        // Datos de elemento
        if line.starts_with("Muro") || line.starts_with("Ventana") || line.starts_with("PPTT") {
            let vv: Vec<&str> = line.split(';').map(|e| e.trim()).collect();
            let tipo = vv[0];
            match tipo {
                "Ventana" => {
                    if vv.len() < 6 {
                        bail!("Línea de datos de hueco con formato desconocido")
                    }
                    let (nombre, a, u, orienta, ff) = (vv[1], vv[2], vv[3], vv[4], vv[5]);
                    huecos.push(Window {
                        id: (Uuid::new_v4()).to_hyphenated().to_string(),
                        nombre: nombre.to_string(),
                        orientacion: orienta.replace("O", "W").to_string(),
                        a: a.replace(",", ".").parse()?,
                        u: u.replace(",", ".").parse()?,
                        ff: ff.replace(",", ".").parse::<f32>()? / 100.0_f32,
                        gglshwi: 1.0, // Se completa a posteriori con datos del .ctehexml
                        fshobst: 1.0, // Se completa a posteriori con datos de los campos qsolwindow
                    });
                }
                "Muro" => {
                    if vv.len() < 5 {
                        bail!("Línea de datos de opaco con formato desconocido")
                    }
                    let (nombre, a, u, btrx) = (vv[1], vv[2], vv[3], vv[4]);
                    opacos.push(Opaque {
                        id: (Uuid::new_v4()).to_hyphenated().to_string(),
                        nombre: nombre.to_string(),
                        a: a.replace(",", ".").parse()?,
                        u: u.replace(",", ".").parse()?,
                        btrx: btrx.replace(",", ".").parse()?,
                    });
                }
                "PPTT" => {
                    if vv.len() < 4 {
                        bail!("Línea de datos de hueco con formato desconocido")
                    }
                    let (l, psi, nombre) = (vv[1], vv[2], vv[3]);
                    pts.push(TB {
                        id: (Uuid::new_v4()).to_hyphenated().to_string(),
                        nombre: nombre.to_string(),
                        l: l.replace(",", ".").parse()?,
                        psi: psi.replace(",", ".").parse()?,
                    })
                }
                _ => println!("Desconocido"),
            };
        // Ganancias solares de hueco
        } else if line.starts_with('"') {
            let vv: Vec<&str> = line.split(';').map(|e| e.trim()).collect();
            if vv.len() < 8 {
                bail!("Línea de datos de ganancias solares de hueco con formato desconocido")
            }
            let (namequot, _azim, _a, htot, _h1, h2, _h3, _ganancia) =
                (vv[0], vv[1], vv[2], vv[3], vv[4], vv[5], vv[6], vv[7]);
            let name = namequot.trim_matches('"').to_string();
            let fshobst_nornd = h2.parse::<f32>()? / htot.parse::<f32>()?;
            let fshobst = (fshobst_nornd * 100.0).round() / 100.0;
            qsolvalues.insert(name, fshobst);
        } else {
            //  rg_comment || rg_kcoef || rg_qsolunknown
            continue;
        }
    }
    // Actualización de valores de fshobst
    for mut hueco in &mut huecos {
        if let Some(val) = qsolvalues.get(&hueco.nombre) {
            hueco.fshobst = *val;
        }
    }
    // Actualización de valores de gglshwi
    if let Some(gglshwimap) = gglshwimap {
        for mut hueco in &mut huecos {
            if let Some(val) = gglshwimap.get(&hueco.nombre) {
                hueco.gglshwi = *val;
            }
        }
    }

    Ok(ElementosEnvolvente {
        huecos,
        opacos,
        pts,
    })
}
