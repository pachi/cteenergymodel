// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Espacios : Space
//!
//! XXX: realmente los tratamos como zonas térmicas y no como recintos

use std::fmt::Display;

use super::{ConsDb, Tilt, Uuid, Wall};
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
    /// TODO: calcular a partir de superficie de suelos
    pub area: f32,
    /// Multiplicador del espacio
    pub multiplier: f32,
    /// Tipo de espacio:
    /// - CONDITIONED: acondicionado,
    /// - UNCONDITIONED: no acondicionado
    /// - UNINHABITED: no habitable
    pub kind: SpaceType,
    /// Pertenencia al interior de la envolvente térmica
    pub inside_tenv: bool,
    /// Altura bruta (suelo a suelo) del espacio (m)
    pub height: f32,
    /// Ventilación, en ren/h
    /// TODO: esto serán condiciones del espacio?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub n_v: Option<f32>,
    /// Cota del espacio respecto al suelo (m)
    pub z: f32,
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

    /// Volumen neto del espacio, m^3
    /// Se calcula con area * altura neta
    pub fn volume_net(&self, walls: &[Wall], cons: &ConsDb) -> f32 {
        self.area * self.height_net(walls, cons)
    }

    /// Iterador de los cerramientos que delimitan un espacio (muros, suelos y techos)
    pub fn walls<'a>(&'a self, walls: &'a [Wall]) -> impl Iterator<Item = &'a Wall> {
        walls
            .iter()
            .filter(move |w| w.space == self.id || w.next_to == Some(self.id))
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
