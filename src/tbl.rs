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

use std::str::FromStr;

use failure::Error;
use failure::ResultExt;

use crate::utils::read_latin1_file;

#[derive(Debug)]
pub enum ElemType {
    WALL = 0, // muro
    WINDOW = 1, // hueco
    WALLADB = -2, // muro adiabático
    FLOORGND = -3, // solera con el terreno
    WALLINT = -4, // tabique interior                                          
    FLOORINT = -5 // forjado interior
}

#[derive(Debug)]
pub struct Element {
  pub name: String, // Nombre del elemento
  pub area: f32, // Área del elemento en m2
  pub u: f32, // Transmitancia térmica en W/m2K
  pub w_or_inf: f32, // Peso en kg/m2 (opacos) o permeabilidad a 100 Pa en m3/hm2 (huecos)
  pub g_winter: f32, // 0.000000 (opacos) o factor solar en invierno (huecos)
  pub g_summer: f32, // 0.000000 (opacos) o factor solar en verano (huecos)
  pub ang_north: f32, // Ángulo formado con el norte
  pub tilt: f32, // Inclinación (respecto a la horizontal. 90=vertical, 0=horizontal)
  pub type_: ElemType, // Tipo de elemento
  pub id_surf: i32, // Código de la superficie
  pub id_space: i32, // Código del espacio  
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split_whitespace().collect();
        if data.len() < 10 {
            return Err(format_err!("Número de datos insuficiente. Se esperaban 10 y se encontraron {}", data.len()))
        }
        let name = data[0].to_owned();
        let area = data[1].parse().unwrap();
        let u = data[2].parse().unwrap();
        let w_or_inf = data[3].parse().unwrap();
        let g_winter = data[4].parse().unwrap();
        let g_summer = data[5].parse().unwrap();
        let ang_north = data[6].parse().unwrap();
        let tilt = data[7].parse().unwrap();
        let type_i32 = data[8].parse::<i32>().unwrap();
        let type_ = match type_i32 {
            0 => ElemType::WALL,
            1 => ElemType::WINDOW,
            -2 => ElemType::WALLADB,
            -3 => ElemType::FLOORGND,
            -4 => ElemType::WALLINT,
            -5 => ElemType::FLOORINT,
            _ => return Err(format_err!("Tipo de elemento desconocido"))
        };
        let id_surf = data[9].parse().unwrap();
        let id_space = data[10].parse().unwrap();
        Ok(Element { name, area, u, w_or_inf, g_winter, g_summer, ang_north, tilt, type_, id_surf, id_space })
    }
}

#[derive(Debug)]
pub struct Space {
    pub name: String, // Nombre del espacio
    pub id_space: i32, // Código de la zona
    pub mult: i32, // Multiplicador de la zona
    pub area: f32, // Superficie de la zona en m2
    pub qint: f32 // Fuentes internas medias en W/m2
}

impl FromStr for Space {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split_whitespace().collect();
        if data.len() < 5 {
            return Err(format_err!("Número de datos insuficiente. Se esperaban 5 y se encontraron {}", data.len()))
        }
        let name = data[0].to_owned();
        let id_space = data[1].parse().unwrap();
        let mult = data[2].parse().unwrap();
        let area = data[3].parse().unwrap();
        let qint = data[4].parse().unwrap();
        Ok(Space { name, id_space, mult, area, qint })
    }
}

#[derive(Debug)]
pub struct Tbl {
    pub elements: Vec<Element>,
    pub spaces: Vec<Space>
}

impl Tbl {
    // Calcula la superficie útil sumando la de los espacios asociados a elementos
    pub fn compute_autil(&self, claves: &[&str]) -> f32 {
        let mut a_util = 0.0_f32;
        let mut spaces = Vec::new();
        for &clave in claves.iter() {
            if let Some(elem) = self.elements.iter().find(|e| e.name == clave) {
                spaces.push(elem.id_space);
            };
        }
        spaces.sort();
        spaces.dedup();
        for space_id in spaces {
            if let Some(space) = self.spaces.iter().find(|s| s.id_space == space_id) {
                a_util += space.area * (space.mult as f32);
            } else {
                println!("Espacio con id {} no encontrado!!", space_id);
            }
        }
        (a_util * 100.0).round() / 100.0
    }
}

// Interpreta archivo .tbl de datos de elementos y espacios del modelo
//
// path: ruta del archivo .tbl
pub fn parse(path: &str) -> Result<Tbl, Error> {
    let utf8buf = read_latin1_file(path)?;

    // Líneas, eliminando dos primeras líneas de comentarios iniciales
    let mut lines = utf8buf.lines().collect::<Vec<&str>>().into_iter().skip(2);

    // Número de elementos y espacios
    let nums: Vec<&str> = lines.next()
        .ok_or_else(|| format_err!("No se ha encontrado la línea de número de elementos y espacios"))?
        .split_whitespace().collect();
    let numelements = nums[0].parse::<i32>()
        .context("No se ha podido determinar el número de elementos")?;
    let numspaces = nums[1].parse::<i32>()
        .context("No se ha podido determinar el número de espacios")?;

    // Datos de elementos
    let mut elements: Vec<Element> = Vec::new();
    let mut idxelem: i32 = 0;
    while let Some(line) = lines.next() {
        let name = line.trim_matches('"').trim();
        let values = lines.next()
            .ok_or_else(|| format_err!("No se ha encontrado la línea de propiedades del elemento {}", name))?;
        let element = (name.to_owned() + " " + values).parse::<Element>()
            .context(format!("Formato desconocido del elemento {}", name))?;
        elements.push(element);
        idxelem += 1;
        if idxelem == numelements { break };
    }

    // Datos de espacios
    let mut spaces: Vec<Space> = Vec::new();
    let mut idxspc: i32 = 0;
    while let Some(line) = lines.next() {
        let name = line.trim_matches('"');
        let values = lines.next()
            .ok_or_else(|| format_err!("No se ha encontrado la línea de propiedades del espacio {}", name))?;
        let space = (name.to_owned() + " " + values).parse::<Space>()
            .context(format!("Formato desconocido del espacio {}", name))?;
        spaces.push(space);
        idxspc += 1;
        if idxspc == numspaces { break };
    }

    Ok(Tbl { elements, spaces })
}
