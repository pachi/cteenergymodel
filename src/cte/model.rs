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

use std::collections::BTreeMap;

use failure::Error;
use serde::Serialize;

use super::{Boundaries, Space, ThermalBridge, Wall, WallCons, Window, WindowCons};
use crate::utils::fround2;

// ---------- Estructura general de datos --------------

// #[serde(into = "SimpleEnvolventeCteData", from = "SimpleEnvolventeCteDataVecs")]
#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub climate: String,
    pub envelope: Envelope,
    pub constructions: Constructions,
    pub spaces: BTreeMap<String, Space>,
    // XXX: Elementos temporalmente almacenados mientras no se pueden calcular correctamente
    /// U de muros
    pub walls_u: Vec<(String, Boundaries, f32)>,
    /// Factor de obstrucción de obstáculos remotos
    pub windows_fshobst: Vec<(String, f32)>,
}

impl Model {
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// Calcula la superficie útil [m²]
    /// Computa únicamente los espacios habitables dentro de la envolvente térmica
    pub fn a_util_ref(&self) -> f32 {
        let a_util: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv && s.space_type.as_str() != "NO_HABITABLE" {
                    s.area * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(a_util)
    }

    /// Calcula el volumen bruto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente
    pub fn vol_env_gross(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv {
                    s.area * s.height_gross * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(v_env)
    }
    /// Calcula el volumen neto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente y
    /// descuenta los volúmenes de forjados y cubiertas
    pub fn vol_env_net(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .values()
            .map(|s| {
                if s.inside_tenv {
                    s.area * s.height_net * s.multiplier
                } else {
                    0.0
                }
            })
            .sum();
        fround2(v_env)
    }
}

// ---------- Elementos de la envolvente --------------

/// Elementos de la envolvente térmica
#[derive(Debug, Clone, Default, Serialize)]
pub struct Envelope {
    /// Huecos
    pub windows: BTreeMap<String, Window>,
    /// Opacos
    pub walls: BTreeMap<String, Wall>,
    /// Puentes térmicos
    pub thermal_bridges: BTreeMap<String, ThermalBridge>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Constructions {
    /// Construcciones de huecos
    pub windows: BTreeMap<String, WindowCons>,
    /// Construcciones de opacos
    pub walls: BTreeMap<String, WallCons>,
}
