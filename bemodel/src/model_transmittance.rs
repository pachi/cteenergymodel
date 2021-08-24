// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación del cálculo de la U de una composión constructiva de opaco, según su posición
//! y del cálculo de la transmitancia térmica global K de la envolvente, según CTE DB-HE 2019
//!
//! Para el cálculo de transmitancias se usa:
//!
//! - UNE-EN ISO 13789:2010 para el cálculo de la transmisión general
//! - UNE-EN ISO 6946:2012 para elementos opacos
//! - UNE-EN ISO 13770:2017 para elementos en contacto con el terremo
#![allow(non_snake_case)]

use std::{collections::HashMap, f32::consts::PI};

use log::{debug, info, warn};

use super::{BoundaryType, KData, Model, SpaceType, ThermalBridgeKind, Tilt, UValues, Wall};

// Resistencias superficiales UNE-EN ISO 6946 [m2·K/W]
const RSI_ASCENDENTE: f32 = 0.10;
const RSI_HORIZONTAL: f32 = 0.13;
const RSI_DESCENDENTE: f32 = 0.17;
const RSE: f32 = 0.04;
// conductividad del terreno no helado, en [W/(m·K)]
const LAMBDA_GND: f32 = 2.0;
const LAMBDA_INS: f32 = 0.035;

impl Model {
    /// Calcula la transmitancia térmica global K (W/m2K)
    /// Transmitancia media de opacos, huecos y puentes térmicos en contacto con el aire exterior o con el terreno
    ///
    /// Se ignoran los huecos y muros para los que no está definida su construcción, transmitancia o espacio
    pub fn K(&self) -> KData {
        let mut k = KData::default();
        // Opacos
        for wall in self.walls_of_envelope_iter() {
            let multiplier = self
                .space_of_wall(wall)
                .map(|s| s.multiplier)
                .unwrap_or(1.0);
            // Huecos de los opacos
            for win in self.wincons_of_window_iter(&wall.id) {
                let wincons = match self.wincons_of_window(win) {
                    Some(wincons) => wincons,
                    _ => continue,
                };
                let area = multiplier * win.area;
                k.windows.a += area;
                k.windows.au += area * wincons.u;
                k.windows.u_max = k
                    .windows
                    .u_max
                    .map(|v| v.max(wincons.u))
                    .or(Some(wincons.u));
                k.windows.u_min = k
                    .windows
                    .u_min
                    .map(|v| v.min(wincons.u))
                    .or(Some(wincons.u));
            }
            let wall_u = match self.u_for_wall(wall) {
                Some(u) => u,
                _ => continue,
            };
            let area = multiplier * wall.area;
            let area_u = area * wall_u;
            let mut element_case = match (wall.bounds, Tilt::from(wall)) {
                (BoundaryType::GROUND, _) => &mut k.ground,
                (_, Tilt::TOP) => &mut k.roofs,
                (_, Tilt::BOTTOM) => &mut k.floors,
                (_, Tilt::SIDE) => &mut k.walls,
            };
            element_case.a += area;
            element_case.au += area_u;
            element_case.u_max = element_case.u_max.map(|v| v.max(wall_u)).or(Some(wall_u));
            element_case.u_min = element_case.u_min.map(|v| v.min(wall_u)).or(Some(wall_u));
        }
        // Valores medios de huecos y opacos
        if k.windows.a > 0.001 {
            k.windows.u_mean = Some(k.windows.au / k.windows.a);
        };
        if k.ground.a > 0.001 {
            k.ground.u_mean = Some(k.ground.au / k.ground.a);
        };
        if k.roofs.a > 0.001 {
            k.roofs.u_mean = Some(k.roofs.au / k.roofs.a);
        };
        if k.floors.a > 0.001 {
            k.floors.u_mean = Some(k.floors.au / k.floors.a);
        };
        if k.walls.a > 0.001 {
            k.walls.u_mean = Some(k.walls.au / k.walls.a);
        };
        // PTs
        for tb in &self.thermal_bridges {
            use ThermalBridgeKind::*;
            let l = tb.l;
            // A veces se incluyen longitudes < 0 para señalar que no se han medido
            if l < 0.0 {
                continue;
            };
            let psil = tb.psi * l;
            let mut tb_case = match tb.kind {
                ROOF => &mut k.tbs.roof,
                BALCONY => &mut k.tbs.balcony,
                CORNER => &mut k.tbs.corner,
                INTERMEDIATEFLOOR => &mut k.tbs.intermediate_floor,
                INTERNALWALL => &mut k.tbs.internal_wall,
                GROUNDFLOOR => &mut k.tbs.ground_floor,
                PILLAR => &mut k.tbs.pillar,
                WINDOW => &mut k.tbs.window,
                GENERIC => &mut k.tbs.generic,
            };
            tb_case.l += l;
            tb_case.psil += psil;
        }
        // Cálculo de K y resumen
        k.compute();

        let s = k.summary;
        info!(
            "K={:.2} W/m²K, A_o={:.2} m², (A.U)_o={:.2} W/K, A_h={:.2} m², (A.U)_h={:.2} W/K, L_pt={:.2} m, Psi.L_pt={:.2} W/K",
            k.K, s.opaques_a, s.opaques_au, s.windows_a, s.windows_au, s.tbs_l, s.tbs_psil
        );

        k
    }

