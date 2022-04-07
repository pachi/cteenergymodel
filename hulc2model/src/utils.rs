// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Utilidades varias

/// Redondea valor a 2 decimales
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}
