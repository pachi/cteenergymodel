// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Utilidades varias para manejo de archivos

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Error};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};

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

    match glob(&pattern)?.next() {
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
            .with_context(|| {
                format!(
                    "No se ha podido leer el archivo {}",
                    path.as_ref().display()
                )
            })?;
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
pub fn read_file<T: AsRef<Path>>(path: T) -> anyhow::Result<String> {
    let mut buf = String::new();
    BufReader::new(File::open(path.as_ref())?)
        .read_to_string(&mut buf)
        .with_context(|| {
            format!(
                "No se ha podido leer el archivo {}",
                path.as_ref().display()
            )
        })?;
    Ok(buf)
}
