/* -*- coding: utf-8 -*-

Copyright (c) 2018-2019 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::utils::fround2;
use serde::{Deserialize, Serialize};

pub use super::common::{
    Boundaries::{self, *},
    Orientation::*,
    SpaceType::{self, *},
    Tilt::*,
};

/// Hueco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Window {
    /// Nombre del hueco
    pub name: String,
    /// Construcción del hueco
    pub cons: String,
    /// Muro al que pertenece el hueco
    pub wall: String,
    /// Ancho del hueco (m)
    pub width: f32,
    /// Altura del hueco (m)
    pub height: f32,
    /// Retranqueo del hueco (m)
    pub setback: f32,
}

impl Window {
    /// Fshobst
    /// No tiene en cuenta la geometría del modelo, sino solo la orientación y las tabla 17 del DA DB-HE/1 (p. 19).
    /// Solo obtiene para huecos verticales
    /// TODO: hacer cálculo de sombras, etc
    /// TODO: Llevar a función auxiliar (fshobst_for_setback(setback, inclination, orientation, width, height) -> f32)
    /// TODO: para obtener estos resultados a partir de datos y no de un objeto
    /// TODO: (lo usaremos para calcular elementos y así no guardamos datos de hueco que solo valen para esto)
    pub fn fshobst(&self, wall: &Wall) -> f32 {
        // Calcular según orientación e inclinación
        match wall.tilt.into() {
            // Elementos verticales
            SIDE => {
                let rh = self.setback / self.height;
                let rw = self.setback / self.width;
                let range_rh = if rh < 0.05 {
                    0
                } else if rh <= 0.1 {
                    1
                } else if rh <= 0.2 {
                    2
                } else if rh <= 0.5 {
                    3
                } else {
                    4
                };
                let range_rw = if rw < 0.05 {
                    0
                } else if rw <= 0.1 {
                    1
                } else if rw <= 0.2 {
                    2
                } else if rw <= 0.5 {
                    3
                } else {
                    4
                };
                match wall.azimuth.into() {
                    S => match (range_rh, range_rw) {
                        (1, 1) => 0.82,
                        (1, 2) => 0.74,
                        (1, 3) => 0.62,
                        (1, 4) => 0.39,
                        (2, 1) => 0.76,
                        (2, 2) => 0.67,
                        (2, 3) => 0.56,
                        (2, 4) => 0.35,
                        (3, 1) => 0.56,
                        (3, 2) => 0.51,
                        (3, 3) => 0.39,
                        (3, 4) => 0.27,
                        (4, 1) => 0.35,
                        (4, 2) => 0.32,
                        (4, 3) => 0.27,
                        (4, 4) => 0.17,
                        _ => 1.0,
                    },
                    SE | SW => match (range_rh, range_rw) {
                        (1, 1) => 0.86,
                        (1, 2) => 0.81,
                        (1, 3) => 0.72,
                        (1, 4) => 0.51,
                        (2, 1) => 0.79,
                        (2, 2) => 0.74,
                        (2, 3) => 0.66,
                        (2, 4) => 0.47,
                        (3, 1) => 0.59,
                        (3, 2) => 0.56,
                        (3, 3) => 0.47,
                        (3, 4) => 0.36,
                        (4, 1) => 0.38,
                        (4, 2) => 0.36,
                        (4, 3) => 0.32,
                        (4, 4) => 0.23,
                        _ => 1.0,
                    },
                    E | W => match (range_rh, range_rw) {
                        (1, 1) => 0.91,
                        (1, 2) => 0.87,
                        (1, 3) => 0.81,
                        (1, 4) => 0.65,
                        (2, 1) => 0.86,
                        (2, 2) => 0.82,
                        (2, 3) => 0.76,
                        (2, 4) => 0.61,
                        (3, 1) => 0.71,
                        (3, 2) => 0.68,
                        (3, 3) => 0.61,
                        (3, 4) => 0.51,
                        (4, 1) => 0.53,
                        (4, 2) => 0.51,
                        (4, 3) => 0.48,
                        (4, 4) => 0.39,
                        _ => 1.0,
                    },
                    _ => 1.0,
                }
            }
            TOP => {
                // TODO: hacer con tabla 19
                1.0
            }
            BOTTOM => 1.0,
        }
    }
}

/// Elemento opaco (muro, cubierta, suelo, partición)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wall {
    /// Nombre del elemento opaco
    pub name: String,
    /// Construcción del opaco
    pub cons: String,
    /// Espacio al que pertenece el elemento opaco
    pub space: String,
    /// Espacio adyacente con el que comunica el elemento opaco
    pub nextto: Option<String>,
    /// Condiciones de contorno del cerramiento:
    /// - UNDERGROUND: cerramientos en contacxto con el terreno
    /// - EXTERIOR: cerramientos en contacto con el aire exterior
    /// - INTERIOR: cerramientos en contacto con el aire de otros espacios
    /// - ADIABATIC: cerramientos sin transmisión de calor
    pub bounds: Boundaries,
    /// Superficie neta del elemento opaco (m2)
    #[serde(rename(serialize = "A"))]
    pub a: f32,
    /// Orientación (gamma) [-180,+180] (S=0, E=+90, W=-90)
    /// Medido como azimuth geográfico de la proyección horizontal de la normal a la superficie
    /// Coincide con el criterio de la UNE-EN ISO 52016-1
    /// Difiere del criterio BDL, que parte del norte, con E+ y W-
    pub azimuth: f32,
    /// Inclinación (beta) [0, 180]
    /// Medido respecto a la horizontal y normal hacia arriba (0 -> suelo, 180 -> techo)
    pub tilt: f32,
    // // TODO: elementos que pertenecen a la construcción
    // /// Transmitancia térmica (W/m2K)
    // #[serde(rename(serialize = "U"))]
    // pub u: f32,
}

impl Wall {
    /// Transmitancia térmica del cerramiento, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - en particiones interiores no se considera el factor b, reductor de temperatura
    /// - NO se ha implementado el cálculo de elementos en contacto con espacios no habitables
    /// - NO se ha implementado el cálculo de cerramientos en contacto con el terreno
    ///     - en HULC los valores por defecto de Ra y D se indican en las opciones generales de
    ///       las construcciones por defecto
    /// - los elementos adiabáticos se reportan con valor 0.0
    /// - las particiones interiores horizontales
    pub fn u(&self, construction: &WallCons) -> f32 {
        let bounds = self.bounds;
        let position = self.tilt.into();

        let r_intrinsic = construction.r_intrinsic;

        // Resistencias superficiales [m2·K/W]
        // Revisar según DA-DB-HE/1 tabla 1
        const RSE: f32 = 0.04;
        const RSI_ASCENDENTE: f32 = 0.10;
        const RSI_HORIZONTAL: f32 = 0.13;
        const RSI_DESCENDENTE: f32 = 0.17;

        let u_noround = match (bounds, position) {
            // TODO: implementar soleras en contacto con el terreno
            (UNDERGROUND, BOTTOM) => Default::default(),
            // TODO: implementar muros enterrados
            (UNDERGROUND, SIDE) => Default::default(),
            // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
            (UNDERGROUND, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
            // Tomamos valor 0.0. Siempre se podría consultar la resistencia intrínseca
            (ADIABATIC, _) => 0.0,
            // HULC no diferencia entre posiciones para elementos interiores
            // TODO: Detectar el caso de contacto con espacios no habitables, con cálculo de b, e implementar
            // TODO: tal vez esto debería recibir el valor b como parámetro
            (INTERIOR, _) => 1.0 / (r_intrinsic + 2.0 * RSI_HORIZONTAL),
            // Elementos en contacto con el exterior
            (EXTERIOR, BOTTOM) => 1.0 / (r_intrinsic + RSI_DESCENDENTE + RSE),
            (EXTERIOR, TOP) => 1.0 / (r_intrinsic + RSI_ASCENDENTE + RSE),
            (EXTERIOR, SIDE) => 1.0 / (r_intrinsic + RSI_HORIZONTAL + RSE),
        };
        fround2(u_noround)
    }
}

/// Puente térmico
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThermalBridge {
    /// Nombre del puente térmico
    pub name: String,
    /// Longitud del puente térmico (m)
    #[serde(rename(serialize = "L"))]
    pub l: f32,
    /// Transmitancia térmica lineal del puente térmico (W/mK)
    pub psi: f32,
}

/// Espacio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    /// Nombre del espacio
    pub name: String,
    /// Superficie de la zona en m2
    pub area: f32,
    /// Altura libre (suelo a techo) de la zona en m
    /// No incluye el volumen de forjados o cubiertas.
    pub height_net: f32,
    /// Altura bruta (suelo a suelo) de la zona en m
    pub height_gross: f32,
    /// Pertenencia al interior de la envolvente térmica
    pub inside_tenv: bool,
    /// Multiplicador
    pub multiplier: f32,
    // Tipo de espacio (ACONDICIONADO, NO_ACONDICIONADO, NO_HABITABLE)
    #[serde(rename(serialize = "type"))]
    pub space_type: SpaceType,
}

/// Definición de construcción de elemento opaco
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WallCons {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Resistencia térmica total sin resistencias superficiales (resistencia intrínseca) [m2K/W]
    #[serde(rename(serialize = "R_intrinsic"))]
    pub r_intrinsic: f32,
    /// Coeficiente de absortividad solar del elemento opaco (alpha) [0-1]
    pub absorptance: f32,
}

/// Definición de construcción de hueco o lucernario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowCons {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica total (incluyendo marco, vidrio y efecto de intercalarios y/o cajones de persiana) [W/m2K]
    #[serde(rename(serialize = "U"))]
    pub u: f32,
    /// Fracción de marco [-]
    #[serde(rename(serialize = "Ff"))]
    pub ff: f32,
    /// Factor solar del hueco sin la protección solar activada (g_glwi = g_gln * 0.90) [-]
    pub gglwi: f32,
    /// Factor solar del hueco con la protección solar activada [-]
    pub gglshwi: f32,
    /// Permeabilidad al aire a 100 Pa [m3/hm2]
    #[serde(rename(serialize = "C_100"))]
    pub infcoeff_100: f32,
}