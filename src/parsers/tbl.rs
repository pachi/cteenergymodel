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

//! Funciones de interpretación de archivos .tbl (NewBDL_O.tbl)
//!
//! Formato documentado en HULC:
//!
//! ```text
//!     #Línea de comentarios
//!     #Línea de comentarios
//!     Número de elementos [ESPACIO] Número de zonas
//!     "Nombre del elemento"
//!     #NOTA: en caso de ser un elemento opaco
//!     Area en m2 [ESPACIO] Transmitancia térmica en W/m2K [ESPACIO] Peso en kg/m2 [ESPACIO] 0.000000 [ESPACIO] 0.000000 [ESPACIO] Ángulo formado con el norte [ESPACIO] Inclinación [ESPACIO] Tipo de elemento (0 muro, 1 hueco, -2 muro interior adiabático, -3 forjado con el terreno, -4 tabique, -5 forjado interior) [ESPACIO] Código de la superficie [ESPACIO] Código del espacio [ESPACIO]
//!     #NOTA: en caso de ser un elemento semitransparente
//!     Area en m2 [ESPACIO] Transmitancia térmica en W/m2K [ESPACIO] Infiltración a 100 Pa en m3/hm2 [ESPACIO] Factor solar en invierno [ESPACIO] Factor solar en verano [ESPACIO] Ángulo formado con el norte [ESPACIO] Inclinación [ESPACIO] Tipo de elemento [ESPACIO] Código de la superficie [ESPACIO] Código del espacio [ESPACIO]
//!     #NOTA: Las líneas 6 y 8 se repiten tantas veces como elementos tenga el edificio
//!     "Nombre de la zona"
//!     Código de la zona [ESPACIO] Multiplicador de la zona [ESPACIO] Superficie de la zona en m2 [ESPACIO] Fuentes internas medias en W/m2
//!     #NOTA: Las líneas 10 y 11 se repiten tantas veces como zonas tenga el edificio
//! ```

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{bail, format_err, Context, Error};

use crate::utils::{find_file_in_basedir, read_latin1_file};

/// Localiza archivo NewBDL_O.txt en el directorio de proyecto basedir
pub fn find_tbl<T: AsRef<str>>(basedir: T) -> Result<Option<PathBuf>, Error> {
    // XXX: No serviría para elementos especiales, que usan los NewBDL_O2.txt?
    find_file_in_basedir(basedir, "NewBDL_O.tbl")
}

/// Tipos de elementos definidos en archivo .tbl
#[derive(Debug)]
pub enum ElemType {
    /// Elemento opaco (muro, suelo o cubierta) en contacto con el aire exterior
    EXTWALL = 0,
    /// Hueco
    WINDOW = 1,
    /// Muro adiabático
    ADBWALL = -2,
    /// Muro o suelo en contacto con el terreno
    GNDWALL = -3,
    /// Muro interior
    INTWALL = -4,
    /// Forjado interior
    INTFLOOR = -5,
}

impl FromStr for ElemType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ElemType::EXTWALL),
            "1" => Ok(ElemType::WINDOW),
            "-2" => Ok(ElemType::ADBWALL),
            "-3" => Ok(ElemType::GNDWALL),
            "-4" => Ok(ElemType::INTWALL),
            "-5" => Ok(ElemType::INTFLOOR),
            _ => Err(format_err!("Tipo de elemento desconocido")),
        }
    }
}

// Elemento opaco o transparente definido en archivo .tbl
#[derive(Debug)]
pub struct Element {
    /// Nombre del elemento
    pub name: String,
    /// Área del elemento en m2
    pub area: f32,
    /// Transmitancia térmica en W/m2K
    pub u: f32,
    /// Peso en kg/m2 (opacos) o permeabilidad a 100 Pa en m3/hm2 (huecos)
    pub w_or_inf: f32,
    /// 0.000000 (opacos) o factor solar en invierno (huecos)
    pub g_winter: f32,
    /// 0.000000 (opacos) o factor solar en verano (huecos)
    pub g_summer: f32,
    /// Ángulo formado con el norte
    pub ang_north: f32,
    /// Inclinación (respecto a la horizontal. 90=vertical, 0=horizontal)
    pub tilt: f32,
    /// Tipo de elemento
    pub type_: ElemType,
    /// Código de la superficie
    pub id_surf: i32,
    /// Código del espacio
    pub id_space: i32,
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split_whitespace().collect();
        if data.len() != 11 {
            return Err(format_err!("Formato incorrecto del elemento: {}", s));
        }
        Ok(Element {
            name: data[0].to_owned(),
            area: data[1].parse()?,
            u: data[2].parse()?,
            w_or_inf: data[3].parse()?,
            g_winter: data[4].parse()?,
            g_summer: data[5].parse()?,
            ang_north: data[6].parse()?,
            tilt: data[7].parse()?,
            type_: data[8].parse()?,
            id_surf: data[9].parse()?,
            id_space: data[10].parse()?,
        })
    }
}

