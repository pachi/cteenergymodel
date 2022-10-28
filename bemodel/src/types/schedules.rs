// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Horarios de funcionamiento, consignas, etc: Schedules

use serde::{Deserialize, Serialize};

use super::Uuid;

/// Horarios
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchedulesDb {
    /// Horarios anuales
    /// El total de repeticiones (semanas) debe sumar 52
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub year: Vec<Schedule>,
    /// Horarios semanales
    /// El total de repeticiones (días) debe sumar 7
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub week: Vec<ScheduleWeek>,
    /// Horarios diarios
    /// El total de valores (horas) es 24
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day: Vec<ScheduleDay>,
}

impl SchedulesDb {
    /// Comprueba si la base de datos está vacía
    pub(crate) fn is_empty(&self) -> bool {
        self.year.is_empty() && self.week.is_empty() && self.day.is_empty()
    }
}

/// Horarios anuales
/// Se forman como lista de tuplas de horario semanal y repeticiones
/// El total de repeticiones debe sumar 52 semanas
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schedule {
    /// Id
    pub id: Uuid,
    /// Nombre del horario
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Secuencia de tuplas de UUID de horario semanal y repeticiones
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<(Uuid, u32)>,
}

/// Horarios semanales
/// Se forman como lista de tuplas de horario diario y repeticiones
/// El total de repeticiones debe sumar 7 días
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduleWeek {
    /// Id
    pub id: Uuid,
    /// Nombre del horario
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Secuencia de tuplas de UUID de horarios diarios y repeticiones
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<(Uuid, u32)>,
}

/// Horarios diarios
/// Se forma como lista de valores horarios
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduleDay {
    /// Id
    pub id: Uuid,
    /// Nombre del horario
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Secuencia de valores horarios
    /// Debe tener 24 valores
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<f32>,
}

impl ScheduleDay {
    /// Lista si cada valor es mayor que (casi) cero
    pub fn values_not_zero(&self) -> Vec<bool> {
        // Aprox > 1 e-5
        self.values
            .iter()
            .map(|v| v.abs() > 100.0 * f32::EPSILON)
            .collect()
    }

    /// Número de elementos distintos de (casi) cero
    pub fn count_not_zero(&self) -> usize {
        self.values_not_zero().iter().filter(|v| **v).count()
    }

    /// Valor medio del horario
    pub fn average(&self) -> f32 {
        self.values.iter().sum::<f32>() / (self.values.len() as f32)
    }
}
