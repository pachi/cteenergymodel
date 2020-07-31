// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

pub mod cte;
pub mod parsers;
pub mod utils;

use std::{convert::TryFrom, path::Path};

use anyhow::{format_err, Error};

use cte::{Boundaries, ExtraData, Model};
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
) -> Result<Model, Error> {
    // Carga .ctehexml y BBDD HULC
    let ctehexmlpath = &ctehexmlpath.ok_or_else(|| {
        format_err!("No se ha podido localizar el archivo .ctehexml del proyecto")
    })?;
    // Genera Model desde BDL
    let ctehexmldata = ctehexml::parse_with_catalog(&ctehexmlpath)?;
    let mut ecdata = Model::try_from(&ctehexmldata)?;
    // Interpreta .kyg y añade datos que faltan con archivos adicionales
    fix_ecdata_from_extra(&mut ecdata, kygpath, tblpath);
    // Devuelve datos ampliados y corregidos (U, Fshobst)
    Ok(ecdata)
}

/// Incorpora datos que no se obtienen desde el xml y añade datos extra cuando el valor calculado y el obtenido no coinciden
pub fn fix_ecdata_from_extra<T: AsRef<Path>>(
    ecdata: &mut Model,
    kygpath: Option<T>,
    tblpath: Option<T>,
) {
    let mut extra = ecdata
        .walls
        .values()
        .map(|w| ExtraData {
            name: w.name.clone(),
            bounds: w.bounds,
            tilt: w.tilt.into(),
            cons: w.cons.clone(),
            u: 0.0,
            computed_u: ecdata.u_for_wall(w),
        })
        .collect::<Vec<_>>();

    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------
    // Interpreta .kyg y añade datos que faltan
    if let Some(kygpath) = &kygpath {
        let kygdata = kyg::parse(&kygpath).unwrap();

        // Modifica U de muros con datos del .kyg
        // XXX: hay que tener cuidado porque estos valores tienen desviaciones con los que se muestran en
        // XXX: pantalla del HE1 en el caso de cerramientos interiores en contacto con otros espacios
        // XXX: no habitables/ acondicionados y en elementos en contacto con el terreno.
        // XXX: probablemente se deba a que HULC calcula con DA DB-HE1 o como DOE2 y no con UNE-EN 13789
        for e in &mut extra {
            let wallname = e.name.as_str();
            let kygwall = kygdata.walls.get(wallname);
            if let Some(kw) = kygwall {
                e.u = fround2(kw.u);
            }
        }

        // Modifica fshobst con datos del .kyg
        for win in ecdata.windows.values_mut() {
            let kygwin = kygdata.windows.get(&win.name);
            if let Some(kw) = kygwin {
                win.fshobst = fround2(kw.fshobst);
            }
        }
    }

    // Actualizamos datos de U de particiones interiores desde el archivo .tbl
    if let Some(tblpath) = &tblpath {
        let tbldata = tbl::parse(&tblpath).unwrap();
        #[allow(unused_assignments, unused_variables)]
        for e in &mut extra {
            if e.bounds != Boundaries::INTERIOR {
                continue;
            };
            let w = tbldata.elements.get(e.name.as_str()).unwrap();
            e.u = fround2(w.u);
        }
    }

    extra.retain(|e| e.u != e.computed_u);

    ecdata.extra = Some(extra);
}
