// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Propiedades energéticas del modelo
//!
//! Permiten el cálculo de indicadores y la descripción de los elementos del modelo

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    utils::fround2, BoundaryType, Model, Orientation, SpaceType, ThermalBridgeKind, Tilt, Uuid,
};

/// Reporte de cálculo de propiedades térmicas y geométricas del modelo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProps {
    /// Propiedades globales del modelo
    pub global: GlobalProps,
    /// Propiedades de espacios
    pub spaces: BTreeMap<Uuid, SpaceProps>,
    /// Propiedades de muros
    pub walls: BTreeMap<Uuid, WallProps>,
    /// Propiedades de huecos
    pub windows: BTreeMap<Uuid, WinProps>,
    /// Propiedades de puentes térmicos
    pub thermal_bridges: BTreeMap<Uuid, TbProps>,
    /// Propiedades de construcciones de muros
    pub wallcons: BTreeMap<Uuid, WallConsProps>,
    /// Propiedades de huecos
    pub wincons: BTreeMap<Uuid, WinConsProps>,
}

impl From<&Model> for EnergyProps {
    /// Completa datos de los elementos (espacios, opacos, huecos,...) por id
    fn from(model: &Model) -> Self {
        // Propiedades de construcciones
        let mut wallcons: BTreeMap<Uuid, WallConsProps> = BTreeMap::new();
        for wc in &model.cons.wallcons {
            let wcp = WallConsProps {
                r_intrinsic: wc.r_intrinsic(&model.mats).ok(),
            };
            wallcons.insert(wc.id, wcp);
        }

        let mut wincons: BTreeMap<Uuid, WinConsProps> = BTreeMap::new();
        for wc in &model.cons.wincons {
            let wcp = WinConsProps {
                c_100: wc.c_100,
                u_value: wc.u_value(&model.mats),
                g_glwi: wc.g_glwi(&model.mats),
                g_glshwi: wc.g_glshwi(&model.mats),
            };
            wincons.insert(wc.id, wcp);
        }

        // Propiedades de espacios
        let mut spaces: BTreeMap<Uuid, SpaceProps> = BTreeMap::new();
        for s in &model.spaces {
            let sp = SpaceProps {
                kind: s.kind,
                inside_tenv: s.inside_tenv,
                area: s.area,
                multiplier: s.multiplier,
                height: s.height,
                height_net: s.height_net(&model.walls, &model.cons),
                volume_net: s.volume_net(&model.walls, &model.cons),
            };
            spaces.insert(s.id, sp);
        }

        // Propiedades de opacos
        let ext_and_gnd_walls_tenv: Vec<_> = model
            .exterior_and_ground_walls_of_envelope_iter()
            .map(|w| w.id)
            .collect();

        let mut walls: BTreeMap<Uuid, WallProps> = BTreeMap::new();
        for w in &model.walls {
            let wp = WallProps {
                space: w.space,
                space_next: w.next_to,
                bounds: w.bounds,
                cons: w.cons,
                orientation: Orientation::from(w),
                tilt: Tilt::from(w),
                area_gross: w.area(),
                area_net: w.area_net(&model.windows),
                multiplier: spaces.get(&w.space).map(|sp| sp.multiplier).unwrap_or(1.0),
                is_ext_or_gnd_tenv: ext_and_gnd_walls_tenv.contains(&w.id),
                u_value: w.u_value(model),
            };
            walls.insert(w.id, wp);
        }

        // Propiedades de huecos
        let mut windows: BTreeMap<Uuid, WinProps> = BTreeMap::new();
        for w in &model.windows {
            let wall = walls.get(&w.wall);
            let wp = WinProps {
                wall: w.wall,
                cons: w.cons,
                orientation: wall.map(|w| w.orientation).unwrap_or_default(),
                tilt: wall.map(|w| w.tilt).unwrap_or_default(),
                area: w.area(),
                multiplier: wall.map(|wp| wp.multiplier).unwrap_or(1.0),
                is_ext_or_gnd_tenv: ext_and_gnd_walls_tenv.contains(&w.wall),
                u_value: wincons.get(&w.cons).and_then(|c| c.u_value),
            };
            windows.insert(w.id, wp);
        }

        // Propiedades de puentes térmicos
        let mut thermal_bridges: BTreeMap<Uuid, TbProps> = BTreeMap::new();
        for tb in &model.thermal_bridges {
            let tbp = TbProps {
                kind: tb.kind,
                l: tb.l,
                psi: tb.psi,
            };
            thermal_bridges.insert(tb.id, tbp);
        }

        // Propiedades globales
        let a_ref: f32 = fround2(
            spaces
                .values()
                .map(|s| {
                    if s.inside_tenv && s.kind != SpaceType::UNINHABITED {
                        s.area * s.multiplier
                    } else {
                        0.0
                    }
                })
                .sum(),
        );
        let vol_env_gross = fround2(
            spaces
                .values()
                .map(|s| {
                    if s.inside_tenv {
                        s.area * s.height * s.multiplier
                    } else {
                        0.0
                    }
                })
                .sum(),
        );
        let vol_env_net = fround2(
            spaces
                .values()
                .map(|s| {
                    if s.inside_tenv {
                        s.area * s.height_net * s.multiplier
                    } else {
                        0.0
                    }
                })
                .sum(),
        );
        let vol_env_inh_net = fround2(
            spaces
                .values()
                .map(|s| {
                    if !s.inside_tenv && s.kind != SpaceType::UNINHABITED {
                        s.area * s.height_net * s.multiplier
                    } else {
                        0.0
                    }
                })
                .sum(),
        );
        let compacity = {
            let exposed_area: f32 = walls
                .values()
                .filter(|w| w.is_ext_or_gnd_tenv)
                .map(|w| w.area_gross * w.multiplier)
                .sum();
            if exposed_area == 0.0 {
                0.0
            } else {
                vol_env_gross / exposed_area
            }
        };
        let global_ventilation_rate = model
            .meta
            .global_ventilation_l_s
            .map(|n_v_g| 3.6 * n_v_g / vol_env_inh_net)
            .unwrap_or_default();

        let global = GlobalProps {
            a_ref,
            vol_env_gross,
            vol_env_net,
            vol_env_inh_net,
            compacity,
            global_ventilation_rate,
        };

        Self {
            global,
            spaces,
            walls,
            windows,
            thermal_bridges,
            wallcons,
            wincons,
        }
    }
}

