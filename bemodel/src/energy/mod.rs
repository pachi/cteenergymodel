// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Información energética relativa al modelo
//!
//! Cálculo de propiedades e indicadores energéticos del modelo y sus elementos

mod aabb;
mod bvh;
mod geometry;
mod indicators;
mod occluder;
mod props;
mod radiation;
mod ray;
mod transmittance;

pub use aabb::AABB;
pub use bvh::{Bounded, Intersectable, BVH};
pub use indicators::EnergyIndicators;
pub use props::EnergyProps;
pub use radiation::{ray_dir_to_sun, QSolJulData};
pub use ray::Ray;

use crate::Model;

impl Model {
    /// Calcula indicadores energéticos
    pub fn energy_indicators(&self) -> EnergyIndicators {
        EnergyIndicators::compute(self)
    }

    /// Tasa global de ventilación del edificio (1/h)
    pub fn global_ventilation_rate(&self) -> f32 {
        use crate::{utils::fround2, SpaceType};

        // Calcula el volumen neto de los espacios habitables de la envolvente [m³]
        // Computa el volumen de todos los espacios (solo habitables) de la envolvente y
        // descuenta los volúmenes de forjados y cubiertas
        // Este método lo tenemos también en los indicadores pero lo necesitamos antes para la ventilación
        // de espacios no acondicionados (habitables), que usan la ventilación de diseño el edificio
        let vol_env_inh_net = fround2(
            self.spaces
                .iter()
                .filter_map(|s| {
                    if s.inside_tenv && s.kind != SpaceType::UNINHABITED {
                        Some(s.volume_net(&self.walls, &self.cons) * s.multiplier)
                    } else {
                        None
                    }
                })
                .sum(),
        );

        self.meta
            .global_ventilation_l_s
            .map(|n_v_g| 3.6 * n_v_g / vol_env_inh_net)
            .unwrap_or_default()
    }
}
