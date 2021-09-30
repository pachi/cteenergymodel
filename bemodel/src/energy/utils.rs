// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Utilidades para cálculo energético y geométrico
use crate::{vector, Point2, Vector3};

/// Normal al polígono plano, en coordenadas locales
pub fn poly_normal(poly: &[Point2]) -> Vector3 {
    let v0 = poly[1] - poly[0];
    let v1 = poly[2] - poly[0];

    vector![v0.x, v0.y, 0.0]
        .cross(&vector![v1.x, v1.y, 0.0])
        .normalize()
}
