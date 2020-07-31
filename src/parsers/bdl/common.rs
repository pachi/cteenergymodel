// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::HashMap;

use anyhow::{bail, format_err, Error};

#[derive(Debug, Clone, Default)]
pub struct AttrMap(pub HashMap<String, BdlValue>);

impl AttrMap {
    /// Constructor
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inserta valor v en la clave k y devuelve el valor existente o None
    pub fn insert<K: ToString>(&mut self, k: K, v: &str) -> Option<BdlValue> {
        let val: BdlValue = match v.parse::<f32>() {
            Ok(num) => BdlValue::Number(num),
            _ => BdlValue::String(v.trim().to_string()),
        };
        self.0.insert(k.to_string(), val)
    }

    /// Devuelve valor como BdlValue
    pub fn get(&self, attr: &str) -> Result<BdlValue, Error> {
        self.0.get(attr).map(|v| v.to_owned()).ok_or_else(|| {
            format_err!(
                "Atributo '{}' no encontrado en el bloque '{:#?}'",
                attr,
                self
            )
        })
    }

    /// Devuelve valor como número
    pub fn get_f32(&self, attr: &str) -> Result<f32, Error> {
        self.0
            .get(attr)
            .and_then(|v| match v {
                BdlValue::Number(num) => Some(*num),
                _ => None,
            })
            .ok_or_else(|| {
                format_err!(
                    "Atributo '{}' no encontrado en el bloque '{:#?}'",
                    attr,
                    self
                )
            })
    }

    /// Devuelve valor como String
    pub fn get_str(&self, attr: &str) -> Result<String, Error> {
        self.0
            .get(attr)
            .and_then(|v| match v {
                BdlValue::String(string) => Some(string.to_string()),
                _ => None,
            })
            .ok_or_else(|| {
                format_err!(
                    "Atributo '{}' no encontrado en el bloque '{:#?}'",
                    attr,
                    self
                )
            })
    }

    /// Elimina un valor del diccionario y devuelve como BdlValue
    pub fn remove(&mut self, attr: &str) -> Result<BdlValue, Error> {
        self.0.remove(attr).ok_or_else(|| {
            format_err!(
                "Atributo '{}' no encontrado en el bloque '{:#?}'",
                attr,
                self
            )
        })
    }

    /// Elimina valor y devuelve como número
    pub fn remove_f32(&mut self, attr: &str) -> Result<f32, Error> {
        self.0
            .remove(attr)
            .and_then(|v| match v {
                BdlValue::Number(num) => Some(num),
                _ => None,
            })
            .ok_or_else(|| {
                format_err!(
                    "Atributo '{}' no encontrado en el bloque '{:#?}'",
                    attr,
                    self
                )
            })
    }

    /// Elimina valor y devuelve como String
    pub fn remove_str(&mut self, attr: &str) -> Result<String, Error> {
        self.0
            .remove(attr)
            .and_then(|v| match v {
                BdlValue::String(string) => Some(string),
                _ => None,
            })
            .ok_or_else(|| {
                format_err!(
                    "Atributo '{}' no encontrado en el bloque '{:#?}'",
                    attr,
                    self
                )
            })
    }
}

#[derive(Debug, Clone)]
pub enum BdlValue {
    String(String),
    Number(f32),
}

impl From<String> for BdlValue {
    fn from(val: String) -> Self {
        BdlValue::String(val)
    }
}

impl From<&str> for BdlValue {
    fn from(val: &str) -> Self {
        BdlValue::String(val.to_string())
    }
}

impl From<f32> for BdlValue {
    fn from(val: f32) -> Self {
        BdlValue::Number(val)
    }
}

impl std::fmt::Display for BdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            BdlValue::String(val) => write!(f, "{}", val),
            BdlValue::Number(val) => write!(f, "{}", val),
        }
    }
}

impl std::convert::TryFrom<BdlValue> for f32 {
    type Error = Error;

    fn try_from(value: BdlValue) -> Result<Self, Self::Error> {
        match value {
            BdlValue::Number(num) => Ok(num),
            _ => bail!("Valor numérico incorrecto: {:?}", value),
        }
    }
}

/// Interpreta lista de nombres con formato "("mat1", "mat2", "mat3", ...)"
pub fn extract_namesvec<S: AsRef<str>>(input: S) -> Vec<String> {
    input
        .as_ref()
        .trim_matches(&[' ', '(', ')'] as &[_])
        .split('"')
        .map(str::trim)
        .filter(|v| *v != "," && *v != "")
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
}

/// Interpreta lista de valores con formato "(num1, num2, num3, ...)"
pub fn extract_f32vec<S: AsRef<str> + std::fmt::Debug>(input: S) -> Result<Vec<f32>, Error> {
    input
        .as_ref()
        .trim_matches(&[' ', '(', ')'] as &[_])
        .split(',')
        .map(|v| {
            v.trim()
                .parse::<f32>()
                .map_err(|_| format_err!("Error al convertir {}", v))
        })
        .collect::<Result<Vec<f32>, _>>()
        .map_err(|_| format_err!("Error en la conversión numérica de {:?}", input))
}
