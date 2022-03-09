// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Construcciones de la envolvente térmica: WallCons, WindowCons

use super::Uuid;
use serde::{Deserialize, Serialize};

// Elementos -----------------------------------------------

/// Definición de construcción de elemento opaco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WallCons {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    #[serde(default)]
    pub group: String,
    /// Capas que forman la construcción de opaco, como lista de tuplas (material, espesor)
    #[serde(default)]
    pub layers: Vec<(String, f32)>,
    /// Grosor del elemento (m) [0.0-]
    pub thickness: f32,
    /// Resistencia térmica total sin resistencias superficiales (resistencia intrínseca) [m2K/W]
    #[serde(rename = "R_intrinsic")]
    pub r_intrinsic: f32,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
}

/// Definición de construcción de hueco o lucernario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowCons {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica total (incluyendo marco, vidrio y efecto de intercalarios y/o cajones de persiana) [W/m2K]
    #[serde(rename = "U")]
    pub u: f32,
    /// Fracción de marco [-]
    #[serde(rename = "Ff")]
    pub ff: f32,
    /// Factor solar del hueco sin la protección solar activada (g_glwi = g_gln * 0.90) [-]
    pub gglwi: f32,
    /// Factor solar del hueco con la protección solar activada [-]
    pub gglshwi: f32,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    #[serde(rename = "C_100")]
    pub infcoeff_100: f32,
}
