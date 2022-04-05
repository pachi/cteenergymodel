// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Información energética relativa al modelo
//!
//! Cálculo de K, qsoljul, Fshobst, etc

use anyhow::Error;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{climatedata, BoundaryType, Model, Orientation, SpaceType, Tilt, Uuid, Warning};

mod aabb;
mod bvh;
mod geometry;
mod n50;
mod occluder;
mod radiation;
mod ray;
mod transmittance;

pub use aabb::AABB;
pub use bvh::{Bounded, Intersectable, BVH};
pub use n50::N50Data;
pub use radiation::{ray_dir_to_sun, QSolJulData};
pub use ray::Ray;

/// Estructura que contiene los resultados del cálculo de indicadores y parámetros energéticos
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyIndicators {
    pub area_ref: f32,
    pub compacity: f32,
    pub vol_env_net: f32,
    pub vol_env_gross: f32,
    pub elements: EnergyProps,
    pub K_data: KData,
    pub q_soljul_data: QSolJulData,
    pub n50_data: N50Data,
    pub warnings: Vec<Warning>,
}

impl EnergyIndicators {
    /// Devuelve resultados en formato JSON
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// Calcula indicadores energéticos del modelo
    pub fn compute(model: &Model) -> Self {
        let climatezone = model.meta.climate;
        let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
        Self {
            area_ref: model.a_ref(),
            compacity: model.compacity(),
            vol_env_net: model.vol_env_net(),
            vol_env_gross: model.vol_env_gross(),

            elements: EnergyProps::from(model),
            K_data: KData::K(model),
            q_soljul_data: model.q_soljul(&totradjul),
            n50_data: model.n50(),

            warnings: model.check(),
        }
    }
}

/// Reporte de cálculo de propiedades térmicas y geométricas de los elementos
/// TODO: llevar a un atributo model las propiedades del modelo y warnings que ahora están en EnergyIndicators y dejar ahí solo K, q_soljul, n50
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

/// Reporte de cálculo de K (HE2019)
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct KData {
    /// K global [W/m²K]
    pub K: f32,
    /// Resumen (K, opacos, huecos, tb)
    pub summary: KSummary,
    /// Muros (aire)
    pub walls: KElementProps,
    /// Cubiertas (aire)
    pub roofs: KElementProps,
    /// Suelos (aire)
    pub floors: KElementProps,
    /// Elementos en contacto con el terreno (de cualquier tipo)
    pub ground: KElementProps,
    /// Huecos
    pub windows: KElementProps,
    /// Puentes térmicos
    pub tbs: KTBElements,
}

/// Resumen de resultados de K
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct KSummary {
    /// Superficie total [m²]
    pub a: f32,
    /// AU + ψL total [W/K]
    pub au: f32,
    /// A de opacos [m²]
    pub opaques_a: f32,
    /// AU de opacos [W/K]
    pub opaques_au: f32,
    /// A de huecos [m²]
    pub windows_a: f32,
    /// AU de huecos [W/K]
    pub windows_au: f32,
    /// L de puentes térmicos [m]
    pub tbs_l: f32,
    /// ψL de puenstes térmicos [W/K]
    pub tbs_psil: f32,
}
/// Propiedades de cada elemento de K
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct KElementProps {
    /// A del elemento [m²]
    pub a: f32,
    /// A·U del elemento [W/K]
    pub au: f32,
    /// U máximo observado [W/m²K]
    pub u_max: Option<f32>,
    /// U mínimo observado [W/m²K]
    pub u_min: Option<f32>,
    /// U medio observado [W/m²K]
    pub u_mean: Option<f32>,
}

