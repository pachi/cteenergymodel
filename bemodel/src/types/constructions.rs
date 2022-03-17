// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Construcciones de la envolvente térmica: WallCons, WindowCons

use serde::{Deserialize, Serialize};

use super::Uuid;
use crate::utils::fround3;

// Elementos -----------------------------------------------

/// Base de datos de construcciones de opacos y huecos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsDb {
    /// Construcciones de opacos
    pub wallcons: Vec<WallCons>,
    /// Construcciones de huecos
    pub wincons: Vec<WindowCons>,
}

impl ConsDb {
    /// Localiza construcción de opaco por id
    pub fn get_wallcons(&self, id: Uuid) -> Option<&WallCons> {
        self.wallcons.iter().find(|w| w.id == id)
    }

    /// Localiza construcción de hueco por id
    pub fn get_wincons(&self, id: Uuid) -> Option<&WindowCons> {
        self.wincons.iter().find(|w| w.id == id)
    }
}

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
    pub layers: Vec<Layer>,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
}

impl WallCons {
    /// Espesor total de una composición de capas [m]
    pub fn thickness(&self) -> f32 {
        fround3(self.layers.iter().map(|Layer { e, .. }| e).sum())
    }
}

/// Definición de capa de opaco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Layer {
    /// ID del material
    pub id: Uuid,
    /// Espesor, m
    pub e: f32,
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
    /// Vidrio del hueco, UUID
    pub glass: Uuid,
    /// Marco del hueco, UUID
    pub frame: Uuid,
    /// Fracción de marco [-]
    pub f_f: f32,
    /// Porcentaje de U debido a intercalarios y cajón de persiana (%)
    pub delta_u: f32,
    /// Factor solar del hueco con la protección solar activada (g_gl;sh;wi) [-]
    /// Si no se define, se supone igual al factor solar sin la protección activada (g_gl;wi)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub g_glshwi: Option<f32>,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    pub c_100: f32,
}
