// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Funciones de interpretación de archivos KyGananciasSolares.txt
//!
//! En este archivo no aparecen los elementos adiabáticos entre los cerramientos

use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use anyhow::{bail, format_err, Error};

use crate::utils::file::{find_file_in_basedir, read_latin1_file};

/// Localiza archivo KyGananciasSolares.txt en el directorio de proyecto basedir
pub fn find_kyg<T: AsRef<str>>(basedir: T) -> Result<Option<PathBuf>, Error> {
    find_file_in_basedir(basedir, "KyGananciasSolares.txt")
}

// Elementos definidos en el archivo KyGanaciasSolares
#[derive(Debug, Default)]
pub struct KyGElements {
    /// Valor de K global
    pub k: f32,
    /// Datos de huecos
    pub windows: BTreeMap<String, Window>,
    /// Datos de opacos
    pub walls: BTreeMap<String, Wall>,
    /// Datos de PTs
    pub thermal_bridges: BTreeMap<String, ThermalBridge>,
    /// Factores de insolación - uso desconocido
    pub hfactors: Vec<f32>,
}

/// Hueco
#[derive(Debug, Default)]
pub struct Window {
    /// Nombre del hueco
    pub name: String,
    /// Muro al que pertenece el hueco
    pub wall: String,
    /// Orientación del hueco (N, S, E, W, H...)
    pub orientation: String,
    /// Azimuth, orientación respecto al norte (N=0, E+, W-) [0,360]
    pub azimuth_n: f32,
    /// Superficie del hueco (m2)
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    pub u: f32,
    /// Fracción de marco [0.0-1.0]
    pub ff: f32,
    /// Factor de obstáculos remotos
    pub fshobst: f32,
    // Campos disponibles en versión CTEHE2019
    /// Factor solar a incidencia normal, ggln
    /// El factor solar del hueco sin la protección solar activada se calcula como:
    /// g_glwi = g_gln * 0.90
    pub ggln: Option<f32>,
    /// Desconocido 1
    pub unknown1: Option<f32>,
    /// Desconocido 2
    pub unknown2: Option<f32>,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    pub infcoeff_100: Option<f32>,
    /// Construcción de hueco
    pub cons: Option<String>,
}

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Default)]
pub struct Wall {
    /// Nombre del elemento opaco
    pub name: String,
    /// Superficie del elemento opaco (m2)
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    pub u: f32,
    /// Coeficiente de transmisión del elemento opaco (-)
    pub btrx: f32, // 0 | 1
    /// Tipo de elemento
    /// ["Muro Exteror"|"Separación No Habitable"|"Muro contacto terreno"|"Suelo"|"Cubierta"]
    pub wtype: Option<String>,
    /// Orientación [N|S|E|O|SO|SE|NO|NE]
    pub orientation: Option<String>,
    /// Construcción
    pub cons: Option<String>,
}

/// Puente térmico
#[derive(Debug, Default)]
pub struct ThermalBridge {
    /// Nombre del puente térmico
    pub name: String,
    /// Longitud del puente térmico (m)
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
    pub psi: f32,
    /// Sistema dimensional
    pub sisdim: String,
}

