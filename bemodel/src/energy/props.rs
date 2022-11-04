// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Propiedades geométricas y energéticas del modelo
//!
//! Permiten el cálculo de indicadores y la descripción de los elementos del modelo

use log::{error, warn};
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
    /// Propiedades de horarios anuales
    pub sch_year: BTreeMap<Uuid, SchYearProps>,
    /// Propiedades de horarios semanales
    pub sch_week: BTreeMap<Uuid, SchWeekProps>,
    /// Propiedades de horarios diarios
    pub sch_day: BTreeMap<Uuid, SchDayProps>,
    /// Propiedades de cargas de espacios
    pub loads: BTreeMap<Uuid, LoadsProps>,
    // TODO: Propiedades de consignas
    // pub thermostats: BTreeMap<Uuid, ThermostatsProps>
}

impl From<&Model> for EnergyProps {
    /// Calcula propiedades de horarios, cargas, consignas, elementos, construcciones y globales
    fn from(model: &Model) -> Self {
        // Propiedades de horarios ---------------------------------------------

        // Diarios
        let mut sch_day: BTreeMap<Uuid, SchDayProps> = BTreeMap::new();
        for s in &model.schedules.day {
            let values = s.values.clone();
            let values_is_not_zero = values
                .iter()
                .map(|v| v.abs() > 100.0 * f32::EPSILON) // Aprox > 1 e-5
                .collect();
            let average = values.iter().sum::<f32>() / (values.len() as f32);
            let e = SchDayProps {
                values,
                values_is_not_zero,
                average,
            };
            sch_day.insert(s.id, e);
        }

        // Semanales
        let mut sch_week: BTreeMap<Uuid, SchWeekProps> = BTreeMap::new();
        for s in &model.schedules.week {
            let e = SchWeekProps {
                values: s.values.clone(),
            };
            sch_week.insert(s.id, e);
        }

        // Anuales
        let mut sch_year: BTreeMap<Uuid, SchYearProps> = BTreeMap::new();
        for s in &model.schedules.year {
            let e = SchYearProps {
                values: s.values.clone(),
            };
            sch_year.insert(s.id, e);
        }

        // Propiedades de cargas -----------------------------------------------

        // Caché de valores medios y acceso a la misma
        let mut avg_value_cache: BTreeMap<Uuid, f32> = BTreeMap::new();
        let mut get_avg = |id: Uuid| -> f32 {
            *avg_value_cache.entry(id).or_insert_with(|| {
                let day_sch = model.schedules.get_year_as_day_sch(id);
                day_sch.iter().map(|ds| sch_day[ds].average).sum::<f32>() / day_sch.len() as f32
            })
        };

        let mut loads: BTreeMap<Uuid, LoadsProps> = BTreeMap::new();
        for s in &model.loads {
            let people_sch_avg = s.people_schedule.map(&mut get_avg).unwrap_or(0.0);
            let lighting_sch_avg = s.lighting_schedule.map(&mut get_avg).unwrap_or(0.0);
            let equipment_sch_avg = s.equipment_schedule.map(&mut get_avg).unwrap_or(0.0);
            let loads_avg = people_sch_avg * s.people_sensible
                + lighting_sch_avg * s.lighting
                + equipment_sch_avg * s.equipment;

            let e = LoadsProps {
                area_per_person: s.area_per_person,
                people_schedule: s.people_schedule,
                people_sensible: s.people_sensible,
                people_latent: s.people_latent,
                equipment: s.equipment,
                equipment_schedule: s.equipment_schedule,
                lighting: s.lighting,
                lighting_schedule: s.lighting_schedule,
                loads_avg,
            };
            loads.insert(s.id, e);
        }

        // Propiedades de consignas --------------------------------------------
        // TODO:

        // Propiedades de construcciones ---------------------------------------

        // Propiedades de construcciones de opacos
        let mut wallcons: BTreeMap<Uuid, WallConsProps> = BTreeMap::new();
        for wc in &model.cons.wallcons {
            let wcp = WallConsProps {
                thickness: wc.thickness(),
                resistance: wc.resistance(&model.cons).ok(),
            };
            wallcons.insert(wc.id, wcp);
        }

        // Propiedades de construcciones de huecos
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

        // Propiedades de elementos --------------------------------------------

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
                loads: s.loads,
                thermostat: s.thermostat,
                n_v: s.n_v,
                illuminance: s.illuminance,
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

        // Propiedades globales ------------------------------------------------

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
        let compactness = {
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

        // Indicadores de ocupación y cargas -----------------------------------

        // Tiempo anual de ocupación
        // 1. Horarios anuales de ocupación diferentes de espacios habitables en la ET
        let occupied_spaces_people_schedules = spaces.values().filter_map(|s| {
            if s.kind != SpaceType::UNINHABITED && s.inside_tenv && s.loads.is_some() {
                loads.get(&s.loads.unwrap()).and_then(|l| l.people_schedule)
            } else {
                None
            }
        });
        // 2. Convierte calendario anual a lista de 365 horarios diarios
        let schedules_as_days: Vec<Vec<Uuid>> = occupied_spaces_people_schedules
            .map(|people_schedule_id| model.schedules.get_year_as_day_sch(people_schedule_id))
            .collect();
        // 3. Comprobación de que todos los horarios tienen la misma duración en días
        let year_len = schedules_as_days
            .first()
            .map(|s| s.len())
            .unwrap_or_default();
        if year_len != 365 {
            warn!("Duración de horarios anuales distinta a 365 días")
        };
        if !schedules_as_days
            .iter()
            .map(|s| s.len())
            .all(|item| item == year_len)
        {
            error!("Horarios anuales con distinta duración en días")
        };
        // 4. Para cada día localiza los horarios diarios diferentes
        let year_distinct_day_sch_by_day = (0..year_len)
            .map(|day_idx| {
                schedules_as_days
                    .iter()
                    .map(|s| s[day_idx])
                    .collect::<Vec<_>>()
            })
            .map(|mut dv| {
                dv.sort_unstable();
                dv.dedup();
                dv.iter()
                    .map(|id| sch_day.get(id).unwrap())
                    .collect::<Vec<_>>()
            });
        // 5. Acumula las horas ocupadas en cada día para todos los horarios diarios
        let occ_spaces_hours_in_use = year_distinct_day_sch_by_day
            .map(|day_scheds| {
                day_scheds
                    .iter()
                    .fold(vec![false; 24], |ac, sch| {
                        ac.iter()
                            .zip(&sch.values_is_not_zero)
                            .map(|(a, b)| *a || *b)
                            .collect()
                    })
                    .iter()
                    .filter(|v| **v)
                    .count()
            })
            .sum::<usize>() as u32;

        // Carga interna media: valor de la carga interna media de los espacios habitables de la ET
        // ponderada por superficie
        let occupied_spaces = spaces
            .values()
            .filter(|s| s.kind != SpaceType::UNINHABITED && s.inside_tenv && s.loads.is_some());
        let (total_load, total_area) =
            occupied_spaces.fold((0.0, 0.0), |(acc_load, acc_area), s| {
                (
                    acc_load
                        + s.loads
                            .map(|loads_id| {
                                loads.get(&loads_id).map(|l| l.loads_avg).unwrap_or(0.0)
                            })
                            .unwrap_or(0.0)
                            * s.area
                            * s.multiplier,
                    acc_area + s.area * s.multiplier,
                )
            });
        let occ_spaces_average_load = if total_area > f32::EPSILON {
            total_load / total_area
        } else {
            0.0
        };

        let global = GlobalProps {
            a_ref,
            vol_env_gross,
            vol_env_net,
            vol_env_inh_net,
            compactness,
            global_ventilation_rate,
            n_50_test_ach: model.meta.n50_test_ach,
            c_o_100,
            occ_spaces_hours_in_use,
            occ_spaces_average_load,
        };

        // Resultado final -----------------------------------------------------

        Self {
            global,
            spaces,
            walls,
            windows,
            thermal_bridges,
            shades,
            wallcons,
            wincons,
            sch_year,
            sch_week,
            sch_day,
            loads,
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
    /// De acuerdo con la definición del DB-HE comprende el volumen interior de
    /// la envolvente térmica (V) y la superficie de opacos y huecos con intercambio
    /// térmico con el aire exterior o el terreno (A)
    /// Tiene en cuenta los multiplicadores de espacios (en superficie y volumen)
    /// Se excluyen los huecos sin opaco definido y los opacos sin espacio definido
    /// Para area expuesta => compacidad = 0.0
    pub compactness: f32,
    /// Tasa de ventilación global del edificio (1/h)
    pub global_ventilation_rate: f32,
    /// Tasa de renovación de aire a 50Pa obtenida mediante ensayo de puerta soplante (1/h)
    pub n_50_test_ach: Option<f32>,
    /// Permeabilidad al aire de opacos de referencia a 100 Pa [m³/hm²]
    /// Permeabilidad de opacos calculada según criterio de edad por defecto DB-HE2019 (1/h)
    /// NOTE: usamos is_new_building pero igual merecería la pena una variable
    /// para permeabilidad mejorada
    pub c_o_100: f32,
    /// Tiempo total de ocupación de los espacios habitables en el interior de
    /// la envolvente térmica, h
    /// Se computa la ocupación de cualquier espacio a cada hora, de modo que el máximo
    /// valor para el edificio es 8760h
    pub occ_spaces_hours_in_use: u32,
    /// Carga media de las fuentes internas (ocupación sensible, iluminación, equipos)
    /// de los espacios habitables dentro de la envolvente térmica, W/m²
    pub occ_spaces_average_load: f32,
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
    /// Cargas del espacio, UUID
    pub loads: Option<Uuid>,
    /// Termostato del espacio, UUID
    pub thermostat: Option<Uuid>,
    /// Ventilación, en ren/h
    pub n_v: Option<f32>,
    /// Iluminancia media en el plano de trabajo, lux
    pub illuminance: Option<f32>,
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

// TODO: Revisar duplicación de métodos con bemodel::ScheduleDB
// TODO: probablemente deberían ir aquí y no en el modelo?

/// Propiedades de horarios diarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchDayProps {
    /// Valores
    pub values: Vec<f32>,
    /// Valores distintos de cero
    pub values_is_not_zero: Vec<bool>,
    /// Valor medio
    pub average: f32,
}

/// Propiedades de horarios mensuales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchWeekProps {
    /// Lista de elementos (id_sch_dia, repeticiones)
    pub values: Vec<(Uuid, u32)>,
}

impl SchWeekProps {
    /// Devuelve semana como lista de 7 valores diarios
    pub fn to_day_sch(&self) -> Vec<Uuid> {
        self.values
            .iter()
            .flat_map(|(id, count)| vec![*id; *count as usize])
            .collect()
    }
}

/// Propiedades de horarios mensuales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchYearProps {
    /// Lista de elementos (id_sch_semana, repeticiones en días)
    pub values: Vec<(Uuid, u32)>,
}

/// Propiedades de cargas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadsProps {
    /// Superficie por ocupante, m²/pers
    pub area_per_person: f32,
    /// Horario anual de fracciones de carga de ocupación
    pub people_schedule: Option<Uuid>,
    /// Carga máxima sensible de ocupación, W/m²
    pub people_sensible: f32,
    /// Carga máxima latente de ocupación, W/m²
    pub people_latent: f32,
    /// Carga total debida a los equipos, W/m²
    pub equipment: f32,
    /// Horario anual de fracciones de carga de equipos
    pub equipment_schedule: Option<Uuid>,
    /// Carga total debida a la iluminación, W/m²
    pub lighting: f32,
    /// Horario anual de fracciones de carga de iluminación
    pub lighting_schedule: Option<Uuid>,
    /// Carga interna media (ocupación sensible, iluminación, equipos), W/m²
    pub loads_avg: f32,
}
