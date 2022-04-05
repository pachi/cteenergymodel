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
use log::{debug, info, warn};

use super::EnergyIndicators;
use crate::types::HasSurface;
use crate::{
    utils::{fround2, fround3},
    BoundaryType, ConsDb, Layer, MatProps, MatsDb, Model, Space, SpaceType, Tilt, Wall, WallCons,
    WinCons,
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
    /// Dimensión característica de un suelo de sótano (B') (en contacto con el terreno), m
    ///
    /// Cálculo según UNE-EN ISO 13370:2010 8.1 - B_1 = gnd_A / (0.5 * gnd_P)
    ///
    /// gnd_A: superficie de la solera, m²
    /// gnd_P: perímetro expuesto del espacio, m
    ///
    /// El perímetro expuesto, P, de un espacio es el que separa el espacio del exterior o de un espacio no calefactado fuera de la estructura aislada
    /// Excluye las parte que separan un espacio calefactado con otros espacios calefactados
    ///
    /// Si no es un espacio con solera o su superficie es nula devuelve None
    pub fn slab_char_dim(&self, walls: &[Wall], spaces: &[Space]) -> Option<f32> {
        use crate::BoundaryType::{ADIABATIC, EXTERIOR, GROUND, INTERIOR};

        let spc_walls: Vec<_> = self.walls(walls).collect();

        // Walls that belong to the space and are floors (floors from .next_to are ceilings of this space)
        let spc_gnd_floors: Vec<_> = spc_walls
            .iter()
            .filter(|w| {
                w.space == self.id
                    && Tilt::from(**w) == Tilt::BOTTOM
                    && w.bounds == BoundaryType::GROUND
            })
            .collect();
        if spc_gnd_floors.len() > 1 {
            warn!(
                "Calculando perímetro expuesto para espacio con más de un suelo: {} ({}), {:#?}",
                self.name, self.id, spc_gnd_floors
            );
        };

        let gnd_floor = match spc_gnd_floors.get(0) {
            Some(space) => space,
            _ => {
                warn!(
                    "Petición de cálculo de dimensión característica de solera de espacio sin solera (espacio {}, {})",
                    self.name, self.id
                );
                return None;
            }
        };

        let gnd_A = gnd_floor.area();

        if gnd_A < 0.001 {
            warn!(
                "Petición de cálculo de dimensión característica de solera con superficie nula (espacio {}, {})",
                self.name, self.id
            );
            return Some(0.0);
        };

        // Area bruta total de muros y área bruta de muros exteriores
        let (total_area, exterior_area) = spc_walls
            .iter()
            .filter(|w| Tilt::from(**w) == Tilt::SIDE)
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

        // Suponemos siempre un valor pequeño pero distinto de cero
        let gnd_P = {
            let p = if total_area < 0.001 {
                0.0
            } else {
                let perimeter =
                    fround2(gnd_floor.geometry.perimeter() * exterior_area / total_area);
                debug!(
                    "Perímetro expuesto del espacio {} ({}): {}",
                    self.name, self.id, perimeter
                );
                perimeter
            };
            if p < 0.01 {
                info!(
                    "Solera {} ({}) con perímetro expuesto nulo o casi nulo) en espacio {}, {}",
                    gnd_floor.name, gnd_floor.id, self.name, self.id
                )
            };
            p.max(0.01)
        };

        Some(fround2(gnd_A / (0.5 * gnd_P)))
    }

    /// Espesor total equivalente de solera (suelo de sótano), d_t, m
    /// Según UNE-EN ISO 13370:2010 9.3.2 (10)
    /// Ponderamos según superficie de suelos en contacto con el terreno
    fn slab_d_t(&self, walls: &[Wall], cons: &ConsDb, mats: &MatsDb) -> Option<f32> {
        let ground_slabs: Vec<_> = self
            .walls(walls)
            .filter(|wall| Tilt::from(*wall) == Tilt::BOTTOM && wall.bounds == BoundaryType::GROUND)
            .collect();

        if ground_slabs.is_empty() {
            return None;
        };

        // Suponemos espesor de muros de sótano = 0.30m para cálculo de soleras
        // NOTA: Podríamos calcular el espesor medio de muros perimetrales bien, pero probablemente no merece la pena
        // NOTA: Ponderamos por superficie las d_t de cada solera para obtener la d_t media
        const W: f32 = 0.3;
        let mut e_tot = 0.0;
        let mut a_total = 0.0;
        for slab in &ground_slabs {
            let a = slab.area();
            a_total += a;
            // NOTA: Cuando el modelo no está completamente definido usamos solo las resistencias superficiales
            let r_intrinsic = cons
                .get_wallcons(slab.cons)
                .and_then(|c| c.r_intrinsic(mats).ok())
                .unwrap_or_default();
            e_tot += a * (W + LAMBDA_GND * (RSI_DESCENDENTE + r_intrinsic + RSE));
        }
        let d_t = e_tot / a_total / ground_slabs.len() as f32;
        Some(d_t)
    }

    /// Transmitancia térmica lineal como efecto del aislamiento perimetral, psi_gnd_ext, W/m
    /// Calculado el efecto del aislamiento perimetral según UNE-EN ISO 13770:2010 Anexo B (B.4).
    ///
    /// # Argumentos
    ///
    /// * `d_t` - Espesor total equivalente de solera (suelo de sótano), m
    /// * `model` - modelo del edificio
    /// TODO: ¿debería usar datos de aislamiento por solera en lugar de global del modelo?
    fn slab_psi_gnd_ext(&self, d_t: f32, model: &Model) -> f32 {
        // d': Espesor equivalente adicional resultante del aislamiento perimetral
        // - R_n: Resistencia del aislamiento perimetral, m²K/W
        // - λ: coeficiente de transmisión del terreno, W/mK
        // - λ_ins: coeficiente de transmisión del aislamiento perimetral, W/mK
        // - d_n: espesor aislamiento perimetral, m = R_n * λ_ins
        // - d' = R'.λ = (R_n - d_n / λ)·λ = R_n·λ - d_n = R_n·λ - R_n·λ_ins = R_n·(λ - λ_ins)
        let d_1 = model.meta.rn_perim_insulation * (LAMBDA_GND - LAMBDA_INS);
        // Suponemos aislamiento perimetral horizontal (B.5)
        // D: profundidad / ancho de la banda de aislamiento perimetral.
        // Para aislamiento horizontal, D es el ancho de la banda perimetral de aislameinto (B.5)
        // Para aislamiento vertical, D es el doble de la profundidad de la banda de aislamiento (B.6)
        let D = model.meta.d_perim_insulation;
        debug!(
            "Aislamiento perimetral en espacio {} ({}): d_1 = {}, D = {}",
            self.name, self.id, d_1, D
        );
        fround3(-LAMBDA_GND / PI * (f32::ln(1.0 + D / d_t) - f32::ln(1.0 + D / (d_t + d_1))))
    }

    /// A.U de los elementos del espacio que dan al exterior o al terreno (excluye interiores)
    /// Como hemos asignado U_bw y U_bf a los muros y suelos en contacto con el terreno, ya se tiene en cuenta
    /// la parte enterrada correctamente (fracción enterrada y superficie expuesta, ya que no se consideran los que dan a interiores)
    fn ua_of_external_and_ground_surfaces(&self, model: &Model) -> f32 {
        let UA_e_k = self
            .walls(&model.walls)
            .filter(|wall| {
                wall.bounds == BoundaryType::GROUND || wall.bounds == BoundaryType::EXTERIOR
            })
            .filter_map(|wall| {
                // A·U de muros (y suelos) + A.U de sus huecos
                let wall_u = wall.u_value(model)?;
                let win_axu = wall
                    .windows(&model.windows)
                    .filter_map(|win| {
                        // Si no está definida la construcción, el hueco no participa de la envolvente
                        let u = &model.cons.get_wincons(win.cons)?.u_value(&model.mats)?;
                        Some(win.area() * u)
                    })
                    .sum::<f32>();
                Some(wall.area_net(&model.windows) * wall_u + win_axu)
            })
            .sum::<f32>();
        UA_e_k
    }
}

