// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Utilidades varias

/// Normaliza número a un intervalo arbitrario (wrapping)
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    // ancho del intervalo
    let width = end - start;
    // convertimos el intervalo a [0, ancho] restando el valor inicial
    let offset = value - start;
    // volvemos a sumar el valor incial para volver al intervalo [start, end]
    (offset - (f32::floor(offset / width) * width)) + start
}
