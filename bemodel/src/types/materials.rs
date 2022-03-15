// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Materiales

use serde::{Deserialize, Serialize};

use super::Uuid;

// Materiales -----------------------------------------------

/// Lista de materiales indexada por Uuid
pub type MaterialsList = Vec<Material>;

/// Elemento opaco (muro, cubierta, suelo, partición)
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
        specificheat: f32,
        // Factor de difusividad al vapor de agua, mu (-)
        vapourdiffusivity: Option<f32>,
    },
    /// Resistencia térmica (R)
    #[serde(rename = "resistance")]
    Resistance {
        /// Resistencia térmica, m²K/W
        resistance: f32,
    },
}
