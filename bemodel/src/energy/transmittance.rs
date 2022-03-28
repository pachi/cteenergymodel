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

use std::f32::consts::PI;

use anyhow::{format_err, Error};
use log::{debug, warn};

use super::EnergyIndicators;
use crate::types::HasSurface;
use crate::{
    utils::fround2, BoundaryType, ConsDb, Layer, MatProps, MatsDb, Model, Space, SpaceType, Tilt,
    Wall, WallCons, Window, WindowCons,
};

// Resistencias superficiales UNE-EN ISO 6946 [m2·K/W]
const RSI_ASCENDENTE: f32 = 0.10;
const RSI_HORIZONTAL: f32 = 0.13;
const RSI_DESCENDENTE: f32 = 0.17;
const RSE: f32 = 0.04;
// conductividad del terreno no helado, en [W/(m·K)]
const LAMBDA_GND: f32 = 2.0;
const LAMBDA_INS: f32 = 0.035;

impl Model {
    /// Calcula indicadores energéticos
    pub fn energy_indicators(&self) -> EnergyIndicators {
        EnergyIndicators::compute(self)
    }
}

impl Space {
    /// Perímetro expuesto del espacio, m
    ///
    /// El perímetro expuesto es el que separa el espacio del exterior o de un espacio no calefactado fuera de la estructura aislada
    /// Excluye las parte que separan un espacio calefactado con otros espacios calefactados
    pub fn perimeter_exposed(&self, walls: &[Wall], spaces: &[Space]) -> f32 {
        use crate::BoundaryType::{ADIABATIC, EXTERIOR, GROUND, INTERIOR};

        let spc_walls: Vec<_> = self.walls(walls).collect();

        let vert_walls = spc_walls.iter().filter(|w| Tilt::from(**w) == Tilt::SIDE);
        // Area bruta total de muros y área bruta de muros exteriores
        let (total_area, exterior_area) = vert_walls
            .map(|w| {
                let area = w.area();
                match w.bounds {
                    // Contactos con el exterior o el terreno
                    EXTERIOR | GROUND => (area, area),
                    // Contactos con otros espacios no acondicionados o no habitables
                    INTERIOR => {
                        w.next_to
                            .and_then(|nxts| spaces.iter().find(|s| s.id == nxts))
                            .and_then(|nextspace| {
                                if self.kind == SpaceType::CONDITIONED
                                    && nextspace.kind != SpaceType::CONDITIONED
                                {
                                    // tenemos en cuenta el contacto de espacios acondicionados con otros tipos
                                    Some((area, area))
                                } else {
                                    None
                                }
                            })
                            // El resto no se considera contacto con el exterior
                            .unwrap_or((area, 0.0))
                    }
                    ADIABATIC => (area, 0.0),
                }
            })
            .fold((0.0, 0.0), |(acc_tot, acc_ext), (el_tot, el_ext)| {
                (acc_tot + el_tot, acc_ext + el_ext)
            });

        if total_area < 0.01 {
            0.0
        } else {
            // Walls that belong to the space and are floors (floors from .next_to are ceilings of this space)
            let spc_floor_walls: Vec<_> = spc_walls
                .iter()
                .filter(|w| w.space == self.id && Tilt::from(**w) == Tilt::BOTTOM)
                .collect();
            if spc_floor_walls.len() > 1 {
                warn!(
                    "Calculando perímetro expuesto para espacio con más de un suelo: {} ({}), {:#?}",
                    self.name, self.id, spc_floor_walls
                );
            };
            match spc_floor_walls.get(0) {
                Some(first_floor) => {
                    if first_floor.bounds != GROUND {
                        warn!(
                            "Calculando perímetro expuesto para muro que no está en contacto con el terreno: {} ({})",
                            first_floor.name, first_floor.id
                        );
                    }
                    let perimeter = first_floor.geometry.perimeter();
                    fround2(perimeter * exterior_area / total_area)
                }
                _ => 0.0,
            }
        }
    }
}

impl WallCons {
    /// Resistencia térmica intrínseca (sin resistencias superficiales) de una composición de capas [W/m²K]
    /// TODO: convertir errores a warning para logging y devolver Option<f32>
    pub fn r_intrinsic(&self, mats: &MatsDb) -> Result<f32, Error> {
        let mut total_resistance = 0.0;
        for Layer { id, e } in &self.layers {
            match mats.materials.iter().find(|m| &m.id==id) {
                None => return Err(format_err!(
                    "No se encuentra el material \"{}\" de la composición de capas \"{}\"",
                    id,
                    self.name
                )),
                Some(mat) => {
                    match mat.properties {
                        MatProps::Detailed{ conductivity, .. } if conductivity > 0.0 => total_resistance += e / conductivity,
                        MatProps::Resistance{ resistance} => total_resistance += resistance,
                        _ => return Err(format_err!(
                            "Material \"{}\" de la composición de capas \"{}\" con conductividad nula o casi nula",
                            mat.name,
                            self.name
                        ))
                    }
                },
            }
        }
        Ok(total_resistance)
    }
}

