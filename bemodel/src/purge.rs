// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de funciones de limpieza del modelo

use std::collections::HashSet;

use super::{Model, Warning, WarningLevel};

impl Model {
    /// Limpia modelo de elementos no utilizados
    /// Elementos:
    /// - espacios sin opacos asignados
    /// - puentes térmicos con longitud nula
    /// Construcción:
    /// - construcciones de opacos sin opacos que las usen
    /// - construcciones de huecos sin huecos que las usen
    /// - materiales no usados en construcciones de opacos
    /// - vidrios no usados en construcciones de huecos
    /// - marcos no usados en construcciones de huecos
    /// Uso:
    /// - definiciones de cargas no usadas en los espacios
    /// - definiciones de consignas no usadas en los espacios
    /// - horarios no usados en definición de cargas o consignas
    pub fn purge_unused(&mut self) -> Vec<Warning> {
        let mut warnings = vec![];
        let start_n_spaces = self.spaces.len();
        let start_n_pts = self.thermal_bridges.len();
        let start_n_wallcons = self.cons.wallcons.len();
        let start_n_wincons = self.cons.wincons.len();
        let start_n_materials = self.cons.materials.len();
        let start_n_glasses = self.cons.glasses.len();
        let start_n_frames = self.cons.frames.len();
        let start_n_loads = self.loads.len();
        let start_n_sys_settings = self.sys_settings.len();
        let start_n_schedules_year = self.schedules.year.len();
        let start_n_schedules_week = self.schedules.week.len();
        let start_n_schedules_day = self.schedules.day.len();

        // Elementos
        self.purge_unused_spaces();
        self.purge_unused_pts();
        // Construcción
        self.purge_unused_wallcons();
        self.purge_unused_wincons();
        self.purge_unused_materials();
        self.purge_unused_glasses();
        self.purge_unused_frames();
        // Uso
        self.purge_unused_loads();
        self.purge_unused_sys_settings();
        self.purge_unused_schedules();

        warnings.push(Warning {
            level: WarningLevel::SUCCESS,
            id: None,
            msg: format!(
                "Eliminación de elementos no usados: {} espacios, \
                {} puentes térmicos, \
                {} construcciones de muro, \
                {} construcciones de hueco, \
                {} materiales, \
                {} vidrios, \
                {} marcos, \
                {} definiciones de cargas, \
                {} definiciones de consignas, \
                {} horarios anuales, \
                {} horarios semanales, \
                {} horarios diarios",
                start_n_spaces - self.spaces.len(),
                start_n_pts - self.thermal_bridges.len(),
                start_n_wallcons - self.cons.wallcons.len(),
                start_n_wincons - self.cons.wincons.len(),
                start_n_materials - self.cons.materials.len(),
                start_n_glasses - self.cons.glasses.len(),
                start_n_frames - self.cons.frames.len(),
                start_n_loads - self.loads.len(),
                start_n_sys_settings - self.sys_settings.len(),
                start_n_schedules_year - self.schedules.year.len(),
                start_n_schedules_week - self.schedules.week.len(),
                start_n_schedules_day - self.schedules.day.len(),
            ),
        });

        warnings
    }

