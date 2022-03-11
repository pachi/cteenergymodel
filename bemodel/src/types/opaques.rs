// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Elementos opacos: Wall, Shade y sus objetos asociados, Geometry

use nalgebra::{
    IsometryMatrix2, IsometryMatrix3, Rotation2, Rotation3, Translation2, Translation3,
};
use serde::{Deserialize, Serialize};

use super::{BoundaryType, HasSurface, Orientation, Point3, Polygon, Tilt, Uuid, Vector2, Vector3};

// Elementos -----------------------------------------------

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wall {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre del elemento opaco
    pub name: String,
    /// Superficie neta (sin huecos) del elemento opaco (m2)
    /// TODO: a eliminar, podría estar definida por el polígono
    #[serde(rename = "A")]
    pub area: f32,
    /// Condiciones de contorno del cerramiento:
    /// - GROUND: cerramientos en contacto con el terreno
    /// - EXTERIOR: cerramientos en contacto con el aire exterior
    /// - INTERIOR: cerramientos en contacto con el aire de otros espacios
    /// - ADIABATIC: cerramientos sin transmisión de calor
    pub bounds: BoundaryType,
    /// Construcción del opaco
    pub cons: Uuid,
    /// Espacio al que pertenece el elemento opaco
    pub space: Uuid,
    /// Espacio adyacente con el que comunica el elemento opaco cuando es interior
    pub nextto: Option<Uuid>,
    /// Geometría del elemento opaco
    pub geometry: WallGeometry,
}

/// Convierte de muro a enum Tilt
impl From<&Wall> for Tilt {
    fn from(wall: &Wall) -> Self {
        Tilt::from(wall.geometry.tilt)
    }
}

/// Convierte opaco a Orientation
impl From<&Wall> for Orientation {
    fn from(wall: &Wall) -> Self {
        match Tilt::from(wall.geometry.tilt) {
            Tilt::SIDE => wall.geometry.azimuth.into(),
            _ => Orientation::HZ,
        }
    }
}

/// Elemento de sombra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shade {
    /// ID del elemento de sombra (en formato UUID)
    pub id: String,
    /// Nombre del elemento opaco
    pub name: String,
    /// Geometría del elemento opaco
    pub geometry: WallGeometry,
}

/// Geometría de muro
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WallGeometry {
    /// Inclinación (beta) [0, 180]
    /// Ángulo con la vertical (0 -> suelo, 180 -> techo)
    pub tilt: f32,
    /// Orientación (gamma) [-180,+180] (S=0, E=+90, W=-90, sentido antihorario)
    /// Medido como azimuth geográfico de la proyección horizontal de la normal a la superficie con el eje -Y del espacio
    /// Coincide con el criterio de la UNE-EN ISO 52016-1
    /// Difiere del criterio BDL, que parte del norte, con E+ y W- y sentido horario
    pub azimuth: f32,
    /// Posición del muro, en coordenadas de espacio
    /// Un valor None señala que no hay definición geométrica completa
    pub position: Option<Point3>,
    /// Polígono del muro, en coordenadas de muro
    pub polygon: Polygon,
}

impl WallGeometry {
    /// Matriz de transformación de coordenadas locales a coordenadas globales
    /// Traslada de coordenadas de opaco / sombra a coordenadas globales (giros y desplazamientos)
    pub fn to_global_coords_matrix(&self) -> Option<IsometryMatrix3<f32>> {
        let trans = Translation3::from(self.position?);
        let zrot = Rotation3::new(Vector3::z() * self.azimuth.to_radians());
        let xrot = Rotation3::new(Vector3::x() * self.tilt.to_radians());

        Some(trans * zrot * xrot)
    }

    /// Matriz de transformación de coordenadas locales de la geometría a coordenadas de polígono interno 2D
    /// Se gira el eje X en la dirección del polígono de muro p1 - p0 y se traslada a p0 el origen
    pub fn to_polygon_coords_matrix(&self) -> Option<IsometryMatrix2<f32>> {
        if self.polygon.len() <= 2 {
            return None;
        };
        let v0 = self.polygon[0];
        let v1 = self.polygon[1];
        let dir_x = v1 - v0;
        let rot = Rotation2::rotation_between(&Vector2::x(), &dir_x);
        let trans = Translation2::from(v0);

        Some(trans * rot)
    }
}

impl HasSurface for WallGeometry {
    /// Vector unitario normal a la geometría, en coordenadas globales
    fn normal(&self) -> Option<Vector3> {
        // Normal al polígono plano, en coordenadas locales
        let n_p = self.polygon.normal().unwrap();
        let zrot = Rotation3::new(Vector3::z() * self.azimuth.to_radians());
        let xrot = Rotation3::new(Vector3::x() * self.tilt.to_radians());
        Some(zrot * xrot * n_p)
    }

    fn area(&self) -> f32 {
        self.polygon.area()
    }

    fn perimeter(&self) -> f32 {
        self.polygon.perimeter()
    }
}
