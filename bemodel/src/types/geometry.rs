// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Tipos y traits de elementos geométricos: HasSurface, Point2, Point3, Vector2, Vector3 y Polygon

use crate::vector;

pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Polygon = Vec<Point2>;

pub trait HasSurface {
    /// Área bruta definida por los vértices (m2)
    fn area(&self) -> f32;
    /// Perímetro (m)
    fn perimeter(&self) -> f32;
    /// Vector unitario normal al plano
    fn normal(&self) -> Vector3;
}

impl HasSurface for Polygon {
    /// Área bruta del polígono definido por vértices (m2)
    fn area(&self) -> f32 {
        // https://www.mathopenref.com/coordpolygonarea2.html
        // https://www.mathopenref.com/coordpolygonarea.html
        // 0.5 * ( \SUM( x_i * y_i+1 - y_i * x_i+1)_(i = de 1 a n) + (x_n * y_1 - y_n * x_1) )
        let area = match self.len() {
            0 => 0.0,
            1 => 0.0,
            n => self
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let w = self[(i + 1) % n];
                    v.x * w.y - v.y * w.x
                })
                .sum(),
        };
        f32::abs(0.5 * area)
    }

    /// Perímetro de un polígono (m)
    fn perimeter(&self) -> f32 {
        match self.len() {
            0 => 0.0,
            1 => 0.0,
            n => self
                .iter()
                .enumerate()
                .map(|(i, v)| (v - self[(i + 1) % n]).magnitude())
                .sum(),
        }
    }

    /// Vector unitario normal al polígono plano, en coordenadas locales del polígono
    fn normal(&self) -> Vector3 {
        if self.len() < 3 {
            return vector![0.0, 0.0, 1.0];
        };
        let v0 = self[1] - self[0];
        let v1 = self[2] - self[0];

        // normal
        // let n = vector![v0.x, v0.y, 0.0].cross(&vector![v1.x, v1.y, 0.0]).normalize();
        // Desarrollando el determinante por la fila 3 -> x=0, y= 0, z es 1 o -1 según signo del adjunto superior
        // assert!(n.x == n2.x && n.y == n2.y && n.z == n2.z);
        if v0.x * v1.y >= v0.y * v1.x {
            vector![0.0, 0.0, 1.0]
        } else {
            vector![0.0, 0.0, -1.0]
        }
    }
}
