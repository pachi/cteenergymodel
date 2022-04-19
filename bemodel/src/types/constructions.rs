// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Construcciones de la envolvente térmica: WallCons, WinCons

use serde::{Deserialize, Serialize};

use super::Uuid;
use crate::utils::fround3;

// Elementos -----------------------------------------------

/// Base de datos de construcciones de opacos y huecos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsDb {
    /// Construcciones de opacos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wallcons: Vec<WallCons>,
    /// Construcciones de huecos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wincons: Vec<WinCons>,
}

impl ConsDb {
    /// Localiza construcción de opaco por id
    pub fn get_wallcons(&self, id: Uuid) -> Option<&WallCons> {
        self.wallcons.iter().find(|w| w.id == id)
    }

    /// Localiza construcción de hueco por id
    pub fn get_wincons(&self, id: Uuid) -> Option<&WinCons> {
        self.wincons.iter().find(|w| w.id == id)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.wallcons.is_empty() && self.wincons.is_empty()
    }
}

/// Definición de construcción de elemento opaco
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallCons {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    #[serde(default)]
    pub group: String,
    /// Capas que forman la construcción de opaco, como lista de tuplas (material, espesor)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

impl Default for WallCons {
    fn default() -> Self {
        WallCons {
            id: Uuid::new_v4(),
            name: "Construcción de opaco".to_string(),
            group: String::default(),
            layers: Vec::default(),
            absorptance: 0.7,
        }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinCons {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    #[serde(default)]
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
    /// Si no se define (valor None), se supone igual al factor solar sin la protección activada (g_gl;wi)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g_glshwi: Option<f32>,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    pub c_100: f32,
}

impl Default for WinCons {
    fn default() -> Self {
        WinCons {
            id: Uuid::new_v4(),
            name: "Construcción de hueco".to_string(),
            group: String::default(),
            glass: Uuid::default(),
            frame: Uuid::default(),
            f_f: 0.20,
            delta_u: 0.0,
            g_glshwi: None,
            c_100: 50.0,
        }
    }
}

