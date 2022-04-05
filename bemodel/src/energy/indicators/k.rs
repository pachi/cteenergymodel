// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Información energética relativa al modelo
//!
//! Cálculo de K, qsoljul, Fshobst, etc

use log::info;
use serde::{Deserialize, Serialize};

use crate::energy::EnergyProps;
use crate::{BoundaryType, Tilt};

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

impl From<&EnergyProps> for KData {
    /// Calcula la transmitancia térmica global K (W/m2K)
    /// Transmitancia media de opacos, huecos y puentes térmicos en contacto con el aire exterior o con el terreno
    ///
    /// Se ignoran los huecos y muros para los que no está definida su construcción, transmitancia o espacio
    // #[allow(non_snake_case)]
    fn from(props: &EnergyProps) -> Self {
        let mut k = Self::default();

        // Opacos
        for (wall_id, wall) in props.walls.iter().filter(|(_, w)| w.is_ext_or_gnd_tenv) {
            let multiplier = wall.multiplier;
            for window in props.windows.values().filter(|win| &win.wall == wall_id) {
                if let Some(win_u) = window.u_value {
                    let area = multiplier * window.area;
                    k.windows.a += area;
                    k.windows.au += area * win_u;
                    k.windows.u_max = k.windows.u_max.map(|v| v.max(win_u)).or(Some(win_u));
                    k.windows.u_min = k.windows.u_min.map(|v| v.min(win_u)).or(Some(win_u));
                }
            }
            if let Some(wall_u) = wall.u_value {
                let area = multiplier * wall.area_net;
                let area_u = area * wall_u;
                let mut element_case = match (wall.bounds, wall.tilt) {
                    (BoundaryType::GROUND, _) => &mut k.ground,
                    (_, Tilt::TOP) => &mut k.roofs,
                    (_, Tilt::BOTTOM) => &mut k.floors,
                    (_, Tilt::SIDE) => &mut k.walls,
                };
                element_case.a += area;
                element_case.au += area_u;
                element_case.u_max = element_case.u_max.map(|v| v.max(wall_u)).or(Some(wall_u));
                element_case.u_min = element_case.u_min.map(|v| v.min(wall_u)).or(Some(wall_u));
            };
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
        for tb in props.thermal_bridges.values() {
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