impl WindowCons {
    /// Transmitancia térmica total de la construcción de hueco, U_W, en una posición dada, en W/m2K
    ///
    /// Incluye el efecto del marco, vidrio y efecto de intercalarios y/o cajones de persiana
    /// Notas:
    /// - estos valores ya deben incluir las resistencias superficiales
    ///   (U_g se calcula con resistencias superficiales y U_w es una ponderación)
    pub fn u_value(&self, mats: &MatsDb) -> Option<f32> {
        let glass = mats.glasses.iter().find(|g| g.id == self.glass)?;
        let frame = mats.frames.iter().find(|f| f.id == self.frame)?;
        Some(fround2(
            (1.0 + self.delta_u / 100.0)
                * (frame.u_value * self.f_f + glass.u_value * (1.0 - self.f_f)),
        ))
    }
}

impl Window {
    /// Transmitancia térmica total del hueco, U_W, en una posición dada, en W/m2K
    ///
    /// Incluye el efecto del marco, vidrio y efecto de intercalarios y/o cajones de persiana
    /// Notas:
    /// - estos valores ya deben incluir las resistencias superficiales
    ///   (U_g se calcula con resistencias superficiales y U_w es una ponderación)
    pub fn u_value(&self, cons: &ConsDb, mats: &MatsDb) -> Option<f32> {
        cons.get_wincons(self.cons)?.u_value(mats)
    }
}

