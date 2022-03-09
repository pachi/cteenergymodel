// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Espacios : Space
//!
//! XXX: realmente los tratamos como zonas térmicas y no como recintos

use std::fmt::Display;

use super::Uuid;
use serde::{Deserialize, Serialize};

// Elementos -----------------------------------------------

/// Espacio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre del espacio
    pub name: String,
    /// Superficie útil del espacio (m2)
    pub area: f32,
    /// Multiplicador del espacio
    pub multiplier: f32,
    /// Tipo de espacio:
    /// - CONDITIONED: acondicionado,
    /// - UNCONDITIONED: no acondicionado
    /// - UNINHABITED: no habitable
    #[serde(rename = "type")]
    pub space_type: SpaceType,
    /// Pertenencia al interior de la envolvente térmica
    pub inside_tenv: bool,
    /// Altura bruta (suelo a suelo) del espacio (m)
    pub height: f32,
    /// Ventilación, en ren/h
    pub n_v: Option<f32>,
    /// Cota del espacio respecto al suelo (m)
    pub z: f32,
    /// Perímetro expuesto del espacio (suelos) (m)
    /// Incluye la parte del perímetro que separa el espacio del exterior
    /// y excluye que lo separa de otros espacios acondicionados.
    pub exposed_perimeter: Option<f32>,
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
