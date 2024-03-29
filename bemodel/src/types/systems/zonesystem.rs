// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Sistemas de zona : ZoneSystem
//!
//! Se relaciona con los espacios y los sistemas secundarios de distribución generales
//! 
//! TODO: algunas propiedades de los espacios podrían calcularse con datos de los sistemas:
//! - nivel de acondicionamiento, a partir de datos de zona (si pertenece o no a una zona y si tiene asignadas consignas, etc)
//! - nivel de ventilación (n_v), ¿a partir de oa_flow de zona en m³/h?

use serde::{Deserialize, Serialize};

use super::super::Uuid;
// use crate::utils::{default_1, default_true, is_default, is_true, multiplier_is_1};

// Elementos -----------------------------------------------

/// Sistemas secundarios de distribución específicos de una zona térmica
///
/// Datos de las zonas térmicas abastecidas por los sistemas:
/// - Termostato (consignas, tipo, etc) de zona
/// - Caudales de zona (impulsión, ventilación y extracción)
/// - Capacidades de calefacción / refrigeración
/// 
/// TODO::
/// - aclarar relación con multiplicadores de espacio (es igual si no se define?)
/// - aclarar relación con tipos de espacios
/// - aclarar relación con n_v de espacios
/// - los espacios definen sus cargas / perfil de uso (SPACE-CONDITIONS) según su tipo
///   y las condiciones operacionales / termostatos (SYSTEM-CONDITIONS)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneSystem {
    /// ID de la zona (en formato UUID)
    pub id: Uuid,

    /// Nombre del espacio
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Espacio(s) asociado(s)
    /// A través del espacio se definen las condiciones operacionales y las cargas
    pub space: Option<Uuid>,

    /// Sistema(s) secundario(s) asignado(s) a la(s) zona(s)
    pub system: Option<Vec<Uuid>>,

    // --- Caudales
    /// Caudal de impulsión de diseño de la zona, m³/h
    /// Si no se define usa la disponible por el sistema
    pub design_flow: Option<f32>,

    // -- Ventilador de extracción
    // TODO: ¿debería ser esto una referencia a un ventilador (zone equipment)?
    /// Caudal de extracción, m³/h
    pub exh_flow: Option<f32>,
    /// Potencia de extracción, kW
    pub exh_kw: Option<f32>,

    // -- Aire exterior --
    /// Caudal de aire primario:
    /// - mínimo por persona con máxima ocupación, m³/h
    /// - total, m³/h
    pub oa_flow: Option<AirFlow>,

    // --- Unidades terminales
    /// Potencia nominal total de refrigeración (sensible + latente) de las unidades terminales, kW
    /// La potencia nominal sensible de refrigeración de la unidad terminal se
    /// supone igual al 75% de la total
    /// Si no se define usa la disponible por el sistema
    pub cool_cap: Option<f32>,

    // -- Calefacción --
    /// Potencia nominal de calefacción de las unidades terminales, kW
    /// Si no se define usa la disponible por el sistema
    pub heat_cap: Option<f32>,
}

impl Default for ZoneSystem {
    fn default() -> Self {
        ZoneSystem {
            id: Uuid::new_v4(),
            name: "Zona".to_string(),
            space: None,
            system: None,
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
