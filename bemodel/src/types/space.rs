// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Espacios : Space
//!
//! Corresponden a zonas térmicas

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{ConsDb, HasSurface, Tilt, Uuid, Wall};
use crate::utils::{default_1, default_true, is_default, is_true, multiplier_is_1};

// Elementos -----------------------------------------------

/// Espacio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre del espacio
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Multiplicador del espacio
    #[serde(default = "default_1", skip_serializing_if = "multiplier_is_1")]
    pub multiplier: f32,
    /// Tipo de espacio:
    /// - CONDITIONED: acondicionado,
    /// - UNCONDITIONED: no acondicionado
    /// - UNINHABITED: no habitable
    #[serde(default, skip_serializing_if = "is_default")]
    pub kind: SpaceType,
    /// Pertenencia al interior de la envolvente térmica
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub inside_tenv: bool,
    /// Altura bruta (suelo a suelo) del espacio (m)
    pub height: f32,
    /// Ventilación, en ren/h
    /// TODO: en el futuro esto serían condiciones de uso del espacio?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_v: Option<f32>,
    /// Cota del espacio respecto al suelo (m)
    #[serde(default, skip_serializing_if = "is_default")]
    pub z: f32,
    /// Perfil de uso del espacio
    /// TODO: esto será más adelante un UUID
    pub space_conds: Option<String>,
    /// Condiciones operacionales del espacio
    /// TODO: esto será más adelante un UUID
    pub system_conds: Option<String>,
}

impl Space {
    /// Altura neta del espacio, m
    /// Se descuenta el grosor del primer forjado superior encontrado para el espacio
    pub fn height_net(&self, walls: &[Wall], cons: &ConsDb) -> f32 {
        // Elemento opaco de techo de un espacio
        // TODO: la altura neta debería calcularse promediando los grosores de **todos** los muros que
        // TODO: cubren el espacio y no solo el primero que se encuentre
        let top_wall_of_space = walls.iter().find(move |w| {
            match w.geometry.tilt.into() {
                // Muros exteriores o cubiertas sobre el espacio
                Tilt::TOP => w.space == self.id,
                // Es un cerramiento interior sobre este espacio
                Tilt::BOTTOM => w.next_to.map(|s| s == self.id).unwrap_or(false),
                _ => false,
            }
        });
        let top_wall_thickness = top_wall_of_space
            .and_then(|w| cons.get_wallcons(w.cons))
            .map(|cons| cons.thickness())
            .unwrap_or(0.0);
        self.height - top_wall_thickness
    }

    /// Superficie del espacio (m²)
    pub fn area(&self, walls: &[Wall]) -> f32 {
        let mut area = 0.0;
        for w in walls {
            if (w.space == self.id) && (Tilt::BOTTOM == w.geometry.tilt.into()) {
                area += w.geometry.area();
            }
        }
        area
    }

    /// Iterador de los cerramientos que delimitan un espacio (muros, suelos y techos)
    pub fn walls<'a>(&'a self, walls: &'a [Wall]) -> impl Iterator<Item = &'a Wall> {
        walls
            .iter()
            .filter(move |w| w.space == self.id || w.next_to == Some(self.id))
    }
}

impl Default for Space {
    fn default() -> Self {
        Space {
            id: Uuid::new_v4(),
            name: "Espacio".to_string(),
            multiplier: 1.0,
            kind: SpaceType::default(),
            inside_tenv: true,
            height: 3.0,
            n_v: None,
            z: 0.0,
            system_conds: None,
            space_conds: None,
        }
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
