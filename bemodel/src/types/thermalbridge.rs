// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Puentes térmicos lineales: ThermalBridge, ThermalBridgeKind

use serde::{Deserialize, Serialize};

// Elementos -----------------------------------------------

/// Puente térmico
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThermalBridge {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre del puente térmico
    pub name: String,
    /// Tipo de puente térmico
    /// Roof|Balcony|Corner|IntermediateFloor|InternalWall|GroundFloor|Pillar|Window|Generic
    pub kind: ThermalBridgeKind,
    /// Longitud del puente térmico (m)
    #[serde(rename = "L")]
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
    pub psi: f32,
}

/// Tipo de puente térmico según el tipo de elementos conectados
///
/// Los elementos conectados pueden ser:
///     cubiertas, balcones, fachadas, soleras / cámaras sanitarias,
///     pilares, huecos, particiones interiores, forjados (suelos interiores)
/// Usamos abreviaturas similares a las de la norma UNE-EN ISO 14683
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
