/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

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

#[macro_use]
extern crate failure;

pub mod cte;
pub mod parsers;
pub mod utils;

use std::{convert::TryFrom, path::Path};

use cte::{Boundaries, Model};
use parsers::{bdl, ctehexml, kyg, tbl};
use utils::fround2;

/// Nombre del programa
pub const PROGNAME: &str = env!("CARGO_PKG_NAME");

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Texto de descripción, copyright y licencia del programa
pub fn get_copytxt() -> String {
    format!(
        "{} {} - Exportación de datos de HULC a EnvolventeCTE

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
                   Daniel Jiménez González <danielj@ietcc.csic.es>
                   Marta Sorribes Gil <msorribes@ietcc.csic.es>

Publicado bajo licencia MIT
",
        PROGNAME, VERSION
    )
}

/// Recoge datos desde archivo .ctehexml y, si se indica, del archivo KyGananciasSolares.txt
pub fn collect_hulc_data<T: AsRef<Path>>(
    ctehexmlpath: Option<T>,
    kygpath: Option<T>,
    tblpath: Option<T>,
) -> Result<Model, failure::Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmlpath = &ctehexmlpath.ok_or_else(|| {
        format_err!("No se ha podido localizar el archivo .ctehexml del proyecto")
    })?;

    // Genera Model desde BDL
    let ctehexmldata = ctehexml::parse_with_catalog(&ctehexmlpath)?;
    let mut ecdata = Model::try_from(&ctehexmldata.bdldata)?;

    // Interpreta .kyg y añade datos que faltan con archivos adicionales
    fix_ecdata_from_extra(&mut ecdata, kygpath, tblpath);

    Ok(ecdata)
}

/// Incluye los datos que todavía no se obtienen todavía desde el xml
pub fn fix_ecdata_from_extra<T: AsRef<Path>>(
    ecdata: &mut Model,
    kygpath: Option<T>,
    tblpath: Option<T>,
) {
    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------
    // Interpreta .kyg y añade datos que faltan
    if let Some(kygpath) = &kygpath {
        let kygdata = kyg::parse(&kygpath).unwrap();

        for tuple in &mut ecdata.walls_u {
            let wallname = tuple.0.as_str();
            let kygwall = kygdata.walls.get(wallname);
            if let Some(kw) = kygwall {
                tuple.2 = fround2(kw.u);
            }
        }

        for win in ecdata.envelope.windows.values_mut() {
            let kygwin = kygdata.windows.get(&win.name);
            if let Some(kw) = kygwin {
                win.fshobst = fround2(kw.fshobst);
            }
        }
    }

    // Actualizamos datos desde el archivo .tbl
    // Básicamente son U de particiones interiores
    if let Some(tblpath) = &tblpath {
        let tbldata = tbl::parse(&tblpath).unwrap();
        #[allow(unused_assignments, unused_variables)]
        for (name, boundary, mut value) in &mut ecdata.walls_u {
            if *boundary != Boundaries::INTERIOR {
                continue;
            };
            let w = tbldata.elements.get(name.as_str()).unwrap();
            value = fround2(w.u);
        }
    }
}
