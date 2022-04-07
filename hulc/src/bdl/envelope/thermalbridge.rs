// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Puentes térmicos (THERMAL-BRIDGE)

use std::convert::TryFrom;

use anyhow::{bail, Error};

use crate::bdl::{extract_f32vec, extract_namesvec, BdlBlock};

/// Puente térmico (THERMAL-BRIDGE)
#[derive(Debug, Clone, Default)]
pub struct ThermalBridge {
    /// Nombre
    pub name: String,
    /// Longitud total (m)
    /// En LIDER antiguo no se guarda la medición en el objeto
    pub length: Option<f32>,
    /// Tipo de puente térmico:
    /// - PILLAR: pilar en fachada,
    /// - WINDOW-FRAME: borde de hueco,
    /// - SLAB: Forjado con cubierta o con suelo en contacto con el aire (anglemin, anglemax, partition)
    /// - MASONRY: Encuentros entre muros (anglemin, anglemax, partition)
    /// - UNDER-EXT: Solera con pared exterior (anglemin, anglemax, partition)
    pub tbtype: String,
    /// Transmitancia térmica W/mK
    pub psi: f32,
    /// Fractor de resistencia superficial frsi (condensaciones)
    pub frsi: f32,
    /// Propiedades geométricas de los encuentros (anglemin, anglemax, partition)
    pub geometry: Option<TbGeometry>,
    /// Datos para definición por catálogo (tipo 3)
    pub catalog: Option<TbByCatalog>,
}

/// Definición por usuario (definition 2)
#[derive(Debug, Clone, Default)]
pub struct TbGeometry {
    /// Tipo de encuentro entre elementos:
    /// - YES -> frente de forjado
    /// - BOTH -> encuentros entre dos particiones exteriores
    pub partition: String,
    /// Ángulo mínimo (grados sexagesimales)
    pub anglemin: f32,
    /// Ángulo máximo (grados sexagesimales)
    pub anglemax: f32,
}

/// Definición por catálogo (definition 3)
#[derive(Debug, Clone, Default)]
pub struct TbByCatalog {
    /// Lista de tipos
    pub classes: Vec<String>,
    /// Lista de porcentajes de la longitud total
    pub pcts: Vec<f32>,
    /// Lista de transmitancias del primer elemento del encuentro (muro) W/m2k
    pub firstelems: Vec<f32>,
    /// Lista de transmitancias del segundo elemento del encuentro (muro) W/m2k
    pub secondelems: Option<Vec<f32>>,
}

impl TryFrom<BdlBlock> for ThermalBridge {
    type Error = Error;

    /// Conversión de bloque BDL a puente térmico (THERMAL-BRIDGE)
    ///
    /// Se pueden de definir (DEFINICION) por defecto (1), por usuario (2) o por catálogo (3?)
    ///
    /// Ejemplo:
    /// ```text
    ///      "LONGITUDES_CALCULADAS" = THERMAL-BRIDGE
    ///            LONG-TOTAL = 0.000000
    ///            DEFINICION = 1
    ///          ..
    ///      "FRENTE_FORJADO" = THERMAL-BRIDGE
    ///            LONG-TOTAL = 171.629913
    ///            DEFINICION = 2
    ///            TTL    = 0.080000
    ///            FRSI        = 0.45
    ///            ANGLE-MIN   = 135
    ///            ANGLE-MAX   = 225
    ///            TYPE        = SLAB
    ///            PARTITION   = YES
    ///          ..
    ///     "UNION_CUBIERTA" = THERMAL-BRIDGE
    ///         LONG-TOTAL = 148.341034
    ///         DEFINICION = 3
    ///         TTL    = 0.226667
    ///         LISTA-N   = ( "Cubiertas planas - Forjado no interrumpe el aislamiento en fachada")
    ///         LISTA-L   = ( 100)
    ///         LISTA-MURO   = ( 0.230000)
    ///         LISTA-MARCO   = ( 0.200000)
    ///         FRSI        = 0.28
    ///         ANGLE-MIN   = 0
    ///         ANGLE-MAX   = 135
    ///         TYPE        = SLAB
    ///         PARTITION   = BOTH
    ///         ..
    /// ```
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let length = attrs.remove_f32("LONG-TOTAL").ok();
        let (psi, frsi) = if name == "LONGITUDES_CALCULADAS" {
            (0.0, 0.0)
        } else {
            (attrs.remove_f32("TTL")?, attrs.remove_f32("FRSI")?)
        };
        let tbtype = attrs.remove_str("TYPE").ok().unwrap_or_default();
        let geometry = match tbtype.as_str() {
            "WINDOW-FRAME" | "PILLAR" | "" => None,
            _ => Some(TbGeometry {
                anglemin: attrs.remove_f32("ANGLE-MIN")?,
                anglemax: attrs.remove_f32("ANGLE-MAX")?,
                partition: attrs.remove_str("PARTITION")?,
            }),
        };
        let defn = attrs.remove_f32("DEFINICION").map(|v| v as i32).ok(); // El LIDER antiguo no usa la definición del tipo
        let catalog = match defn {
            // Definido con valor por defecto o por el usuario
            Some(1) | Some(2) | None => None,
            // Definido por catálogo de PTs
            Some(3) => Some(TbByCatalog {
                classes: extract_namesvec(attrs.remove_str("LISTA-N")?),
                pcts: attrs
                    .remove_str("LISTA-L")
                    .and_then(extract_f32vec)
                    .unwrap_or_default(),
                firstelems: attrs
                    .remove_str("LISTA-MURO")
                    .and_then(extract_f32vec)
                    .unwrap_or_default(),
                secondelems: if let Ok(list) = attrs.remove_str("LISTA-MARCO") {
                    Some(extract_f32vec(list)?)
                } else {
                    None
                },
            }),
            Some(v) => bail!("Puente térmico '{}' con tipo desconocido ({})", name, v),
        };
        Ok(Self {
            name,
            length,
            tbtype,
            psi,
            frsi,
            geometry,
            catalog,
        })
    }
}