/// Propiedades generales del modelo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalProps {
    /// Superficie útil de los espacios habitables de la envolvente térmica [m²]
    pub a_ref: f32,
    /// Volumen bruto de los espacios de la envolvente [m³]
    pub vol_env_gross: f32,
    /// Volumen neto de los espacios de la envolvente [m³]
    pub vol_env_net: f32,
    /// Volumen neto de los espacios habitables de la envolvente [m³]
    /// Descuenta los volúmenes de forjados y cubiertas del volumen bruto
    pub vol_env_inh_net: f32,
    /// Compacidad de la envolvente térmica del edificio V/A (m³/m²)
    /// De acuerdo con la definición del DB-HE comprende el volumen interior de la envolvente térmica (V)
    /// y la superficie de muros y huecos con intercambio térmico con el aire exterior o el terreno (A)
    /// Tiene en cuenta los multiplicadores de espacios (en superficie y volumen)
    /// Se excluyen los huecos sin muro definido y los muros sin espacio definido
    /// Para area expuesta => compacidad = 0.0
    pub compacity: f32,
    /// Tasa de ventilación global del edificio (1/h)
    pub global_ventilation_rate: f32,
}

/// Propiedades de espacios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceProps {
    /// Nivel de acondicionamiento del espacio, [-]
    pub kind: SpaceType,
    /// ¿Pertenece al interior de la envolvente térmica?
    pub inside_tenv: bool,
    /// Superficie del espacio, [m²]
    pub area: f32,
    /// Multiplicador del espacio, [-]
    pub multiplier: f32,
    /// Altura bruta (suelo a suelo) del espacio, [m]
    pub height: f32,
    /// Altura neta (suelo a techo) del espacio, [m]
    pub height_net: f32,
    /// Volumen neto del espacio, [m³]
    pub volume_net: f32,
}

/// Propiedades de muros
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallProps {
    /// Espacio al que pertenece el muro
    pub space: Uuid,
    /// Espacio adyacente
    pub space_next: Option<Uuid>,
    /// Condición de contorno
    pub bounds: BoundaryType,
    /// Construcción de muro
    pub cons: Uuid,
    /// Orientación del opaco
    pub orientation: Orientation,
    /// Inclinación del opaco
    pub tilt: Tilt,
    /// Superficie bruta del muro, [m²]
    pub area_gross: f32,
    /// Superficie neta del muro, [m²]
    pub area_net: f32,
    /// Multiplicador del espacio, [-]
    pub multiplier: f32,
    /// ¿Pertenece este muro a la envolvente térmica?
    pub is_ext_or_gnd_tenv: bool,
    /// U de muro, [W/m²K]
    pub u_value: Option<f32>,
}

/// Propiedades de huecos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinProps {
    /// Construcción de hueco
    pub cons: Uuid,
    /// Opaco al que está asociado
    pub wall: Uuid,
    /// Orientación del hueco (heredada del muro)
    pub orientation: Orientation,
    /// Inclinación del hueco (heredada del muro)
    pub tilt: Tilt,
    /// Superficie del hueco, [m²]
    pub area: f32,
    /// Multiplicador del espacio, [-]
    pub multiplier: f32,
    /// ¿Pertenece este muro a la envolvente térmica?
    pub is_ext_or_gnd_tenv: bool,
    /// U de huecos, [W/m²K]
    pub u_value: Option<f32>,
    // TODO: completar propiedades
    // /// Factor de obstrucción de obstáculos remotos (usuario), [-]
    // pub f_shobst_user: Option<f32>,
    // /// Factor de obstrucción de obstáculos remotos (calculado), [-]
    // pub f_shobst: f32,
}

/// Propiedades de puentes térmicos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TbProps {
    /// Tipo de puente térmico
    /// Roof|Balcony|Corner|IntermediateFloor|InternalWall|GroundFloor|Pillar|Window|Generic
    pub kind: ThermalBridgeKind,
    /// Longitud del puente térmico (m)
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
    pub psi: f32,
}

/// Propiedades de construcciones de opacos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallConsProps {
    // R intrínseca de construcción, [m²K/W]
    pub r_intrinsic: Option<f32>,
}

/// Propiedades de construcciones de opacos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinConsProps {
    /// Transmitancia térmica total del acristalamiento, sin protecciones solares, [-]
    pub g_glwi: Option<f32>,
    /// Transmitancia térmica total del acristalamiento, con protecciones solares, [-]
    pub g_glshwi: Option<f32>,
    /// U de construcción de hueco, [W/m²K]
    pub u_value: Option<f32>,
    /// Permeabilidad al aire del hueco a 100 Pa, [m³/h·m²]
    pub c_100: f32,
}
