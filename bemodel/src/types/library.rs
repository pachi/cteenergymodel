// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Biblioteca de materiales y construcciones
//!
//! Relaciona un UUID de un tipo de material (material, glass, frame) o construcción (wallcons, wincons) con un grupo

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{ConsDb, MatsDb, Uuid};

/// Base de datos de materiales y construcciones
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Library {
    /// Materiales de opacos, vidrios y marcos
    #[serde(default)]
    pub mats: MatsDb,
    /// Construcciones de opacos y huecos
    #[serde(default)]
    pub cons: ConsDb,
    /// Grupos de materiales y construcciones
    #[serde(default)]
    pub groups: Groups,
}

/// Grupos de materiales de opacos, vidrios y marcos y construcciones de opacos y huecos
///
/// Asocia al UUID de cada tipo de objeto un nombre de grupo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Groups {
    /// Lista de materiales para elementos opacos (muro, cubierta, suelo, partición)
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub materials: BTreeMap<String, Vec<Uuid>>,
    /// Lista de vidrios
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub glasses: BTreeMap<String, Vec<Uuid>>,
    /// Lista de marcos
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub frames: BTreeMap<String, Vec<Uuid>>,
    /// Construcciones de opacos
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub wallcons: BTreeMap<String, Vec<Uuid>>,
    /// Construcciones de huecos
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub wincons: BTreeMap<String, Vec<Uuid>>,
}
