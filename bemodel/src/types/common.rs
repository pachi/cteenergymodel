// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)
#![allow(clippy::upper_case_acronyms)]

//! Tipos comunes al modelo del edificio: BoundaryType, Tilt, Orientation

use std::{fmt::Display};

use serde::{Deserialize, Serialize};

use crate::utils::normalize;

/// Condiciones de contorno de los cerramientos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoundaryType {
    /// Cerramiento en contacto con el aire exterior
    EXTERIOR,
    /// Cerramiento en contacto con el aire de otro espacio
    INTERIOR,
    /// Cerramiento en contacto con el terreno
    GROUND,
    /// Cerramiento sin transmisión térmica
    ADIABATIC,
}

impl Display for BoundaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            BoundaryType::EXTERIOR => "EXTERIOR",
            BoundaryType::INTERIOR => "INTERIOR",
            BoundaryType::GROUND => "GROUND",
            BoundaryType::ADIABATIC => "ADIABATIC",
        };
        write!(f, "{}", printable)
    }
}

impl Default for BoundaryType {
    fn default() -> Self {
        BoundaryType::EXTERIOR
    }
}

/// Posiciones de los cerramientos según su inclinación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tilt {
    /// Suelo (inclinación < 60º)
    BOTTOM,
    /// Cubierta (inclinación > 120º)
    TOP,
    /// Muro (inclinación entre 60 y 120º)
    SIDE,
}

impl Display for Tilt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Tilt::BOTTOM => "BOTTOM",
            Tilt::TOP => "TOP",
            Tilt::SIDE => "SIDE",
        };
        write!(f, "{}", printable)
    }
}

/// Convierte de inclinación a enum Tilt
impl From<f32> for Tilt {
    fn from(tilt: f32) -> Self {
        let tilt = normalize(tilt, 0.0, 360.0);
        if tilt <= 60.0 {
            Tilt::TOP
        } else if tilt < 120.0 {
            Tilt::SIDE
        } else if tilt < 240.0 {
            Tilt::BOTTOM
        } else if tilt < 300.0 {
            Tilt::SIDE
        } else {
            Tilt::TOP
        }
    }
}

/// Nombres para la orientación de un elemento, según los puntos cardinales y elemento horizontal
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Orientation {
    /// Norte
    N,
    /// Noreste
    NE,
    /// Este
    E,
    /// Sureste
    SE,
    /// Sur
    S,
    /// Suroeste
    SW,
    /// Oeste
    W,
    /// Noroeste
    NW,
    /// Horizontal
    HZ,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Orientation::N => "N",
            Orientation::NE => "NE",
            Orientation::E => "E",
            Orientation::SE => "SE",
            Orientation::S => "S",
            Orientation::SW => "SW",
            Orientation::W => "W",
            Orientation::NW => "NW",
            Orientation::HZ => "Horiz.",
        };
        write!(f, "{}", printable)
    }
}

/// Convierte del ángulo entre normal del elemento constructivo y sur geográfico (azimuth geográfico) a enum Orientation
/// Sigue el criterio de la UNE-EN ISO 52016-1, medido desde el sur, positivo al este, negativo al oeste (S=0, E=+90, W=-90)
/// Nota: difiere del criterio BDL, que parte del norte, con E+ y W-
impl From<f32> for Orientation {
    fn from(azimuth: f32) -> Self {
        let azimuth = normalize(azimuth, 0.0, 360.0);
        if azimuth < 18.0 {
            Self::S
        } else if azimuth < 69.0 {
            Self::SE
        } else if azimuth < 120.0 {
            Self::E
        } else if azimuth < 157.5 {
            Self::NE
        } else if azimuth < 202.5 {
            Self::N
        }
        // 202.5 = 360 - 157.5
        else if azimuth < 240.0 {
            Self::NW
        }
        // 240 = 360 - 120
        else if azimuth < 291.0 {
            Self::W
        }
        // 291 = 360 - 69
        else if azimuth < 342.0 {
            Self::SW
        }
        // 342 = 360 - 18
        else {
            Self::S
        }
    }
}

/// Convierte str a Orientation
impl From<&str> for Orientation {
    fn from(azimuth: &str) -> Self {
        match azimuth {
            "S" => Self::S,
            "SE" => Self::SE,
            "E" => Self::E,
            "NE" => Self::NE,
            "N" => Self::N,
            "NW" => Self::NW,
            "W" => Self::W,
            "SW" => Self::SW,
            _ => Self::HZ,
        }
    }
}
