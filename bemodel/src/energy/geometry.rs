// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de los traits Bounded e Intersectable para Geometry

// use log::{debug, info, warn};

use super::{Bounded, Intersectable, Ray, AABB};
use crate::{point, types::HasSurface, WallGeom};

// -------------------------- Funciones auxiliares ---------------------------

impl Intersectable for WallGeom {
    /// Calcula la intersección entre rayo y geometría, e indica el factor t en la dirección del rayo
    ///
    /// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
    /// ray_dir: dirección del rayo en coordenadas globales (Vector3)
    ///
    /// Si es un punto interior devuelve t tal que la intersección se produce en ray_origin + t * ray_dir
    /// Comprueba la intersección transformando el rayo con la transformación inversa de la geometría
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        // Matrices de transformación de geometría
        let trans_inv = self.to_global_coords_matrix().map(|m| m.inverse());
        // Normal to the planar polygon
        ray.intersects_with_data(&self.polygon, trans_inv.as_ref(), &self.polygon.normal())
    }
}

impl Bounded for WallGeom {
    fn aabb(&self) -> AABB {
        if let Some(trans) = self.to_global_coords_matrix() {
            let mut min_x = f32::INFINITY;
            let mut min_y = f32::INFINITY;
            let mut min_z = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut max_y = f32::NEG_INFINITY;
            let mut max_z = f32::NEG_INFINITY;

            for p in self.polygon.iter().map(|p| trans * point![p.x, p.y, 0.0]) {
                min_x = min_x.min(p.x);
                max_x = max_x.max(p.x);
                min_y = min_y.min(p.y);
                max_y = max_y.max(p.y);
                min_z = min_z.min(p.z);
                max_z = max_z.max(p.z);
            }

            AABB {
                min: point![min_x, min_y, min_z],
                max: point![max_x, max_y, max_z],
            }
        } else {
            Default::default()
        }
    }
}
