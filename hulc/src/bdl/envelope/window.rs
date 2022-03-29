// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos WINDOW de la envolvente térmica

use std::convert::TryFrom;

use anyhow::{bail, format_err, Error};

use crate::bdl::{extract_f32vec, BdlBlock};

// Hueco (WINDOW) -------------------------------------------------

/// Hueco o lucernario (WINDOW)
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// Nombre
    pub name: String,
    /// Muro, cubierta o suelo en el que se sitúa
    pub wall: String,
    /// Definición de la composición del hueco (WindowCons::name)
    pub cons: String,
    /// Distancia (m) del borde izquierdo del hueco al borde izquierdo del cerramiento que lo contiene (mirando desde fuera)
    pub x: f32,
    /// Distancia (m) del borde inferior del hueco al borde inferior del cerramiento que lo contiene (mirando desde fuera)
    pub y: f32,
    /// Altura del hueco (m)
    pub height: f32,
    /// Anchura del hueco (m)
    pub width: f32,
    /// Retranqueo del hueco (m)
    pub setback: f32,
    /// Coeficientes de corrección por dispositivo de sombra estacional
    /// - 0: Corrección de factor solar fuera de la temporada veraniega (-)
    /// - 1: Corrección de factor solar dentro de la temporada veraniega (-)
    /// - 2: Corrección de transmitancia térmica fuera de la temporada veraniega (-)
    /// - 3: Corrección de transmitancia térmica dentro de la temporada veraniega (-)
    pub coefs: Option<Vec<f32>>,
    /// Alero sobre el hueco
    pub overhang: Option<Overhang>,
    /// Aleta izquierda
    pub left_fin: Option<Fin>,
    /// Aleta derecha
    pub right_fin: Option<Fin>,
    /// Lamas fijas horizontales o verticales
    pub louvres: Option<Louvres>,
}

impl TryFrom<BdlBlock> for Window {
    type Error = Error;

    /// Conversión de bloque BDL a hueco o lucernario (WINDOW)
    ///
    /// ¿Puede definirse con GLASS-TYPE, WINDOW-LAYER o GAP?
    /// y puede pertenecer a un INTERIOR-WALL o EXTERIOR-WALL
    /// (trasnmisividadJulio)
    /// XXX:
    /// COEFF son los factores (f1, f2, f3, f4), donde f1 y f2 son los correctores del
    /// factor solar (fuera de la temporada de activación de las sombras estacionales y dentro de esa temporada)
    /// y f3 y f4 los correctores de la transmitancia térmica del hueco en las mismas temporadas
    /// (desactivado y con la sombra estacional activada)
    /// XXX: las propiedades del marco y vidrio se consultan a través del GAP
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///     "P01_E02_PE005_V" = WINDOW
    ///     X              =            0.2
    ///     Y              =            0.1
    ///     SETBACK        =              0
    ///     HEIGHT         =            2.6
    ///     WIDTH          =              5
    ///     GAP            = "muro_cortina_controlsolar"
    ///     COEFF = ( 1.000000, 1.000000, 1.000000, 1.000000)
    ///     transmisividadJulio        = 0.220000
    ///     GLASS-TYPE     = "Doble baja emisividad argon"
    ///     FRAME-WIDTH   =      0.1329403
    ///     FRAME-CONDUCT =       5.299999
    ///     FRAME-ABS     =            0.7
    ///     INF-COEF       =              9
    ///     OVERHANG-A     =              0
    ///     OVERHANG-B     =              0
    ///     OVERHANG-W     =              0
    ///     OVERHANG-D     =              0
    ///     OVERHANG-ANGLE =              0
    ///     LEFT-FIN-A     =              0
    ///     LEFT-FIN-B     =              0
    ///     LEFT-FIN-H     =              0
    ///     LEFT-FIN-D     =              0
    ///     RIGHT-FIN-A    =              0
    ///     RIGHT-FIN-B    =              0
    ///     RIGHT-FIN-H    =              0
    ///     RIGHT-FIN-D    =              0
    ///     ..
    /// ```
    /// TODO: atributos no trasladados:
    /// TODO:  GLASS-TYPE, FRAME-WIDTH, FRAME-CONDUCT, FRAME-ABS, INF-COEF
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            parent,
            mut attrs,
            ..
        } = value;
        let wall = parent.ok_or_else(|| format_err!("Hueco sin muro asociado '{}'", &name))?;
        let cons = attrs.remove_str("GAP")?;
        let x = attrs.remove_f32("X")?;
        let y = attrs.remove_f32("Y")?;
        let height = attrs.remove_f32("HEIGHT")?;
        let width = attrs.remove_f32("WIDTH")?;
        let setback = attrs.remove_f32("SETBACK")?;
        let coefs = match attrs.remove_str("COEFF").ok() {
            None => None, // LIDER antiguo no define estos parámetros
            Some(vals) => match extract_f32vec(vals) {
                Ok(vec) if vec.len() == 4 => Some(vec),
                _ => bail!(
                    "Definición incorrecta de coeficientes de corrección en el hueco '{}'",
                    name
                ),
            },
        };

        let overhang = {
            let o = Overhang {
                a: attrs.remove_f32("OVERHANG-A").unwrap_or_default(),
                b: attrs.remove_f32("OVERHANG-B").unwrap_or_default(),
                depth: attrs.remove_f32("OVERHANG-D").unwrap_or_default(),
                width: attrs.remove_f32("OVERHANG-W").unwrap_or_default(),
                angle: attrs.remove_f32("OVERHANG-ANGLE").unwrap_or_default(),
            };
            if o.depth * o.width > 0.0 {
                Some(o)
            } else {
                None
            }
        };

        let left_fin = {
            let f = Fin {
                a: attrs.remove_f32("LEFT-FIN-A").unwrap_or_default(),
                b: attrs.remove_f32("LEFT-FIN-B").unwrap_or_default(),
                depth: attrs.remove_f32("LEFT-FIN-D").unwrap_or_default(),
                height: attrs.remove_f32("LEFT-FIN-H").unwrap_or_default(),
            };
            if f.depth * f.height > 0.0 {
                Some(f)
            } else {
                None
            }
        };

        let right_fin = {
            let f = Fin {
                a: attrs.remove_f32("RIGHT-FIN-A").unwrap_or_default(),
                b: attrs.remove_f32("RIGHT-FIN-B").unwrap_or_default(),
                depth: attrs.remove_f32("RIGHT-FIN-D").unwrap_or_default(),
                height: attrs.remove_f32("RIGHT-FIN-H").unwrap_or_default(),
            };
            if f.depth * f.height > 0.0 {
                Some(f)
            } else {
                None
            }
        };

        let louvres = {
            let ll = Louvres {
                is_horizontal: matches!(
                    attrs
                        .remove_str("POSITION-LAMAS")
                        .unwrap_or_default()
                        .as_str(),
                    "Horizontal"
                ),
                width: attrs.remove_f32("LAMAS-WIDTH").unwrap_or_default(),
                distance: attrs.remove_f32("LAMAS-DISTANCE").unwrap_or_default(),
                angle: attrs.remove_f32("LAMAS-ANGLE").unwrap_or_default(),
                transmisivity: attrs.remove_f32("LAMAS-TRANSMISIVITY").unwrap_or_default(),
                reflectivity: attrs.remove_f32("LAMAS-REFLECTIVITY").unwrap_or_default(),
            };
            if ll.width > 0.0 {
                Some(ll)
            } else {
                None
            }
        };

        Ok(Self {
            name,
            wall,
            cons,
            x,
            y,
            height,
            width,
            setback,
            coefs,
            overhang,
            left_fin,
            right_fin,
            louvres,
        })
    }
}

