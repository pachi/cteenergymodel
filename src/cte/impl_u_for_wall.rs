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
//! - UNE-EN ISO 13789:2010 transmisión general
//! - UNE-EN ISO 6946:2012 para elementos opacos
//! - UNE-EN ISO 13770:2017 para elementos en contacto con el terremo
//! TODO: No estamos teniendo en cuenta la transmitancia de los huecos
//! TODO: comprobar si medimos superficies netas o brutas en muros

use std::f32::consts::PI;

use crate::utils::fround2;

pub use super::{Boundaries, Model, Orientation, SpaceType, Tilt, Wall, WallCons, Window};

// Resistencias superficiales UNE-EN ISO 6946 [m2·K/W]
const RSI_ASCENDENTE: f32 = 0.10;
const RSI_HORIZONTAL: f32 = 0.13;
const RSI_DESCENDENTE: f32 = 0.17;
const RSE: f32 = 0.04;
// conductividad del terreno no helado, en [W/(m·K)]
const LAMBDA_GND: f32 = 2.0;
const LAMBDA_INS: f32 = 0.035;

impl Model {
    /// Devuelve huecos del muro
    pub fn get_wall_windows(&self, wall: &Wall) -> Vec<&Window> {
        self.windows
            .values()
            .filter(|w| w.wall == wall.name)
            .collect()
    }

