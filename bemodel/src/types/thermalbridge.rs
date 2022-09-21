// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Puentes térmicos lineales: ThermalBridge, ThermalBridgeKind

use serde::{Deserialize, Serialize};

use super::Uuid;
use crate::utils::is_default;

// Elementos -----------------------------------------------

/// Puente térmico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalBridge {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre del puente térmico
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Tipo de puente térmico
    /// Roof|Balcony|Corner|IntermediateFloor|InternalWall|GroundFloor|Pillar|Window|Generic
    #[serde(default, skip_serializing_if = "is_default")]
    pub kind: ThermalBridgeKind,
    /// Longitud del puente térmico (m)
    #[serde(default, skip_serializing_if = "is_default")]
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
    #[serde(default, skip_serializing_if = "is_default")]
    pub psi: f32,
}

impl Default for ThermalBridge {
    fn default() -> Self {
        ThermalBridge {
            id: Uuid::new_v4(),
            name: "Puente térmico".to_string(),
            kind: ThermalBridgeKind::default(),
            l: 1.0,
            psi: 0.0,
        }
    }
}

/// Tipo de puente térmico según el tipo de elementos conectados
///
/// Los elementos conectados pueden ser:
///     cubiertas, balcones, fachadas, soleras / cámaras sanitarias,
///     pilares, huecos, particiones interiores, forjados (suelos interiores)
/// Usamos abreviaturas similares a las de la norma UNE-EN ISO 14683
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThermalBridgeKind {
    /// Cubierta-fachada (R)
    ROOF,
    /// Balcón-fachada (B)
    BALCONY,
    /// Fachada-fachada (C)
    CORNER,
    /// Suelo interior-fachada (IF)
    INTERMEDIATEFLOOR,
    /// Partición interior (muro)-fachada o Partición interior(muro)-cubierta (IW)
    INTERNALWALL,
    /// Solera-fachada, Cámara sanitaria-fachada o Muro enterrado-fachada (GF)
    GROUNDFLOOR,
    /// Pilar (P)
    PILLAR,
    /// Contorno de hueco, ventana o puerta (W)
    WINDOW,
    /// Genérico, otros (G)
    GENERIC,
}

impl Default for ThermalBridgeKind {
    fn default() -> Self {
        Self::GENERIC
    }
}
