// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Modelo del edificio que comprende los elementos de la envolvente térmica, espacios, construcciones y metadatos

use anyhow::Error;
use serde::{Deserialize, Serialize};

pub use super::{
    BoundaryType, ClimateZone, KDetail, N50HEDetail, Orientation, SpaceType, Tilt, UValues,
    Warning, WarningLevel,
};

// ---------- Estructura general de datos --------------

/// Modelo del edificio
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Model {
    /// Metadatos
    pub meta: Meta,
    /// Espacios
    pub spaces: Vec<Space>,
    /// Opacos
    pub walls: Vec<Wall>,
    /// Huecos
    pub windows: Vec<Window>,
    /// Puentes térmicos
    pub thermal_bridges: Vec<ThermalBridge>,
    /// Construcciones de opacos
    pub wallcons: Vec<WallCons>,
    /// Construcciones de huecos
    pub wincons: Vec<WindowCons>,
    // XXX: Lista de elementos con diferencias con HULC, mientras no se pueda asegurar que el cálculo es correcto
    pub extra: Option<Vec<ExtraData>>,
}

impl Model {
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
}

/// Metadatos del edificio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// Nombre del proyecto
    pub name: String,
    /// ¿Edificio nuevo?
    pub is_new_building: bool,
    /// ¿Es uso residencial?
    pub is_dwelling: bool,
    /// Número de viviendas
    pub num_dwellings: i32,
    /// Zona climática
    pub climate: ClimateZone,
    /// Ventilación global del edificio, para los espacios habitables de uso residencial, en l/s
    /// Las zonas no habitables y todas las zonas de uso terciario tienen definida su tasa
    /// de ventilación definida (en renh)
    pub global_ventilation_l_s: Option<f32>,
    /// n50 medido mediante ensayo [renh]
    pub n50_test_ach: Option<f32>,
    /// Anchura o profundidad del aislamiento perimetral horizontal o vertical de la solera [m]
    pub d_perim_insulation: f32,
    /// Resistencia térmica del aislamiento perimetral horizontal o vertical de la solera [m2K/W]
    pub rn_perim_insulation: f32,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            name: "Nombre del proyecto".to_string(),
            is_new_building: true,
            is_dwelling: true,
            num_dwellings: 1,
            climate: ClimateZone::D3,
            global_ventilation_l_s: None,
            n50_test_ach: None,
            d_perim_insulation: 0.0,
            rn_perim_insulation: 0.0,
        }
    }
}

// Elementos -----------------------------------------------

/// Espacio
/// XXX: en teoría se podrían asignar los espacios a zonas térmicas, aunque simplificadamente
/// XXX: consideraremos que cada espacio se corresponde con una ZT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre del espacio
    pub name: String,
    /// Superficie útil del espacio (m2)
    pub area: f32,
    /// Multiplicador del espacio
    pub multiplier: f32,
    /// Tipo de espacio:
    /// - CONDITIONED: acondicionado,
    /// - UNCONDITIONED: no acondicionado
    /// - UNINHABITED: no habitable
    #[serde(rename = "type")]
    pub space_type: SpaceType,
    /// Pertenencia al interior de la envolvente térmica
    pub inside_tenv: bool,
    /// Altura bruta (suelo a suelo) del espacio (m)
    pub height: f32,
    /// Ventilación, en ren/h
    pub n_v: Option<f32>,
    /// Cota del espacio respecto al suelo (m)
    pub z: f32,
    /// Perímetro expuesto del espacio (suelos) (m)
    /// Incluye la parte del perímetro que separa el espacio del exterior
    /// y excluye que lo separa de otros espacios acondicionados.
    pub exposed_perimeter: Option<f32>,
}

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wall {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre del elemento opaco
    pub name: String,
    /// Superficie neta (sin huecos) del elemento opaco (m2)
    #[serde(rename = "A")]
    pub area: f32,
    /// Condiciones de contorno del cerramiento:
    /// - GROUND: cerramientos en contacto con el terreno
    /// - EXTERIOR: cerramientos en contacto con el aire exterior
    /// - INTERIOR: cerramientos en contacto con el aire de otros espacios
    /// - ADIABATIC: cerramientos sin transmisión de calor
    pub bounds: BoundaryType,
    /// Construcción del opaco
    pub cons: String,
    /// Espacio al que pertenece el elemento opaco
    pub space: String,
    /// Espacio adyacente con el que comunica el elemento opaco cuando es interior
    pub nextto: Option<String>,
    /// Orientación (gamma) [-180,+180] (S=0, E=+90, W=-90)
    /// Medido como azimuth geográfico de la proyección horizontal de la normal a la superficie
    /// Coincide con el criterio de la UNE-EN ISO 52016-1
    /// Difiere del criterio BDL, que parte del norte, con E+ y W-
    pub azimuth: f32,
    /// Inclinación (beta) [0, 180]
    /// Medido respecto a la horizontal y normal hacia arriba (0 -> suelo, 180 -> techo)
    pub tilt: f32,
}

