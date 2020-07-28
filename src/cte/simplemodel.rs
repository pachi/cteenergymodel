/* -*- coding: utf-8 -*-

Copyright (c) 2018-2019 Rafael Villar Burke <pachi@ietcc.csic.es>

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
    model::{Constructions, Envelope},
    Boundaries, Model, Space, ThermalBridge, Wall, WallCons, Window, WindowCons,
};

/// Elementos de la envolvente térmica
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleEnvelope {
    /// Huecos
    pub windows: Vec<Window>,
    /// Opacos
    pub walls: Vec<Wall>,
    /// Puentes térmicos
    pub thermal_bridges: Vec<ThermalBridge>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleConstructions {
    /// Construcciones de huecos
    pub windows: Vec<WindowCons>,
    /// Construcciones de opacos
    pub walls: Vec<WallCons>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleModel {
    pub climate: String,
    pub envelope: SimpleEnvelope,
    pub constructions: SimpleConstructions,
    pub spaces: Vec<Space>,
    // XXX: Elementos temporalmente almacenados mientras no se pueden calcular correctamente
    /// U de muros
    pub walls_u: Vec<(String, Boundaries, f32)>,
    /// Factor de obstrucción de obstáculos remotos
    pub windows_fshobst: Vec<(String, f32)>,
}

impl From<Model> for SimpleModel {
    fn from(m: Model) -> Self {
        Self {
            climate: m.climate.clone(),
            envelope: SimpleEnvelope {
                windows: m.envelope.windows.values().cloned().collect(),
                walls: m.envelope.walls.values().cloned().collect(),
                thermal_bridges: m.envelope.thermal_bridges.values().cloned().collect(),
            },
            constructions: SimpleConstructions {
                windows: m.constructions.windows.values().cloned().collect(),
                walls: m.constructions.walls.values().cloned().collect(),
            },
            spaces: m.spaces.values().cloned().collect(),
            walls_u: m.walls_u.clone(),
            windows_fshobst: m.windows_fshobst.clone(),
        }
    }
}

impl From<SimpleModel> for Model {
    fn from(m: SimpleModel) -> Self {
        Self {
            climate: m.climate.clone(),
            envelope: Envelope {
                windows: m
                    .envelope
                    .windows
                    .iter()
                    .map(|w| (w.name.clone(), w.clone()))
                    .collect(),
                walls: m
                    .envelope
                    .walls
                    .iter()
                    .map(|w| (w.name.clone(), w.clone()))
                    .collect(),
                thermal_bridges: m
                    .envelope
                    .thermal_bridges
                    .iter()
                    .map(|tb| (tb.name.clone(), tb.clone()))
                    .collect(),
            },
            constructions: Constructions {
                windows: m
                    .constructions
                    .windows
                    .iter()
                    .map(|w| (w.name.clone(), w.clone()))
                    .collect(),
                walls: m
                    .constructions
                    .walls
                    .iter()
                    .map(|w| (w.name.clone(), w.clone()))
                    .collect(),
            },
            spaces: m
                .spaces
                .iter()
                .map(|s| (s.name.clone(), s.clone()))
                .collect(),
            walls_u: m.walls_u.clone(),
            windows_fshobst: m.windows_fshobst.clone(),
        }
    }
}