    /// Elimina espacios no usados en los opacos
    pub fn purge_unused_spaces(&mut self) {
        let spaces_used_ids: HashSet<_> = self
            .walls
            .iter()
            .flat_map(|v| [Some(v.space), v.next_to])
            .flatten()
            .collect();
        self.spaces = self
            .spaces
            .iter()
            .cloned()
            .filter(|v| spaces_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina puentes térmicos con longitud nula
    pub fn purge_unused_pts(&mut self) {
        self.thermal_bridges = self
            .thermal_bridges
            .iter()
            .cloned()
            .filter(|v| v.l.abs() > f32::EPSILON)
            .collect();
    }

    /// Elimina construcciones de opacos no usadas en los opacos
    pub fn purge_unused_wallcons(&mut self) {
        let wallcons_used_ids: HashSet<_> = self.walls.iter().map(|v| v.cons).collect();
        self.cons.wallcons = self
            .cons
            .wallcons
            .iter()
            .cloned()
            .filter(|v| wallcons_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina construcciones de huecos no usadas en los huecos
    pub fn purge_unused_wincons(&mut self) {
        let wincons_used_ids: HashSet<_> = self.windows.iter().map(|v| v.cons).collect();
        self.cons.wincons = self
            .cons
            .wincons
            .iter()
            .cloned()
            .filter(|v| wincons_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina materiales no usados en las construcciones de opacos
    pub fn purge_unused_materials(&mut self) {
        let materials_used_ids: HashSet<_> = self
            .cons
            .wallcons
            .iter()
            .flat_map(|v| v.layers.iter().map(|layer| layer.material))
            .collect();
        self.cons.materials = self
            .cons
            .materials
            .iter()
            .cloned()
            .filter(|v| materials_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina vidrios no usados en las construcciones de huecos
    pub fn purge_unused_glasses(&mut self) {
        let glasses_used_ids: HashSet<_> = self.cons.wincons.iter().map(|v| v.glass).collect();
        self.cons.glasses = self
            .cons
            .glasses
            .iter()
            .cloned()
            .filter(|v| glasses_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina marcos no usados en las construcciones de huecos
    pub fn purge_unused_frames(&mut self) {
        let frames_used_ids: HashSet<_> = self.cons.wincons.iter().map(|v| v.frame).collect();
        self.cons.frames = self
            .cons
            .frames
            .iter()
            .cloned()
            .filter(|v| frames_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina definiciones de cargas no usadas en los espacios
    pub fn purge_unused_loads(&mut self) {
        let loads_used_ids: HashSet<_> = self.spaces.iter().flat_map(|v| v.loads).collect();
        self.loads = self
            .loads
            .iter()
            .cloned()
            .filter(|v| loads_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina definiciones de consignas no usadas en los espacios
    pub fn purge_unused_sys_settings(&mut self) {
        let sys_settings_used_ids: HashSet<_> =
            self.spaces.iter().flat_map(|v| v.sys_settings).collect();
        self.sys_settings = self
            .sys_settings
            .iter()
            .cloned()
            .filter(|v| sys_settings_used_ids.contains(&v.id))
            .collect();
    }

    /// Elimina definiciones de horarios no usadas en las definiciones de cargas o consignas
    pub fn purge_unused_schedules(&mut self) {
        // Eliminar perfiles no usados en cargas o consignas
        let loads_ids = self
            .loads
            .iter()
            .flat_map(|v| [v.people_schedule, v.equipment_schedule, v.lighting_schedule])
            .flatten();
        let sys_settings_ids = self
            .sys_settings
            .iter()
            .flat_map(|v| [v.temp_max, v.temp_min])
            .flatten();
        // Horarios anuales - elimina no usados
        let year_used_ids: HashSet<_> = loads_ids.chain(sys_settings_ids).collect();
        // Elimina horarios anuales no usados
        self.schedules.year = self
            .schedules
            .year
            .iter()
            .cloned()
            .filter(|v| year_used_ids.contains(&v.id))
            .collect();
        // Horarios semanales
        // Horarios semanales usados en horarios anuales
        let week_used_ids: HashSet<_> = self
            .schedules
            .year
            .iter()
            .flat_map(|v| v.values.iter().map(|e| e.0))
            .collect();
        // Filtrar no usados
        self.schedules.week = self
            .schedules
            .week
            .iter()
            .cloned()
            .filter(|v| week_used_ids.contains(&v.id))
            .collect();
        // Horarios diarios
        // Horarios diarios usados en horarios semanales
        let day_used_ids: HashSet<_> = self
            .schedules
            .week
            .iter()
            .flat_map(|v| v.values.iter().map(|e| e.0))
            .collect();
        // Filtrar no usados
        self.schedules.day = self
            .schedules
            .day
            .iter()
            .cloned()
            .filter(|v| day_used_ids.contains(&v.id))
            .collect();
    }
}