/// Hueco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Window {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre del hueco
    pub name: String,
    /// Superficie del hueco (m2)
    #[serde(rename = "A")]
    pub area: f32,
    /// Construcción del hueco
    pub cons: String,
    /// Muro al que pertenece el hueco
    pub wall: String,
    /// Factor de obstáculos remotos
    pub fshobst: f32,
}

/// Puente térmico
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThermalBridge {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre del puente térmico
    pub name: String,
    /// Longitud del puente térmico (m)
    #[serde(rename = "L")]
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
    pub psi: f32,
}

/// Definición de construcción de elemento opaco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WallCons {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Grosor del elemento (m) [0.0-]
    pub thickness: f32,
    /// Resistencia térmica total sin resistencias superficiales (resistencia intrínseca) [m2K/W]
    #[serde(rename = "R_intrinsic")]
    pub r_intrinsic: f32,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
}

/// Definición de construcción de hueco o lucernario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowCons {
    /// ID del espacio (en formato UUID)
    pub id: String,
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica total (incluyendo marco, vidrio y efecto de intercalarios y/o cajones de persiana) [W/m2K]
    #[serde(rename = "U")]
    pub u: f32,
    /// Fracción de marco [-]
    #[serde(rename = "Ff")]
    pub ff: f32,
    /// Factor solar del hueco sin la protección solar activada (g_glwi = g_gln * 0.90) [-]
    pub gglwi: f32,
    /// Factor solar del hueco con la protección solar activada [-]
    pub gglshwi: f32,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    #[serde(rename = "C_100")]
    pub infcoeff_100: f32,
}

/// Datos adicionales para comprobación de muros
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtraData {
    // Nombre del muro
    pub name: String,
    // Condiciones de contorno del muro
    pub bounds: BoundaryType,
    // Tipo de espacio
    pub spacetype: SpaceType,
    // Espacio adyacente
    pub nextspace: Option<String>,
    // Tipo de espacio adyacente
    pub nextspacetype: Option<SpaceType>,
    // Inclinación del muro
    pub tilt: Tilt,
    // Construcción
    pub cons: String,
    // U por defecto u obtenida de archivo KyGananciasSolares.txt
    pub u: f32,
    // U calculada con UNE-EN ISO 13789
    pub computed_u: f32,
}

/// Convierte de muro a enum Tilt
impl From<&Wall> for Tilt {
    fn from(wall: &Wall) -> Self {
        Tilt::from(wall.tilt)
    }
}

/// Convierte opaco a Orientation
impl From<&Wall> for Orientation {
    fn from(wall: &Wall) -> Self {
        match Tilt::from(wall.tilt) {
            Tilt::SIDE => wall.azimuth.into(),
            _ => Orientation::HZ,
        }
    }
}
