/* -*- coding: utf-8 -*-

Copyright (c) 2018-2019 Rafael Villar Burke <pachi@ietcc.csic.es>

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

use failure::Error;
use serde::Serialize;
use serde_json;

// ---------- Estructura general de datos --------------

#[derive(Debug, Serialize)]
pub struct EnvolventeCteData {
    #[serde(rename(serialize = "Autil"))]
    pub autil: f32,
    pub clima: String,
    pub envolvente: ElementosEnvolvente,
    // pub spaces: Vec<Spaces>,
}

impl EnvolventeCteData {
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }
}

// ---------- Elementos de la envolvente --------------

#[derive(Debug, Serialize)]
pub struct ElementosEnvolvente {
    pub huecos: Vec<Hueco>,
    pub opacos: Vec<Opaco>,
    pub pts: Vec<PT>,
}

#[derive(Debug, Serialize)]
pub struct Hueco {
    pub id: String,
    pub nombre: String,
    pub orientacion: String,
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    #[serde(rename(serialize = "Ff"))]
    pub ff: f32,
    pub gglshwi: f32,
    #[serde(rename(serialize = "Fshobst"))]
    pub fshobst: f32,
}

#[derive(Debug, Serialize)]
pub struct Opaco {
    pub id: String,
    pub nombre: String,
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    pub btrx: f32, // 0 | 1
}

#[derive(Debug, Serialize)]
pub struct PT {
    pub id: String,
    pub nombre: String,
    #[serde(rename(serialize = "L"))]
    pub l: f32,
    pub psi: f32,
}
