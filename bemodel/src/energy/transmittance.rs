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
    utils::{fround2, fround3},
    BoundaryType, ConsDb, Layer, MatProps, MatsDb, Model, Space, SpaceType, Tilt, Wall, WallCons,
    Window, WindowCons,
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

    /// Espesor total equivalente de solera (suelo de sótano), d_t, m
    /// Según UNE-EN ISO 13370:2010 9.3.2 (10)
    /// Ponderamos según superficie de suelos en contacto con el terreno
    pub fn slab_on_ground_d_t(&self, walls: &[Wall], cons: &ConsDb, mats: &MatsDb) -> Option<f32> {
        // TODO: No sería mejor ponderar por superficie para obtener la d_t?

        // Suponemos espesor de muros de sótano = 0.30m para cálculo de soleras
        // TODO: No podemos calcular este valor bien?
        const w: f32 = 0.3;

        let mut d_t = self
            .walls(walls)
            .filter(|wall| Tilt::from(*wall) == Tilt::BOTTOM)
            .zip(1..)
            .fold(0.0, |mean, (gwall_i, i)| {
                // Si no está definida la construcción no participa de la envolvente
                cons.get_wallcons(gwall_i.cons)
                    .map(|cons_i| match cons_i.r_intrinsic(mats).ok() {
                        Some(r_intrinsic_i) => {
                            let R_ground_i = RSI_DESCENDENTE + r_intrinsic_i + RSE;
                            // e = lambda * R // e_tot = 0.30m + lambda * R_mean
                            (w + LAMBDA_GND * R_ground_i + mean * (i - 1) as f32) / i as f32
                        }
                        // BUG: Aquí no sería un valor != 0, calculado con r_intrinsic_i == 0?
                        _ => 0.0,
                    })
                    .unwrap_or(0.0)
            });
        Some(d_t)
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
        let r_intrinsic = model
            .cons
            .get_wallcons(self.cons)?
            .r_intrinsic(&model.mats)
            .ok();
        match self.bounds {
            // Elementos adiabáticos -----------------------------
            // Transmitancia térmica de una composición de cerramiento adiabático, en una posición dada, en W/m2K
            // Notas:
            // - los elementos adiabáticos se reportan con valor 0.0
            BoundaryType::ADIABATIC => {
                debug!("{} (adiabático) U=0.0", self.name);
                Some(0.0)
            }
            // Elementos en contacto con el exterior -------------
            BoundaryType::EXTERIOR => {
                let u = self.u_value_exterior(r_intrinsic);
                debug!(
                    "{} ({}) U={:.2}",
                    self.name,
                    position_to_name(Tilt::from(self)),
                    u.unwrap_or_default()
                );
                u
            }
            // Elementos enterrados ------------------------------
            // Transmitancia térmica de una composición de cerramiento enterrado, en una posición dada, en W/m2K
            // Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
            // Notas:
            // - los elementos mal definidos (muros sin construcción o sin espacio asignado) se reportan con valor 0.0
            // - se usan resistencias superficiales de referencia (DB-HE)
            //
            // d_t: espesor equivalente total de solera (suelo del sótano) (10)
            // psi_gnd_ext: transmitancia térmica lineal como efecto del aislamiento perimetral, psi_gnd_ext
            // U_w: transmitancia del elemento considerado en contacto con el exterior
            // gnd_P: perímetro expuesto del espacio
            BoundaryType::GROUND => {
                let space = model.get_space(self.space)?;
                let d_t = space.slab_on_ground_d_t(&model.walls, &model.cons, &model.mats)?;
                let U_w = self.u_value_exterior(r_intrinsic)?;
                // psi_gnd_ext: transmitancia térmica lineal como efecto del aislamiento perimetral, psi_gnd_ext
                // Calculado el efecto del aislamiento perimetral según UNE-EN ISO 13770:2010 Anexo B (B.4).
                let psi_gnd_ext = {
                    // Espesor aislamiento perimetral d_n = r_n_perim_ins * lambda_ins
                    // d': Espesor equivalente adicional resultante del aislamiento perimetral (d')
                    let d_1 = model.meta.rn_perim_insulation * (LAMBDA_GND - LAMBDA_INS);

                    fround3(
                        -LAMBDA_GND / PI
                            * (f32::ln(1.0 + 2.0 * model.meta.d_perim_insulation / d_t)
                                - f32::ln(1.0 + 2.0 * model.meta.d_perim_insulation / (d_t + d_1))),
                    )
                };
                let gnd_P = space.perimeter_exposed(&model.walls, &model.spaces);

                match Tilt::from(self) {
                    Tilt::TOP => self.u_value_gnd_top(U_w),
                    Tilt::BOTTOM => {
                        self.u_value_gnd_slab(gnd_P, psi_gnd_ext, space.area, space.z, d_t)
                    }
                    Tilt::SIDE => self.u_value_gnd_wall(space, U_w, d_t, &model.walls, &model.cons),
                }
            }
            // Elementos en contacto con otros espacios ---------------------
            BoundaryType::INTERIOR => self.u_value_interior(
                r_intrinsic,
                &model.walls,
                &model.windows,
                &model.spaces,
                &model.cons,
                &model.mats,
                model,
            ),
        }
    }

    /// Transmitancia térmica de una composición de cerramiento exterior, en una posición dada, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - los elementos mal definidos (muros sin construcción o sin espacio asignado) se reportan con valor 0.0
    /// - se usan resistencias superficiales de referencia (DB-HE)
    pub fn u_value_exterior(&self, r_intrinsic: Option<f32>) -> Option<f32> {
        let r = r_intrinsic?;
        let rsi = match Tilt::from(self) {
            Tilt::BOTTOM => RSI_DESCENDENTE,
            Tilt::TOP => RSI_ASCENDENTE,
            Tilt::SIDE => RSI_HORIZONTAL,
        };
        Some(fround2(1.0 / (r + rsi + RSE)))
    }

    /// Transmitancia térmica de una composición de cerramiento enterrado, en una posición dada, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - los elementos mal definidos (muros sin construcción o sin espacio asignado) se reportan con valor 0.0
    /// - se usan resistencias superficiales de referencia (DB-HE)
    pub fn u_value_interior(
        &self,
        r_intrinsic: Option<f32>,
        walls: &[Wall],
        windows: &[Window],
        spaces: &[Space],
        cons: &ConsDb,
        mats: &MatsDb,
        model: &Model,
    ) -> Option<f32> {
        use SpaceType::*;
        use Tilt::*;

        // Dos casos:
        // - Suelos en contacto con sótanos no acondicionados / no habitables en contacto con el terreno - ISO 13370:2010 (9.4)
        // - Elementos en contacto con espacios no acondicionados / no habitables - UNE-EN ISO 6946:2007 (5.4.3)
        let space = spaces.iter().find(|s| s.id == self.space)?;
        let nextto = self.next_to?;
        let nextspace = spaces.iter().find(|s| s.id == nextto)?;

        // Calculamos la resistencial del elemento R_f, e identificamos el espacio no acondicionado
        // Resistencia del elemento teniendo en cuenta el flujo de calor (UNE-EN ISO 13789:2017 Tabla 8)
        let (R_f, uncondspace, are_equally_conditioned) = match (
            space.kind == CONDITIONED,
            Tilt::from(self),
            nextspace.kind == CONDITIONED,
        ) {
            (true, BOTTOM, false) => {
                // Flujo descendente
                // Suelo de espacio acondicionado hacia no acondicionado inferior
                (r_intrinsic? + 2.0 * RSI_DESCENDENTE, nextspace, false)
            }
            (false, TOP, true) => {
                // Flujo descendente
                // Techo de espacio no acondicionado hacia acondicionado superior
                (r_intrinsic? + 2.0 * RSI_DESCENDENTE, space, false)
            }
            (true, TOP, false) => {
                // Flujo ascendente
                // Techo de espacio acondicionado hacia no acondicionado superior
                (r_intrinsic? + 2.0 * RSI_ASCENDENTE, nextspace, false)
            }
            (false, BOTTOM, true) => {
                // Flujo ascendente
                // Suelo de espacio no acondicionado hacia acondicionado inferior
                (r_intrinsic? + 2.0 * RSI_ASCENDENTE, space, false)
            }
            (true, SIDE, false) => {
                // Flujo horizontal
                // Muro entre espacios con distinto nivel de acondicionamiento
                (r_intrinsic? + 2.0 * RSI_HORIZONTAL, nextspace, false)
            }
            (_, _, _) => {
                // Flujo entre espacios igualmente acondicionados
                (r_intrinsic? + 2.0 * RSI_HORIZONTAL, space, true)
            }
        };

        if are_equally_conditioned {
            // 1) Elemento interior que comunica dos espacios igualmente acondicionados
            let U = fround2(1.0 / R_f);
            debug!(
                "{} ({} acond-acond / no acond-no acond) U_int={:.2}",
                self.name,
                position_to_name(Tilt::from(self)),
                U
            );
            Some(U)
        } else {
            // 2) Elemento interior que comunica un espacio acondicionado con otro no acondicionado
            let uncondspace_v = uncondspace.height_net(walls, cons) * uncondspace.area;
            let n_ven = match uncondspace.n_v {
                Some(n_v) => n_v,
                _ => match model.meta.global_ventilation_l_s {
                    Some(global_ventilation) => 3.6 * global_ventilation / model.vol_env_inh_net(),
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
                .walls(walls)
                .filter(|wall| {
                    wall.bounds == BoundaryType::GROUND || wall.bounds == BoundaryType::EXTERIOR
                })
                .filter_map(|wall| {
                    // A·U de muros (y suelos) + A.U de sus huecos
                    let wall_u = self.u_value(model)?;
                    let win_axu = wall
                        .windows(windows)
                        .filter_map(|win| {
                            // Si no está definida la construcción, el hueco no participa de la envolvente
                            win.u_value(cons, mats).map(|u| Some(win.area() * u))?
                        })
                        .sum::<f32>();
                    Some(wall.area_net(windows) * wall_u + win_axu)
                })
                .sum::<f32>();
            // 1/U = 1/U_f + A_i / (sum_k(A_e_k·U_e_k) + 0.33·n·V) (17)
            // En la fórmula anterior, para espacios no acondicionados, se indica que se excluyen suelos, pero no entiendo bien por qué.
            // Esta fórmula, cuando los A_e_k y U_e_k incluyen los muros y suelos con el terreno U_bw y U_bf, con la parte proporcional de
            // muros al exterior, es equivalente a la que indica la 13370
            let A_i = self.area_net(windows);
            let H_ue = UA_e_k + 0.33 * n_ven * uncondspace_v;
            let R_u = A_i / H_ue;
            let U = fround2(1.0 / (R_f + R_u));

            debug!(
                            "{} ({} acond-no acond/sotano) U={:.2} (R_f={:.3}, R_u={:.3}, A_i={:.2}, U_f=1/R_f={:.2}",
                            self.name, position_to_name(Tilt::from(self)), U, R_f, R_u, A_i, 1.0/R_f
                        );
            Some(U)
        }
    }

    /// Transmitancia térmica de una cubierta enterrada, W/m²K
    ///
    /// La composición del muro debe incluir una capa de terreno con lambda = 2 W/K
    fn u_value_gnd_top(&self, U_w: f32) -> Option<f32> {
        debug!("{} (cubierta enterrada) U={:.2}", self.name, U_w);
        Some(U_w)
    }

    /// Transmitancia térmica de solera sobre el terreno (suelo de sótano)
    ///
    /// Cálculo según UNE-EN ISO 13370:2010 Apartado 9.1 y 9.3.2
    /// Hipótesis:
    /// - la cota z del suelo del espacio es relativa a la cota 0 del terreno
    /// Simplificaciones:
    /// - forma cuadrada para calcular el perímetro si no está disponible
    /// - ancho de muros externos w = 0.3m
    /// - lambda de aislamiento = 0,035 W/mK
    ///
    /// HULC parece estar calculando algo más parecido al método de Winkelman o:
    /// let u = 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE + 0.25 / LAMBDA_GND + 0.01 / LAMBDA_INS);
    ///
    /// psi_gnd_ext: transmitancia térmica lineal como efecto del aislamiento perimetral
    fn u_value_gnd_slab(
        &self,
        gnd_P: f32,
        psi_gnd_ext: f32,
        space_area: f32,
        space_z: f32,
        d_t: f32,
    ) -> Option<f32> {
        let gnd_A = space_area;
        // XXX: Estamos suponiendo que la cota z es la del suelo del espacio
        let z = if space_z < 0.0 { -space_z } else { 0.0 };
        // Soleras sin contacto perimetral con el exterior P=0 -> B' -> inf. Cambiamos P a un valor pequeño
        let gnd_P = if gnd_P.abs() < 0.001 {
            warn!(
                "{} (solera con perímetro expuesto nulo o casi nulo {:.2})",
                self.name, gnd_P,
            );
            0.000001
        } else {
            gnd_P
        };
        // B': Dimensión caracterísitica del suelo de sótano. Ver UNE-EN ISO 13370:2010 8.1
        let B_1 = gnd_A / (0.5 * gnd_P);
        // 2 casos:
        // 1. Es un suelo de un espacio acondicionado (9.1 y 9.3.2)
        // 2. Es un suelo de un espacio no acondicionado (9.2 - tomamos U_bg igual a U_g, según 9.2 (8) o a U_bf según E.2)
        // Los dos casos equivalen aproximadamente al U_bf de 9.3.2
        let B_limit = d_t + 0.5 * z;
        let U_bf = if B_limit < B_1 {
            // Soleras sin aislar y moderadamente aisladas (11)
            (2.0 * LAMBDA_GND / (PI * B_1 + B_limit)) * f32::ln(1.0 + PI * B_1 / B_limit)
        } else {
            // Soleras bien aisladas (12)
            LAMBDA_GND / (0.457 * B_1 + B_limit)
        };

        let U = fround2(U_bf + 2.0 * psi_gnd_ext / B_1);
        // H_g sería U * A
        debug!(
            "{} (suelo de sótano) U={:.2} (A={:.2}, P={:.2}, B'={:.2}, z={:.2}, d_t={:.2}, U_bf={:.2}, psi_ge = {:.3})",
            self.name,
            U,
            gnd_A,
            gnd_P,
            B_1,
            z,
            d_t,
            U_bf,
            psi_gnd_ext
        );
        Some(U)
    }

    /// Transmitancia térmica de muro enterrado, en W/m²K
    ///
    /// Cálculo según UNE-EN ISO 13370:2010 9.3.3
    /// Hipótesis:
    /// - la cota z del suelo del espacio es relativa a la cota 0 del terreno
    fn u_value_gnd_wall(
        &self,
        space: &Space,
        U_w: f32,
        d_t: f32,
        walls: &[Wall],
        cons: &ConsDb,
    ) -> Option<f32> {
        let z = if space.z < 0.0 { -space.z } else { 0.0 };
        // Muros que realmente no son enterrados
        if z.abs() < 0.01 {
            warn!(
                "{} (muro de sótano no enterrado z=0) U_w={:.2} (z={:.2})",
                self.name, U_w, z,
            );
            return Some(U_w);
        };
        // d_w: Espesor equivalente de los muros de sótano (13)
        let d_w = LAMBDA_GND / U_w;
        // U del muro completamente enterrado a profundidad z (14)
        let U_bw = if z.abs() < f32::EPSILON {
            // Muro no enterrado
            // Si z = 0 -> U = U_w
            U_w
        } else {
            // Muro parcialmente enterrado
            // Si d_w < d_t -> d_t = d_w
            let d_t = d_w.min(d_t);
            fround2(
                (2.0 * LAMBDA_GND / (PI * z))
                    * (1.0 + 0.5 * d_t / (d_t + z))
                    * f32::ln(z / d_w + 1.0),
            )
        };
        // Altura neta
        let height_net = space.height_net(walls, cons);
        // Altura sobre el terreno (muro no enterrado)
        let h = if height_net > z { height_net - z } else { 0.0 };
        // Si el muro no es enterrado en toda su altura ponderamos U por altura
        let U = if h.abs() < f32::EPSILON {
            // Muro completamente enterrado
            U_bw
        } else {
            // Muro con z parcialmente enterrado
            fround2((z * U_bw + h * U_w) / height_net)
        };
        debug!(
            "{} (muro enterrado) U={:.2} (z={:.2}, h={:.2}, U_w={:.2}, U_bw={:.2}, d_t={:.2}, d_w={:.2})",
            self.name, U, z, h, U_w, U_bw, d_t, d_w,
        );
        Some(U)
    }
}

/// Convierte inclinación a nombre de elemento (suelo, techo, muro)
fn position_to_name<'a>(position: Tilt) -> &'a str {
    match position {
        Tilt::BOTTOM => "suelo",
        Tilt::TOP => "techo",
        Tilt::SIDE => "muro",
    }
}
