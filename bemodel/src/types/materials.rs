// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Materiales

use serde::{Deserialize, Serialize};

use super::Uuid;

// Materiales -----------------------------------------------

/// Base de datos de materiales
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatsDb {
    /// Lista de materiales para elementos opacos (muro, cubierta, suelo, partición)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<Material>,
    /// Lista de vidrios
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub glasses: Vec<Glass>,
    /// Lista de marcos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

    /// Comprueba si la base de datos está vacía
    pub(crate) fn is_empty(&self) -> bool {
        self.materials.is_empty() && self.glasses.is_empty() && self.frames.is_empty()
    }
}

/// Material de elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// ID del material (UUID)
    pub id: Uuid,
    /// Nombre del material
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    #[serde(default)]
    pub group: String,
    /// Definición de propiedades, detallada (lambda, rho, C_p, mu, ...) o solo resistencia
    #[serde(flatten)]
    pub properties: MatProps,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            id: Uuid::new_v4(),
            name: "Fábrica 1/2' LP G > 80".to_string(),
            group: String::default(),
            properties: MatProps::default(),
        }
    }
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
        // Calor específico, C_p (J/kg K) (valor por defecto 1000 J/kg·K)
        specific_heat: f32,
        // Factor de difusividad al vapor de agua, mu (-)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        vapour_diff: Option<f32>,
    },
    /// Resistencia térmica (R)
    #[serde(rename = "resistance")]
    Resistance {
        /// Resistencia térmica, m²K/W
        resistance: f32,
    },
}

impl Default for MatProps {
    fn default() -> Self {
        // Caso por defecto (Fábrica 1/2' LP G > 80 del CEC)
        MatProps::Detailed {
            conductivity: 0.23,
            density: 900.0,
            specific_heat: 1000.0,
            vapour_diff: Some(10.0),
        }
    }
}

/// Vidrio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    /// ID del vidrio (UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    #[serde(default)]
    pub group: String,
    /// Conductividad W/m²K
    pub u_value: f32,
    /// Factor solar del vidrio a incidencia normal
    pub g_gln: f32,
}

impl Default for Glass {
    fn default() -> Self {
        // Caso por defecto (Acristalamiento vidrio sencillo 6mm vert del CEC)
        Glass {
            id: Uuid::new_v4(),
            name: "Vidrio sencillo 6mm (Vert)".to_string(),
            group: "".to_string(),
            u_value: 5.7,
            g_gln: 0.83,
        }
    }
}

/// Marco de hueco
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    /// ID del marco (UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    #[serde(default)]
    pub group: String,
    /// Transmitancia térmica, U (W/m²K)
    pub u_value: f32,
    /// Absortividad del marco, alpha (-)
    pub absorptivity: f32,
}

impl Default for Frame {
    fn default() -> Self {
        // Caso por defecto (Marco metálico con RPT > 12mm)
        Frame {
            id: Uuid::new_v4(),
            name: "Marco metálico con RPT > 12 mm".to_string(),
            group: "".to_string(),
            u_value: 3.2,
            absorptivity: 0.6,
        }
    }
}