/// Tipos de elementos térmicos y sus propiedades
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct KTBElements {
    /// Puentes térmicos fachada-cubierta (R)
    pub roof: KTBElementProps,
    /// Puentes térmicos balcón (B)
    pub balcony: KTBElementProps,
    /// Puentes térmicos fachada-fachada (C)
    pub corner: KTBElementProps,
    /// Puentes térmicos forjado-fachada (IF)
    pub intermediate_floor: KTBElementProps,
    /// Puentes térmicos muro-fachada/cubierta (IW)
    pub internal_wall: KTBElementProps,
    /// Puentes térmicos solera/cámara sanitaria/muro ent.-fachada (GF)
    pub ground_floor: KTBElementProps,
    /// Puentes térmicos pilares (P)
    pub pillar: KTBElementProps,
    /// Puentes térmicos contorno de hueco (W)
    pub window: KTBElementProps,
    /// Puentes térmicos genéricos (G)
    pub generic: KTBElementProps,
}

/// Propiedades de cada elemento de K
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct KTBElementProps {
    /// L del elemento [m]
    pub l: f32,
    /// ψ·L del elemento [W/K]
    pub psil: f32,
}

impl KData {
    /// Calcula la transmitancia térmica global K (W/m2K)
    /// Transmitancia media de opacos, huecos y puentes térmicos en contacto con el aire exterior o con el terreno
    ///
    /// Se ignoran los huecos y muros para los que no está definida su construcción, transmitancia o espacio
    #[allow(non_snake_case)]
    pub fn K(model: &Model) -> KData {
        let mut k = Self::default();
        // Opacos
        for wall in model.exterior_and_ground_walls_of_envelope_iter() {
            let multiplier = model
                .get_space(wall.space)
                .map(|s| s.multiplier)
                .unwrap_or(1.0);
            // Huecos de los opacos
            for win in wall.windows(&model.windows) {
                let win_u = match &model
                    .cons
                    .get_wincons(win.cons)
                    .and_then(|c| c.u_value(&model.mats))
                {
                    Some(u) => *u,
                    _ => continue,
                };
                let area = multiplier * win.area();
                k.windows.a += area;
                k.windows.au += area * win_u;
                k.windows.u_max = k.windows.u_max.map(|v| v.max(win_u)).or(Some(win_u));
                k.windows.u_min = k.windows.u_min.map(|v| v.min(win_u)).or(Some(win_u));
            }
            let wall_u = match wall.u_value(model) {
                Some(u) => u,
                _ => continue,
            };
            let area = multiplier * wall.area_net(&model.windows);
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
        for tb in &model.thermal_bridges {
            use crate::ThermalBridgeKind::*;
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
        #[allow(non_snake_case)]
        let Self {
            roofs,
            floors,
            walls,
            ground,
            tbs,
            windows,
            mut summary,
            ..
        } = k;
        summary.opaques_a = roofs.a + floors.a + walls.a + ground.a;
        summary.opaques_au = roofs.au + floors.au + walls.au + ground.au;
        summary.windows_a = windows.a;
        summary.windows_au = windows.au;
        summary.tbs_l = tbs.roof.l
            + tbs.balcony.l
            + tbs.corner.l
            + tbs.intermediate_floor.l
            + tbs.internal_wall.l
            + tbs.ground_floor.l
            + tbs.pillar.l
            + tbs.window.l
            + tbs.generic.l;
        summary.tbs_psil = tbs.roof.psil
            + tbs.balcony.psil
            + tbs.corner.psil
            + tbs.intermediate_floor.psil
            + tbs.internal_wall.psil
            + tbs.ground_floor.psil
            + tbs.pillar.psil
            + tbs.window.psil
            + tbs.generic.psil;
        summary.a = summary.opaques_a + summary.windows_a;
        summary.au = summary.opaques_au + summary.windows_au + summary.tbs_psil;
        k.K = if summary.a < 0.01 {
            0.0
        } else {
            summary.au / summary.a
        };

        let s = k.summary;
        info!(
            "K={:.2} W/m²K, A_o={:.2} m², (A.U)_o={:.2} W/K, A_h={:.2} m², (A.U)_h={:.2} W/K, L_pt={:.2} m, Psi.L_pt={:.2} W/K",
            k.K, s.opaques_a, s.opaques_au, s.windows_a, s.windows_au, s.tbs_l, s.tbs_psil
        );

        k
    }
}
