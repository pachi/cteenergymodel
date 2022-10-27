// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Propiedades geométricas y energéticas del modelo
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
    /// Propiedades de opacos
    pub walls: BTreeMap<Uuid, WallProps>,
    /// Propiedades de huecos
    pub windows: BTreeMap<Uuid, WinProps>,
    /// Propiedades de puentes térmicos
    pub thermal_bridges: BTreeMap<Uuid, TbProps>,
    /// Propiedades de sombras
    pub shades: BTreeMap<Uuid, ShadeProps>,
    /// Propiedades de construcciones de opacos
    pub wallcons: BTreeMap<Uuid, WallConsProps>,
    /// Propiedades de huecos
    pub wincons: BTreeMap<Uuid, WinConsProps>,
    // Propiedades de horarios
    // TODO:
    // Propiedades de cargas de espacios
    // TODO:
    // Propiedades de consignas
    // TODO:
}

impl From<&Model> for EnergyProps {
    /// Completa datos de los elementos (espacios, opacos, huecos,...) por id
    fn from(model: &Model) -> Self {
        // Propiedades de construcciones
        let mut wallcons: BTreeMap<Uuid, WallConsProps> = BTreeMap::new();
        for wc in &model.cons.wallcons {
            let wcp = WallConsProps {
                thickness: wc.thickness(),
                resistance: wc.resistance(&model.cons).ok(),
            };
            wallcons.insert(wc.id, wcp);
        }

        let mut wincons: BTreeMap<Uuid, WinConsProps> = BTreeMap::new();
        for wc in &model.cons.wincons {
            // Valores por defecto para elementos sin vidrio definido
            // corresponde a vidrio sencillo: g_gl;n = 0.85; g_gl;wi = g_gl;n * 0.9 = 0.77
            let g_glwi = wc.g_glwi(&model.cons).unwrap_or(0.77);
            let g_glshwi = wc.g_glshwi(&model.cons).unwrap_or(g_glwi);
            let wcp = WinConsProps {
                c_100: wc.c_100,
                u_value: wc.u_value(&model.cons),
                g_glwi,
                g_glshwi,
                f_f: wc.f_f,
            };
            wincons.insert(wc.id, wcp);
        }

        // Propiedades de espacios
        let mut spaces: BTreeMap<Uuid, SpaceProps> = BTreeMap::new();
        for s in &model.spaces {
            let area = s.area(&model.walls);
            let height_net = s.height_net(&model.walls, &model.cons);
            let sp = SpaceProps {
                kind: s.kind,
                inside_tenv: s.inside_tenv,
                area,
                multiplier: s.multiplier,
                height: s.height,
                height_net,
                volume_net: area * height_net,
            };
            spaces.insert(s.id, sp);
        }

        // Propiedades de opacos
        // Opacos de la envolvente térmica
        let tenv_wall_ids: Vec<_> = model
            .walls
            .iter()
            .filter(|w| {
                let this_space_is_inside_tenv = model
                    .get_space(w.space)
                    .map(|s| s.inside_tenv)
                    .unwrap_or(false);
                let next_space_is_inside_tenv = w
                    .next_to
                    .and_then(|next_to| model.get_space(next_to))
                    .map(|s| s.inside_tenv)
                    .unwrap_or(false);
                match w.bounds {
                    BoundaryType::EXTERIOR | BoundaryType::GROUND | BoundaryType::ADIABATIC => {
                        this_space_is_inside_tenv
                    }
                    BoundaryType::INTERIOR => {
                        this_space_is_inside_tenv != next_space_is_inside_tenv
                    }
                }
            })
            .map(|w| w.id)
            .collect();

        let mut walls: BTreeMap<Uuid, WallProps> = BTreeMap::new();
        for w in &model.walls {
            let wall_override = model.overrides.walls.get(&w.id);
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
                is_tenv: tenv_wall_ids.contains(&w.id),
                u_value: w.u_value(model),
                u_value_override: wall_override.and_then(|o| o.u_value),
            };
            walls.insert(w.id, wp);
        }

