// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Material (MATERIAL) (tipo PROPERTIES o RESISTANCE))

use std::convert::TryFrom;

use anyhow::{format_err, Error};

use crate::bdl::{extract_f32vec, extract_namesvec, extract_u32vec, BdlBlock};

/// Tipos de horarios
#[derive(Debug, Clone, PartialEq)]
pub enum Schedule {
    Day(DaySchedule),
    Week(WeekSchedule),
    Year(YearSchedule),
}

/// Tipos de datos de los horarios
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ScheduleKind {
    /// Datos fraccionarios (0.0-1.0))
    #[default]
    Fraction,
    /// Datos de encendido (1) / apagado (0)
    OnOff,
    /// Datos de temperatura (f32)
    Temperature,
}

impl TryFrom<&str> for ScheduleKind {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "FRACTION" => Ok(Self::Fraction),
            "ON/OFF" => Ok(Self::OnOff),
            "TEMPERATURE" => Ok(Self::Temperature),
            _ => Err(format_err!("Tipo de horario desconocido {}", value)),
        }
    }
}

/// Horario diario definido con sus valores hora a hora
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DaySchedule {
    /// Nombre del horario diario
    pub name: String,
    /// Tipo de horario
    pub kind: ScheduleKind,
    /// valores horarios
    pub values: Vec<f32>,
}

impl TryFrom<BdlBlock> for DaySchedule {
    type Error = Error;

    /// Conversión de bloque BDL a horario diario
    /// NOTE: La base de datos tiene algunos nombres con dobles espacios, que se convierten a espacios simples
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///    "HA27_SS_HD_FUN_8_A_16" = DAY-SCHEDULE-PD
    ///         TYPE  = "ON/OFF"
    ///         GROUP = "Equipos"
    ///         INDEX = 0
    ///         VALUES  = ( 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
    ///         ..
    /// ```
    /// TODO: Propiedades no convertidas:
    /// TODO: INDEX, GROUP
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            mut name,
            mut attrs,
            ..
        } = value;
        name = name.replace("  ", " ");
        let kind = attrs.remove_str("TYPE")?.as_str().try_into()?;
        let values = extract_f32vec(attrs.remove_str("VALUES")?)?;
        if !(values.len() == 24 || values.len() == 1) {
            Err(format_err!(
                "Longitud de valores horarios incorrecta en DAY-SCHEDULE-PD: {}",
                name
            ))?;
        }

        Ok(Self { name, kind, values })
    }
}

/// Horario semanal definido a partir de secuencia de días
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WeekSchedule {
    /// Nombre del horario semanal
    pub name: String,
    /// Tipo de horario
    pub kind: ScheduleKind,
    /// Horarios diarios que componen la semana
    pub days: Vec<String>,
}

impl TryFrom<BdlBlock> for WeekSchedule {
    type Error = Error;

    /// Conversión de bloque BDL a material
    /// NOTE: La base de datos tiene algunos nombres con dobles espacios, que se convierten a espacios simples
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///     "HA_INF_16_A_8" = SCHEDULE-PD
    ///         TYPE   = "FRACTION"
    ///         GROUP  = "Internas"
    ///         MONTH = ( 12)
    ///         DAY   = ( 31)
    ///         WEEK-SCHEDULES = ( "HA26_HS0_SS_")
    ///         ..
    /// ```
    /// TODO: Propiedades no convertidas:
    /// TODO: GROUP, INDEX
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            mut name,
            mut attrs,
            ..
        } = value;
        name = name.replace("  ", " ");
        let kind = attrs.remove_str("TYPE")?.as_str().try_into()?;
        let days = extract_namesvec(attrs.remove_str("DAY-SCHEDULES")?);

        if !(days.len() == 7 || days.len() == 1) {
            Err(format_err!(
                "Longitud de valores semanales incorrecta en WEEK-SCHEDULE-PD: {}",
                name
            ))?;
        }

        Ok(Self { name, kind, days })
    }
}

/// Horario anual definido a partir de secuencia de horarios semanales
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct YearSchedule {
    /// Nombre del horario semanal
    pub name: String,
    /// Tipo de horario
    pub kind: ScheduleKind,
    /// Lista de días en el que finalizan los horarios semanales
    pub days: Vec<u32>,
    /// Lista de meses que en el que finalizan los horarios semanales
    pub months: Vec<u32>,
    /// Horarios semanales que componen la semana
    pub weeks: Vec<String>,
}

impl TryFrom<BdlBlock> for YearSchedule {
    type Error = Error;

    /// Conversión de bloque BDL a material
    /// NOTE: La base de datos tiene algunos nombres con dobles espacios, que se convierten a espacios simples
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///    "UsoEspacio-8h" = SCHEDULE-PD
    ///         TYPE   = "FRACTION"
    ///         GROUP  = "Internas"
    ///         MONTH = ( 12)
    ///         DAY   = ( 31)
    ///         WEEK-SCHEDULES = ( "HA6_HS0_SS_")
    ///         ..
    /// ```
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            mut name,
            mut attrs,
            ..
        } = value;
        name = name.replace("  ", " ");
        let kind = attrs.remove_str("TYPE")?.as_str().try_into()?;
        let days = extract_u32vec(attrs.remove_str("DAY")?)?;
        let months = extract_u32vec(attrs.remove_str("MONTH")?)?;
        let weeks = extract_namesvec(attrs.remove_str("WEEK-SCHEDULES")?);

        Ok(Self {
            name,
            kind,
            days,
            months,
            weeks,
        })
    }
}
