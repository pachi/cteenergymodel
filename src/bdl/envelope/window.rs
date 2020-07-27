//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos WINDOW de la envolvente térmica

use failure::Error;
use std::convert::TryFrom;

use crate::bdl::{extract_f32vec, BdlBlock, Data};

// Hueco (WINDOW) -------------------------------------------------

/// Hueco o lucernario (WINDOW)
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// Nombre
    pub name: String,
    /// Muro, cubierta o suelo en el que se sitúa
    pub wall: String,
    /// Definición de la composición del hueco (WindowCons::name)
    pub construction: String,
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
}

// TODO: Muchas de estas cosas seguramente tendrían que ir a types y quedar Window como datos simplemente
impl Window {
    /// Superficie de la ventana [m2]
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Inclinación de la ventana (grados)
    /// Es el ángulo respecto al eje Z de la normal a la superficie en la que está la ventana
    pub fn tilt(&self, db: &Data) -> Result<f32, Error> {
        let wall = db.get_wall(&self.wall).ok_or_else(|| {
            format_err!(
                "Muro {} al que pertenece la ventana {} no encontrado. No se puede calcular la inclinación",
                self.wall,
                self.name
            )
        })?;
        Ok(wall.tilt)
    }

    /// Azimut de la ventana (grados)
    /// Es el ángulo respecto al eje Z de la normal a la superficie en la que está la ventana
    pub fn azimuth(&self, northangle: f32, db: &Data) -> Result<f32, Error> {
        let wall = db.get_wall(&self.wall).ok_or_else(|| {
            format_err!(
                "Muro {} al que pertenece la ventana {} no encontrado. No se puede calcular el azimut",
                self.wall,
                self.name
            )
        })?;
        wall.azimuth(northangle, db)
    }

    /// Perímetro del hueco [m]
    pub fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }

    /// Fshobst
    /// No tiene en cuenta la geometría del modelo, sino solo la orientación y las tabla 17 del DA DB-HE/1 (p. 19).
    /// Solo obtiene para huecos verticales
    /// TODO: hacer cálculo de sombras, etc
    pub fn fshobst(&self, northangle: f32, db: &Data) -> Result<f32, Error> {
        let wall = db.get_wall(&self.wall).ok_or_else(|| {
            format_err!(
                "Muro {} al que pertenece la ventana {} no encontrado. No se puede calcular el azimut",
                self.wall,
                self.name
            )
        })?;
        // Calcular según orientación e inclinación
        use super::Positions::*;

        match wall.position() {
            // Elementos verticales
            SIDE => {
                use crate::utils::*;
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
                let fshobst =
                    match angle_name(orientation_bdl_to_52016(wall.azimuth(northangle, &db)?))
                        .as_ref()
                    {
                        "S" => match (range_rh, range_rw) {
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
                        "SE" | "SW" => match (range_rh, range_rw) {
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
                        "E" | "W" => match (range_rh, range_rw) {
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
                    };
                Ok(fshobst)
            }
            TOP => {
                // TODO: hacer con tabla 19
                Ok(1.0)
            }
            BOTTOM => Ok(1.0),
        }
    }
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
    /// TODO:  GLASS-TYPE, FRAME-WIDTH, FRAME-CONDUCT, FRAME-ABS, INF-COEF,
    /// TODO: propiedades para definir salientes y voladizos o para lamas:
    /// TODO: OVERHANG-A, OVERHANG-B, OVERHANG-W, OVERHANG-D, OVERHANG-ANGLE,
    /// TODO: LEFT-FIN-A, LEFT-FIN-B, LEFT-FIN-H, LEFT-FIN-D, RIGHT-FIN-A, RIGHT-FIN-B, RIGHT-FIN-H, RIGHT-FIN-D
    /// TODO: propiedades para definición de lamas
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            parent,
            mut attrs,
            ..
        } = value;
        let wall = parent.ok_or_else(|| format_err!("Hueco sin muro asociado '{}'", &name))?;
        let construction = attrs.remove_str("GAP")?;
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

        Ok(Self {
            name,
            wall,
            construction,
            x,
            y,
            height,
            width,
            setback,
            coefs,
        })
    }
}