        // Propiedades de huecos
        let fshobstmap = model.compute_fshobst();
        let mut windows: BTreeMap<Uuid, WinProps> = BTreeMap::new();
        for w in &model.windows {
            let wall = walls.get(&w.wall);
            let win_override = model.overrides.windows.get(&w.id);
            let wp = WinProps {
                wall: w.wall,
                cons: w.cons,
                orientation: wall.map(|w| w.orientation).unwrap_or_default(),
                tilt: wall.map(|w| w.tilt).unwrap_or_default(),
                area: w.area(),
                multiplier: wall.map(|wp| wp.multiplier).unwrap_or(1.0),
                bounds: wall.map(|w| w.bounds).unwrap_or_default(),
                is_tenv: tenv_wall_ids.contains(&w.wall),
                u_value: wincons.get(&w.cons).and_then(|c| c.u_value),
                u_value_override: win_override.and_then(|o| o.u_value),
                f_shobst: fshobstmap.get(&w.id).copied(),
                f_shobst_override: win_override.and_then(|o| o.f_shobst),
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

        // Propiedades de sombras
        let mut shades: BTreeMap<Uuid, ShadeProps> = BTreeMap::new();
        for s in &model.shades {
            let sp = ShadeProps {
                orientation: Orientation::from(s),
                tilt: Tilt::from(s),
                area: s.area(),
            };
            shades.insert(s.id, sp);
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
                .filter(|w| {
                    w.is_tenv
                        && (w.bounds == BoundaryType::EXTERIOR || w.bounds == BoundaryType::GROUND)
                })
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

        // Manejo de los opacos según disponibilidad de ensayo
        // Permeabilidad de opacos calculada según criterio de edad por defecto DB-HE2019 (1/h)
        // NOTE: usamos is_new_building pero igual merecería la pena una variable para permeabilidad mejorada
        let c_o_100 = if model.meta.is_new_building {
            16.0
        } else {
            29.0
        };

        let global = GlobalProps {
            a_ref,
            vol_env_gross,
            vol_env_net,
            vol_env_inh_net,
            compacity,
            global_ventilation_rate,
            n_50_test_ach: model.meta.n50_test_ach,
            c_o_100,
        };

        Self {
            global,
            spaces,
            walls,
            windows,
            thermal_bridges,
            shades,
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
    /// y la superficie de opacos y huecos con intercambio térmico con el aire exterior o el terreno (A)
    /// Tiene en cuenta los multiplicadores de espacios (en superficie y volumen)
    /// Se excluyen los huecos sin opaco definido y los opacos sin espacio definido
    /// Para area expuesta => compacidad = 0.0
    pub compacity: f32,
    /// Tasa de ventilación global del edificio (1/h)
    pub global_ventilation_rate: f32,
    /// Tasa de renovación de aire a 50Pa obtenida mediante ensayo de puerta soplante (1/h)
    pub n_50_test_ach: Option<f32>,
    /// Permeabilidad al aire de opacos de referencia a 100 Pa [m³/hm²]
    /// Permeabilidad de opacos calculada según criterio de edad por defecto DB-HE2019 (1/h)
    /// NOTE: usamos is_new_building pero igual merecería la pena una variable para permeabilidad mejorada
    pub c_o_100: f32,
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

/// Propiedades de opacos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallProps {
    /// Espacio al que pertenece el opaco
    pub space: Uuid,
    /// Espacio adyacente
    pub space_next: Option<Uuid>,
    /// Condición de contorno
    pub bounds: BoundaryType,
    /// Construcción de opaco
    pub cons: Uuid,
    /// Orientación del opaco
    pub orientation: Orientation,
    /// Inclinación del opaco
    pub tilt: Tilt,
    /// Superficie bruta del opaco, [m²]
    pub area_gross: f32,
    /// Superficie neta del opaco, [m²]
    pub area_net: f32,
    /// Multiplicador del espacio, [-]
    pub multiplier: f32,
    /// ¿Pertenece este opaco a la envolvente térmica?
    pub is_tenv: bool,
    /// U de opaco (calculado), [W/m²K]
    pub u_value: Option<f32>,
    /// U de opaco (usuario), [W/m²K]
    pub u_value_override: Option<f32>,
}

/// Propiedades de huecos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinProps {
    /// Construcción de hueco
    pub cons: Uuid,
    /// Opaco al que está asociado
    pub wall: Uuid,
    /// Orientación del hueco (heredada del opaco)
    pub orientation: Orientation,
    /// Inclinación del hueco (heredada del opaco)
    pub tilt: Tilt,
    /// Superficie del hueco, [m²]
    pub area: f32,
    /// Multiplicador del espacio, [-]
    pub multiplier: f32,
    /// Condiciones de contorno del opaco en el que se sitúa el hueco
    pub bounds: BoundaryType,
    /// ¿Pertenece a la envolvente térmica el opaco en el que se sitúa el hueco?
    pub is_tenv: bool,
    /// U de huecos (calculado), [W/m²K]
    pub u_value: Option<f32>,
    /// U de huecos (usuario), [W/m²K]
    pub u_value_override: Option<f32>,
    // Factor de obstrucción de obstáculos remotos (calculado), [-]
    pub f_shobst: Option<f32>,
    /// Factor de obstrucción de obstáculos remotos (usuario), [-]
    pub f_shobst_override: Option<f32>,
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

/// Propiedades de sombras
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadeProps {
    /// Orientación de la sombra
    pub orientation: Orientation,
    /// Inclinación de la sombra
    pub tilt: Tilt,
    /// Superficie bruta de la sombra, [m²]
    pub area: f32,
}

/// Propiedades de construcciones de opacos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallConsProps {
    /// Espesor total de la construcción, [m]
    pub thickness: f32,
    // Resistencia térmica de la construcción (excluyendo resistencias superficiales), [m²K/W]
    pub resistance: Option<f32>,
}

/// Propiedades de construcciones de opacos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinConsProps {
    /// Transmitancia térmica total del acristalamiento, sin protecciones solares, [-]
    /// Si no está definido en el modelo se usa el del vidrio sencillo = 0.77
    pub g_glwi: f32,
    /// Transmitancia térmica total del acristalamiento, con protecciones solares, [-]
    /// Si no está definido en el modelo se usa el valor de g_glwi
    pub g_glshwi: f32,
    /// U de construcción de hueco, [W/m²K]
    pub u_value: Option<f32>,
    /// Permeabilidad al aire del hueco a 100 Pa, [m³/h·m²]
    pub c_100: f32,
    /// Fracción de marco del hueco, [-]
    pub f_f: f32,
}
