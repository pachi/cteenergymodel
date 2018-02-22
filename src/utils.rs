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

use std::path::PathBuf;

use failure::Error;
use glob::glob;

#[derive(Debug)]
pub struct HulcFiles {
    pub ctehexml: String,
    pub tbl: String,
    pub kyg: String
}

// Busca el primer archivo que coincida con el patrón dado
pub fn find_first_file(pattern: &str) -> Result<PathBuf, Error> {
    let globiter = glob(pattern)?;
    let results: Vec<PathBuf> = globiter.map(|r| r.unwrap()).collect();
    if results.is_empty() {
        bail!("No se ha encontrado ningún archivo {}", pattern);
    }
    Ok(results[0].clone())
}

// Localiza los archivos relevantes
pub fn find_hulc_files(basedir: &str) -> Result<HulcFiles, Error> {
    if !PathBuf::from(basedir).exists() {
        bail!("No se ha localizado el directorio base {}", basedir);
    }

    let ctehexmlpattern = [basedir, "*.ctehexml"].iter().collect::<PathBuf>()
        .to_string_lossy().into_owned();
    let ctehexmlpath = find_first_file(&ctehexmlpattern)?;

    let tblpattern = [basedir, "NewBDL_O.tbl"].iter().collect::<PathBuf>()
        .to_string_lossy().into_owned();
    let tblpath = find_first_file(&tblpattern)?;

    let kygpattern = [basedir, "KyGananciasSolares.txt"].iter().collect::<PathBuf>()
        .to_string_lossy().into_owned();
    let kygpath = find_first_file(&kygpattern)?;

    Ok(HulcFiles {
        ctehexml: ctehexmlpath.to_string_lossy().into_owned(),
        tbl: tblpath.to_string_lossy().into_owned(),
        kyg: kygpath.to_string_lossy().into_owned()
    })
}
