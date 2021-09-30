// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de los traits Bounded e Intersectable para Geometry

// use log::{debug, info, warn};
use nalgebra::{
    IsometryMatrix2, IsometryMatrix3, Rotation2, Rotation3, Translation2, Translation3,
};

use super::{Bounded, Intersectable, Ray, AABB, utils::poly_normal};
use crate::{point, Geometry, Vector2, Vector3};

// -------------------------- Funciones auxiliares ---------------------------

impl Geometry {
    /// Vector unitario normal a la geometría, en coordenadas globales
    pub fn normal(&self) -> Vector3 {
        let n_p = poly_normal(&self.polygon);
        let zrot = Rotation3::new(Vector3::z() * self.azimuth.to_radians());
        let xrot = Rotation3::new(Vector3::x() * self.tilt.to_radians());
        zrot * xrot * n_p
    }

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

impl Intersectable for Geometry {
    /// Calcula la intersección entre rayo y geometría, e indica el factor t en la dirección del rayo
    ///
    /// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
    /// ray_dir: dirección del rayo en coordenadas globales (Vector3)
    ///
    /// Si es un punto interior devuelve t tal que la intersección se produce en ray_origin + t * ray_dir
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        // Matrices de transformación de geometría
        let trans_inv = self.to_global_coords_matrix().map(|m| m.inverse());
        // Normal to the planar polygon
        let n_p = &poly_normal(&self.polygon);
        ray.intersects_with_data(&self.polygon, trans_inv.as_ref(), n_p)
    }
}

impl Bounded for Geometry {
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
