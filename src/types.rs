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

use std::fmt::Display;

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

#[derive(Debug, Default, Serialize)]
pub struct EnvelopeElements {
    pub windows: Vec<Window>,
    pub walls: Vec<Wall>,
    pub thermal_bridges: Vec<ThermalBridge>,
}

/// Posiciones de los cerramientos según su inclinación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Positions {
    /// Suelo (inclinación < 60º)
    BOTTOM,
    /// Cubierta (inclinación > 120º)
    TOP,
    /// Muro (inclinación entre 60 y 120º)
    SIDE,
}

/// Condiciones de contorno de los cerramientos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Boundaries {
    /// Cerramiento en contacto con el aire exterior
    EXTERIOR,
    /// Cerramiento en contacto con el aire de otro espacio
    INTERIOR,
    /// Cerramiento en contacto con el terreno
    UNDERGROUND,
    /// Cerramiento sin transmisión térmica
    ADIABATIC,
}

impl Display for Boundaries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Boundaries::EXTERIOR => "EXTERIOR",
            Boundaries::INTERIOR => "INTERIOR",
            Boundaries::UNDERGROUND => "UNDERGROUND",
            Boundaries::ADIABATIC => "ADIABATIC",
        };
        write!(f, "{}", printable)
    }
}

impl Default for Boundaries {
    fn default() -> Self {
        Boundaries::EXTERIOR
    }
}

/// Hueco
#[derive(Debug, Default, Serialize)]
pub struct Window {
    /// Nombre del hueco
    pub name: String,
    /// Muro al que pertenece el hueco
    pub wall: String,
    /// Orientación del hueco (N, S, E, W, H...)
    pub orientation: String,
    /// Superficie del hueco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    /// Esta transmitancia incluye el efecto del marco, vidrio e incremento de u por intercalarios y cajones de persiana
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
#[derive(Debug, Default, Serialize)]
pub struct Wall {
    /// Nombre del elemento opaco
    pub name: String,
    /// Espacio al que pertenece el elemento opaco
    pub space: String,
    /// Espacio adyacente con el que comunica el elemento opaco
    pub nextto: Option<String>,
    /// Superficie del elemento opaco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Transmitancia térmica (W/m2K)
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Condiciones de contorno del cerramiento:
    /// - UNDERGROUND: cerramientos en contacxto con el terreno
    /// - EXTERIOR: cerramientos en contacto con el aire exterior
    /// - INTERIOR: cerramientos en contacto con el aire de otros espacios
    /// - ADIABATIC: cerramientos sin transmisión de calor
    pub bounds: Boundaries,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
    /// Orientación (gamma) [-180,+180]
    /// Medido como azimuth geográfico de la proyección horizontal de la normal a la superficie
    /// Criterio: medido desde el sur, positivo al este, negativo al oeste
    /// Corresponde al criterio de la UNE-EN ISO 52016-1. S=0, E=+90, W=-90
    /// Difiere del criterio BDL, que parte del norte, con E+ y W-
    pub orientation: f32,
    /// Inclinación (beta) [0, 180]
    /// Medido respecto a la horizontal y normal hacia arriba (0 -> suelo, 180 -> techo)
    pub tilt: f32,
}

/// Puente térmico
#[derive(Debug, Default, Serialize)]
pub struct ThermalBridge {
    /// Nombre del puente térmico
    pub name: String,
    /// Longitud del puente térmico (m)
    #[serde(rename(serialize = "L"))]
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
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