impl Wall {
    /// Transmitancia térmica de una composición de cerramiento, en una posición dada, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - los elementos adiabáticos se reportan con valor 0.0
    /// - los elementos mal definidos (muros sin construcción o sin espacio asignado) se reportan con valor 0.0
    /// - se usan resistencias superficiales de referencia (DB-HE)
    pub fn u_value(&self, model: &Model) -> Option<f32> {
        use {BoundaryType::*, SpaceType::*, Tilt::*};

        let position = Tilt::from(self);
        let bounds: BoundaryType = self.bounds;
        let R_n_perim_ins = model.meta.rn_perim_insulation;
        let D_perim_ins = model.meta.d_perim_insulation;

        let R_intrinsic = model
            .cons
            .get_wallcons(self.cons)?
            .r_intrinsic(&model.mats)
            .ok()?;

        match (bounds, position) {
            // Elementos adiabáticos -----------------------------
            (ADIABATIC, _) => {
                let U = 0.0;
                debug!("{} (adiabático) U={:.2}", self.name, U);
                Some(U)
            }
            // Elementos en contacto con el exterior -------------
            (EXTERIOR, BOTTOM) => {
                let U = fround2(1.0 / (R_intrinsic + RSI_DESCENDENTE + RSE));
                debug!("{} (suelo) U={:.2}", self.name, U);
                Some(U)
            }
            (EXTERIOR, TOP) => {
                let U = fround2(1.0 / (R_intrinsic + RSI_ASCENDENTE + RSE));
                debug!("{} (cubierta) U={:.2}", self.name, U);
                Some(U)
            }
            (EXTERIOR, SIDE) => {
                let U = fround2(1.0 / (R_intrinsic + RSI_HORIZONTAL + RSE));
                debug!("{} (muro) U={:.2}", self.name, U);
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
                let wspace = model.get_space(self.space)?;
                let gnd_A = wspace.area;
                let mut gnd_P = wspace.perimeter_exposed(&model.walls, &model.spaces);

                // Soleras sin contacto perimetral con el exterior P=0 -> B' -> inf. Cambiamos P a un valor pequeño
                if gnd_P.abs() < 0.001 {
                    warn!(
                        "{} (solera con perímetro expuesto nulo o casi nulo {:.2})",
                        self.name, gnd_P,
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

                let U = fround2(U_bf + 2.0 * psi_ge / B_1); // H_g sería U * A

                debug!(
                    "{} (suelo de sótano) U={:.2} (R_n={:.2}, D={:.2}, A={:.2}, P={:.2}, B'={:.2}, z={:.2}, d_t={:.2}, R_f={:.3}, U_bf={:.2}, psi_ge = {:.3})",
                    self.name,
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
                let space = model.get_space(self.space)?;
                // XXX: Estamos suponiendo que la cota z es la del suelo del espacio
                let z = if space.z < 0.0 { -space.z } else { 0.0 };
                // Muros que realmente no son enterrados
                if z.abs() < 0.01 {
                    warn!(
                        "{} (muro de sótano no enterrado z=0) U_w={:.2} (z={:.2})",
                        self.name, U_w, z,
                    );
                    return Some(U_w);
                };

                // Dimensión característica del suelo del sótano.
                // Suponemos espesor de muros de sótano = 0.30m para cálculo de soleras
                // Usamos el promedio de los suelos del espacio
                // TODO: ¿Tendría que ser la media ponderada por superficie?
                let mut d_t = space
                    .walls(&model.walls)
                    .filter(|w| Tilt::from(*w) == BOTTOM)
                    .zip(1..)
                    .fold(0.0, |mean, (w, i)| {
                        // Si no está definida la construcción no participa de la envolvente
                        model
                            .cons
                            .get_wallcons(w.cons)
                            .map(|wallcons| match wallcons.r_intrinsic(&model.mats).ok() {
                                Some(wallcons_r_intrinsic) => {
                                    (W + LAMBDA_GND
                                        * (RSI_DESCENDENTE + wallcons_r_intrinsic + RSE)
                                        + mean * (i - 1) as f32)
                                        / i as f32
                                }
                                _ => 0.0,
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
                let height_net = space.height_net(&model.walls, &model.cons);

                // Altura sobre el terreno (muro no enterrado)
                let h = if height_net > z { height_net - z } else { 0.0 };

                // Si el muro no es enterrado en toda su altura ponderamos U por altura
                let U = fround2(if h == 0.0 {
                    // Muro completamente enterrado
                    U_bw
                } else {
                    // Muro con z parcialmente enterrado
                    (z * U_bw + h * U_w) / height_net
                });

                debug!(
                    "{} (muro enterrado) U={:.2} (z={:.2}, h={:.2}, U_w={:.2}, U_bw={:.2}, d_t={:.2}, d_w={:.2})",
                    self.name, U, z, h, U_w, U_bw, d_t, d_w,
                );
                Some(U)
            }
            // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
            (GROUND, TOP) => {
                let U = fround2(1.0 / (R_intrinsic + RSI_ASCENDENTE + RSE));
                debug!(
                    "{} (cubierta enterrada) U={:.2} (R_f={:.3})",
                    self.name, U, R_intrinsic
                );
                Some(U)
            }
            // Elementos en contacto con otros espacios ---------------------
            (INTERIOR, position) => {
                // Dos casos:
                // - Suelos en contacto con sótanos no acondicionados / no habitables en contacto con el terreno - ISO 13370:2010 (9.4)
                // - Elementos en contacto con espacios no acondicionados / no habitables - UNE-EN ISO 6946:2007 (5.4.3)
                let space = model.get_space(self.space)?;
                let nextto = match self.next_to {
                    Some(s) => s,
                    _ => {
                        warn!(
                            "Muro {} ({}) sin definición de espacio adyacente",
                            self.id, self.name
                        );
                        return None;
                    }
                };

                let nextspace = match model.get_space(nextto) {
                    Some(s) => s,
                    _ => {
                        warn!(
                            "Muro {} ({}) con definición de espacio adyacente incorrecta {}",
                            self.id, self.name, nextto
                        );
                        return None;
                    }
                };

                let nexttype = nextspace.kind;

                let posname = match position {
                    BOTTOM => "suelo",
                    TOP => "techo",
                    SIDE => "muro",
                };

                if nexttype == CONDITIONED && space.kind == CONDITIONED {
                    // Elemento interior con otro espacio acondicionado
                    // HULC no diferencia entre RS según posiciones para elementos interiores
                    let U = fround2(1.0 / (R_intrinsic + 2.0 * RSI_HORIZONTAL));
                    debug!(
                        "{} ({} acondicionado-acondicionado) U_int={:.2}",
                        self.name, posname, U
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
                    let uncondspace_v =
                        uncondspace.height_net(&model.walls, &model.cons) * uncondspace.area;
                    let n_ven = match uncondspace.n_v {
                        Some(n_v) => n_v,
                        _ => match model.meta.global_ventilation_l_s {
                            Some(global_ventilation) => {
                                3.6 * global_ventilation / model.vol_env_inh_net()
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
                    let UA_e_k = uncondspace
                        .walls(&model.walls)
                        .filter(|wall| wall.bounds == GROUND || wall.bounds == EXTERIOR)
                        .filter_map(|wall| {
                            // A·U de muros (y suelos) + A.U de sus huecos
                            let wall_u = self.u_value(model)?;
                            let win_axu = wall
                                .windows(&model.windows)
                                .filter_map(|win| {
                                    // Si no está definida la construcción, el hueco no participa de la envolvente
                                    win.u_value(&model.cons, &model.mats)
                                        .map(|u| Some(win.area() * u))?
                                })
                                .sum::<f32>();
                            Some(wall.area_net(&model.windows) * wall_u + win_axu)
                        })
                        .sum::<f32>();
                    // 1/U = 1/U_f + A_i / (sum_k(A_e_k·U_e_k) + 0.33·n·V) (17)
                    // En la fórmula anterior, para espacios no acondicionados, se indica que se excluyen suelos, pero no entiendo bien por qué.
                    // Esta fórmula, cuando los A_e_k y U_e_k incluyen los muros y suelos con el terreno U_bw y U_bf, con la parte proporcional de
                    // muros al exterior, es equivalente a la que indica la 13370
                    let A_i = self.area_net(&model.windows);
                    let H_ue = UA_e_k + 0.33 * n_ven * uncondspace_v;
                    let R_u = A_i / H_ue;
                    let U = fround2(1.0 / (R_f + R_u));

                    debug!(
                            "{} ({} acondicionado-no acondicionado/sotano) U={:.2} (R_f={:.3}, R_u={:.3}, A_i={:.2}, U_f=1/R_f={:.2}",
                            self.name, posname, U, R_f, R_u, A_i, 1.0/R_f
                        );
                    Some(U)
                }
            }
        }
    }
}
