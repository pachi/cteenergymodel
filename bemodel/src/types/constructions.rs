// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Construcciones de la envolvente térmica: WallCons, WinCons

use serde::{Deserialize, Serialize};

use super::{Uuid, Wall, Window};
use crate::utils::fround3;

// Elementos -----------------------------------------------

/// Base de datos de construcciones de opacos y huecos y sus componentes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsDb {
    /// Construcciones de opacos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wallcons: Vec<WallCons>,
    /// Construcciones de huecos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wincons: Vec<WinCons>,
    /// Lista de materiales para elementos opacos (muro, cubierta, suelo, partición)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<Material>,
    /// Lista de vidrios
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub glasses: Vec<Glass>,
    /// Lista de marcos
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frames: Vec<Frame>,
}

impl ConsDb {
    /// Localiza construcción de opaco por id
    pub fn get_wallcons(&self, id: Uuid) -> Option<&WallCons> {
        self.wallcons.iter().find(|w| w.id == id)
    }

    /// Localiza construcción de hueco por id
    pub fn get_wincons(&self, id: Uuid) -> Option<&WinCons> {
        self.wincons.iter().find(|w| w.id == id)
    }

    /// Localiza material de opaco por id
    pub fn get_material(&self, id: Uuid) -> Option<&Material> {
        self.materials.iter().find(|w| w.id == id)
    }

    /// Localiza vidrio por id
    pub fn get_glass(&self, id: Uuid) -> Option<&Glass> {
        self.glasses.iter().find(|w| w.id == id)
    }

    /// Localiza marco por id
    pub fn get_frame(&self, id: Uuid) -> Option<&Frame> {
        self.frames.iter().find(|w| w.id == id)
    }

    /// Comprueba si la base de datos está vacía
    pub(crate) fn is_empty(&self) -> bool {
        self.wallcons.is_empty()
            && self.wincons.is_empty()
            && self.materials.is_empty()
            && self.glasses.is_empty()
            && self.frames.is_empty()
    }

    /// Limpia elementos no usados de la base de datos
    pub(crate) fn purge_unused(&mut self, walls: &[Wall], windows: &[Window]) {
        // Elimina construcciones de opacos no usadas
        self.wallcons
            .retain(|cons| walls.iter().any(|wall| wall.cons == cons.id));
        // Elimina construcciones de huecos no usadas
        self.wincons
            .retain(|cons| windows.iter().any(|win| win.cons == cons.id));

        // Elimina materiales no usados
        let used_mats_ids: Vec<Uuid> = self
            .wallcons
            .iter()
            .flat_map(|cons| cons.layers.iter().map(|l| l.id).collect::<Vec<Uuid>>())
            .collect();
        self.materials
            .retain(|mat| used_mats_ids.iter().any(|used_id| *used_id == mat.id));
        // Elimina vidrios no usados
        let used_glasses_ids: Vec<Uuid> = self.wincons.iter().map(|cons| cons.glass).collect();
        self.glasses
            .retain(|gl| used_glasses_ids.iter().any(|used_id| *used_id == gl.id));
        // Elimina marcos no usados
        let used_frames_ids: Vec<Uuid> = self.wincons.iter().map(|cons| cons.frame).collect();
        self.frames
            .retain(|w| used_frames_ids.iter().any(|used_id| *used_id == w.id));
    }
}

/// Definición de construcción de elemento opaco
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallCons {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Capas que forman la construcción de opaco, como lista de tuplas (material, espesor)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Layer>,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
}

impl WallCons {
    /// Espesor total de una composición de capas [m]
    pub fn thickness(&self) -> f32 {
        fround3(self.layers.iter().map(|Layer { e, .. }| e).sum())
    }
}

impl Default for WallCons {
    fn default() -> Self {
        WallCons {
            id: Uuid::new_v4(),
            name: "Construcción de opaco".to_string(),
            layers: Vec::default(),
            absorptance: 0.7,
        }
    }
}

/// Definición de capa de opaco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Layer {
    /// ID del material
    pub id: Uuid,
    /// Espesor, m
    pub e: f32,
}

/// Definición de construcción de hueco o lucernario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinCons {
    /// ID del espacio (en formato UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Vidrio del hueco, UUID
    pub glass: Uuid,
    /// Marco del hueco, UUID
    pub frame: Uuid,
    /// Fracción de marco [-]
    pub f_f: f32,
    /// Porcentaje de U debido a intercalarios y cajón de persiana (%)
    pub delta_u: f32,
    /// Factor solar del hueco con la protección solar activada (g_gl;sh;wi) [-]
    /// Si no se define (valor None), se supone igual al factor solar sin la protección activada (g_gl;wi)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g_glshwi: Option<f32>,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    pub c_100: f32,
}

impl Default for WinCons {
    fn default() -> Self {
        WinCons {
            id: Uuid::new_v4(),
            name: "Construcción de hueco".to_string(),
            glass: Uuid::default(),
            frame: Uuid::default(),
            f_f: 0.20,
            delta_u: 0.0,
            g_glshwi: None,
            c_100: 50.0,
        }
    }
}

/// Material de elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// ID del material (UUID)
    pub id: Uuid,
    /// Nombre del material
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Definición de propiedades, detallada (lambda, rho, C_p, mu, ...) o solo resistencia
    #[serde(flatten)]
    pub properties: MatProps,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            id: Uuid::new_v4(),
            name: "Fábrica 1/2' LP G > 80".to_string(),
            properties: MatProps::default(),
        }
    }
}

/// Tipos de propiedades de materiales
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatProps {
    /// Lista detallada de propiedades de materiales (lambda, rho, C_p, mu)
    #[serde(rename = "properties")]
    Detailed {
        // Conductividad térmica, lambda (W/mK)
        conductivity: f32,
        // Densidad, rho (kg/m3)
        density: f32,
        // Calor específico, C_p (J/kg K) (valor por defecto 1000 J/kg·K)
        specific_heat: f32,
        // Factor de difusividad al vapor de agua, mu (-)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        vapour_diff: Option<f32>,
    },
    /// Resistencia térmica (R)
    #[serde(rename = "resistance")]
    Resistance {
        /// Resistencia térmica, m²K/W
        resistance: f32,
    },
}

impl Default for MatProps {
    fn default() -> Self {
        // Caso por defecto (Fábrica 1/2' LP G > 80 del CEC)
        MatProps::Detailed {
            conductivity: 0.23,
            density: 900.0,
            specific_heat: 1000.0,
            vapour_diff: Some(10.0),
        }
    }
}

/// Vidrio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    /// ID del vidrio (UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Conductividad W/m²K
    pub u_value: f32,
    /// Factor solar del vidrio a incidencia normal
    pub g_gln: f32,
}

impl Default for Glass {
    fn default() -> Self {
        // Caso por defecto (Acristalamiento vidrio sencillo 6mm vert del CEC)
        Glass {
            id: Uuid::new_v4(),
            name: "Vidrio sencillo 6mm (Vert)".to_string(),
            u_value: 5.7,
            g_gln: 0.83,
        }
    }
}

/// Marco de hueco
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    /// ID del marco (UUID)
    pub id: Uuid,
    /// Nombre
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Transmitancia térmica, U (W/m²K)
    pub u_value: f32,
    /// Absortividad del marco, alpha (-)
    pub absorptivity: f32,
}

impl Default for Frame {
    fn default() -> Self {
        // Caso por defecto (Marco metálico con RPT > 12mm)
        Frame {
            id: Uuid::new_v4(),
            name: "Marco metálico con RPT > 12 mm".to_string(),
            u_value: 3.2,
            absorptivity: 0.6,
        }
    }
}