/// Aleros sobre huecos
#[derive(Debug, Clone, Default)]
pub struct Overhang {
    /// Distancia horizontal (hacia la izquierda) del vértice superior izquierdo del hueco al vértice izquierdo del alero en el muro [m]
    pub a: f32,
    /// Distancia vertical (hacia arriba) del vértice superior izquierdo del hueco al vértice izquierdo del alero en el muro [m]
    pub b: f32,
    /// Profundidad del alero [m]
    pub depth: f32,
    /// Ancho del alero (hacia la derecha) [m]
    pub width: f32,
    /// Inclinación del alero (90 es perpendicular al hueco y 0 es paralelo al hueco) (0 - 360) [º]
    pub angle: f32,
}

/// Aletas laterales a los huecos
/// Puede ser una aleta a la derecha o a la izquierda del hueco
#[derive(Debug, Clone, Default)]
pub struct Fin {
    /// Distancia horizontal desde el lado del hueco al vértice superior de la aleta [m]
    /// En aletas a la izquierda esta es una distancia hacia la izquierda y en las aletas a la derecha, hacia la derecha desde el lado más próximo
    pub a: f32,
    /// Distancia vertical (hacia abajo) desde el lado superior del hueco al vértice superior de la aleta en el muro [m]
    pub b: f32,
    /// Profundidad de la aleta (en perpendicular al hueco, desde a) [m]
    pub depth: f32,
    /// Altura de la aleta (hacia abajo, desde b) [m]
    pub height: f32,
}

/// Lamas horizontales o verticales en el hueco
///     POSITION-LAMAS      = Horizontal
///     LAMAS-WIDTH         =            0.2
///     LAMAS-DISTANCE      =            0.2
///     LAMAS-ANGLE         =             45
///     LAMAS-TRANSMISIVITY =              0
///     LAMAS-REFLECTIVITY  =              0
#[derive(Debug, Clone, Default)]
pub struct Louvres {
    /// Si son lamas horizontales o verticales
    pub is_horizontal: bool,
    /// Ancho de las lamas (profundidad) [m]
    pub width: f32,
    /// Distancia vertical (lamas horizontales) u horizontal (lamas verticales) entre lamas [m]
    pub distance: f32,
    /// Ángulo de inclinación de las lamas (-180, 180) [º]
    pub angle: f32,
    /// transmitancia (0-1) [-]
    pub transmisivity: f32,
    /// reflectividad (0-1) [-]
    pub reflectivity: f32,
}
