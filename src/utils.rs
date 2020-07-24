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
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use failure::Error;
use failure::ResultExt;
use glob::glob;

/// Localiza archivo que sigue el patrón pat en el directorio dir
/// Falla si hay algún error en el patrón
pub fn find_file_in_basedir<T: AsRef<str>>(dir: T, pat: &str) -> Result<Option<PathBuf>, Error> {
    let dir = dir.as_ref();
    if !PathBuf::from(dir).exists() {
        bail!("No se ha localizado el directorio {}", dir);
    }

    let pattern = [dir, pat]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .into_owned();

    let globiter = glob(&pattern)?;
    match globiter.map(|r| r).next() {
        Some(p) => Ok(Some(p?)),
        None => Ok(None),
    }
}

// Busca el primer archivo que coincida con el patrón dado
pub fn find_first_file(pattern: &str) -> Result<Option<PathBuf>, Error> {
    let globiter = glob(pattern)?;
    match globiter.map(|r| r).next() {
        Some(p) => Ok(Some(p?)),
        None => Ok(None),
    }
}

// Lee a una cadena un archivo en latin1
pub fn read_latin1_file<T: AsRef<Path>>(path: T) -> Result<String, Error> {
    let buf = {
        let mut buf = Vec::new();
        BufReader::new(File::open(path.as_ref())?)
            .read_to_end(&mut buf)
            .context("No se ha podido leer el archivo")?;
        buf
    };

    match ISO_8859_1.decode(&buf, DecoderTrap::Replace) {
        Ok(utf8buf) => Ok(utf8buf),
        _ => bail!(
            "Error de codificación del archivo {}",
            path.as_ref().display()
        ),
    }
}

// Lee a una cadena un archivo en utf8
pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String, Error> {
    let mut buf = String::new();
    BufReader::new(File::open(path.as_ref())?)
        .read_to_string(&mut buf)
        .context("No se ha podido leer el archivo")?;
    Ok(buf)
}

/// Redondea valor a 2 decimales
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}

/// Normaliza número a un intervalo arbitrario (wrapping)
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    // ancho del intervalo
    let width = end - start;
    // convertimos el intervalo a [0, ancho] restando el valor inicial
    let offset = value - start;
    // volvemos a sumar el valor incial para volver al intervalo [start, end]
    (offset - (f32::floor(offset / width) * width)) + start
}

/// Convierte ángulo desde el criterio del BDL al criterio de la 52016-1
/// BDL: Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del muro
/// UNE-EN ISO 52016-1: S=0, E=+90, W=-90
pub fn orientation_bdl252016(azimuth: f32) -> f32 {
    normalize(180.0 - azimuth, -180.0, 180.0)
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
        "NW"
    }
    // 240 = 360 - 120
    else if angle < 291.0 {
        "W"
    }
    // 291 = 360 - 69
    else if angle < 342.0 {
        "SW"
    }
    // 342 = 360 - 18
    else {
        "S"
    };
    name.to_string()
}
