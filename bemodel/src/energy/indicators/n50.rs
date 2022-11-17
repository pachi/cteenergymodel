// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación del cálculo de la tasa de renovación de aire a 50 Pa del edificio, según CTE DB-HE 2019

use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::{energy::EnergyProps, BoundaryType};

/// Reporte de cálculo de n50 con valores de referencia (teóricos) y de ensayo (si está disponible)
/// El valor teórico usa las permeabilidades del CTE DB-HE 2019
/// Cuando se dispone de valor de ensayo n50 se utiliza para calcular la permeabilidad de opacos
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct N50Data {
    /// Relación de cambio de aire a 50 Pa (n50) calculado con valor de ensayo, si está disponible, o valor de referencia [1/h]
    pub n50: f32,
    /// Relación de cambio de aire a 50 Pa (n50) calculado con la permeabilidad de opacos de referencia [1/h]
    pub n50_ref: f32,
    /// Superficie (A_o) de los opacos que participan en la n_50 [m²]
    pub walls_a: f32,
    /// Permeabilidad de referencia (C_o) de los opacos [m³/h·m²]
    pub walls_c_ref: f32,
    /// C_o_ref · A_o de los opacos con valor de permeabilidad de referencia [m³/h]
    pub walls_c_a_ref: f32,
    /// Permeabilidad (C_o) de los opacos que participan en la n_50 obtenida de ensayo, si está disponible, o igual al de referencia [m³/h·m²]
    pub walls_c: f32,
    /// C_o · A_o de los opacos que participan en la n_50, usando valor de ensayo, si está disponible, o de referencia [m³/h]
    pub walls_c_a: f32,
    /// Superficie (A_h) de los huecos que participan en la n_50 [m²]
    pub windows_a: f32,
    /// Permeabilidad (C_h) media de los huecos que participan en la n_50 [m³/h·m²]
    pub windows_c: f32,
    /// C_h · A_h de los huecos que participan en la n_50 [m³/h]
    pub windows_c_a: f32,
    /// Volumen interior de los espacios interiores a la envolvente térmica [m³]
    pub vol: f32,
}

impl From<&EnergyProps> for N50Data {
    /// Calcula la tasa teórica de intercambio de aire a 50Pa según DB-HE2019 (1/h)
    /// Se considera:
    /// - las superficies opacos en contacto con el aire exterior
    /// - las permeabilidad al aire de opacos en función de si es nuevo (o permeab. mejorada) o existente
    /// - los huecos de las superficies opacas anteriores
    /// - la permeabilidad al aire de huecos definida en su construcción
    /// - el volumen interior de la envolvente térmica ()
    /// Los huecos sin construcción definida se consideran con la permeabilidad al aire
    /// de huecos poco estancos, C_100=100 m³/h·m² (clase 0)
    fn from(props: &EnergyProps) -> Self {
        let mut data = N50Data {
            vol: props.global.vol_env_net,
            ..Default::default()
        };

        props
            .walls
            .iter()
            .filter(|(_, w)| w.is_tenv && w.bounds == BoundaryType::EXTERIOR)
            .for_each(|(wall_id, wall)| {
                let multiplier = wall.multiplier;
                let mut win_ah = 0.0;
                let mut win_ah_ch = 0.0;
                for (win_id, win) in props.windows.iter().filter(|(_, win)| &win.wall == wall_id) {
                    let win_c_100 = if let Some(win_c_100) = props.wincons.get(&win.cons).map(|wc| wc.c_100) {
                        win_c_100
                    } else {
                        warn!("No se ha podido calcular el valor C_100 del hueco {}. Se usará el valor por defecto C_100 = 100 m³/h·m² en el cálculo de n_50", win_id);
                        100.0
                    };
                    win_ah += win.area;
                    win_ah_ch += win.area * win_c_100;
                }
                data.walls_a += wall.area_net * multiplier;
                data.windows_a += win_ah * multiplier;
                data.windows_c_a += win_ah_ch * multiplier;
            });

        // Promedio de permeabilidad de huecos
        if data.windows_a > 0.001 {
            data.windows_c = data.windows_c_a / data.windows_a;
        };

        // Manejo de los opacos según disponibilidad de ensayo
        // Permeabilidad de opacos calculada según criterio de edad por defecto DB-HE2019 (1/h)
        // NOTE: usamos is_new_building pero igual merecería la pena una variable para permeabilidad mejorada
        data.walls_c_ref = props.global.c_o_100;
        data.walls_c_a_ref = data.walls_a * data.walls_c_ref;

        if data.vol > 0.001 {
            // 0.629 = (50/100)^0.67 -> factor de cambio de presiones
            data.n50_ref = 0.629 * (data.walls_c_a_ref + data.windows_c_a) / data.vol;
        };

        // Si hay valor de ensayo se usa, si no usamos valor de referencia
        if let Some(n50test) = props.global.n_50_test_ach {
            data.n50 = n50test;
            if data.walls_a > 0.001 {
                data.walls_c = ((n50test * data.vol) / 0.629 - data.windows_c_a) / data.walls_a;
                data.walls_c_a = data.walls_a * data.walls_c;
            } else {
                data.walls_c = data.walls_c_ref;
                data.walls_c_a = data.walls_c_a_ref;
            }
        } else {
            data.n50 = data.n50_ref;
            data.walls_c = data.walls_c_ref;
            data.walls_c_a = data.walls_c_a_ref;
        };

        info!(
            "n_50={:.2} 1/h, n_50_ref={:.2} 1/h, A_o={:.2} m², C_o={:.2} m³/h·m², Σ(A_o.C_o)={:.2} m³/h, C_o_ref={:.2} m³/h·m², Σ(A_o.C_o_ref)={:.2} m³/h, A_h={:.2} m², C_h={:.2} m³/h·m², Σ(A_h.C_h)={:.2} m³/h, vol={:.2} m³",
            data.n50, data.n50_ref, data.walls_a, data.walls_c, data.walls_c_a, data.walls_c_ref, data.walls_c_a_ref, data.windows_a, data.windows_c, data.windows_c_a, data.vol
        );
        data
    }
}
