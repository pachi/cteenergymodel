/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Funciones para el cálculo de parámetros de los elementos
//! - Fshobst para un hueco, a partir de su retranqueo
//! - U de una composión constructiva de opaco, según su posición

use crate::utils::fround2;

pub use super::common::{
    Boundaries::{self, *},
    Orientation::{self, *},
    SpaceType::{self, *},
    Tilt::{self, *},
};
pub use super::WallCons;

/// Factor de obstáculos remotos (Fshobst) en función del retranqueo, orientación y geometría del hueco
/// Se calcula, para huecos verticales, de acuerdo a la tabla 17 del DA DB-HE/1 (p. 19).
pub fn fshobst_for_setback(tilt: f32, azimuth: f32, width: f32, height: f32, setback: f32) -> f32 {
    // Calcular según orientación e inclinación
    match tilt.into() {
        // Elementos verticales
        SIDE => {
            let rh = setback / height;
            let rw = setback / width;
            let range_rh = if rh < 0.05 {
                0
            } else if rh <= 0.1 {
                1
            } else if rh <= 0.2 {
                2
            } else if rh <= 0.5 {
                3
            } else {
                4
            };
            let range_rw = if rw < 0.05 {
                0
            } else if rw <= 0.1 {
                1
            } else if rw <= 0.2 {
                2
            } else if rw <= 0.5 {
                3
            } else {
                4
            };
            match azimuth.into() {
                S => match (range_rh, range_rw) {
                    (1, 1) => 0.82,
                    (1, 2) => 0.74,
                    (1, 3) => 0.62,
                    (1, 4) => 0.39,
                    (2, 1) => 0.76,
                    (2, 2) => 0.67,
                    (2, 3) => 0.56,
                    (2, 4) => 0.35,
                    (3, 1) => 0.56,
                    (3, 2) => 0.51,
                    (3, 3) => 0.39,
                    (3, 4) => 0.27,
                    (4, 1) => 0.35,
                    (4, 2) => 0.32,
                    (4, 3) => 0.27,
                    (4, 4) => 0.17,
                    _ => 1.0,
                },
                SE | SW => match (range_rh, range_rw) {
                    (1, 1) => 0.86,
                    (1, 2) => 0.81,
                    (1, 3) => 0.72,
                    (1, 4) => 0.51,
                    (2, 1) => 0.79,
                    (2, 2) => 0.74,
                    (2, 3) => 0.66,
                    (2, 4) => 0.47,
                    (3, 1) => 0.59,
                    (3, 2) => 0.56,
                    (3, 3) => 0.47,
                    (3, 4) => 0.36,
                    (4, 1) => 0.38,
                    (4, 2) => 0.36,
                    (4, 3) => 0.32,
                    (4, 4) => 0.23,
                    _ => 1.0,
                },
                E | W => match (range_rh, range_rw) {
                    (1, 1) => 0.91,
                    (1, 2) => 0.87,
                    (1, 3) => 0.81,
                    (1, 4) => 0.65,
                    (2, 1) => 0.86,
                    (2, 2) => 0.82,
                    (2, 3) => 0.76,
                    (2, 4) => 0.61,
                    (3, 1) => 0.71,
                    (3, 2) => 0.68,
                    (3, 3) => 0.61,
                    (3, 4) => 0.51,
                    (4, 1) => 0.53,
                    (4, 2) => 0.51,
                    (4, 3) => 0.48,
                    (4, 4) => 0.39,
                    _ => 1.0,
                },
                _ => 1.0,
            }
        }
        TOP => {
            // TODO: hacer con tabla 19
            1.0
        }
        BOTTOM => 1.0,
    }
}

/// Transmitancia térmica de una composición de cerramiento, en una posición dada, en W/m2K
/// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
/// Notas:
/// - en particiones interiores NO se considera el factor b, reductor de temperatura
/// - NO se ha implementado el cálculo de elementos en contacto con espacios no habitables
/// - NO se ha implementado el cálculo de cerramientos en contacto con el terreno
///     - en HULC los valores por defecto de Ra y D se indican en las opciones generales de
///       las construcciones por defecto
/// - los elementos adiabáticos se reportan con valor 0.0
pub fn u_for_wall(position: Tilt, bounds: Boundaries, construction: &WallCons) -> f32 {
    let r_intrinsic = construction.r_intrinsic;

    // Resistencias superficiales [m2·K/W]
    // Revisar según DA-DB-HE/1 tabla 1
    const RSE: f32 = 0.04;
    const RSI_ASCENDENTE: f32 = 0.10;
    const RSI_HORIZONTAL: f32 = 0.13;
    const RSI_DESCENDENTE: f32 = 0.17;

    let u_noround = match (bounds, position) {
        // TODO: implementar soleras en contacto con el terreno
        (UNDERGROUND, BOTTOM) => Default::default(),
        // TODO: implementar muros enterrados
        (UNDERGROUND, SIDE) => Default::default(),
        // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
        (UNDERGROUND, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
        // Tomamos valor 0.0. Siempre se podría consultar la resistencia intrínseca
        (ADIABATIC, _) => 0.0,
        // HULC no diferencia entre posiciones para elementos interiores
        // TODO: Detectar el caso de contacto con espacios no habitables, con cálculo de b, e implementar
        // TODO: tal vez esto debería recibir el valor b como parámetro
        (INTERIOR, _) => 1.0 / (r_intrinsic + 2.0 * RSI_HORIZONTAL),
        // Elementos en contacto con el exterior
        (EXTERIOR, BOTTOM) => 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE),
        (EXTERIOR, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
        (EXTERIOR, SIDE) => 1.0 / (r_intrinsic + RSI_HORIZONTAL + RSE),
    };
    fround2(u_noround)
}
