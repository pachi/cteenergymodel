// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Sistemas secundarios : System
//!
//! Se relacionan con las zonas térmicas

use serde::{Deserialize, Serialize};

use super::{super::Uuid, Carrier};

/// Sistema de ACS
///
/// Son los equipos y dispositivos encargados del tratamiento y distribución del
/// ACS a los puntos de consumo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DhwSystem {
    /// ID del sistema de ACS (en formato UUID)
    pub id: Uuid,

    /// Nombre del sistema
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Espacio(s) asociado(s)
    /// Debería ser uno o más
    pub spaces: Vec<Uuid>,

    // /// Tipo de sistema
    // pub kind: SystemKind,
    /// Bombas de circulación
    pub pumps: Option<Pump>,

    /// Potencial nominal de generación de ACS, kW
    pub dhw_cap: Option<f32>,
    /// Generadores de ACS
    pub dhw_gen: Vec<SysGenerator>,
    // demanda total, l/d?
    // horario de consumo de ACS?
}

impl Default for DhwSystem {
    fn default() -> Self {
        DhwSystem {
            id: Uuid::new_v4(),
            name: "Sistema".to_string(),
            spaces: vec![],
            pumps: None,
            dhw_cap: None,
            dhw_gen: vec![],
        }
    }
}

/// Sistema (subsistema secundario)
///
/// Son los equipos y dispositivos encargados del tratamiento y distribución del
/// aire a los locales.
///
/// Incluye las UTA (sección de baterías (frío y calor), sección de humidificación,
/// de los ventiladores, las zonas térmicas, los termostatos, las unidades
/// terminales, etc.
///
/// En general, los subsistemas secundarios se dividen a nivel de sistema (o de UTA)
/// y de zona.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct System {
    /// ID del sistema (en formato UUID)
    pub id: Uuid,

    /// Nombre del sistema
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Espacio(s) asociado(s)
    /// Debería ser uno o más
    pub spaces: Vec<Uuid>,

    /// Tipo de sistema (lado del aire)
    pub kind: SystemKind,

    /// Zona de control
    pub control_zone: Option<Uuid>,

    /// Bombas de circulación
    pub pumps: Option<Pump>,

    /// Potencia nominal de generación de calor, kW
    pub heating_cap: Option<f32>,
    /// Generadores de calor
    pub heating_gen: Vec<SysGenerator>,

    /// Potencia nominal para generación de frío, kW
    pub cooling_cap: Option<f32>,
    /// Generadores de frío
    pub cooling_gen: Vec<SysGenerator>,

    /// Ventilador de impulsión y retorno
    pub supply_fan: Option<Fan>,

    /// Ventilador de retorno
    pub return_fan: Option<Fan>,

    /// Técnicas de recuperación y opciones
    pub recovery: Option<SysOptions>,
}

impl Default for System {
    fn default() -> Self {
        System {
            id: Uuid::new_v4(),
            name: "Sistema".to_string(),
            kind: SystemKind::Generic,
            spaces: vec![],
            pumps: None,
            control_zone: None,
            heating_cap: None,
            heating_gen: vec![],
            cooling_cap: None,
            cooling_gen: vec![],
            supply_fan: None,
            return_fan: None,
            recovery: None,
        }
    }
}

/// Tipo de sistema secundario
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemKind {
    /// Genérico
    #[default]
    Generic,
}

/// Ventiladores de un subsistema secundario
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fan {
    /// Caudal del ventilador, m³/h
    pub flow: f32,
    /// Potencia del ventilador, kW
    pub kw: f32,
}

/// Técnicas de recuperación de un subsistema secundario
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SysOptions {
    /// Enfriamiento evaporativo
    pub evaporative_cooling: bool,

    /// Economizador del lado del agua
    pub ws_economizer: bool,

    /// Enfriamiento gratuito (economizador del lado del aire)
    /// Se podría acotar si es por temperatura o entalpía
    pub free_cooling: bool,

    /// Recuperación de calor (del aire de extracción)
    /// Se podría acotar si es de calor sensible o entálpico
    /// Si hay, valor por defecto = 0.76
    pub exhaust_recovery_eff: Option<f32>,

    /// Control de humedad - Humidificación / Deshumidificación
    pub humidity_control: bool,
}

/// Generadores térmicos
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SysGenerator {
    /// Generador térmico genérico
    ThermalGenerator {
        /// ID del sistema (en formato UUID)
        id: Uuid,

        /// Nombre del sistema
        #[serde(default, skip_serializing_if = "String::is_empty")]
        name: String,
        fuel: Carrier,
        kind: GeneratorKind,
        heating: Option<ThermalProps>,
        cooling: Option<ThermalProps>,
        dhw: Option<ThermalProps>,
        hw_storage: Option<HotWaterStorageTank>,
    },
    /// Torre de refrigeración
    HeatRejection {
        /// ID del sistema (en formato UUID)
        id: Uuid,

        /// Nombre del sistema
        #[serde(default, skip_serializing_if = "String::is_empty")]
        name: String,
        fuel: Carrier,
        kind: GeneratorKind,
        /// Capacidad nominal de refrigeración en condiciones CTI, kW
        capacity: f32,
        /// Consumo de ventiladores por celda en condiciones nominales, kW
        fan_kw_cell: f32,
        /// Número de celdas
        number_of_cells: u32,
    }, // HeatPump,
       // Chiller,
       // Boiler,
       // DistrictCooling,
       // DistrictHeating,
       // DhWaterHeater,
       // Cogen
       // GroundLoopHeatExchanger
}

/// Tipo de sistema de generación (primario)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneratorKind {
    /// Genérico
    #[default]
    Generic,
}

/// Propiedades de generación
///
/// NOTA: No estamos guardando algunas propiedades que podrían ser útiles como
/// la eficiencia térmica (del segundo vector) en equipos que consumen
/// electricidad y otro vector, como sucede con la absorción por llama directa
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThermalProps {
    /// Potencia nominal de generación, kW
    pub capacity: f32,
    /// Potencia sensible de generación, kW
    /// Si no se aporta, se considera igual a la total o, en frío, al 75% de la total
    pub capacity_sh: Option<f32>,
    /// Eficiencia nominal de generación (COP, EER, eff_th, ...), -
    /// En equipos de generación eléctrica, rendimiento térmico nominal (el_prod / fuel_cons_pcs)
    pub eff: f32,
    /// Eficiencia estacional de generación (SCOP, SEER, eff_th_s), -
    pub eff_season: Option<f32>,
}

/// Acumulador de agua caliente
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotWaterStorageTank {
    /// ID (en formato UUID)
    pub id: Uuid,

    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Volumen, m³
    pub volume: f32,
    /// Coeficiente de pérdidas global del depósito, UA (W/ºC)
    pub ua: f32,
}

/// Bomba de circulación. En circuitos o equipos (como enfriadoras)
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pump {
    /// ID del sistema (en formato UUID)
    pub id: Uuid,

    /// Nombre del sistema
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Tipo de control
    pub kind: PumpKind,
    /// Caudal Q, l/h
    pub flow: f32,
    /// Potencia de la bomba, kW
    /// P = rho ·  g · Q · H / n
    pub capacity: f32,
}

/// Tipo de bomba hidráulica
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PumpKind {
    /// Bomba de caudal constante
    #[default]
    CaudalConstante,
    /// Bomba de caudal variable
    CaudalVariable,
}
