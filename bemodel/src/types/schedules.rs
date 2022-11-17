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
    /// El total de repeticiones de días debe sumar 365
    /// El año empieza en lunes (se toma de referencia el 2001)
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

    /// Localiza horario de año según id
    pub fn get_year(&self, id: Uuid) -> Option<&Schedule> {
        self.year.iter().find(|s| s.id == id)
    }

    /// Localiza horario de semana según id
    pub fn get_week(&self, id: Uuid) -> Option<&ScheduleWeek> {
        self.week.iter().find(|s| s.id == id)
    }

    /// Localiza horario diario según id
    pub fn get_day(&self, id: Uuid) -> Option<&ScheduleDay> {
        self.day.iter().find(|s| s.id == id)
    }

    /// Devuelve el año como lista de 365 horarios diarios
    pub fn get_year_as_day_sch(&self, id: Uuid) -> Vec<Uuid> {
        let mut current_count = 0;
        self.get_year(id)
            .map(|s| {
                s.values
                    .iter()
                    .flat_map(|(week_id, count)| {
                        let skip_count = current_count % 7;
                        current_count += *count as usize;
                        self.get_week(*week_id)
                            .map(ScheduleWeek::to_day_sch)
                            .unwrap_or_default()
                            .into_iter()
                            .cycle()
                            .skip(skip_count)
                            .take(*count as usize)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Lista de valores anuales para el horario anual con uuid dado
    pub fn year_values(&self, id: Uuid) -> Vec<f32> {
        self.get_year_as_day_sch(id)
            .iter()
            .flat_map(|day_id| {
                self.get_day(*day_id)
                    .map(|ds| ds.values.clone())
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
    }

    /// Lista de condiciones de valor distinto de (casi) cero para el horario anual con uuid dado
    pub fn year_values_is_not_zero(&self, id: Uuid) -> Vec<bool> {
        self.get_year_as_day_sch(id)
            .iter()
            .flat_map(|day_id| {
                self.get_day(*day_id)
                    .map(ScheduleDay::values_is_not_zero)
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
    }
}

/// Horarios anuales
/// Se forman como lista de tuplas de horario semanal y repeticiones
/// El total de repeticiones debe sumar 365 días semanas
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schedule {
    /// Id
    pub id: Uuid,
    /// Nombre del horario
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Secuencia de tuplas de UUID de horario semanal y número de días activo
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
    /// Secuencia de tuplas de UUID de horarios diarios y repeticiones del día
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<(Uuid, u32)>,
}

impl ScheduleWeek {
    /// Devuelve semana como lista de 7 valores diarios
    pub fn to_day_sch(&self) -> Vec<Uuid> {
        self.values
            .iter()
            .flat_map(|(id, count)| vec![*id; *count as usize])
            .collect()
    }
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
    /// Vector de la condición de valor mayor que (casi) cero
    pub fn values_is_not_zero(&self) -> Vec<bool> {
        // Aprox > 1 e-5
        self.values
            .iter()
            .map(|v| v.abs() > 100.0 * f32::EPSILON)
            .collect()
    }
}