// Lee estructura de datos desde cadena con formato de archivo KyGananciasSolares.txt
pub fn parse(data: &str) -> Result<KyGElements, Error> {
    let lines = data
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>()
        .into_iter();

    let mut kyg = KyGElements::default();
    let mut qsolvalues: HashMap<String, (f32, f32)> = HashMap::default();

    for line in lines {
        // Comentarios y líneas en blanco
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        // Datos de elemento
        if line.starts_with("Muro") || line.starts_with("Ventana") || line.starts_with("PPTT") {
            let vv: Vec<&str> = line.split(';').map(str::trim).collect();
            let tipo = vv[0];
            match tipo {
                "Ventana" => {
                    if vv.len() < 6 {
                        bail!("Línea de datos de hueco con formato desconocido")
                    }
                    let (nombre, a, u, orienta, ff) = (vv[1], vv[2], vv[3], vv[4], vv[5]);
                    let (ggln, unknown1, unknown2, infcoeff_100, cons) = if vv.len() > 10 {
                        (
                            Some(vv[6].replace(',', ".").parse()?),
                            Some(vv[7].replace(',', ".").parse()?),
                            Some(vv[8].replace(',', ".").parse()?),
                            Some(vv[9].replace(',', ".").parse()?),
                            Some(vv[10].to_string()),
                        )
                    } else {
                        (None, None, None, None, None)
                    };
                    kyg.windows.insert(
                        nombre.to_string(),
                        Window {
                            name: nombre.to_string(),
                            orientation: orienta.replace('O', "W").to_string(),
                            azimuth_n: 0.0, // Valor temporal, se completa más abajo
                            wall: String::default(),
                            a: a.replace(',', ".").parse()?,
                            u: u.replace(',', ".").parse()?,
                            ff: ff.replace(',', ".").parse::<f32>()? / 100.0_f32,
                            fshobst: 0.0, // Valor temporal, se completa más abajo
                            ggln,
                            unknown1,
                            unknown2,
                            infcoeff_100,
                            cons,
                        },
                    );
                }
                "Muro" => {
                    if vv.len() < 5 {
                        bail!("Línea de datos de opaco con formato desconocido")
                    }
                    // Datos de muro
                    let (nombre, a, u, btrx) = (vv[1], vv[2], vv[3], vv[4]);
                    // CTEHE2019 y mayores
                    let (wtype, orientation, cons) = if vv.len() > 7 {
                        (
                            Some(vv[5].to_string()),
                            Some(vv[6].to_string()),
                            Some(vv[7].to_string()),
                        )
                    } else {
                        (None, None, None)
                    };

                    kyg.walls.insert(
                        nombre.to_string(),
                        Wall {
                            name: nombre.to_string(),
                            a: a.replace(',', ".").parse()?,
                            u: u.replace(',', ".").parse()?,
                            btrx: btrx.replace(',', ".").parse()?,
                            wtype,
                            orientation,
                            cons,
                        },
                    );
                }
                "PPTT" => {
                    if vv.len() < 4 {
                        bail!("Línea de datos de hueco con formato desconocido")
                    }
                    // En versiones más recientes se añade el sistema dimensional como dato extra
                    let (l, psi, nombre) = (vv[1], vv[2], vv[3]);
                    let sisdim = (if vv.len() > 4 { vv[4] } else { "" }).to_string();
                    kyg.thermal_bridges.insert(
                        nombre.to_string(),
                        ThermalBridge {
                            name: nombre.to_string(),
                            l: l.replace(',', ".").parse()?,
                            psi: psi.replace(',', ".").parse()?,
                            sisdim,
                        },
                    );
                }
                _ => println!("Desconocido"),
            };
        }
        // Ganancias solares de hueco
        else if line.starts_with('"') {
            let vv: Vec<&str> = line.split(';').map(str::trim).collect();
            if vv.len() < 8 {
                bail!("Línea de datos de ganancias solares de hueco con formato desconocido")
            }
            let (name, azimuth_n, _a, htot, _h1, _h2, h3, _ganancia) = (
                vv[0].trim_matches('"').to_string(), // name
                vv[1].parse::<f32>()?, // azimuth (grados respecto al norte, N=0, NE=45, E=90)
                vv[2].parse::<f32>()?, // a - area (m2)
                vv[3].parse::<f32>()?, // htot - radiación solar global en el plano del vidrio sin obstáculos (Wh/m2)
                vv[4].parse::<f32>()?, // _h1 - radiación solar global en el plano del vidrio tras obstáculos remotos (Wh/m2)
                vv[5].parse::<f32>()?, // _h2 - radiación solar global en el plano del vidrio tras obstáculos de fachada (Wh/m2)
                vv[6].parse::<f32>()?, // h3 - radiación solar global en el plano del vidrio tras sombras por lamas (Wh/m2)
                vv[7].parse::<f32>()?, // _ganancia solar a través de este hueco (Wh/m2)
            );
            let fshobst = h3 / htot;
            qsolvalues.insert(name, (azimuth_n, fshobst));
        }
        // K global
        else if line.starts_with("Coeficiente K") {
            let kval: f32 = line
                .split(';')
                .nth(1)
                .ok_or_else(|| format_err!("No se encuentra la definición de K global"))?
                .trim()
                .replace(',', ".")
                .parse()?;
            kyg.k = kval;
        }
        // factores insolación
        else if "012345678".contains(line.chars().next().unwrap_or('x')) {
            let val: f32 = line
                .split(';')
                .nth(1)
                .ok_or_else(|| {
                    format_err!(
                        "Formato inesperado de línea de factor de insolación {}",
                        line
                    )
                })?
                .trim()
                .replace(',', ".")
                .parse()?;
            kyg.hfactors.push(val);
        }
        // rg_qsolunknown
        else {
            continue;
        }
    }

    // Actualización de valores de fshobst disponibles en el KyGananciasSolares.txt
    for (name, mut hueco) in &mut kyg.windows {
        if let Some((azimuth_n, fshobst)) = qsolvalues.get(name) {
            hueco.azimuth_n = *azimuth_n;
            hueco.fshobst = *fshobst;
        }
    }

    Ok(kyg)
}

// Lee estructura de datos desde una ruta de archivo KyGananciasSolares.txt
pub fn parse_from_path<T: AsRef<Path>>(path: T) -> Result<KyGElements, Error> {
    let utf8buf = read_latin1_file(path.as_ref())?;
    parse(&utf8buf)
}
