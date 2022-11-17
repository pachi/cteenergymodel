// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

pub(crate) mod utils;

use std::{convert::TryFrom, path::Path};

use anyhow::{format_err, Error};

use bemodel::{BoundaryType, ExtraData, Model};
use hulc::{ctehexml, kyg, tbl};
use utils::fround2;

/// Nombre del programa
pub const PROGNAME: &str = env!("CARGO_PKG_NAME");

/// Versión del programa
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Texto de descripción, copyright y licencia del programa
pub fn get_copytxt() -> String {
    format!(
        "{} {} - Exportación de datos de HULC a EnvolventeCTE

Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
                   Daniel Jiménez González <danielj@ietcc.csic.es>
                   Marta Sorribes Gil <msorribes@ietcc.csic.es>

Publicado bajo licencia MIT
",
        PROGNAME, VERSION
    )
}

/// Recoge datos desde archivo .ctehexml y, si se indica, del archivo KyGananciasSolares.txt
pub fn collect_hulc_data<T: AsRef<str>>(
    basedir: T,
    use_kyg: bool,
    use_tbl: bool,
) -> Result<Model, Error> {
    let ctehexmlpath = ctehexml::find_ctehexml(&basedir)?;
    let kygpath = if use_kyg {
        kyg::find_kyg(&basedir)?
    } else {
        None
    };
    let tblpath = if use_tbl {
        tbl::find_tbl(&basedir)?
    } else {
        None
    };

    // Carga .ctehexml y BBDD HULC
    let ctehexmlpath = &ctehexmlpath.ok_or_else(|| {
        format_err!("No se ha podido localizar el archivo .ctehexml del proyecto")
    })?;
    // Genera Model desde BDL
    let ctehexmldata = ctehexml::parse_with_catalog_from_path(ctehexmlpath)?;

    let mut ecdata = Model::try_from(&ctehexmldata)?;
    // Interpreta .kyg y añade datos que faltan con archivos adicionales
    fix_ecdata_from_extra(&mut ecdata, &kygpath, &tblpath);
    // Devuelve datos ampliados y corregidos (U, Fshobst)
    Ok(ecdata)
}

/// Incorpora datos que no se obtienen desde el xml y añade datos extra cuando el valor de U calculado y el obtenido no coinciden
pub fn fix_ecdata_from_extra<T: AsRef<Path>>(
    model: &mut Model,
    kygpath: &Option<T>,
    tblpath: &Option<T>,
) {
    let ind = model.energy_indicators();

    let mut extra = model
        .walls
        .iter()
        .map(|w| ExtraData {
            name: w.name.clone(),
            bounds: w.bounds,
            spacetype: model.get_space(w.space).unwrap().kind,
            nextspace: w.next_to,
            nextspacetype: w
                .next_to
                .as_ref()
                .and_then(|s| model.get_space(*s))
                .map(|s| s.kind),
            tilt: w.geometry.tilt.into(),
            cons: w.cons,
            u: 0.0,
            computed_u: fround2(
                ind.props
                    .walls
                    .get(&w.id)
                    .and_then(|wp| wp.u_value)
                    .unwrap_or(0.0),
            ),
        })
        .collect::<Vec<_>>();

    // Actualizaciones de los datos del ctehexmldata con valores del archivo kyg -------
    // Interpreta .kyg y añade datos que faltan
    // TODO: Los añadimos al overrides... podríamos eliminar el extra
    if let Some(kygpath) = &kygpath {
        let kygdata = kyg::parse_from_path(kygpath).unwrap();

        // Modifica U de muros con datos del .kyg
        // XXX: hay que tener cuidado porque estos valores tienen desviaciones con los que se muestran en
        // XXX: pantalla del HE1 en el caso de cerramientos interiores en contacto con otros espacios
        // XXX: no habitables/ acondicionados y en elementos en contacto con el terreno.
        // XXX: probablemente se deba a que HULC calcula con DA DB-HE1 o como DOE2 y no con UNE-EN 13789
        for e in &mut extra {
            let wallname = e.name.as_str();
            let kygwall = kygdata.walls.get(wallname);
            if let Some(kw) = kygwall {
                let u_value_override = fround2(kw.u);
                e.u = u_value_override;

                // Si la diferencia con la U calculada no es significativa no guardamos el dato como override
                let u_diff_significant = f32::abs(e.u - e.computed_u) > 0.001;
                if !u_diff_significant {
                    continue;
                };

                if let Some(wall) = model.get_wall_by_name(wallname) {
                    let wall_id = wall.id;
                    // Actualiza overrides
                    let mut props = model.overrides.walls.entry(wall_id).or_default();
                    props.u_value = Some(u_value_override);
                }
            }
        }

        // Guarda overrides de f_shobst con datos del .kyg
        for win in model.windows.iter_mut() {
            let kygwin = kygdata.windows.get(&win.name);
            if let Some(kw) = kygwin {
                let f_shobst_override = fround2(kw.fshobst);

                // Guardamos como override si la diferencia con el fshobst calculado es significativo o este no existe
                let fshobst_diff_significant = ind
                    .props
                    .windows
                    .get(&win.id)
                    .and_then(|wp| wp.f_shobst)
                    .map_or(true, |computed| {
                        f32::abs(computed - f_shobst_override) > 0.01
                    });
                if !fshobst_diff_significant {
                    continue;
                };
                let mut props = model.overrides.windows.entry(win.id).or_default();
                props.f_shobst = Some(f_shobst_override);
            }
        }
    }

    // Actualizamos datos de U de particiones interiores desde el archivo .tbl
    if let Some(tblpath) = &tblpath {
        let tbldata = tbl::parse(tblpath).unwrap();
        for e in &mut extra {
            if e.bounds != BoundaryType::INTERIOR {
                continue;
            };
            let w = tbldata.elements.get(e.name.as_str()).unwrap();
            let u_value_override = fround2(w.u);
            e.u = u_value_override;

            // Si la diferencia con la U calculada no es significativa no guardamos el dato como override
            let u_diff_significant = f32::abs(e.u - e.computed_u) > 0.001;
            if !u_diff_significant {
                continue;
            };

            // Actualiza overrides
            if let Some(wall) = model.get_wall_by_name(&w.name) {
                let wall_id = wall.id;
                let mut props = model.overrides.walls.entry(wall_id).or_default();
                props.u_value = Some(u_value_override);
            }
        }
    }

    extra.retain(|e| f32::abs(e.u - e.computed_u) > 0.001);

    model.extra = Some(extra);
}
