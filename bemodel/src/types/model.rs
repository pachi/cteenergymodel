// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo del edificio que comprende los elementos de la envolvente térmica, espacios, construcciones y metadatos

use std::collections::HashSet;

pub use nalgebra::{point, vector};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::{
    BoundaryType, ConsDb, Meta, PropsOverrides, SchedulesDb, Shade, Space, SpaceLoads,
    SpaceSysConditions, SpaceType, ThermalBridge, Tilt, Uuid, Wall, Warning, Window,
};

// ---------- Estructura general de datos --------------

/// Modelo del edificio
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Model {
    /// Metadatos
    pub meta: Meta,
    /// Espacios
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spaces: Vec<Space>,
    /// Opacos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub walls: Vec<Wall>,
    /// Huecos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub windows: Vec<Window>,
    /// Puentes térmicos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub thermal_bridges: Vec<ThermalBridge>,
    /// Sombras
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shades: Vec<Shade>,
    /// Construcciones
    #[serde(default, skip_serializing_if = "ConsDb::is_empty")]
    pub cons: ConsDb,
    /// Horarios
    #[serde(default, skip_serializing_if = "SchedulesDb::is_empty")]
    pub schedules: SchedulesDb,
    /// Definición de cargas de los espacios
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loads: Vec<SpaceLoads>,
    /// Definición de consignas de los espacios
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sys_settings: Vec<SpaceSysConditions>,
    /// Overrides de propiedades de elementos (opacos y huecos)
    #[serde(default, skip_serializing_if = "PropsOverrides::is_empty")]
    pub overrides: PropsOverrides,
    /// Avisos de consistencia o de conversión del modelo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Warning>,
    // XXX: Lista de elementos con diferencias con HULC, mientras no se pueda asegurar que el cálculo es correcto
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<ExtraData>>,
}

impl Model {
    // ---------------- Conversión hacia y desde JSON

    /// Devuelve el modelo en formato JSON
    pub fn as_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// Lee un modelo desde JSON
    pub fn from_json(data: &str) -> Result<Self, Error> {
        let model: Model = serde_json::from_str(data)?;
        Ok(model)
    }

    // ---------------- Aceso e identificación de elementos

    /// Localiza espacio
    pub fn get_space(&self, id: Uuid) -> Option<&Space> {
        self.spaces.iter().find(|s| s.id == id)
    }

    /// Localiza espacio por nombre
    pub fn get_space_by_name<'a>(&'a self, name: &'a str) -> Option<&'a Space> {
        self.spaces.iter().find(|s| s.name == name)
    }

    /// Localiza opaco
    pub fn get_wall(&self, id: Uuid) -> Option<&Wall> {
        self.walls.iter().find(|w| w.id == id)
    }

    /// Localiza opaco por nombre
    pub fn get_wall_by_name<'a>(&'a self, name: &'a str) -> Option<&'a Wall> {
        self.walls.iter().find(|w| w.name == name)
    }

    /// Localiza hueco
    pub fn get_window(&self, id: Uuid) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }

    /// Localiza hueco por nombre
    pub fn get_window_by_name<'a>(&'a self, name: &'a str) -> Option<&'a Window> {
        self.windows.iter().find(|w| w.name == name)
    }

    /// Limpia modelo de elementos no utilizados
    /// Elementos:
    /// - espacios sin opacos asignados
    /// - opacos sin espacio asignado
    /// - huecos sin opaco asignado
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
    pub fn purge_unused(&mut self) {
        // Elementos
        self.purge_unused_spaces();
        self.purge_unused_walls();
        self.purge_unused_windows();
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

    /// Elimina muros no usados en los espacios
    /// /// NOTE: esto en realidad es un chequeo de que las ids de espacio son correctas
    pub fn purge_unused_walls(&mut self) {
        let spaces_ids: HashSet<_> = self.spaces.iter().map(|v| v.id).collect();
        self.walls = self
            .walls
            .iter()
            .cloned()
            .filter(|v| spaces_ids.contains(&v.space))
            .collect();
    }

    /// Elimina huecos no usados en los opacos
    /// NOTE: esto en realidad es un chequeo de que las ids de muro son correctas
    pub fn purge_unused_windows(&mut self) {
        let walls_ids: HashSet<_> = self.walls.iter().map(|v| v.id).collect();
        self.windows = self
            .windows
            .iter()
            .cloned()
            .filter(|v| walls_ids.contains(&v.wall))
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

/// Datos adicionales para comprobación de muros
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtraData {
    // Nombre del opaco
    pub name: String,
    // Condiciones de contorno del opaco
    pub bounds: BoundaryType,
    // Tipo de espacio
    pub spacetype: SpaceType,
    // Espacio adyacente
    pub nextspace: Option<Uuid>,
    // Tipo de espacio adyacente
    pub nextspacetype: Option<SpaceType>,
    // Inclinación del opaco
    pub tilt: Tilt,
    // Construcción
    pub cons: Uuid,
    // U por defecto u obtenida de archivo KyGananciasSolares.txt
    pub u: f32,
    // U calculada con UNE-EN ISO 13789
    pub computed_u: f32,
}
