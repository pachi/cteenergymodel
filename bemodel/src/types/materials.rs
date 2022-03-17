// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Materiales

use serde::{Deserialize, Serialize};

use super::{fround2, Uuid};

// Materiales -----------------------------------------------

/// Base de datos de materiales
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatsDb {
    /// Lista de materiales para elementos opacos (muro, cubierta, suelo, partición)
    pub materials: Vec<Material>,
    // /// Lista de vidrios
    pub glasses: Vec<Glass>,
    // /// Lista de marcos
    pub frames: Vec<Frame>,
}

impl MatsDb {
    /// Localiza material de opaco por id
    pub fn get_material(&self, id: Uuid) -> Option<&Material> {
        self.materials.iter().find(|w| w.id == id)
    }

    /// Localiza vidrio por id
    pub fn get_glass(&self, id: Uuid) -> Option<&Glass> {
        self.glasses.iter().find(|w| w.id == id)
    }

    /// Localiza marco por id
    pub fn get_frame(&self, id: Uuid) -> Option<&Frame> {
        self.frames.iter().find(|w| w.id == id)
    }
}

/// Material de elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// ID del material (UUID)
    pub id: Uuid,
    /// Nombre del material
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Definición de propiedades, detallada (lambda, rho, C_p, mu, ...) o solo resistencia
    #[serde(flatten)]
    pub properties: MatProps,
}

/// Tipos de propiedades de materiales
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatProps {
    /// Lista detallada de propiedades de materiales (lambda, rho, C_p, mu)
    #[serde(rename = "properties")]
    Detailed {
        // Conductividad térmica, lambda (W/mK)
        conductivity: f32,
        // Densidad, rho (kg/m3)
        density: f32,
        // Calor específico, C_p (J/kg K) (valor por defecto 800 J/kg·K)
        specific_heat: f32,
        // Factor de difusividad al vapor de agua, mu (-)
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        vapour_diff: Option<f32>,
    },
    /// Resistencia térmica (R)
    #[serde(rename = "resistance")]
    Resistance {
        /// Resistencia térmica, m²K/W
        resistance: f32,
    },
}

/// Vidrio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Glass {
    /// ID del vidrio (UUID)
    pub id: Uuid,
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Conductividad W/m²K
    pub u_value: f32,
    /// Factor solar del vidrio a incidencia normal
    pub g_gln: f32,
}

impl Glass {
    /// Transmitancia térmica total del acristalmiento (g_glwi = g_gln * 0.90) [-]
    /// Corresponde al factor solar sin protección solar activada
    pub fn g_glwi(&self) -> f32 {
        fround2(self.g_gln * 0.90)
    }
}

/// Marco de hueco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frame {
    /// ID del marco (UUID)
    pub id: Uuid,
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica, U (W/m²K)
    pub u_value: f32,
    /// Absortividad del marco, alpha (-)
    pub absorptivity: f32,
}
