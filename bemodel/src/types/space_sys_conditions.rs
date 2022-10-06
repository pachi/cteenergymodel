// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Consignas de funcionamiento de los espacios

use serde::{Deserialize, Serialize};

use super::Uuid;

/// Consignas de temperatura de los espacios
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceSysConditions {
    /// Horarios anuales
    /// Id
    pub id: Uuid,
    /// Nombre del horario
    pub name: String,
    /// Horario anual de temperaturas de consigna de refrigeración, ºC
    /// Para oscilación libre se usa un valor elevado > 100ºC, como 999.00
    /// Si no se define se suponen todos los valores == 999.00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_max: Option<Uuid>,
    /// Horario anual de temperaturas de consigna de calefacción, ºC
    /// Para oscilación libre usar un valor bajo < -100ºC, como -999.00
    /// Si no se define se suponen todos los valores == -999.00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_min: Option<Uuid>,
}
