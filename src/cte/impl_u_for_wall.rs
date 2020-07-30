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

//! Implementación del cálculo de la U de una composión constructiva de opaco, según su posición
//! - factores de (UNE-EN ISO 13789:2017):
//!     - b de un elemento de separación con un espacio no acondicionado (UNE-EN ISO 13789:2017) (salvo que esté en contacto con cámara sanitaria UNE-EN ISO 13370)
//!         - q_iu = 0
//!         - q_ue = V_u · n_ue; clase de permeablidad (tipo) y n_ue (tabla 7) ->  1 => 0.1 renh, 2 => 0.5 renh, 3 => 1 renh, 4 => 3 renh, 5 => 10 renh.
//!         - UNE-EN ISO 6946 -> 5.4.3
//!     - bm para acoplamiento con el terremo
//!     - b con edificios adyacentes -> b = 0 (depende de la diferencia de temperaturas, pero es cero si es igual)

use std::f32::consts::PI;

use crate::utils::fround2;

pub use super::{Boundaries, Model, Orientation, SpaceType, Tilt, Wall, WallCons};

// Resistencias superficiales UNE-EN ISO 6946 [m2·K/W]
const RSI_ASCENDENTE: f32 = 0.10;
const RSI_HORIZONTAL: f32 = 0.13;
const RSI_DESCENDENTE: f32 = 0.17;
const RSE: f32 = 0.04;
// conductividad del terreno no helado, en [W/(m·K)]
const LAMBDA: f32 = 2.0;
impl Model {
    // TODO: probablemente esto debería ser una función global que accede a espacios y elementos para poder localizar elementos de contorno
    // TODO: en elementos enterrados es necesario así como en elementos en contacto con otros espacios no calefactados.

    /// Transmitancia térmica de una composición de cerramiento, en una posición dada, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - en particiones interiores NO se considera el factor b, reductor de temperatura
    /// - NO se ha implementado el cálculo de elementos en contacto con espacios no habitables
    /// - NO se ha implementado el cálculo de cerramientos en contacto con el terreno
    ///     - en HULC los valores por defecto de Ra y D se indican en las opciones generales de
    ///       las construcciones por defecto
    /// - los elementos adiabáticos se reportan con valor 0.0
    pub fn u_for_wall(&self, wall: &Wall) -> f32 {
        use {Boundaries::*, Orientation::*, SpaceType::*, Tilt::*};

        let position: Tilt = wall.tilt.into();
        let bounds: Boundaries = wall.bounds.into();
        let area = wall.area;
        let zground = wall.zground;

        let cons = self.wallcons.get(&wall.cons).unwrap();
        let r_intrinsic = cons.r_intrinsic;

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
                let u_bw = (2.0 * LAMBDA / (PI * z))
                    * (1.0 + 0.5 * d_t / (d_t + z))
                    * f32::ln(z / d_w + 1.0);
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
}