    /// Vector de muros (incluyendo suelos y techos) que delimitan un espacio
    pub fn get_space_walls(&self, space: &str) -> Vec<&Wall> {
        self.walls
            .values()
            .filter(|w| {
                w.space == space
                    || (if let Some(ref spc) = w.nextto {
                        spc == space
                    } else {
                        false
                    })
            })
            // .map(|w| w.name.as_str())
            .collect()
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
    pub fn u_for_wall(&self, wall: &Wall) -> f32 {
        use {Boundaries::*, Tilt::*};

        let position: Tilt = wall.tilt.into();
        let bounds: Boundaries = wall.bounds.into();
        let z = wall.zground.unwrap_or(0.0);
        let r_n_perim_ins = self.meta.rn_perim_insulation;
        let d_perim_ins = self.meta.d_perim_insulation;

        let cons = self.wallcons.get(&wall.cons).unwrap();
        let r_intrinsic = cons.r_intrinsic;

        let u_noround = match (bounds, position) {
            // Elementos adiabáticos -----------------------------
            (ADIABATIC, _) => 0.0,
            // Elementos en contacto con el exterior -------------
            (EXTERIOR, BOTTOM) => 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE),
            (EXTERIOR, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
            (EXTERIOR, SIDE) => 1.0 / (r_intrinsic + RSI_HORIZONTAL + RSE),
            // Elementos enterrados ------------------------------
            (UNDERGROUND, BOTTOM) => {
                // 1. Solera sobre el terreno: UNE-EN ISO 13370:2010 Apartado 9.1 y 9.3.2
                // Simplificaciones:
                // - forma cuadrada para calcular el perímetro
                // - ancho de muros externos w = 0.3m
                // - lambda de aislamiento = 0,035 W/mK
                //
                // HULC parece estar calculando algo más parecido al método de Winkelman o:
                // let u = 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE + 0.25 / LAMBDA_GND + 0.01 / LAMBDA_INS);

                let otherwalls = self.get_space_walls(&wall.space);
                let gnd_a = otherwalls
                    .iter()
                    .filter(|w| w.bounds == UNDERGROUND && Tilt::from(w.tilt) == BOTTOM)
                    .map(|w| w.area)
                    .sum();
                // Si no está definido, entonces suponemos superficie cuadrada
                let gnd_p = if let Some(perimeter) = wall.perimeter {
                    perimeter
                } else {
                    4.0 * f32::sqrt(gnd_a)
                };

                // Dimensión característica del suelo (B'). Ver UNE-EN ISO 13370:2010 8.1
                let b_1 = gnd_a / (0.5 * gnd_p);

                const W: f32 = 0.3; // Simplificación: espesor supuesto de los muros perimetrales
                let d_t = W + LAMBDA_GND * (RSI_DESCENDENTE + r_intrinsic + RSE);

                let u_bf = if (d_t + 0.5 * z) < b_1 {
                    // Soleras sin aislar y moderadamente aisladas
                    (2.0 * LAMBDA_GND / (PI * b_1 + d_t + 0.5 * z))
                        * f32::ln(1.0 + PI * b_1 / (d_t + 0.5 * z))
                } else {
                    // Soleras bien aisladas
                    LAMBDA_GND / (0.457 * b_1 + d_t + 0.5 * z)
                };

                // Efecto del aislamiento perimetral 13770 Anexo B.
                // Espesor aislamiento perimetral d_n = r_n_perim_ins * lambda_ins
                // Espesor equivalente adicional resultante del aislamiento perimetral (d')
                let d_1 = r_n_perim_ins * (LAMBDA_GND - LAMBDA_INS);
                let psi_ge = -LAMBDA_GND / PI
                    * (f32::ln(d_perim_ins / d_t + 1.0) - f32::ln(1.0 + d_perim_ins / (d_t + d_1)));

                let u = u_bf + 2.0 * psi_ge / b_1; // H_g sería U * A

                log::info!(
                    "U de suelo de sótano {}: {} (Rn={}, D={}, gnd_a={}, gnd_p={}, B'={}, z={}, d_t={}, R_f={}, U_bf={}, psi_ge = {})",
                    wall.name,
                    u,
                    r_n_perim_ins,
                    d_perim_ins,
                    gnd_a,
                    gnd_p,
                    b_1,
                    z,
                    d_t,
                    r_intrinsic,
                    u_bf,
                    psi_ge
                );
                u
            }
            (UNDERGROUND, SIDE) => {
                // 2. Muros enterrados UNE-EN ISO 13370:2010 9.3.3
                let u_ext = 1.0 / (RSI_HORIZONTAL + r_intrinsic + RSE);

                // Muros que realmente no son enterrados
                if z.abs() < 0.1 {
                    log::info!(
                        "U de muro de sótano no enterrado {}: {} (z={})",
                        wall.name,
                        u_ext,
                        z,
                    );
                    return u_ext;
                };

                // Dimensión característica del suelo del sótano.
                // Suponemos espesor de muros de sótano = 0.30m para cálculo de soleras
                // Usamos el promedio de los suelos del espacio
                let space = self.spaces.get(&wall.space).unwrap();
                let mut d_t = self
                    .get_space_walls(space.name.as_str())
                    .iter()
                    .filter(|w| Tilt::from(w.tilt) == BOTTOM)
                    .zip(1..)
                    .fold(0.0, |mean, (w, i)| {
                        (W + LAMBDA_GND
                            * (RSI_DESCENDENTE
                                + self.wallcons.get(&w.cons).unwrap().r_intrinsic
                                + RSE)
                            + mean * (i - 1) as f32)
                            / i as f32
                    });
                const W: f32 = 0.3;

                // Espesor equivalente de los muros de sótano (13)
                let d_w = LAMBDA_GND * (RSI_HORIZONTAL + r_intrinsic + RSE);

                if d_w < d_t {
                    d_t = d_w
                };

                // U del muro completamente enterrado a profundidad z (14)
                let u_bw = if z != 0.0 {
                    (2.0 * LAMBDA_GND / (PI * z))
                        * (1.0 + 0.5 * d_t / (d_t + z))
                        * f32::ln(z / d_w + 1.0)
                } else {
                    u_ext
                };

                // Si el muro no es enterrado en toda su altura ponderamos U por altura
                let h_muro = space.height_net;
                let u = if z >= h_muro {
                    // Muro completamente enterrado
                    u_bw
                } else {
                    // Muro con z parcialmente enterrado
                    (z * u_bw + (h_muro - z) * u_ext) / h_muro
                };

                log::info!(
                    "U de muro de sótano {}: {} (z={}, h_muro={}, u_ext={}, u_bw={}, d_t={}, d_w={})",
                    wall.name,
                    u,
                    z,
                    h_muro,
                    u_ext,
                    u_bw,
                    d_t,
                    d_w,
                );

                u
            }
            // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
            (UNDERGROUND, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
            // Elementos en contacto con otros espacios ---------------------
            (INTERIOR, position @ _) => {
                // Tres casos:
                // - Elementos en contacto con otros espacios habitables
                // - Suelos en contacto con sótanos no habitables en contacto con el terreno
                // - Elementos en contacto con espacios no habitables
                use SpaceType::*;

                let nextto = wall.nextto.as_ref().unwrap();
                let nextspace = self.spaces.get(nextto.as_str()).unwrap();
                let nexttype = nextspace.space_type;

                if nexttype == CONDITIONED {
                    // Elemento interior con otro espacio habitable
                    // HULC no diferencia entre RS según posiciones para elementos interiores
                    1.0 / (r_intrinsic + 2.0 * RSI_HORIZONTAL)
                } else {
                    // Conexión con sótanos no calefactados o sobreelevados - 13370:2010 (9.4)
                    // Conexión con espacios no calefactados - UNE-EN ISO 6946:2007 (5.4.3)
                    let r_f = match position {
                        BOTTOM => r_intrinsic + 2.0 * RSI_DESCENDENTE,
                        TOP => r_intrinsic + 2.0 * RSI_ASCENDENTE,
                        SIDE => r_intrinsic + 2.0 * RSI_HORIZONTAL,
                    };

                    // Intercambio de aire del espacio adyacente
                    let nextspace_v = nextspace.height_net * nextspace.area;
                    let n_ven = match nextspace.n_v {
                        Some(n_v) => n_v,
                        _ => {
                            3.6 * self.meta.global_ventilation_l_s.unwrap() / self.vol_env_inh_net()
                        }
                    };

                    // Calculamos el A.U de los elementos del espacio que dan al exterior o al terreno (excluye interiores))
                    let nextwalls = self.get_space_walls(nextspace.name.as_str());
                    let ua_nextwalls = nextwalls
                        .iter()
                        .filter(|w| w.bounds == UNDERGROUND || w.bounds == EXTERIOR)
                        .map(|w| {
                            // A·U de muros + A.U de sus huecos
                            w.area * self.u_for_wall(w)
                                + self
                                    .get_wall_windows(&w)
                                    .iter()
                                    .map(|win| win.area * self.wincons.get(&win.cons).unwrap().u)
                                    .sum::<f32>()
                        })
                        .sum::<f32>();
                    // 1/U = 1/U_f + A / (sum A·U + 0.33·n·V) (17)
                    let r_u = wall.area / (ua_nextwalls + 0.33 * n_ven * nextspace_v);
                    let u = 1.0 / (r_f + r_u);
                    log::info!(
                        "Elemento en contacto con espacio no acondicionado o no habitable {}: {}",
                        wall.name,
                        u
                    );
                    u
                }
            }
        };
        fround2(u_noround)
    }
}
