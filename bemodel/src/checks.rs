// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de una función de comprobación del modelo

use std::collections::HashSet;

use super::{Model, Uuid, Warning, WarningLevel};

impl Model {
    /// Comprueba consistencia del modelo y devuelve lista de avisos / errores detectados
    ///
    /// 1. Elementos mal definidos que se ignorarán en el cálculo:
    ///     - Huecos sin referencias de construcciones válidas
    ///     - Huecos sin referencias de muros válidas
    ///     - Muros sin referencias de espacios válidas
    ///     - Muros sin referencias de construcciones válidas
    ///     - Muros con nextto con referencia no válida
    /// TODO: Comprobaciones pendientes
    ///     - comprobar que elementos geométricos tengan punto de inserción != None
    ///     - la superficie de elemento (wall.a) coincide con la de su polígono
    pub fn check(&self) -> Vec<Warning> {
        use WarningLevel::WARNING;

        let spaceids: HashSet<Uuid> = self.spaces.iter().map(|s| s.id).collect();
        let wallids: HashSet<Uuid> = self.walls.iter().map(|w| w.id).collect();
        let wallconsids: HashSet<Uuid> = self.cons.wallcons.iter().map(|c| c.id).collect();
        let winconsids: HashSet<Uuid> = self.cons.wincons.iter().map(|c| c.id).collect();

        let mut warnings = Vec::new();

        // Muros con referencias e espacios, construcciones o nextto incorrectas
        self.walls.iter().for_each(|w| {
            if !spaceids.contains(&w.space) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id),
                    msg: format!(
                        "Muro {} ({}) con referencia incorrecta de espacio {}",
                        w.id, w.name, w.space
                    ),
                })
            };
            if !wallconsids.contains(&w.cons) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id),
                    msg: format!(
                        "Muro {} ({}) con referencia incorrecta de construcción {}",
                        w.id, w.name, w.cons
                    ),
                })
            };
            if w.next_to.is_some() && !spaceids.contains(&w.next_to.unwrap()) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id),
                    msg: format!(
                        "Muro {} ({}) con referencia incorrecta de espacio adyacente {}",
                        w.id,
                        w.name,
                        w.next_to.unwrap()
                    ),
                })
            };
            // TODO: avisar con elemento horizontal en contacto con el terreno y con p_ext == 0
        });

        // Huecos con referencias de muros o construcciones incorrectas
        self.windows.iter().for_each(|w| {
            if !wallids.contains(&w.wall) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id),
                    msg: format!(
                        "Hueco {} ({}) con referencia incorrecta de opaco {}",
                        w.id, w.name, w.wall
                    ),
                })
            };
            if !winconsids.contains(&w.cons) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id),
                    msg: format!(
                        "Hueco {} ({}) con referencia incorrecta de construcción {}",
                        w.id, w.name, w.cons
                    ),
                })
            };
        });
        // Huecos con referencias de muros o construcciones incorrectas
        self.thermal_bridges.iter().for_each(|tb| {
            if tb.l < 0.0 {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(tb.id),
                    msg: format!(
                        "Puente térmico {} ({}) con longitud negativa ({}).",
                        tb.id, tb.name, tb.l
                    ),
                })
            };
        });

        warnings
    }
}
