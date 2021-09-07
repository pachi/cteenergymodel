// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)
#![allow(clippy::upper_case_acronyms)]

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::Orientation;

/// Nivel de aviso para condiciones de chequeo del modelo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WarningLevel {
    SUCCESS,
    DANGER,
    WARNING,
    INFO,
}

/// Muestra WarningLevel
impl Display for WarningLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use WarningLevel::*;
        let printable = match *self {
            SUCCESS => "SUCCESS",
            DANGER => "DANGER",
            WARNING => "WARNING",
            _ => "INFO",
        };
        write!(f, "{}", printable)
    }
}

/// Convierte str a WarningLevel
impl From<&str> for WarningLevel {
    fn from(level: &str) -> Self {
        match level.to_uppercase().as_str() {
            "SUCCESS" => Self::SUCCESS,
            "DANGER" => Self::DANGER,
            "WARNING" => Self::WARNING,
            _ => Self::INFO,
        }
    }
}

/// Reporte de avisos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    /// Nivel de afectación
    pub level: WarningLevel,
    /// Id del elemento afectado, en su caso
    pub id: Option<String>,
    /// Mensaje del aviso
    pub msg: String,
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
    /// Rellena datos del resumen de cálculo de K
    pub fn compute(&mut self) {
        #[allow(non_snake_case)]
        let Self {
            roofs,
            floors,
            walls,
            ground,
            tbs,
            windows,
            summary,
            K,
        } = self;
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
        *K = if summary.a < 0.01 {
            0.0
        } else {
            summary.au / summary.a
        };
    }
}

/// Reporte de cálculo del parámetro de control solar q_sol:jul (HE2019)
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QSolJulData {
    /// Parámetro de control solar q_sol:jul [kWh/m²·mes]
    pub q_soljul: f32,
    /// Ganancias para el mes de julio (Q_soljul) [kWh/mes]
    pub Q_soljul: f32,
    /// Superficie total de huecos [m²]
    pub a_wp: f32,
    /// Irradiación solar acumulada, media ponderada por superficie de huecos [kWh/m²·mes]
    pub irradiance_mean: f32,
    /// Factor de obstáculos remoto, media ponderada por superficie de huecos [-]
    pub fshobst_mean: f32,
    /// Factor solar del hueco con los elementos de sombra activados, media ponderada por superficie de huecos [-]
    pub gglshwi_mean: f32,
    /// Fracción de marco, media ponderada por superficie de huecos [-]
    #[serde(rename = "Ff_mean")]
    pub ff_mean: f32,
    /// Datos de ganancias solares (Q_soljul) resumidos por orientaciones
    pub detail: HashMap<Orientation, QSolJulDetail>,
}

/// Detalles del parámetro de control solar q_sol:jul (HE2019) por orientación
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct QSolJulDetail {
    /// Ganancias para el mes de julio (Q_soljul) para la orientación [kWh/mes]
    pub gains: f32,
    /// Superficie de huecos por orientación [m²]
    pub a: f32,
    /// Irradiación solar acumulada en el mes de julio (H_sol;jul) para la orientación [kWh/m²·mes]
    pub irradiance: f32,
    /// Fracción de marco media de la orientación, ponderada por superficie de huecos [-]
    #[serde(rename = "Ff_mean")]
    pub ff_mean: f32,
    /// Factor solar con sombras móviles activadas medio de la orientación, ponderada por superficie de huecos [-]
    pub gglshwi_mean: f32,
    /// Factor de obstáculos remotos medio de la orientación, ponderado por superficie de huecos [-]
    pub fshobst_mean: f32,
}

/// Reporte de cálculo de n50 con valores de referencia (teóricos) y de ensayo (si está disponible)
/// El valor teórico usa las permeabilidades del CTE DB-HE 2019
/// Cuando se dispone de valor de ensayo n50 se utiliza para calcular la permeabilidad de opacos
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct N50Data {
    /// Relación de cambio de aire a 50 Pa (n50) calculado con valor de ensayo, si está disponible, o valor de referencia [1/h]
    pub n50: f32,
    /// Relación de cambio de aire a 50 Pa (n50) calculado con la permeabilidad de opacos de referencia [1/h]
    pub n50_ref: f32,
    /// Superficie (A_o) de los opacos que participan en la n_50 [m²]
    pub walls_a: f32,
    /// Permeabilidad de referencia (C_o) de los opacos [m³/h·m²]
    pub walls_c_ref: f32,
    /// C_o_ref · A_o de los opacos con valor de permeabilidad de referencia [m³/h]
    pub walls_c_a_ref: f32,
    /// Permeabilidad (C_o) de los opacos que participan en la n_50 obtenida de ensayo, si está disponible, o igual al de referencia [m³/h·m²]
    pub walls_c: f32,
    /// C_o · A_o de los opacos que participan en la n_50, usando valor de ensayo, si está disponible, o de referencia [m³/h]
    pub walls_c_a: f32,
    /// Superficie (A_h) de los huecos que participan en la n_50 [m²]
    pub windows_a: f32,
    /// Permeabilidad (C_h) media de los huecos que participan en la n_50 [m³/h·m²]
    pub windows_c: f32,
    /// C_h · A_h de los huecos que participan en la n_50 [m³/h]
    pub windows_c_a: f32,
    /// Volumen interior de los espacios interiores a la envolvente térmica [m³]
    pub vol: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Reporte de cálculo de las transmitancias de los elementos
/// TODO: cambiar a ElementProps, no separar muros y huecos, e incluir
/// TODO: si el elemento pertenece o no a la ET ElementProps{walls: {id: String, ElementData {et: bool, a: f32, u: Option<f32>}}}
pub struct UValues {
    /// U de muros
    pub walls: HashMap<String, Option<f32>>,
    /// U de huecos
    pub windows: HashMap<String, Option<f32>>,
}
