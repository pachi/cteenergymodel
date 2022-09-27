// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Zona térmica : Zone
//!
//! Se relaciona con los espacios
//! TODO: algunas propiedades actualmente en los espacios pertenecen a las zonas:
//! - nivel de acondicionamiento, a partir de datos de zona (si pertenece o no a una zona y si tiene asignadas consignas, etc)
//! - nivel de ventilación (n_v), a partir de oa_flow en m³/h

use serde::{Deserialize, Serialize};

use super::super::Uuid;
// use crate::utils::{default_1, default_true, is_default, is_true, multiplier_is_1};

// Elementos -----------------------------------------------

/// Zona
///
/// Datos de las zonas térmicas abastecidas por los sistemas:
/// - Termostato (consignas, tipo, etc)
/// - Caudales de zona (impulsión, ventilación y extracción)
/// - Unidades terminales (potencias, caudales de agua, etc)
///
/// TODO:
/// - aclarar relación con multiplicadores de espacio (es igual si no se define?)
/// - aclarar relación con tipos de espacios
/// - aclarar relación con n_v de espacios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// ID de la zona (en formato UUID)
    pub id: Uuid,

    /// Nombre del espacio
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Espacio(s) asociado(s)
    /// Debería ser uno o más
    pub space: Vec<Uuid>,

    /// Sistema(s) secundario(s) asignado(s) a la(s) zona(s)
    pub system: Option<Vec<Uuid>>,

    // --- Termostatos
    /// Consigna de calefacción
    /// TODO: si no hay, la temperatura no es controlada por la zona
    pub heat_temp_sch: Option<Uuid>,
    /// Consigna de refrigeración
    /// TODO: si no hay, la temperatura no es controlada por la zona
    pub cool_temp_sch: Option<Uuid>,

    // --- Caudales
    /// Caudal de impulsión de diseño de la zona, m³/h
    /// Si no se define usa la disponible por el sistema
    pub design_flow: Option<f32>,

    // -- Ventilador de extracción
    /// Caudal de extracción, m³/h
    pub exh_flow: Option<f32>,
    /// Potencia de extracción, kW
    /// TODO: ¿debería ser esto una referencia a un ventilador (zone equipment)?
    pub exh_kw: Option<f32>,

    // -- Aire exterior --
    /// Caudal de aire primario:
    /// - mínimo por persona con máxima ocupación, m³/h
    /// - total, m³/h
    /// TODO: confirmar si el caudal total es constante o con máxima ocupación
    pub oa_flow: Option<AirFlow>,

    // --- Unidades terminales
    /// Potencia nominal total de refrigeración (sensible + latente) de las unidades terminales, kW
    /// La potencia nominal sensible de refrigeración de la unidad terminal se
    /// supone igual al 75% de la total
    /// Si no se define usa la disponible por el sistema
    /// TODO: ¿debería esto ir a zone equipment?
    pub cool_cap: Option<f32>,

    // -- Calefacción --
    /// Potencia nominal de calefacción de las unidades terminales, kW
    /// Si no se define usa la disponible por el sistema
    /// TODO: ¿debería ir esto a zone equipment?
    pub heat_cap: Option<f32>,
}

impl Default for Zone {
    fn default() -> Self {
        Zone {
            id: Uuid::new_v4(),
            name: "Zona".to_string(),
            space: vec![],
            system: None,
            heat_temp_sch: None,
            cool_temp_sch: None,
            design_flow: None,
            exh_flow: None,
            exh_kw: None,
            oa_flow: None,
            cool_cap: None,
            heat_cap: None,
        }
    }
}

/// Definición del flujo de aire primario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AirFlow {
    // Caudal de aire por superficie, m³/h·m²
    // PerArea(f32)
    /// Caudal de aire por persona con ocupación máxima, m³/h
    PerPerson(f32),
    /// Caudal de aire total, m³/h
    Total(f32),
    /// Caudal de aire en renovaciones por hora, 1/h
    Changes(f32),
}

impl Default for AirFlow {
    fn default() -> Self {
        AirFlow::Total(0.0)
    }
}
