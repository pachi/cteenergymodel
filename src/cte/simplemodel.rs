// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo simplificado del edificio que usa Vecs en lugar de BTreeMaps para una representación más compacta en JSON

use serde::{Deserialize, Serialize};

use super::{ExtraData, Meta, Model, Space, ThermalBridge, Wall, WallCons, Window, WindowCons};

/// Modelo simplificado para la exportación a JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleModel {
    /// Metadatos
    pub meta: Meta,
    /// Espacios
    pub spaces: Vec<Space>,
    /// Opacos
    pub walls: Vec<Wall>,
    /// Huecos
    pub windows: Vec<Window>,
    /// Puentes térmicos
    pub thermal_bridges: Vec<ThermalBridge>,
    /// Construcciones de opacos
    pub wallcons: Vec<WallCons>,
    /// Construcciones de huecos
    pub wincons: Vec<WindowCons>,
    /// Datos extra
    pub extra: Option<Vec<ExtraData>>,
}

impl From<Model> for SimpleModel {
    fn from(m: Model) -> Self {
        Self {
            meta: m.meta.clone(),
            spaces: m.spaces.values().cloned().collect(),
            walls: m.walls.values().cloned().collect(),
            windows: m.windows.values().cloned().collect(),
            thermal_bridges: m.thermal_bridges.values().cloned().collect(),
            wallcons: m.wallcons.values().cloned().collect(),
            wincons: m.wincons.values().cloned().collect(),
            extra: m.extra.clone(),
        }
    }
}

impl From<SimpleModel> for Model {
    fn from(m: SimpleModel) -> Self {
        Self {
            meta: m.meta.clone(),
            spaces: m
                .spaces
                .iter()
                .map(|s| (s.name.clone(), s.clone()))
                .collect(),
            walls: m
                .walls
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            windows: m
                .windows
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            thermal_bridges: m
                .thermal_bridges
                .iter()
                .map(|tb| (tb.name.clone(), tb.clone()))
                .collect(),
            wallcons: m
                .wallcons
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            wincons: m
                .wincons
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            extra: m.extra.clone(),
        }
    }
}