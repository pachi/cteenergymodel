// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::{fmt::Debug, ops::Deref};

use super::bvh::{Bounded, Intersectable};
use super::ray::Ray;
use crate::{point, Point3};

/// Axis aligned bounding box definida por puntos extremos
#[derive(Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl Debug for AABB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = self.min;
        let max = self.max;
        write!(
            f,
            "AABB (min: point![{}, {}, {}], max: point![{}, {}, {}])",
            min.x, min.y, min.z, max.x, max.y, max.z
        )
    }
}

impl AABB {
    /// Constructor
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    /// Punto medio de la AABB
    pub fn center(self) -> Point3 {
        nalgebra::center(&self.max, &self.min)
    }

    /// Calcula AABB que incluye a este y otro elemento
    pub fn join(self, other: Self) -> Self {
        let minx: f32 = self.min.x.min(other.min.x);
        let miny: f32 = self.min.y.min(other.min.y);
        let minz: f32 = self.min.z.min(other.min.z);
        let maxx: f32 = self.max.x.max(other.max.x);
        let maxy: f32 = self.max.y.max(other.max.y);
        let maxz: f32 = self.max.z.max(other.max.z);
        Self {
            min: point![minx, miny, minz],
            max: point![maxx, maxy, maxz],
        }
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: point![f32::INFINITY, f32::INFINITY, f32::INFINITY],
            max: point![f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY],
        }
    }
}

impl Bounded for AABB {
    fn aabb(&self) -> AABB {
        *self
    }
}

impl<T: Bounded, U: Deref<Target = [T]>> Bounded for U {
    fn aabb(&self) -> AABB {
        self.iter()
            .fold(AABB::default(), |res, elem| res.join(elem.aabb()))
    }
}

impl Intersectable for AABB {
    /// Detecta si existe intersección de AABB y rayo usando el algoritmo de Cyrus-Beck
    /// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter3/raycast_aabb.html
    /// NaN es siempre distinto, de modo que las comparaciones con NaN son correctas
    /// Las AABB deben tener ancho > 0 en todas las dimensiones
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let idx = 1.0 / ray.dir.x;
        let idy = 1.0 / ray.dir.y;
        let idz = 1.0 / ray.dir.z;

        let t1 = (self.min.x - ray.origin.x) * idx;
        let t2 = (self.max.x - ray.origin.x) * idx;
        let t3 = (self.min.y - ray.origin.y) * idy;
        let t4 = (self.max.y - ray.origin.y) * idy;
        let t5 = (self.min.z - ray.origin.z) * idz;
        let t6 = (self.max.z - ray.origin.z) * idz;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // Si tmax < 0 la línea interseca pero el AABB está detrás
        if tmax < 0.0 {
            // t = tmax;
            return None;
        }

        // Si tmin > tmax el rayo no corta AABB
        if tmin > tmax {
            // t = tmax;
            return None;
        }
        // t = tmin;
        Some(tmin)
    }
}
