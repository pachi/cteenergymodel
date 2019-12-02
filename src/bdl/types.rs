//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::HashMap;

use failure::bail;
use failure::Error;

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

    /// Devuelve valor
    pub fn get(&self, attr: &str) -> Result<BdlValue, Error> {
        self.0
            .get(attr)
            .map(|v| v.to_owned())
            .ok_or_else(|| format_err!("Atributo inexistente: {}", attr))
    }

    /// Devuelve valor como número
    pub fn get_f32(&self, attr: &str) -> Result<f32, Error> {
        self.0
            .get(attr)
            .and_then(|v| match v {
                BdlValue::Number(num) => Some(*num),
                _ => None,
            })
            .ok_or_else(|| format_err!("Atributo inexistente o con valor incorrecto: {}", attr))
    }
}


#[derive(Debug, Clone)]
pub enum BdlValue {
    String(String),
    Number(f32),
}

impl From<String> for BdlValue {
    fn from(val: String) -> Self {
        BdlValue::String(val.to_string())
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