/// Espacio definido en archivo .tbl
#[derive(Debug)]
pub struct Space {
    /// Nombre del espacio
    pub name: String,
    /// Código de la zona
    pub id_space: i32,
    /// Multiplicador de la zona
    pub mult: i32,
    /// Superficie de la zona en m2
    pub area: f32,
    /// Fuentes internas medias en W/m2   
    pub qint: f32,
}

impl FromStr for Space {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split_whitespace().collect();
        if data.len() != 5 {
            return Err(format_err!("Formato incorrecto del espacio: {}", s));
        }
        Ok(Space {
            name: data[0].to_owned(),
            id_space: data[1].parse()?,
            mult: data[2].parse()?,
            area: data[3].parse()?,
            qint: data[4].parse()?,
        })
    }
}

/// Conjunto de elementos y espacios interpretados de un archivo .tbl
#[derive(Debug)]
pub struct Tbl {
    /// Lista de elementos
    pub elements: BTreeMap<String, Element>,
    /// Lista de espacios (zonas térmicas)
    pub spaces: BTreeMap<String, Space>,
}

/// Interpreta archivo .tbl de datos de elementos y espacios del modelo
///
/// path: ruta del archivo .tbl
pub fn parse<T: AsRef<Path>>(path: T) -> Result<Tbl, Error> {
    let utf8buf = read_latin1_file(path.as_ref())?;

    // Líneas, eliminando dos primeras líneas de comentarios iniciales
    let mut lines = utf8buf.lines().skip(2); //.collect::<Vec<&str>>().into_iter();

    // Número de elementos y espacios
    let nums = lines
        .next()
        .ok_or_else(|| {
            format_err!("Error al leer el archivo .tbl: no se ha localizado el número de elementos y espacios")
        })?
        .split_whitespace()
        .map(|s|
            s.parse::<i32>()
                .with_context(|| "Error al leer el archivo .tbl: no se ha podido determinar el número de elementos")
        )
        .collect::<Result<Vec<i32>,_>>()?;
    if nums.len() < 2 {
        bail!("Error al leer el archivo .tbl: formato incorrecto del número de elementos")
    };
    let numelements = nums[0];
    let numspaces = nums[1];

    // Datos de elementos
    let mut elements: BTreeMap<String, Element> = BTreeMap::new();
    let mut idxelem: i32 = 0;
    while let Some(line) = lines.next() {
        let name = line.trim_matches('"').trim();
        let values = lines.next()
            .ok_or_else(|| format_err!("Error al leer el archivo .tbl: no se ha encontrado la línea de propiedades del elemento {}", name))?;
        let element = (name.to_owned() + " " + values)
            .parse::<Element>()
            .with_context(|| {
                format!(
                    "Error al leer el archivo .tbl: formato desconocido del elemento {}",
                    name
                )
            })?;
        elements.insert(name.to_string(), element);
        idxelem += 1;
        if idxelem == numelements {
            break;
        };
    }

    // Datos de espacios
    let mut spaces: BTreeMap<String, Space> = BTreeMap::new();
    let mut idxspc: i32 = 0;
    while let Some(line) = lines.next() {
        let name = line.trim_matches('"');
        let values = lines.next().ok_or_else(|| {
            format_err!(
                "Error al leer el archivo .tbl: no se ha encontrado la línea de propiedades del espacio {}",
                name
            )
        })?;
        let space = (name.to_owned() + " " + values)
            .parse::<Space>()
            .with_context(|| {
                format!(
                    "Error al leer el archivo .tbl: formato desconocido del espacio {}",
                    name
                )
            })?;
        spaces.insert(name.to_string(), space);
        idxspc += 1;
        if idxspc == numspaces {
            break;
        };
    }

    Ok(Tbl { elements, spaces })
}
