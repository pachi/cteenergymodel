// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

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


// Utilidades para serialización y deserialización de datos ---------------------

/// Comprueba que el valor coincide con su valor por defecto
/// Útil para evitar serialización de algunos valores
pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}


/// Devuelve el default de multiplicador
pub(crate) fn default_1() -> f32 {
    1.0
}

/// Comprueba si el multiplicador es 1.0
pub(crate) fn multiplier_is_1(m: &f32) -> bool {
    *m == 1.0
}

/// Comprueba si el valor es true
pub(crate) fn is_true(b: &bool) -> bool {
    *b
}

/// Devuelve como default true
pub(crate) fn default_true() -> bool {
    true
}
