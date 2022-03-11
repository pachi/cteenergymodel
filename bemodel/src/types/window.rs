// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Elementos semitransparentes del edificio: Window, WindowGeometry

use serde::{Deserialize, Serialize};

use super::{vector, HasSurface, Point2, Uuid, Vector3};

// Elementos -----------------------------------------------

/// Hueco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Window {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre del hueco
    pub name: String,
    /// Superficie del hueco (m2)
    #[serde(rename = "A")]
    pub area: f32,
    /// Construcción del hueco
    pub cons: Uuid,
    /// Muro al que pertenece el hueco
    pub wall: Uuid,
    /// Factor de obstáculos remotos
    pub fshobst: f32,
    /// Geometría de hueco
    pub geometry: WindowGeometry,
}

/// Geometría de hueco
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowGeometry {
    /// Posición del hueco, en coordenadas de muro
    /// Un valor None señala que no hay definición geométrica completa
    pub position: Option<Point2>,
    /// Altura del hueco, m
    pub height: f32,
    /// Anchuro del hueco, m
    pub width: f32,
    /// Retranqueo, m
    pub setback: f32,
}

impl Default for WindowGeometry {
    fn default() -> Self {
        WindowGeometry {
            position: None,
            height: 1.0,
            width: 1.0,
            setback: 0.0,
        }
    }
}

impl HasSurface for WindowGeometry {
    /// Vector unitario normal a la geometría
    fn normal(&self) -> Vector3 {
        vector![0.0, 0.0, 1.0]
    }

    /// Superficie del hueco, m²
    fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Perímetro del hueco, m
    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}
