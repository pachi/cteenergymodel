// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Propiedades energéticas del modelo
//!
//! Permiten el cálculo de indicadores y la descripción de los elementos del modelo

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{BoundaryType, Model, Orientation, SpaceType, Uuid};

/// Reporte de cálculo de propiedades térmicas y geométricas del modelo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProps {
    /// Propiedades de espacios
    pub spaces: BTreeMap<Uuid, SpaceProps>,
    /// Propiedades de muros
    pub walls: BTreeMap<Uuid, WallProps>,
    /// Propiedades de huecos
    pub windows: BTreeMap<Uuid, WinProps>,
    /// Propiedades de construcciones de muros
    pub wallcons: BTreeMap<Uuid, WallConsProps>,
    /// Propiedades de huecos
    pub wincons: BTreeMap<Uuid, WinConsProps>,
}

impl From<&Model> for EnergyProps {
    /// Completa datos de los elementos (espacios, opacos, huecos,...) por id
    fn from(model: &Model) -> Self {
        let mut spaces: BTreeMap<Uuid, SpaceProps> = BTreeMap::new();
        for s in &model.spaces {
            let sp = SpaceProps {
                kind: s.kind,
                area: s.area,
                multiplier: s.multiplier,
                height_net: s.height_net(&model.walls, &model.cons),
                volume_net: s.volume_net(&model.walls, &model.cons),
            };
            spaces.insert(s.id, sp);
        }

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

        let ext_and_gnd_walls_tenv: Vec<_> = model
            .exterior_and_ground_walls_of_envelope_iter()
            .map(|w| w.id)
            .collect();

        let mut walls: BTreeMap<Uuid, WallProps> = BTreeMap::new();
        for w in &model.walls {
            let wp = WallProps {
                bounds: w.bounds,
                cons: w.cons,
                orientation: Orientation::from(w),
                area_gross: w.area(),
                area_net: w.area_net(&model.windows),
                multiplier: spaces.get(&w.space).map(|sp| sp.multiplier).unwrap_or(1.0),
                is_ext_or_gnd_tenv: ext_and_gnd_walls_tenv.contains(&w.id),
                u_value: w.u_value(model),
            };
            walls.insert(w.id, wp);
        }

        let mut windows: BTreeMap<Uuid, WinProps> = BTreeMap::new();
        for w in &model.windows {
            let wall = walls.get(&w.wall);
            let wp = WinProps {
                cons: w.cons,
                orientation: wall.map(|w| w.orientation).unwrap_or_default(),
                area: w.area(),
                multiplier: wall.map(|wp| wp.multiplier).unwrap_or(1.0),
                is_ext_or_gnd_tenv: ext_and_gnd_walls_tenv.contains(&w.wall),
                u_value: wincons.get(&w.cons).and_then(|c| c.u_value),
            };
            windows.insert(w.id, wp);
        }
        Self {
            spaces,
            walls,
            windows,
            wallcons,
            wincons,
        }
    }
}

/// Propiedades de espacios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceProps {
    /// Nivel de acondicionamiento del espacio, [-]
    pub kind: SpaceType,
    /// Superficie del espacio, [m²]
    pub area: f32,
    /// Multiplicador del espacio, [-]
    pub multiplier: f32,
    /// Altura neta del espacio, [m]
    pub height_net: f32,
    /// Volumen neto del espacio, [m³]
    pub volume_net: f32,
}

/// Propiedades de muros
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallProps {
    /// Condición de contorno
    pub bounds: BoundaryType,
    /// Construcción de muro
    pub cons: Uuid,
    /// Orientación (heredada del muro)
    pub orientation: Orientation,
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
    /// Orientación (heredada del muro)
    pub orientation: Orientation,
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
