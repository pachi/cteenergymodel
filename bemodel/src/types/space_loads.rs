// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Condiciones de carga de los espacios (ocupación, equipos, iluminación)

use serde::{Deserialize, Serialize};

use super::Uuid;

/// Cargas de los espacios por ocupación, equipos e iluminación
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceLoads {
    /// Horarios anuales
    /// Id
    pub id: Uuid,
    /// Nombre del horario
    pub name: String,
    /// Horario anual de fracciones de carga de ocupación
    /// Si no se define se supone que no existe ocupación (carga = 0)
    pub people_schedule: Option<Uuid>,
    /// Carga máxima sensible de ocupación, W/m²
    pub people_sensible: f32,
    /// Carga máxima latente de ocupación, W/m²
    pub people_latent: f32,
    /// Carga total debida a los equipos, W/m²
    pub equipment: f32,
    /// Horario anual de fracciones de carga de equipos
    /// Si no se define se suponen todos los valores == 0
    pub equipment_schedule: Option<Uuid>,
    /// Carga total debida a la iluminación, W/m²
    pub lighting: f32,
    /// Horario anual de fracciones de carga de iluminación
    /// Si no se define se suponen todos los valores == 0
    pub lighting_schedule: Option<Uuid>,
    /// Iluminancia media en el plano de trabajo, lux
    pub illuminance: Option<f32>,
    /// Superficie por ocupante, m²/pers
    pub area_per_person: f32
    // TODO: nivel de infiltraciones?
}
