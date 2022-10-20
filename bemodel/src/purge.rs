// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de funciones de limpieza del modelo

use std::collections::HashSet;

use super::{Model, Warning, WarningLevel};

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
pub fn purge_unused(model: &mut Model) -> Vec<Warning> {
    let mut warnings = vec![];
    let start_n_spaces = model.spaces.len();
    let start_n_pts = model.thermal_bridges.len();
    let start_n_wallcons = model.cons.wallcons.len();
    let start_n_wincons = model.cons.wincons.len();
    let start_n_materials = model.cons.materials.len();
    let start_n_glasses = model.cons.glasses.len();
    let start_n_frames = model.cons.frames.len();
    let start_n_loads = model.loads.len();
    let start_n_sys_settings = model.sys_settings.len();
    let start_n_schedules_year = model.schedules.year.len();
    let start_n_schedules_week = model.schedules.week.len();
    let start_n_schedules_day = model.schedules.day.len();

    // Elementos
    purge_unused_spaces(model);
    purge_unused_pts(model);
    // Construcción
    purge_unused_wallcons(model);
    purge_unused_wincons(model);
    purge_unused_materials(model);
    purge_unused_glasses(model);
    purge_unused_frames(model);
    // Uso
    purge_unused_loads(model);
    purge_unused_sys_settings(model);
    purge_unused_schedules(model);

    warnings.push(Warning {
        level: WarningLevel::INFO,
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
            start_n_spaces - model.spaces.len(),
            start_n_pts - model.thermal_bridges.len(),
            start_n_wallcons - model.cons.wallcons.len(),
            start_n_wincons - model.cons.wincons.len(),
            start_n_materials - model.cons.materials.len(),
            start_n_glasses - model.cons.glasses.len(),
            start_n_frames - model.cons.frames.len(),
            start_n_loads - model.loads.len(),
            start_n_sys_settings - model.sys_settings.len(),
            start_n_schedules_year - model.schedules.year.len(),
            start_n_schedules_week - model.schedules.week.len(),
            start_n_schedules_day - model.schedules.day.len(),
        ),
    });

    warnings
}

/// Elimina espacios no usados en los opacos
pub(crate) fn purge_unused_spaces(model: &mut Model) {
    let spaces_used_ids: HashSet<_> = model
        .walls
        .iter()
        .flat_map(|v| [Some(v.space), v.next_to])
        .flatten()
        .collect();
    model.spaces = model
        .spaces
        .iter()
        .cloned()
        .filter(|v| spaces_used_ids.contains(&v.id))
        .collect();
}

/// Elimina puentes térmicos con longitud nula
pub(crate) fn purge_unused_pts(model: &mut Model) {
    model.thermal_bridges = model
        .thermal_bridges
        .iter()
        .cloned()
        .filter(|v| v.l.abs() > f32::EPSILON)
        .collect();
}

/// Elimina construcciones de opacos no usadas en los opacos
pub(crate) fn purge_unused_wallcons(model: &mut Model) {
    let wallcons_used_ids: HashSet<_> = model.walls.iter().map(|v| v.cons).collect();
    model.cons.wallcons = model
        .cons
        .wallcons
        .iter()
        .cloned()
        .filter(|v| wallcons_used_ids.contains(&v.id))
        .collect();
}

/// Elimina construcciones de huecos no usadas en los huecos
pub(crate) fn purge_unused_wincons(model: &mut Model) {
    let wincons_used_ids: HashSet<_> = model.windows.iter().map(|v| v.cons).collect();
    model.cons.wincons = model
        .cons
        .wincons
        .iter()
        .cloned()
        .filter(|v| wincons_used_ids.contains(&v.id))
        .collect();
}

/// Elimina materiales no usados en las construcciones de opacos
pub(crate) fn purge_unused_materials(model: &mut Model) {
    let materials_used_ids: HashSet<_> = model
        .cons
        .wallcons
        .iter()
        .flat_map(|v| v.layers.iter().map(|layer| layer.material))
        .collect();
    model.cons.materials = model
        .cons
        .materials
        .iter()
        .cloned()
        .filter(|v| materials_used_ids.contains(&v.id))
        .collect();
}

/// Elimina vidrios no usados en las construcciones de huecos
pub(crate) fn purge_unused_glasses(model: &mut Model) {
    let glasses_used_ids: HashSet<_> = model.cons.wincons.iter().map(|v| v.glass).collect();
    model.cons.glasses = model
        .cons
        .glasses
        .iter()
        .cloned()
        .filter(|v| glasses_used_ids.contains(&v.id))
        .collect();
}

/// Elimina marcos no usados en las construcciones de huecos
pub(crate) fn purge_unused_frames(model: &mut Model) {
    let frames_used_ids: HashSet<_> = model.cons.wincons.iter().map(|v| v.frame).collect();
    model.cons.frames = model
        .cons
        .frames
        .iter()
        .cloned()
        .filter(|v| frames_used_ids.contains(&v.id))
        .collect();
}

/// Elimina definiciones de cargas no usadas en los espacios
pub(crate) fn purge_unused_loads(model: &mut Model) {
    let loads_used_ids: HashSet<_> = model.spaces.iter().flat_map(|v| v.loads).collect();
    model.loads = model
        .loads
        .iter()
        .cloned()
        .filter(|v| loads_used_ids.contains(&v.id))
        .collect();
}

/// Elimina definiciones de consignas no usadas en los espacios
pub(crate) fn purge_unused_sys_settings(model: &mut Model) {
    let sys_settings_used_ids: HashSet<_> =
        model.spaces.iter().flat_map(|v| v.sys_settings).collect();
    model.sys_settings = model
        .sys_settings
        .iter()
        .cloned()
        .filter(|v| sys_settings_used_ids.contains(&v.id))
        .collect();
}

/// Elimina definiciones de horarios no usadas en las definiciones de cargas o consignas
pub(crate) fn purge_unused_schedules(model: &mut Model) {
    // Eliminar perfiles no usados en cargas o consignas
    let loads_ids = model
        .loads
        .iter()
        .flat_map(|v| [v.people_schedule, v.equipment_schedule, v.lighting_schedule])
        .flatten();
    let sys_settings_ids = model
        .sys_settings
        .iter()
        .flat_map(|v| [v.temp_max, v.temp_min])
        .flatten();
    // Horarios anuales - elimina no usados
    let year_used_ids: HashSet<_> = loads_ids.chain(sys_settings_ids).collect();
    // Elimina horarios anuales no usados
    model.schedules.year = model
        .schedules
        .year
        .iter()
        .cloned()
        .filter(|v| year_used_ids.contains(&v.id))
        .collect();
    // Horarios semanales
    // Horarios semanales usados en horarios anuales
    let week_used_ids: HashSet<_> = model
        .schedules
        .year
        .iter()
        .flat_map(|v| v.values.iter().map(|e| e.0))
        .collect();
    // Filtrar no usados
    model.schedules.week = model
        .schedules
        .week
        .iter()
        .cloned()
        .filter(|v| week_used_ids.contains(&v.id))
        .collect();
    // Horarios diarios
    // Horarios diarios usados en horarios semanales
    let day_used_ids: HashSet<_> = model
        .schedules
        .week
        .iter()
        .flat_map(|v| v.values.iter().map(|e| e.0))
        .collect();
    // Filtrar no usados
    model.schedules.day = model
        .schedules
        .day
        .iter()
        .cloned()
        .filter(|v| day_used_ids.contains(&v.id))
        .collect();
}
