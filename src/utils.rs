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

// Utilidades varias

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use failure::Error;
use failure::ResultExt;
use glob::glob;

// Busca el primer archivo que coincida con el patrón dado
pub fn find_first_file(pattern: &str) -> Result<PathBuf, Error> {
    let globiter = glob(pattern)?;
    let results: Vec<PathBuf> = globiter.map(|r| r.unwrap()).collect();
    if results.is_empty() {
        bail!("No se ha encontrado ningún archivo {}", pattern);
    }
    Ok(results[0].clone())
}

// Lee a una cadena un archivo en latin1
pub fn read_latin1_file<T: AsRef<str>>(path: T) -> Result<String, Error> {
    let buf = {
        let mut buf = Vec::new();
        File::open(path.as_ref())?
            .read_to_end(&mut buf)
            .context("No se ha podido leer el archivo")?;
        buf
    };

    match ISO_8859_1.decode(&buf, DecoderTrap::Replace) {
        Ok(utf8buf) => Ok(utf8buf),
        _ => bail!("Error de codificación del archivo {}", path.as_ref()),
    }
}

// Lee a una cadena un archivo en utf8
pub fn read_file<T: AsRef<str>>(path: T) -> Result<String, Error> {
    let mut buf = String::new();
    File::open(path.as_ref())?
        .read_to_string(&mut buf)
        .context("No se ha podido leer el archivo")?;
    Ok(buf)
}

// Normaliza número a un intervalo arbitrario (wrapping)
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    // ancho del intervalo
    let width = end - start;
    // convertimos el intervalo a [0, ancho] restando el valor inicial
    let offset = value - start;
    // volvemos a sumar el valor incial para volver al intervalo [start, end]
    (offset - (f32::floor(offset / width) * width)) + start
}

/// Nombre del ángulo a partir de su valor sexagesimal (0 -> 360)
/// El ángulo se define respecto al sur (sur = 0)
/// y crece en sentido antihorario, según DB-HE1 figura A.1
pub fn angle_name(angle: f32) -> String {
    let angle = normalize(angle, 0.0, 360.0);
    let name = if angle < 18.0 {
        "S"
    } else if angle < 69.0 {
        "SE"
    } else if angle < 120.0 {
        "E"
    } else if angle < 157.5 {
        "NE"
    } else if angle < 202.5 {
        "N"
    }
    // 202.5 = 360 - 157.5
    else if angle < 240.0 {
        "NO"
    }
    // 240 = 360 - 120
    else if angle < 291.0 {
        "O"
    }
    // 291 = 360 - 69
    else if angle < 342.0 {
        "SO"
    }
    // 342 = 360 - 18
    else {
        "S"
    };
    name.to_string()
}