    /// Diccionario de transmitancias de elementos del modelo (walls, windows) según id
    pub fn u_values(&self) -> UValues {
        let wallsmap: HashMap<String, Option<f32>> = self
            .walls
            .iter()
            .map(|w| (w.id.clone(), self.u_for_wall(w)))
            .collect();
        let windowsmap: HashMap<String, Option<f32>> = self
            .windows
            .iter()
            .map(|w| (w.id.clone(), self.wincons_of_window(w).map(|w| w.u)))
            .collect();
        UValues {
            walls: wallsmap,
            windows: windowsmap,
        }
    }

    /// Transmitancia térmica de una composición de cerramiento, en una posición dada, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - los elementos adiabáticos se reportan con valor 0.0
    /// - los elementos mal definidos (muros sin construcción o sin espacio asignado) se reportan con valor 0.0
    pub fn u_for_wall(&self, wall: &Wall) -> Option<f32> {
        use {BoundaryType::*, SpaceType::*, Tilt::*};

        let position = Tilt::from(wall);
        let bounds: BoundaryType = wall.bounds;
        let R_n_perim_ins = self.meta.rn_perim_insulation;
        let D_perim_ins = self.meta.d_perim_insulation;

        let cons = self.wallcons_for_wall(wall)?;
        let R_intrinsic = cons.r_intrinsic;

        match (bounds, position) {
            // Elementos adiabáticos -----------------------------
            (ADIABATIC, _) => {
                let U = 0.0;
                debug!("{} (adiabático) U={:.2}", wall.name, U);
                Some(U)
            }
            // Elementos en contacto con el exterior -------------
            (EXTERIOR, BOTTOM) => {
                let U = 1.0 / (R_intrinsic + RSI_DESCENDENTE + RSE);
                debug!("{} (suelo) U={:.2}", wall.name, U);
                Some(U)
            }
            (EXTERIOR, TOP) => {
                let U = 1.0 / (R_intrinsic + RSI_ASCENDENTE + RSE);
                debug!("{} (cubierta) U={:.2}", wall.name, U);
                Some(U)
            }
            (EXTERIOR, SIDE) => {
                let U = 1.0 / (R_intrinsic + RSI_HORIZONTAL + RSE);
                debug!("{} (muro) U={:.2}", wall.name, U);
                Some(U)
            }
            // Elementos enterrados ------------------------------
            (GROUND, BOTTOM) => {
                // 1. Solera sobre el terreno: UNE-EN ISO 13370:2010 Apartado 9.1 y 9.3.2
                // Simplificaciones:
                // - forma cuadrada para calcular el perímetro si no está disponible
                // - ancho de muros externos w = 0.3m
                // - lambda de aislamiento = 0,035 W/mK
                //
                // HULC parece estar calculando algo más parecido al método de Winkelman o:
                // let u = 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE + 0.25 / LAMBDA_GND + 0.01 / LAMBDA_INS);

                // Dimensión característica del suelo (B'). Ver UNE-EN ISO 13370:2010 8.1
                // Calculamos la dimensión característica del **espacio** en el que sitúa el suelo
                // Si este espacio no define el perímetro, lo calculamos suponiendo una superficie cuadrada
                let wspace = self.space_of_wall(wall)?;
                let gnd_A = wspace.area;
                let mut gnd_P = wspace
                    .exposed_perimeter
                    .unwrap_or_else(|| 4.0 * f32::sqrt(gnd_A));

                // Soleras sin contacto perimetral con el exterior P=0 -> B' -> inf. Cambiamos P a un valor pequeño
                if gnd_P.abs() < 0.001 {
                    warn!(
                        "{} (solera con perímetro expuesto nulo o casi nulo {:.2})",
                        wall.name, gnd_P,
                    );
                    gnd_P = 0.000001;
                };

                let B_1 = gnd_A / (0.5 * gnd_P);
                // XXX: Estamos suponiendo que la cota z es la del suelo del espacio
                let z = if wspace.z < 0.0 { -wspace.z } else { 0.0 };
                const W: f32 = 0.3; // Simplificación: espesor supuesto de los muros perimetrales
                let d_t = W + LAMBDA_GND * (RSI_DESCENDENTE + R_intrinsic + RSE);

                // 2 casos:
                // 1. Es un suelo de un espacio acondicionado (9.1 y 9.3.2)
                // 2. Es un suelo de un espacio no acondicionado (9.2 - tomamos U_bg igual a U_g, según 9.2 (8) o a U_bf según E.2)
                // Los dos casos equivalen aproximadamente al U_bf de 9.3.2
                let U_bf = if (d_t + 0.5 * z) < B_1 {
                    // Soleras sin aislar y moderadamente aisladas
                    (2.0 * LAMBDA_GND / (PI * B_1 + d_t + 0.5 * z))
                        * f32::ln(1.0 + PI * B_1 / (d_t + 0.5 * z))
                } else {
                    // Soleras bien aisladas
                    LAMBDA_GND / (0.457 * B_1 + d_t + 0.5 * z)
                };

                // Efecto del aislamiento perimetral 13770 Anexo B (B.4).
                // Espesor aislamiento perimetral d_n = r_n_perim_ins * lambda_ins
                // Espesor equivalente adicional resultante del aislamiento perimetral (d')
                let D_1 = R_n_perim_ins * (LAMBDA_GND - LAMBDA_INS);
                let psi_ge = -LAMBDA_GND / PI
                    * (f32::ln(D_perim_ins / d_t + 1.0) - f32::ln(1.0 + D_perim_ins / (d_t + D_1)));

                let U = U_bf + 2.0 * psi_ge / B_1; // H_g sería U * A

                debug!(
                    "{} (suelo de sótano) U={:.2} (R_n={:.2}, D={:.2}, A={:.2}, P={:.2}, B'={:.2}, z={:.2}, d_t={:.2}, R_f={:.3}, U_bf={:.2}, psi_ge = {:.3})",
                    wall.name,
                    U,
                    R_n_perim_ins,
                    D_perim_ins,
                    gnd_A,
                    gnd_P,
                    B_1,
                    z,
                    d_t,
                    R_intrinsic,
                    U_bf,
                    psi_ge
                );
                Some(U)
            }
            (GROUND, SIDE) => {
                // 2. Muros enterrados UNE-EN ISO 13370:2010 9.3.3
                let U_w = 1.0 / (RSI_HORIZONTAL + R_intrinsic + RSE);
                let space = self.space_of_wall(wall)?;
                // XXX: Estamos suponiendo que la cota z es la del suelo del espacio
                let z = if space.z < 0.0 { -space.z } else { 0.0 };
                // Muros que realmente no son enterrados
                if z.abs() < 0.01 {
                    warn!(
                        "{} (muro de sótano no enterrado z=0) U_w={:.2} (z={:.2})",
                        wall.name, U_w, z,
                    );
                    return Some(U_w);
                };

                // Dimensión característica del suelo del sótano.
                // Suponemos espesor de muros de sótano = 0.30m para cálculo de soleras
                // Usamos el promedio de los suelos del espacio
                let mut d_t = self
                    .walls_of_space_iter(&space.id)
                    .filter(|w| Tilt::from(*w) == BOTTOM)
                    .zip(1..)
                    .fold(0.0, |mean, (w, i)| {
                        // Si no está definida la construcción no participa de la envolvente
                        self.wallcons_for_wall(w)
                            .map(|wallcons| {
                                (W + LAMBDA_GND * (RSI_DESCENDENTE + wallcons.r_intrinsic + RSE)
                                    + mean * (i - 1) as f32)
                                    / i as f32
                            })
                            .unwrap_or(0.0)
                    });
                const W: f32 = 0.3;

                // Espesor equivalente de los muros de sótano (13)
                let d_w = LAMBDA_GND * (RSI_HORIZONTAL + R_intrinsic + RSE);

                if d_w < d_t {
                    d_t = d_w
                };

                // U del muro completamente enterrado a profundidad z (14)
                let U_bw = if z != 0.0 {
                    (2.0 * LAMBDA_GND / (PI * z))
                        * (1.0 + 0.5 * d_t / (d_t + z))
                        * f32::ln(z / d_w + 1.0)
                } else {
                    U_w
                };

                // Altura neta
                let height_net = space.height - self.top_wall_thickness_of_space(&space.id);

                // Altura sobre el terreno (muro no enterrado)
                let h = if height_net > z { height_net - z } else { 0.0 };

                // Si el muro no es enterrado en toda su altura ponderamos U por altura
                let U = if h == 0.0 {
                    // Muro completamente enterrado
                    U_bw
                } else {
                    // Muro con z parcialmente enterrado
                    (z * U_bw + h * U_w) / height_net
                };

                debug!(
                    "{} (muro enterrado) U={:.2} (z={:.2}, h={:.2}, U_w={:.2}, U_bw={:.2}, d_t={:.2}, d_w={:.2})",
                    wall.name, U, z, h, U_w, U_bw, d_t, d_w,
                );
                Some(U)
            }
            // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
            (GROUND, TOP) => {
                let U = 1.0 / (R_intrinsic + RSI_ASCENDENTE + RSE);
                debug!(
                    "{} (cubierta enterrada) U={:.2} (R_f={:.3})",
                    wall.name, U, R_intrinsic
                );
                Some(U)
            }
            // Elementos en contacto con otros espacios ---------------------
            (INTERIOR, position) => {
                // Dos casos:
                // - Suelos en contacto con sótanos no acondicionados / no habitables en contacto con el terreno - ISO 13370:2010 (9.4)
                // - Elementos en contacto con espacios no acondicionados / no habitables - UNE-EN ISO 6946:2007 (5.4.3)
                let space = self.space_of_wall(wall)?;
                let nextto = match wall.nextto.as_ref() {
                    Some(s) => s,
                    _ => {
                        warn!(
                            "Muro {} ({}) sin definición de espacio adyacente",
                            wall.id, wall.name
                        );
                        return None;
                    }
                };

                let nextspace = match self.space_by_id(nextto.as_str()) {
                    Some(s) => s,
                    _ => {
                        warn!(
                            "Muro {} ({}) con definición de espacio adyacente incorrecta {}",
                            wall.id,
                            wall.name,
                            nextto.as_str()
                        );
                        return None;
                    }
                };

                let nexttype = nextspace.space_type;

                let posname = match position {
                    BOTTOM => "suelo",
                    TOP => "techo",
                    SIDE => "muro",
                };

                if nexttype == CONDITIONED && space.space_type == CONDITIONED {
                    // Elemento interior con otro espacio acondicionado
                    // HULC no diferencia entre RS según posiciones para elementos interiores
                    let U = 1.0 / (R_intrinsic + 2.0 * RSI_HORIZONTAL);
                    debug!(
                        "{} ({} acondicionado-acondicionado) U_int={:.2}",
                        wall.name, posname, U
                    );
                    Some(U)
                } else {
                    // Comunica un espacio acondicionado con otro no acondicionado

                    // Localizamos el espacio no acondicionado
                    let (uncondspace, thiscondspace) = if nexttype == CONDITIONED {
                        (space, false)
                    } else {
                        (nextspace, true)
                    };

                    // Resistencia del elemento teniendo en cuenta el flujo de calor (UNE-EN ISO 13789 Tabla 8)
                    let R_f = match (position, thiscondspace) {
                        // Suelo de espacio acondicionado hacia no acondicionado inferior
                        // Techo de espacio no acondicionado hacia acondicionado inferior
                        (BOTTOM, true) | (TOP, false) => R_intrinsic + 2.0 * RSI_DESCENDENTE,
                        // Techo de espacio acondicionado hacia no acondicionado superior
                        // Suelo de espacio no acondicionado hacia acondicionado superior
                        (TOP, true) | (BOTTOM, false) => R_intrinsic + 2.0 * RSI_ASCENDENTE,
                        // Muro
                        (SIDE, _) => R_intrinsic + 2.0 * RSI_HORIZONTAL,
                    };

                    // Intercambio de aire en el espacio no acondicionado (¿o podría ser el actual si es el no acondicionado?)
                    let uncondspace_v = (uncondspace.height
                        - self.top_wall_thickness_of_space(&uncondspace.id))
                        * uncondspace.area;
                    let n_ven = match uncondspace.n_v {
                        Some(n_v) => n_v,
                        _ => match self.meta.global_ventilation_l_s {
                            Some(global_ventilation) => {
                                3.6 * global_ventilation / self.vol_env_inh_net()
                            }
                            _ => {
                                // Espacio mal definido (ni tiene n_v ni hay definición global de ventilación)
                                warn!("Definición global (l/s) no definida para espacio no acondicionado sin n_v {} ({})", uncondspace.id, uncondspace.name);
                                0.0
                            }
                        },
                    };

                    // CASO: interior en contacto con sótano no calefactado - ISO 13370:2010 (9.4)
                    // CASO: interior en contacto con otro espacio no habitable / no acondicionado - UNE-EN ISO 6946:2007 (5.4.3)
                    // Calculamos el A.U de los elementos del espacio que dan al exterior o al terreno (excluye interiores))
                    // Como hemos asignado U_bw y U_bf a los muros y suelos en contacto con el terreno, ya se tiene en cuenta
                    // la parte enterrada correctamente (fracción enterrada y superficie expuesta, ya que no se consideran los que dan a interiores)
                    let UA_e_k = self
                        .walls_of_space_iter(&uncondspace.id)
                        .filter(|wall| wall.bounds == GROUND || wall.bounds == EXTERIOR)
                        .filter_map(|wall| {
                            // A·U de muros (y suelos) + A.U de sus huecos
                            let wall_u = self.u_for_wall(wall)?;
                            let win_axu = self
                                .wincons_of_window_iter(&wall.id)
                                .filter_map(|win| {
                                    self.wincons_of_window(win)
                                        // Si no está definida la construcción no participa de la envolvente
                                        .map(|wincons| Some(win.area * wincons.u))?
                                })
                                .sum::<f32>();
                            Some(wall.area * wall_u + win_axu)
                        })
                        .sum::<f32>();
                    // 1/U = 1/U_f + A_i / (sum_k(A_e_k·U_e_k) + 0.33·n·V) (17)
                    // En la fórmula anterior, para espacios no acondicionados, se indica que se excluyen suelos, pero no entiendo bien por qué.
                    // Esta fórmula, cuando los A_e_k y U_e_k incluyen los muros y suelos con el terreno U_bw y U_bf, con la parte proporcional de
                    // muros al exterior, es equivalente a la que indica la 13370
                    let A_i = wall.area;
                    let H_ue = UA_e_k + 0.33 * n_ven * uncondspace_v;
                    let R_u = A_i / H_ue;
                    let U = 1.0 / (R_f + R_u);

                    debug!(
                            "{} ({} acondicionado-no acondicionado/sotano) U={:.2} (R_f={:.3}, R_u={:.3}, A_i={:.2}, U_f=1/R_f={:.2}",
                            wall.name, posname, U, R_f, R_u, A_i, 1.0/R_f
                        );
                    Some(U)
                }
            }
        }
    }
}
