// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Funciones auxiliares para interpretación de XML

use anyhow::{format_err, Error};

/// Devuelve contenido de la etiqueta como texto
pub fn get_tag_text<'a>(parent: &'a roxmltree::Node, tag: &str) -> Option<&'a str> {
    parent
        .children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|e| e.text())
}

/// Devuelve contenido de la etiqueta como texto
pub fn get_tag_as_str<'a>(parent: &'a roxmltree::Node, tag: &str) -> &'a str {
    get_tag_text(parent, tag).unwrap_or("").trim()
}

/// Devuelve contenido de la etiqueta como f32
pub fn get_tag_as_f32(parent: &roxmltree::Node, tag: &str) -> Result<f32, Error> {
    get_tag_as_str(parent, tag)
        .parse::<f32>()
        .map_err(|_e| format_err!("Error al convertir número"))
}

/// Devuelve contenido de la etiqueta como f32
pub fn get_tag_as_f32_or_default(parent: &roxmltree::Node, tag: &str) -> f32 {
    get_tag_as_str(parent, tag)
        .parse::<f32>()
        .unwrap_or_default()
}

/// Devuelve contenido de la etiqueta como i32
pub fn get_tag_as_i32(parent: &roxmltree::Node, tag: &str) -> Result<i32, Error> {
    get_tag_as_str(parent, tag)
        .parse::<i32>()
        .map_err(|_e| format_err!("Error al convertir número"))
}

pub fn get_tag_as_u32_or(parent: &roxmltree::Node, tag: &str, default: u32) -> u32 {
    parent
        .children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|n| n.text())
        .map(|v| v.parse::<u32>().unwrap_or(1))
        .unwrap_or(default)
}
