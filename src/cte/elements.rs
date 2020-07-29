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

use serde::{Deserialize, Serialize};

pub use super::common::{
    Boundaries::{self, *},
    Orientation::*,
    SpaceType::{self, *},
    Tilt::*,
};

/// Hueco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Window {
    /// Nombre del hueco
    pub name: String,
    /// Construcción del hueco
    pub cons: String,
    /// Muro al que pertenece el hueco
    pub wall: String,
    /// Superficie del hueco (m2)
    #[serde(rename(serialize = "A"))]
    pub area: f32,
    /// Factor de obstáculos remotos
    pub fshobst: f32,
}

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wall {
    /// Nombre del elemento opaco
    pub name: String,
    /// Construcción del opaco
    pub cons: String,
    /// Espacio al que pertenece el elemento opaco
    pub space: String,
    /// Espacio adyacente con el que comunica el elemento opaco cuando es interior
    pub nextto: Option<String>,
    /// Condiciones de contorno del cerramiento:
    /// - UNDERGROUND: cerramientos en contacxto con el terreno
    /// - EXTERIOR: cerramientos en contacto con el aire exterior
    /// - INTERIOR: cerramientos en contacto con el aire de otros espacios
    /// - ADIABATIC: cerramientos sin transmisión de calor
    pub bounds: Boundaries,
    /// Superficie neta del elemento opaco (m2)
    #[serde(rename(serialize = "A"))]
    pub area: f32,
    /// Orientación (gamma) [-180,+180] (S=0, E=+90, W=-90)
    /// Medido como azimuth geográfico de la proyección horizontal de la normal a la superficie
    /// Coincide con el criterio de la UNE-EN ISO 52016-1
    /// Difiere del criterio BDL, que parte del norte, con E+ y W-
    pub azimuth: f32,
    /// Inclinación (beta) [0, 180]
    /// Medido respecto a la horizontal y normal hacia arriba (0 -> suelo, 180 -> techo)
    pub tilt: f32,
    /// Profundidad del elemento en el terreno (m)
    /// (solo en cerramientos en contacto con el terreno)
    pub zground: Option<f32>,
}

/// Puente térmico
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Tipo de espacio (ACONDICIONADO, NO_ACONDICIONADO, NO_HABITABLE)
    #[serde(rename(serialize = "type"))]
    pub space_type: SpaceType,
    /// Ventilación, en ren/h
    pub n_v: Option<f32>,
}

/// Definición de construcción de elemento opaco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WallCons {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Resistencia térmica total sin resistencias superficiales (resistencia intrínseca) [m2K/W]
    #[serde(rename(serialize = "R_intrinsic"))]
    pub r_intrinsic: f32,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
}

/// Definición de construcción de hueco o lucernario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowCons {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica total (incluyendo marco, vidrio y efecto de intercalarios y/o cajones de persiana) [W/m2K]
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Fracción de marco [-]
    #[serde(rename(serialize = "Ff"))]
    pub ff: f32,
    /// Factor solar del hueco sin la protección solar activada (g_glwi = g_gln * 0.90) [-]
    pub gglwi: f32,
    /// Factor solar del hueco con la protección solar activada [-]
    pub gglshwi: f32,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    #[serde(rename(serialize = "C_100"))]
    pub infcoeff_100: f32,
}
