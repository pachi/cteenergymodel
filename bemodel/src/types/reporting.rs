// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Clases para emisión de avisos

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Nivel de aviso para condiciones de chequeo del modelo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WarningLevel {
    SUCCESS,
    DANGER,
    WARNING,
    INFO,
}

/// Muestra WarningLevel
impl Display for WarningLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use WarningLevel::*;
        let printable = match *self {
            SUCCESS => "SUCCESS",
            DANGER => "DANGER",
            WARNING => "WARNING",
            _ => "INFO",
        };
        write!(f, "{}", printable)
    }
}

/// Convierte str a WarningLevel
impl From<&str> for WarningLevel {
    fn from(level: &str) -> Self {
        match level.to_uppercase().as_str() {
            "SUCCESS" => Self::SUCCESS,
            "DANGER" => Self::DANGER,
            "WARNING" => Self::WARNING,
            _ => Self::INFO,
        }
    }
}

/// Reporte de avisos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    /// Nivel de afectación
    pub level: WarningLevel,
    /// Id del elemento afectado, en su caso
    pub id: Option<String>,
    /// Mensaje del aviso
    pub msg: String,
}
