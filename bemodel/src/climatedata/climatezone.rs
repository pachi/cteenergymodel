// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Definición de zonas climáticas CTE y conversiones desde y en cadenas
use std::{convert::TryFrom, error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

/// Nombres para la orientación de un elemento, según los puntos cardinales
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClimateZone {
    /// A1 Canarias
    A1c,
    /// A2 Canarias
    A2c,
    /// A3 Canarias
    A3c,
    /// A4 Canarias
    A4c,
    /// Alfa1 Canarias
    Alfa1c,
    /// Alfa2 Canarias
    Alfa2c,
    /// Alfa3 Canarias
    Alfa3c,
    /// Alfa4 Canarias
    Alfa4c,
    /// B1 Canarias
    B1c,
    /// B2 Canarias
    B2c,
    /// B3 Canarias
    B3c,
    /// B4 Canarias
    B4c,
    /// C1 Canarias
    C1c,
    /// C2 Canarias
    C2c,
    /// C3 Canarias
    C3c,
    /// C4 Canarias
    C4c,
    /// D1 Canarias
    D1c,
    /// D2 Canarias
    D2c,
    /// D3 Canarias
    D3c,
    /// E1 Canarias
    E1c,
    /// A3
    A3,
    /// A4
    A4,
    /// B3
    B3,
    /// B4
    B4,
    /// C1
    C1,
    /// C2
    C2,
    /// C3
    C3,
    /// C4
    C4,
    /// D1
    D1,
    /// D2
    D2,
    /// D3
    D3,
    /// E1
    E1,
}

impl Display for ClimateZone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ClimateZone::*;
        let printable = match *self {
            A1c => "A1c",
            A2c => "A2c",
            A3c => "A3c",
            A4c => "A4c",
            Alfa1c => "Alfa1c",
            Alfa2c => "Alfa2c",
            Alfa3c => "Alfa3c",
            Alfa4c => "Alfa4c",
            B1c => "B1c",
            B2c => "B2c",
            B3c => "B3c",
            B4c => "B4c",
            C1c => "C1c",
            C2c => "C2c",
            C3c => "C3c",
            C4c => "C4c",
            D1c => "D1c",
            D2c => "D2c",
            D3c => "D3c",
            E1c => "E1c",
            A3 => "A3",
            A4 => "A4",
            B3 => "B3",
            B4 => "B4",
            C1 => "C1",
            C2 => "C2",
            C3 => "C3",
            C4 => "C4",
            D1 => "D1",
            D2 => "D2",
            D3 => "D3",
            E1 => "E1",
        };
        write!(f, "{}", printable)
    }
}

/// Convierte str a ClimateZone
impl TryFrom<&str> for ClimateZone {
    type Error = Box<dyn Error + 'static>;
    fn try_from(climatezone: &str) -> Result<Self, Self::Error> {
        use ClimateZone::*;
        match climatezone {
            "A1c" => Ok(A1c),
            "A2c" => Ok(A2c),
            "A3c" => Ok(A3c),
            "A4c" => Ok(A4c),
            "Alfa1c" | "alfa1c" => Ok(Alfa1c),
            "Alfa2c" | "alfa2c" => Ok(Alfa2c),
            "Alfa3c" | "alfa3c" => Ok(Alfa3c),
            "Alfa4c" | "alfa4c" => Ok(Alfa4c),
            "B1c" => Ok(B1c),
            "B2c" => Ok(B2c),
            "B3c" => Ok(B3c),
            "B4c" => Ok(B4c),
            "C1c" => Ok(C1c),
            "C2c" => Ok(C2c),
            "C3c" => Ok(C3c),
            "C4c" => Ok(C4c),
            "D1c" => Ok(D1c),
            "D2c" => Ok(D2c),
            "D3c" => Ok(D3c),
            "E1c" => Ok(E1c),
            "A3" => Ok(A3),
            "A4" => Ok(A4),
            "B3" => Ok(B3),
            "B4" => Ok(B4),
            "C1" => Ok(C1),
            "C2" => Ok(C2),
            "C3" => Ok(C3),
            "C4" => Ok(C4),
            "D1" => Ok(D1),
            "D2" => Ok(D2),
            "D3" => Ok(D3),
            "E1" => Ok(E1),
            _ => Err("Zona climática desconocida".into()),
        }
    }
}