impl WallCons {
    /// Resistencia térmica intrínseca (sin resistencias superficiales) de una composición de capas [W/m²K]
    /// TODO: convertir errores a logging y devolver Option<f32>
    pub fn r_intrinsic(&self, mats: &MatsDb) -> Result<f32, Error> {
        let mut total_resistance = 0.0;
        for Layer { id, e } in &self.layers {
            match mats.get_material(*id) {
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

impl WinCons {
    /// Transmitancia térmica total de la construcción de hueco, U_W, en una posición dada, en W/m2K
    ///
    /// Incluye el efecto del marco, vidrio y efecto de intercalarios y/o cajones de persiana
    ///
    /// Notas:
    /// - los valores de U de acristalamiento y marco son para su posición final
    /// - los valores de acristalamiento y marco ya deben incluir las resistencias superficiales
    ///   (U_g se calcula con resistencias superficiales y U_w es una ponderación)
    pub fn u_value(&self, mats: &MatsDb) -> Option<f32> {
        let glass = mats.get_glass(self.glass)?;
        let frame = mats.get_frame(self.frame)?;
        Some(fround2(
            (1.0 + self.delta_u / 100.0)
                * (frame.u_value * self.f_f + glass.u_value * (1.0 - self.f_f)),
        ))
    }

    /// Transmitancia térmica total del acristalmiento (g_glwi = g_gln * 0.90) [-]
    /// Corresponde al factor solar sin protección solar activada
    pub fn g_glwi(&self, mats: &MatsDb) -> Option<f32> {
        let glass = mats.get_glass(self.glass)?;
        Some(fround2(glass.g_gln * 0.90))
    }

    /// Transmitancia térmica del acristalamiento con protecciones solares activadas, g_glshwi [-]
    /// Corresponde al factor solar con protección solar activada
    pub fn g_glshwi(&self, mats: &MatsDb) -> Option<f32> {
        self.g_glshwi.map(fround2).or_else(|| self.g_glwi(mats))
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
        use BoundaryType::*;
        use SpaceType::*;
        use Tilt::*;

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
            ADIABATIC => {
                debug!("{} (adiabático) U=0.0", self.name);
                Some(0.0)
            }
            // Elementos en contacto con el exterior -------------
            EXTERIOR => {
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
            GROUND => {
                // U_w: transmitancia del elemento considerado en contacto con el exterior
                let U_w = self.u_value_exterior(r_intrinsic)?;

                // TODO: Parámetros ligados al espacio: d_t, psi_gnd_ext, char_dim, z, space_height_net
                let space = model.get_space(self.space)?;
                // d_t: espesor equivalente total de solera (suelo del sótano) (10)
                let d_t = space.slab_d_t(&model.walls, &model.cons, &model.mats)?;
                // transmitancia térmica lineal como efecto del aislamiento perimetral, psi_gnd_ext
                let psi_gnd_ext = space.slab_psi_gnd_ext(d_t, model);
                // Suponemos valor cuando se calcule en espacios sin solera (no podría pasar)
                let char_dim = space
                    .slab_char_dim(&model.walls, &model.spaces)
                    .unwrap_or_default();
                // TODO: calcular altura neta del muro y no la del espacio a partir de sus datos geométricos
                // Altura neta
                let space_height_net = space.height_net(&model.walls, &model.cons);
                // Profundidad enterrada
                let z = (-space.z).max(0.0); // if z < 0.0 { -z } else { 0.0 }

                match Tilt::from(self) {
                    TOP => Some(self.u_value_gnd_top(U_w)),
                    BOTTOM => Some(self.u_value_gnd_slab(z, d_t, char_dim, psi_gnd_ext)),
                    SIDE => self.u_value_gnd_wall(z, U_w, d_t, space_height_net),
                }
            }
            // Elementos en contacto con otros espacios ---------------------
            INTERIOR => {
                // Dos casos:
                // - Suelos en contacto con sótanos no acondicionados / no habitables en contacto con el terreno - ISO 13370:2010 (9.4)
                // - Elementos en contacto con espacios no acondicionados / no habitables - UNE-EN ISO 6946:2007 (5.4.3)
                let space = model.get_space(self.space)?;
                let nextto = self.next_to?;
                let nextspace = model.get_space(nextto)?;

                let this_cond = space.kind == CONDITIONED;
                let next_cond = nextspace.kind == CONDITIONED;

                let uncondspace_by_condspace = match (this_cond, next_cond) {
                    (true, false) => Some(nextspace),
                    (false, true) => Some(space),
                    _ => None,
                };

                // Calculamos la resistencial del elemento R_f, e identificamos el espacio no acondicionado
                // Resistencia del elemento teniendo en cuenta el flujo de calor (UNE-EN ISO 13789:2017 Tabla 8)
                let R_f = match (this_cond, next_cond, Tilt::from(self)) {
                    // Suelo de espacio acondicionado hacia no acondicionado inferior
                    // Techo de espacio no acondicionado hacia acondicionado superior
                    (true, false, BOTTOM) | (false, true, TOP) => {
                        // Flujo descendente
                        r_intrinsic? + 2.0 * RSI_DESCENDENTE
                    }
                    // Techo de espacio acondicionado hacia no acondicionado superior
                    // Suelo de espacio no acondicionado hacia acondicionado inferior
                    (true, false, TOP) | (false, true, BOTTOM) => {
                        // Flujo ascendente
                        r_intrinsic? + 2.0 * RSI_ASCENDENTE
                    }
                    // Muro entre espacios con distinto nivel de acondicionamiento
                    // Flujo entre espacios acondicionados
                    _ => {
                        // Flujo horizontal
                        r_intrinsic? + 2.0 * RSI_HORIZONTAL
                    }
                };

                match uncondspace_by_condspace {
                    None => {
                        // 1) Elemento interior que comunica dos espacios igualmente acondicionados
                        let U = fround2(1.0 / R_f);
                        debug!(
                            "{} ({} acond-acond / no acond-no acond) U_int={:.2}",
                            self.name,
                            position_to_name(Tilt::from(self)),
                            U
                        );
                        Some(U)
                    }
                    Some(uncondspace) => {
                        // 1) Elemento interior que comunica un espacio acondicionado con otro no acondicionado

                        let A_i = self.area();

                        // TODO: UA_e_k + q_ue es la transmisión al exterior y es propia de cada espacio.
                        // TODO: Se usa en cálculos de elementos interiores en contacto con los espacios no acondicionados
                        // TODO: y podríamos calcularlo en espacios solo para los espacios no acondicionados.

                        // Calculamos el A.U de los elementos del espacio que dan al exterior o al terreno (excluye interiores))
                        let UA_e_k = uncondspace.ua_of_external_and_ground_surfaces(model);
                        // Flow rate between the unheated space and the external environment 13789, (12), m³/h
                        // En los no habitables debe estar definido n_v pero en los no acondicionados no
                        // Se puede obtener n_v a partir de la Tabla 6 de la UNE-EN ISO 13789:2017 y n_50/20.
                        // Para sótanos no calefactados la 13370:2007 (9.4) dice que se podría usar n_v = 0.30
                        let q_ue = {
                            let volume = uncondspace.volume_net(&model.walls, &model.cons);
                            let n_v = uncondspace
                                .n_v
                                .unwrap_or_else(|| model.global_ventilation_rate());
                            if n_v.abs() < f32::EPSILON {
                                // Espacio mal definido (ni tiene n_v ni hay definición global de ventilación)
                                warn!("Nivel de ventilación (1/h) nulo o casi nulo del espacio no acondicionado {} ({})", uncondspace.id, uncondspace.name);
                            }
                            // m^3 * 1/h
                            volume * n_v
                        };
                        self.u_value_interior_cond_uncond(A_i, R_f, UA_e_k, q_ue)
                    }
                }
            }
        }
    }

    /// Transmitancia térmica de una composición de cerramiento exterior, en una posición dada, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - los elementos mal definidos (muros sin construcción o sin espacio asignado) se reportan con valor 0.0
    /// - se usan resistencias superficiales de referencia (DB-HE)
    ///
    /// # Argumentos
    ///
    /// * `r_intrinsic`: Resistencia intrínseca del elemento, en W/m²K
    pub fn u_value_exterior(&self, r_intrinsic: Option<f32>) -> Option<f32> {
        let r = r_intrinsic?;
        let rsi = match Tilt::from(self) {
            Tilt::BOTTOM => RSI_DESCENDENTE,
            Tilt::TOP => RSI_ASCENDENTE,
            Tilt::SIDE => RSI_HORIZONTAL,
        };
        Some(fround2(1.0 / (r + rsi + RSE)))
    }

    /// Transmitancia térmica de cerramiento interior entre espacio acondicionado y no acondicionado, en W/m2K
    ///
    /// Cálculo de elemento interior en contacto con sótano no calefactado según ISO 13370:2010 (9.4)
    /// y de elemento interior en contacto con otro espacio no habitable / no acondicionado según UNE-EN ISO 6946:2007 (5.4.3)
    ///
    /// Notas:
    /// -  Los huecos de las particiones interiores se ignoran para el cálculo de A_i;
    ///
    /// # Argumentos
    ///
    /// * `A_i` - Area de la partición interior, en m²
    /// * `R_f` - Resistencia térmica del elemento, sin considerar corrección por diferencia de acondicionamiento
    /// * `UA_e_k` - U.A de los elementos al exterior o con el terreno del espacio no acondicionado
    /// * `q_ue` - tasa de ventilación entre el espacio no acondicionado y el exterior, m³/h
    pub fn u_value_interior_cond_uncond(
        &self,
        A_i: f32,
        R_f: f32,
        UA_e_k: f32,
        q_ue: f32,
    ) -> Option<f32> {
        // CASO: interior en contacto con sótano no calefactado - ISO 13370:2010 (9.4)
        // CASO: interior en contacto con otro espacio no habitable / no acondicionado - UNE-EN ISO 6946:2007 (5.4.3)
        // 1/U = 1/U_f + A_i / (sum_k(A_e_k·U_e_k) + 0.33·n·V) (17)
        // En la fórmula anterior, para espacios no acondicionados, se indica que se excluyen suelos, pero no entiendo bien por qué.
        // Esta fórmula, cuando los A_e_k y U_e_k incluyen los muros y suelos con el terreno U_bw y U_bf, con la parte proporcional de
        // muros al exterior, es equivalente a la que indica la 13370
        let H_ue = UA_e_k + 0.33 * q_ue;
        let R_u = A_i / H_ue;
        let U = fround2(1.0 / (R_f + R_u));

        debug!(
            "{} ({} acond-no acond/sotano) U={:.2} (R_f={:.3}, R_u={:.3}, A_i={:.2}, U_f=1/R_f={:.2}",
            self.name, position_to_name(Tilt::from(self)), U, R_f, R_u, A_i, 1.0/R_f
        );
        Some(U)
    }

    /// Transmitancia térmica de una cubierta enterrada, W/m²K
    ///
    /// La composición del muro debe incluir una capa de terreno con lambda = 2 W/K
    ///
    /// # Argumentos
    ///
    /// * `U_w`- transmitancia del elemento considerado en contacto con el exterior
    fn u_value_gnd_top(&self, U_w: f32) -> f32 {
        debug!("{} (cubierta enterrada) U={:.2}", self.name, U_w);
        U_w
    }

    /// Transmitancia térmica de solera sobre el terreno (suelo de sótano)
    ///
    /// Cálculo según UNE-EN ISO 13370:2010 Apartado 9.1 y 9.3.2
    /// 2 casos:
    /// 1. Es un suelo de un espacio acondicionado (9.1 y 9.3.2)
    /// 2. Es un suelo de un espacio no acondicionado (9.2 - tomamos U_bg igual a U_g, según 9.2 (8) o a U_bf según E.2)
    /// Los dos casos equivalen aproximadamente al U_bf de 9.3.2
    ///
    /// Hipótesis:
    /// - la cota z=0 del suelo del espacio coincide con la cota 0 del terreno
    /// Simplificaciones:
    /// - forma cuadrada para calcular el perímetro si no está disponible
    /// - ancho de muros externos w = 0.3m
    /// - lambda de aislamiento = 0,035 W/mK
    ///
    /// HULC parece estar calculando algo más parecido al método de Winkelman o:
    /// let u = 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE + 0.25 / LAMBDA_GND + 0.01 / LAMBDA_INS);
    ///
    /// # Argumentos
    ///
    /// * `z`- profundidad (> 0) de la solera respecto a la cota 0 del terreno, m
    /// * `d_t`- espesor equivalente total de solera (suelo del sótano) (10)
    /// * `char_dim` - dimensión característica de la solera (B', según UNE-EN ISO 13370:2010 8.1), m
    /// * `psi_gnd_ext` - transmitancia térmica lineal de la solera considerando aislamiento perimetral, W/mK
    fn u_value_gnd_slab(&self, z: f32, d_t: f32, char_dim: f32, psi_gnd_ext: f32) -> f32 {
        let B_limit = d_t + 0.5 * z;
        let U_bf = if B_limit < char_dim {
            // Soleras sin aislar y moderadamente aisladas (11)
            (2.0 * LAMBDA_GND / (PI * char_dim + B_limit)) * f32::ln(1.0 + PI * char_dim / B_limit)
        } else {
            // Soleras bien aisladas (12)
            LAMBDA_GND / (0.457 * char_dim + B_limit)
        };

        // Valor teniendo en cuenta el efecto del aislamiento perimetral en régimen estacionario B.4
        let U = fround2(U_bf + 2.0 * psi_gnd_ext / char_dim);
        debug!(
            "{} (suelo de sótano) U={:.2} (z={:.2}, d_t={:.2}, B'={:.2}, U_bf={:.2}, psi_ge = {:.3})",
            self.name,
            U,
            z,
            d_t,
            char_dim,
            U_bf,
            psi_gnd_ext
        );
        U
    }

    /// Transmitancia térmica de muro enterrado, en W/m²K
    ///
    /// Cálculo según UNE-EN ISO 13370:2010 9.3.3
    /// Hipótesis:
    /// - la cota z del suelo del espacio es relativa a la cota 0 del terreno
    ///
    /// # Argumentos
    ///
    /// `z` - profundidad (> 0) de la base respecto a la cota 0 del terreno, m
    /// `U_w` - transmitancia del elemento considerado en contacto con el exterior
    /// `d_t` - espesor equivalente total de solera (suelo del sótano), m (10)
    /// `space_height_net` - altura neta del espacio, m
    fn u_value_gnd_wall(&self, z: f32, U_w: f32, d_t: f32, space_height_net: f32) -> Option<f32> {
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
        // Altura sobre el terreno (muro no enterrado)
        let h = if space_height_net > z {
            space_height_net - z
        } else {
            0.0
        };
        // Si el muro no es enterrado en toda su altura ponderamos U por altura
        let U = if h.abs() < f32::EPSILON {
            // Muro completamente enterrado
            U_bw
        } else {
            // Muro con z parcialmente enterrado
            fround2((z * U_bw + h * U_w) / space_height_net)
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
