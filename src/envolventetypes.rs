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
    pub clima: String,
    pub envolvente: ElementosEnvolvente,
    pub espacios: Vec<Space>,
}

impl EnvolventeCteData {
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// Calcula la superficie útil [m²]
    /// Computa únicamente los espacios habitables dentro de la envolvente térmica
    pub fn a_util_ref(&self) -> f32 {
        let a_util: f32 = self
            .espacios
            .iter()
            .map(|s| {
                if s.dentroet && s.tipo.as_str() != "NOHABITABLE" {
                    s.area * s.mult
                } else {
                    0.0
                }
            })
            .sum();
        (a_util * 100.0).round() / 100.0
    }
}

// ---------- Elementos de la envolvente --------------

#[derive(Debug, Serialize)]
pub struct ElementosEnvolvente {
    pub huecos: Vec<Window>,
    pub opacos: Vec<Opaque>,
    pub pts: Vec<TB>,
}

#[derive(Debug, Serialize)]
pub enum OpaqueTypes {
    Roof,
    Wall,
    Floor,
    Ground,
    Interior,
    Adiabatic,
    // Window,
    // ThermalBridge,
}

/// Hueco
#[derive(Debug, Serialize)]
pub struct Window {
    /// UID del hueco
    pub id: String,
    /// Nombre del hueco
    pub nombre: String,
    /// Orientación del hueco (N, S, E, W, H...)
    pub orientacion: String,
    /// Superficie del hueco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Fracción de marco
    #[serde(rename(serialize = "Ff"))]
    pub ff: f32,
    /// Factor solar del hueco con la protección solar activada
    pub gglshwi: f32,
    /// Factor de obstáculos remotos
    #[serde(rename(serialize = "Fshobst"))]
    pub fshobst: f32,
}

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Serialize)]
pub struct Opaque {
    /// UID del elemento opaco
    pub id: String,
    /// Nombre del elemento opaco
    pub nombre: String,
    /// Superficie del elemento opaco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Coeficiente de transmisión del elemento opaco (-)
    pub btrx: f32, // 0 | 1

                   // TODO: propiedades que se podrían incorporar
                   // Orientación del elemento opaco (N, S, E, W, H...)
                   // pub orientacion: String,
                   // Absortividad del elemento opaco (-)
                   //pub abs: f32,
                   // Tipo - Muro, cubierta, suelo, terreno, adiabático, partición interior
                   // Orientación - azimuth criterio 52016 (distinto en BDL) ->(0 -> sur)
                   // Inclinación - respecto a la horizontal y hacia arriba (0 -> suelo, 180 -> techo)
}

/// Puente térmico
#[derive(Debug, Serialize)]
pub struct TB {
    /// UID del puente térmico
    pub id: String,
    /// Nombre del puente térmico
    pub nombre: String,
    /// Longitud del puente térmico (m)
    #[serde(rename(serialize = "L"))]
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmic (W/mK)
    pub psi: f32,
}

/// Espacio
#[derive(Debug, Serialize)]
pub struct Space {
    /// Nombre del espacio
    pub nombre: String,
    /// Superficie de la zona en m2
    pub area: f32,
    /// Altura libre de la zona en m
    pub altura: f32,
    /// Pertenencia al interior de la envolvente térmica
    pub dentroet: bool,
    /// Multiplicador
    pub mult: f32,
    // Tipo de espacio (ACONDICIONADO, NOACONDICIONADO, NOHABITABLE)
    pub tipo: String,
}
