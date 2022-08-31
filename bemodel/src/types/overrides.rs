// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Elementos opacos: Wall, Shade y sus objetos asociados, Geometry

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::Uuid;

/// Valores fijados por el usuario con prioridad sobre los calculados
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PropsOverrides {
    pub walls: BTreeMap<Uuid, WallPropsOverrides>,
    pub windows: BTreeMap<Uuid, WinPropsOverrides>,
}

impl PropsOverrides {
    pub(crate) fn is_empty(&self) -> bool {
        self.walls.is_empty() && self.windows.is_empty()
    }
}

/// Propiedades de elemento opaco (muro, cubierta, suelo, partición) definidas por el usuario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WallPropsOverrides {
    /// U de opaco, [W/m²K]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_value: Option<f32>,
}

/// Propiedades de hueco definidas por el usuario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WinPropsOverrides {
    /// U de huecos, [W/m²K]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_value: Option<f32>,
    /// Factor de obstrucción de obstáculos remotos, [-]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f_shobst: Option<f32>,
}
