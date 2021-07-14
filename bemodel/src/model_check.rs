// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de una función de comprobación del modelo

use std::collections::HashSet;

use super::{Model, Warning, WarningLevel};

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
    pub fn check_model(&self) -> Vec<Warning> {
        use WarningLevel::WARNING;

        let spaceids: HashSet<&str> = self.spaces.iter().map(|s| s.id.as_str()).collect();
        let wallids: HashSet<&str> = self.walls.iter().map(|w| w.id.as_str()).collect();
        let wallconsids: HashSet<&str> = self.wallcons.iter().map(|c| c.id.as_str()).collect();
        let winconsids: HashSet<&str> = self.wincons.iter().map(|c| c.id.as_str()).collect();

        let mut warnings = Vec::new();

        // Muros con referencias e espacios, construcciones o nextto incorrectas
        self.walls.iter().for_each(|w| {
            if !spaceids.contains(w.space.as_str()) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id.clone()),
                    msg: format!(
                        "Muro {} ({}) con referencia incorrecta de espacio {}",
                        w.id, w.name, w.space
                    ),
                })
            };
            if !wallconsids.contains(w.cons.as_str()) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id.clone()),
                    msg: format!(
                        "Muro {} ({}) con referencia incorrecta de construcción {}",
                        w.id, w.name, w.cons
                    ),
                })
            };
            if w.nextto.is_some() && !spaceids.contains(w.nextto.clone().unwrap().as_str()) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id.clone()),
                    msg: format!(
                        "Muro {} ({}) con referencia incorrecta de espacio adyacente {}",
                        w.id,
                        w.name,
                        w.nextto.clone().unwrap()
                    ),
                })
            };
            // TODO: avisar con elemento horizontal en contacto con el terreno y con p_ext == 0
        });

        // Huecos con referencias de muros o construcciones incorrectas
        self.windows.iter().for_each(|w| {
            if !wallids.contains(w.wall.as_str()) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id.clone()),
                    msg: format!(
                        "Hueco {} ({}) con referencia incorrecta de opaco {}",
                        w.id, w.name, w.wall
                    ),
                })
            };
            if !winconsids.contains(w.cons.as_str()) {
                warnings.push(Warning {
                    level: WARNING,
                    id: Some(w.id.clone()),
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
                    id: Some(tb.id.clone()),
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
