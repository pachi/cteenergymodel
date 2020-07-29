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
//! - Fshobst para un hueco
//!     - a partir de su retranqueo
//!     - TODO: para lamas horizontales y/o verticales
//!     - TODO: para voladizos
//! - U de una composión constructiva de opaco, según su posición
//! - factores de (UNE-EN ISO 13789:2017):
//!     - b de un elemento de separación con un espacio no acondicionado (UNE-EN ISO 13789:2017) (salvo que esté en contacto con cámara sanitaria UNE-EN ISO 13370)
//!         - q_iu = 0
//!         - q_ue = V_u · n_ue; clase de permeablidad (tipo) y n_ue (tabla 7) ->  1 => 0.1 renh, 2 => 0.5 renh, 3 => 1 renh, 4 => 3 renh, 5 => 10 renh.
//!         - UNE-EN ISO 6946 -> 5.4.3
//!     - bm para acoplamiento con el terremo
//!     - b con edificios adyacentes -> b = 0 (depende de la diferencia de temperaturas, pero es cero si es igual)

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
    let rh = setback / height;
    let rw = setback / width;
    match tilt.into() {
        // Elementos verticales - Tabla 17 del DA DB-HE/1 (p.19)
        SIDE => {
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
            // Elementos horizontales: tabla 19 DA DB-HE/1 p.19
            let range_rh = if rh <= 0.1 {
                0
            } else if rh <= 0.5 {
                1
            } else if rh <= 1.0 {
                2
            } else if rh <= 2.0 {
                3
            } else if rh <= 5.0 {
                4
            } else {
                5
            };
            let range_rw = if rw <= 0.1 {
                0
            } else if rw <= 0.5 {
                1
            } else if rw <= 1.0 {
                2
            } else if rw <= 2.0 {
                3
            } else if rw <= 5.0 {
                4
            } else {
                5
            };
            let rmin = i32::min(range_rh, range_rw);
            let rmax = i32::max(range_rh, range_rw);
            match (rmax, rmin) {
                (0, 0) => 0.42,
                (1, 0) => 0.43,
                (1, 1) => 0.46,
                (2, 0) => 0.43,
                (2, 1) => 0.48,
                (2, 2) => 0.52,
                (3, 0) => 0.43,
                (3, 1) => 0.50,
                (3, 2) => 0.55,
                (3, 3) => 0.60,
                (4, 0) => 0.44,
                (4, 1) => 0.51,
                (4, 2) => 0.58,
                (4, 3) => 0.66,
                (4, 4) => 0.75,
                (5, 0) => 0.44,
                (5, 1) => 0.52,
                (5, 2) => 0.59,
                (5, 3) => 0.68,
                (5, 4) => 0.79,
                _ => 0.85,
            }
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
pub fn u_for_wall(
    position: Tilt,
    bounds: Boundaries,
    area: f32,
    zground: Option<f32>,
    construction: &WallCons,
) -> f32 {
    use std::f32::consts::PI;

    let r_intrinsic = construction.r_intrinsic;

    // Resistencias superficiales [m2·K/W]
    // Revisar según DA-DB-HE/1 tabla 1
    const RSE: f32 = 0.04;
    const RSI_ASCENDENTE: f32 = 0.10;
    const RSI_HORIZONTAL: f32 = 0.13;
    const RSI_DESCENDENTE: f32 = 0.17;
    // conductividad del terreno no helado, en [W/(m·K)]
    const LAMBDA: f32 = 2.0;

    let u_noround = match (bounds, position) {
        (UNDERGROUND, BOTTOM) => {
            // TODO: implementar soleras en contacto con el terreno
            // Inicialmente implementamos la opción sin aislamientos perimetrales

            // 1. Solera sobre el terreno: UNE-EN ISO 13370:2010 Apartado 9.1 y 9.3.2
            const W: f32 = 0.3; // Simplificación: espesor supuesto de los muros perimetrales
            let d_t = W + LAMBDA * (RSI_DESCENDENTE + r_intrinsic + RSE);
            // TODO: Simplificación: dimensión característica del suelo (B') solo a partir de A de la solera...
            // TODO: debería ser de todo el suelo de la planta en contacto con el terreno ver 8.1
            let b_1 = area / (0.5 * 4.0 * f32::sqrt(area));
            let z = zground.unwrap();

            let u_bf = if d_t < b_1 {
                // Soleras sin aislar y moderadamente aisladas
                (2.0 * LAMBDA / (PI * b_1 + d_t + 0.5 * z))
                    * f32::ln(1.0 + PI * b_1 / (d_t + 0.5 * z))
            } else {
                // Soleras bien aisladas
                LAMBDA / (0.457 * b_1 + d_t)
            };
            log::warn!("U de suelo de sótano: {}", u_bf);
            u_bf
        }
        (UNDERGROUND, SIDE) => {
            // Muros enterrados UNE-EN ISO 13370:2010 9.3.3

            // TODO: Dimensión característica del suelo del sótano
            // TODO: esto tendría que venir del espacio (la r_intrinsic de su suelo)
            const W: f32 = 0.3;
            let d_t = W + LAMBDA * (RSI_DESCENDENTE + r_intrinsic + RSE);

            // Dimensión característica del muro de sótano
            let d_w = LAMBDA * (RSI_HORIZONTAL + r_intrinsic + RSE);
            let z = zground.unwrap();
            // TODO: esto vale 0 si z=0 -> tenemos que calcular la parte enterrada en relación a la altura total y calcular una parte enterrada y otra al exterior...
            // Ver cómo se pondera en DA DB-HE/1
            let u_bw =
                (2.0 * LAMBDA / (PI * z)) * (1.0 + 0.5 * d_t / (d_t + z)) * f32::ln(z / d_w + 1.0);
            log::warn!(
                "U de muro de sótano: {} (z={}, U_ext={})",
                u_bw,
                z,
                1.0 / (r_intrinsic + RSI_HORIZONTAL + RSE)
            );
            u_bw
        }
        // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
        (UNDERGROUND, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
        // Tomamos valor 0.0. Siempre se podría consultar la resistencia intrínseca
        (ADIABATIC, _) => 0.0,
        // HULC no diferencia entre posiciones para elementos interiores
        // TODO: Detectar el caso de contacto con espacios no habitables, con cálculo de b, e implementar
        // TODO: tal vez esto debería recibir el valor b como parámetro
        // TODO: también está el caso del elemento interior que comunica con un sótano no calefactado:
        // TODO: Ver UNE_EN ISO 13370:2010 9.4 que pondera las partes enterradas y no enterradas, adeḿas de la U del elemento interior
        (INTERIOR, _) => 1.0 / (r_intrinsic + 2.0 * RSI_HORIZONTAL),
        // Elementos en contacto con el exterior
        (EXTERIOR, BOTTOM) => 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE),
        (EXTERIOR, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
        (EXTERIOR, SIDE) => 1.0 / (r_intrinsic + RSI_HORIZONTAL + RSE),
    };
    fround2(u_noround)
}
