// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::fmt::Display;

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

/// Tipo de espacio según su nivel de acondicionamiento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceType {
    /// Acondicionado
    CONDITIONED,
    /// No acondicionado
    UNCONDITIONED,
    /// No habitable
    UNINHABITED,
}

impl Display for SpaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            SpaceType::CONDITIONED => "CONDITIONED",
            SpaceType::UNCONDITIONED => "UNCONDITIONED",
            SpaceType::UNINHABITED => "UNINHABITED",
        };
        write!(f, "{}", printable)
    }
}

impl Default for SpaceType {
    fn default() -> Self {
        SpaceType::CONDITIONED
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

/// Datos mensuales de radiación por superficie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceMonthlyRadiation {
    /// Zona climática
    pub zone: ClimateZone,
    /// Orientación u horizontal
    pub orientation: Orientation,
    /// Inclinación (Horiz=0, vertical=90)
    pub beta: f32,
    /// Orientación (N=0, E=-90, W=+90, S=180)
    /// TODO: convertir a orientación UNE-EN ISO 52016-1, medido desde el sur, positivo al este, negativo al oeste (S=0, E=+90, W=-90)
    pub gamma: f32,
    /// Radiación mensual directa
    pub dir: [f32; 12],
    /// Radiación mensual difusa
    pub dif: [f32; 12],
    /// Radiación mensual total
    pub tot: [f32; 12],
}

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
