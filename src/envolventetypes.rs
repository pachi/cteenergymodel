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
    pub climate: String,
    pub envelope: EnvelopeElements,
    pub spaces: Vec<Space>,
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
            .spaces
            .iter()
            .map(|s| {
                if s.inside_tenv && s.space_type.as_str() != "NO_HABITABLE" {
                    s.area * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        (a_util * 100.0).round() / 100.0
    }

    /// Calcula el volumen bruto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente
    pub fn vol_env_gross(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .iter()
            .map(|s| {
                if s.inside_tenv {
                    s.area * s.height_gross * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        (v_env * 100.0).round() / 100.0
    }
    /// Calcula el volumen neto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente y
    /// descuenta los volúmenes de forjados y cubiertas
    pub fn vol_env_net(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .iter()
            .map(|s| {
                if s.inside_tenv {
                    s.area * s.height_net * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        (v_env * 100.0).round() / 100.0
    }
}

// ---------- Elementos de la envolvente --------------

#[derive(Debug, Serialize)]
pub struct EnvelopeElements {
    pub windows: Vec<Window>,
    pub walls: Vec<Wall>,
    pub thermal_bridges: Vec<ThermalBridge>,
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
    /// Nombre del hueco
    pub name: String,
    /// Orientación del hueco (N, S, E, W, H...)
    pub orientation: String,
    /// Superficie del hueco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Fracción de marco
    #[serde(rename(serialize = "Ff"))]
    pub ff: f32,
    /// Factor solar del hueco sin la protección solar activada (g_glwi = g_gln * 0.90)
    pub gglwi: f32,
    /// Factor solar del hueco con la protección solar activada
    pub gglshwi: f32,
    /// Factor de obstáculos remotos
    #[serde(rename(serialize = "Fshobst"))]
    pub fshobst: f32,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    #[serde(rename(serialize = "C_100"))]
    pub infcoeff_100: f32,
}

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Serialize)]
pub struct Wall {
    /// Nombre del elemento opaco
    pub name: String,
    /// Superficie del elemento opaco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Coeficiente de transmisión del elemento opaco (-)
    pub btrx: f32, // 0 | 1
    /// Tipo de cerramiento:
    /// - UNDERGROUND-WALL
    /// - EXTERIOR-WALL
    /// - ROOF
    /// - PARTITION
    /// - ADIABATIC
    #[serde(rename(serialize = "type"))]
    pub wall_type: String,
}
// TODO: propiedades que se podrían incorporar a los cerramientos
// Orientación del elemento opaco (N, S, E, W, H...)
// pub orientacion: String,
// Absortividad del elemento opaco (-)
//pub abs: f32,
// Orientación - azimuth criterio 52016 (distinto en BDL) ->(0 -> sur)
// Inclinación - respecto a la horizontal y hacia arriba (0 -> suelo, 180 -> techo)

/// Puente térmico
#[derive(Debug, Serialize)]
pub struct ThermalBridge {
    /// Nombre del puente térmico
    pub name: String,
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
    pub name: String,
    /// Superficie de la zona en m2
    pub area: f32,
    /// Altura libre (suelo a techo) de la zona en m
    /// No incluye el volumen de forjados o cubiertas.
    pub height_net: f32,
    /// Altura bruta (suelo a suelo) de la zona en m
    pub height_gross: f32,
    /// Pertenencia al interior de la envolvente térmica
    pub inside_tenv: bool,
    /// Multiplicador
    pub multiplier: f32,
    // Tipo de espacio (ACONDICIONADO, NO_ACONDICIONADO, NO_HABITABLE)
    #[serde(rename(serialize = "type"))]
    pub space_type: String,
}
