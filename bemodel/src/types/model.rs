// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo del edificio que comprende los elementos de la envolvente térmica, espacios, construcciones y metadatos

pub use nalgebra::{point, vector};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::{
    BoundaryType, ConsDb, MatsDb, Meta, PropsOverrides, Shade, Space, SpaceType, ThermalBridge,
    Tilt, Uuid, Wall, Window,
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
    /// Materiales
    #[serde(default, skip_serializing_if = "MatsDb::is_empty")]
    pub mats: MatsDb,
    /// Overrides de propiedades de elementos (opacos y huecos)
    #[serde(default, skip_serializing_if = "PropsOverrides::is_empty")]
    pub overrides: PropsOverrides,
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

    /// Iterador de los huecos de la envolvente térmica en contacto con el aire exterior
    /// Se excluyen los huecos sin espacio definido
    /// TODO: podríamos llevar esta lógica a ElementProps y allí dejar esto calculado
    pub fn exterior_windows_of_envelope_iter(&self) -> impl Iterator<Item = &Window> {
        self.walls
            .iter()
            .filter(|w| w.bounds == BoundaryType::EXTERIOR)
            .filter(move |w| {
                self.get_space(w.space)
                    .map(|s| s.inside_tenv)
                    .unwrap_or(false)
            })
            .flat_map(move |wall| self.windows.iter().filter(move |w| w.wall == wall.id))
    }

    // ---------------- Cálculos geométricos generales

    /// Genera todas las sombras de retranqueo de los huecos del modelo
    pub fn windows_setback_shades(&self) -> Vec<(Uuid, Shade)> {
        self.windows
            .iter()
            .filter_map(|window| {
                self.get_wall(window.wall)
                    .map(|wall| window.shades_for_setback(&wall.geometry))
            })
            .flatten()
            .flatten()
            .collect()
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
