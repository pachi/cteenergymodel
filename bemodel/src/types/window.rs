// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Elementos semitransparentes del edificio: Window, WinGeom

use serde::{Deserialize, Serialize};

use super::{point, uuid_from_str, vector, HasSurface, Point2, Shade, Uuid, Vector3, WallGeom};

// Elementos -----------------------------------------------

/// Hueco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Window {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre del hueco
    pub name: String,
    /// Construcción del hueco
    pub cons: Uuid,
    /// Muro al que pertenece el hueco
    pub wall: Uuid,
    /// Factor de obstáculos remotos suministrado por el usuario
    pub f_shobst: Option<f32>,
    /// Factor de obstáculos remotos calculado a partir del modelo
    pub f_shobst_calc: Option<f32>,
    /// Geometría de hueco
    pub geometry: WinGeom,
}

impl Window {
    /// Superficie del hueco, m²
    #[inline]
    pub fn area(&self) -> f32 {
        self.geometry.area()
    }

    /// Perímetro del hueco, m
    #[inline]
    pub fn perimeter(&self) -> f32 {
        self.geometry.perimeter()
    }

    /// Crea elementos de sombra correpondientes el perímetro de retranqueo del hueco
    pub(crate) fn shades_for_setback(&self, wallgeom: &WallGeom) -> Vec<(Uuid, Shade)> {
        let wing = &self.geometry;
        // Si no hay retranqueo no se genera geometría
        if wing.setback.abs() < 0.01 {
            return vec![];
        };
        let wpos = match wing.position {
            Some(pos) => pos,
            // Si no hay definición geométrica completa no se calcula geometría
            _ => return vec![],
        };

        // TODO: Eliminar expect permitiendo fallar a esta función
        let wall2world = wallgeom
            .to_global_coords_matrix()
            .expect("El muro debe tener definición geométrica completa");

        let overhang = Shade {
            id: uuid_from_str(&format!("{}-top_setback", self.id)),
            name: format!("{}_top_setback", self.name),
            geometry: WallGeom {
                // inclinación: con 90º es perpendicular al hueco
                tilt: wallgeom.tilt + 90.0,
                azimuth: wallgeom.azimuth,
                position: Some(wall2world * point![wpos.x, wpos.y + wing.height, 0.0]),
                polygon: vec![
                    point![0.0, 0.0],
                    point![0.0, -wing.setback],
                    point![wing.width, -wing.setback],
                    point![wing.width, 0.0],
                ],
            },
        };

        let left_fin = Shade {
            id: uuid_from_str(&format!("{}-left_setback", self.id)),
            name: format!("{}_left_setback", self.name),
            geometry: WallGeom {
                tilt: wallgeom.tilt,
                azimuth: wallgeom.azimuth + 90.0,
                position: Some(wall2world * point![wpos.x, wpos.y + wing.height, 0.0]),
                polygon: vec![
                    point![0.0, 0.0],
                    point![0.0, -wing.height],
                    point![wing.setback, -wing.height],
                    point![wing.setback, 0.0],
                ],
            },
        };

        let right_fin = Shade {
            id: uuid_from_str(&format!("{}-right_setback", self.id)),
            name: format!("{}_right_setback", self.name),
            geometry: WallGeom {
                tilt: wallgeom.tilt,
                azimuth: wallgeom.azimuth - 90.0,
                position: Some(wall2world * point![wpos.x + wing.width, wpos.y + wing.height, 0.0]),
                polygon: vec![
                    point![0.0, 0.0],
                    point![-wing.setback, 0.0],
                    point![-wing.setback, -wing.height],
                    point![0.0, -wing.height],
                ],
            },
        };

        let sill = Shade {
            id: uuid_from_str(&format!("{}-sill_setback", self.id)),
            name: format!("{}_sill_setback", self.name),
            geometry: WallGeom {
                tilt: wallgeom.tilt - 90.0,
                azimuth: wallgeom.azimuth,
                position: Some(wall2world * point![wpos.x, wpos.y, 0.0]),
                polygon: vec![
                    point![0.0, 0.0],
                    point![wing.width, 0.0],
                    point![wing.width, wing.setback],
                    point![0.0, wing.setback],
                ],
            },
        };

        vec![
            (self.id, overhang),
            (self.id, left_fin),
            (self.id, right_fin),
            (self.id, sill),
        ]
    }
}

/// Geometría de hueco
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinGeom {
    /// Posición del hueco, en coordenadas de muro
    /// Un valor None señala que no hay definición geométrica completa
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub position: Option<Point2>,
    /// Altura del hueco, m
    pub height: f32,
    /// Anchuro del hueco, m
    pub width: f32,
    /// Retranqueo, m
    pub setback: f32,
}

impl Default for WinGeom {
    fn default() -> Self {
        WinGeom {
            position: None,
            height: 1.0,
            width: 1.0,
            setback: 0.0,
        }
    }
}

impl HasSurface for WinGeom {
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
