// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Utilidades varias

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

/// Convierte el azimuth desde el criterio del BDL al criterio de la 52016-1
/// BDL: Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del muro (N=0, E=+90, W=-90)
/// UNE-EN ISO 52016-1: S=0, E=+90, W=-90
pub fn orientation_bdl_to_52016(azimuth: f32) -> f32 {
    normalize(180.0 - azimuth, -180.0, 180.0)
}

/// Calcula UUID a partir de hash MD5 del objeto
///
/// Este no es un método muy robusto pero da valores estables para los mismos objetos
pub fn uuid_from_obj(obj: &impl std::fmt::Debug) -> String {
    let h = format!("{:x}", md5::compute(format!("{:?}", obj).as_bytes()));
    format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    )
}
