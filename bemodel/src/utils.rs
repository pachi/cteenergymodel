// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Utilidades varias de redondeo, normalización de datos y generación de UUID

use crate::Uuid;

/// Redondea valor a 2 decimales
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}

/// Redondea valor a 3 decimales
pub fn fround3(val: f32) -> f32 {
    (val * 1000.0).round() / 1000.0
}

/// Normaliza número a un intervalo arbitrario (wrapping)
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    // ancho del intervalo
    let width = end - start;
    // convertimos el intervalo a [0, ancho] restando el valor inicial
    let offset = value - start;
    // volvemos a sumar el valor inicial para volver al intervalo [start, end]
    (offset - (f32::floor(offset / width) * width)) + start
}

/// Calcula UUID a partir de hash MD5 del objeto
///
/// Este no es un método muy robusto pero da valores estables para los mismos objetos
pub fn uuid_from_obj(obj: &impl std::fmt::Debug) -> Uuid {
    let h = format!("{:x}", md5::compute(format!("{:?}", obj).as_bytes()));
    Uuid::parse_str(&format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    ))
    .unwrap()
}

/// Calcula UUID a partir de cadena
///
/// Este no es un método muy robusto pero da valores estables para los mismos objetos
pub fn uuid_from_str(str: &str) -> Uuid {
    let h = format!("{:x}", md5::compute(str.as_bytes()));
    Uuid::parse_str(&format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    ))
    .unwrap()
}
