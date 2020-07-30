/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Modelo simplificado del edificio que usa Vecs en lugar de BTreeMaps para una representación más compacta en JSON

use serde::{Deserialize, Serialize};

use super::{
    Boundaries, Meta, Model, Space, ThermalBridge, Tilt, Wall, WallCons, Window, WindowCons,
};

/// Modelo simplificado para la exportación a JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleModel {
    /// Metadatos
    pub meta: Meta,
    /// Huecos
    pub windows: Vec<Window>,
    /// Opacos
    pub walls: Vec<Wall>,
    /// Puentes térmicos
    pub thermal_bridges: Vec<ThermalBridge>,
    /// Espacios
    pub spaces: Vec<Space>,
    /// Construcciones de huecos
    pub wincons: Vec<WindowCons>,
    /// Construcciones de opacos
    pub wallcons: Vec<WallCons>,
    // XXX: Elementos temporalmente almacenados mientras no se pueden calcular correctamente
    /// U de muros
    pub walls_u: Vec<(String, Boundaries, Tilt, f32)>,
}

impl From<Model> for SimpleModel {
    fn from(m: Model) -> Self {
        Self {
            meta: m.meta.clone(),
            windows: m.windows.values().cloned().collect(),
            walls: m.walls.values().cloned().collect(),
            thermal_bridges: m.thermal_bridges.values().cloned().collect(),
            spaces: m.spaces.values().cloned().collect(),
            wincons: m.wincons.values().cloned().collect(),
            wallcons: m.wallcons.values().cloned().collect(),
            walls_u: m.walls_u.clone(),
        }
    }
}

impl From<SimpleModel> for Model {
    fn from(m: SimpleModel) -> Self {
        Self {
            meta: m.meta.clone(),
            windows: m
                .windows
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            walls: m
                .walls
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            thermal_bridges: m
                .thermal_bridges
                .iter()
                .map(|tb| (tb.name.clone(), tb.clone()))
                .collect(),
            spaces: m
                .spaces
                .iter()
                .map(|s| (s.name.clone(), s.clone()))
                .collect(),
            wincons: m
                .wincons
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),
            wallcons: m
                .wallcons
                .iter()
                .map(|w| (w.name.clone(), w.clone()))
                .collect(),

            walls_u: m.walls_u.clone(),
        }
    }
}
