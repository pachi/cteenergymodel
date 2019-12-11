//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos de la envolvente térmica
//!
//! - Huecos (WINDOW)
//! - muros (EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL)
//! - cubiertas (ROOF)
//!
//! Todos menos el hueco tienen una construcción y pertenecen a un espacio (location)

use failure::Error;
use std::convert::TryFrom;

use super::{extract_f32vec, BdlBlock};

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlEnvType {
    Window(Window),
    ExteriorWall(BdlBlock),
    InteriorWall(BdlBlock),
    UndergroundWall(BdlBlock),
    Roof(BdlBlock),
}

// TODO: ver sisten/libreria_sisgen/claseEdificio.py

// Hueco (WINDOW) -------------------------------------------------

/// Hueco o lucernario (WINDOW)
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// Nombre
    pub name: String,
    /// Definición de la composición del hueco
    pub gap: String,
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
    /// Corrección de factor solar fuera de la temporada veraniega (-)
    pub corrg0: f32,
    /// Coeficientes de corrección por dispositivo de sombra estacional
    /// Corrección de factor solar dentro de la temporada veraniega (-)
    pub corrg1: f32,
    /// Coeficientes de corrección por dispositivo de sombra estacional
    /// Corrección de transmitancia térmica fuera de la temporada veraniega (-)
    pub corru0: f32,
    /// Coeficientes de corrección por dispositivo de sombra estacional
    /// Corrección de transmitancia térmica dentro de la temporada veraniega (-)
    pub corru1: f32,
    /// Transmitancia total de energía del acristalameinto con los dispositivo de sombra móvil activados (g_gl;sh;wi) (-)
    pub gglshwi: f32,
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
    /// TODO: propiedades para definir salientes y voladizos o para lamas:
    /// TODO:  GLASS-TYPE, FRAME-WIDTH, FRAME-CONDUCT, FRAME-ABS, INF-COEF,
    /// TODO: OVERHANG-A, OVERHANG-B, OVERHANG-W, OVERHANG-D, OVERHANG-ANGLE,
    /// TODO: LEFT-FIN-A, LEFT-FIN-B, LEFT-FIN-H, LEFT-FIN-D, RIGHT-FIN-A, RIGHT-FIN-B, RIGHT-FIN-H, RIGHT-FIN-D
    /// TODO: propiedades para definición de lamas
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let gap = attrs.remove_str("GAP")?;
        let x = attrs.remove_f32("X")?;
        let y = attrs.remove_f32("Y")?;
        let height = attrs.remove_f32("HEIGHT")?;
        let width = attrs.remove_f32("WIDTH")?;
        let setback = attrs.remove_f32("SETBACK")?;
        let coefs = extract_f32vec(attrs.remove_str("COEFF")?)?;
        let [corrg0, corrg1, corru0, corru1] = match coefs.as_slice() {
            [c1, c2, c3, c4] => [*c1, *c2, *c3, *c4],
            _ => bail!(
                "Definición incorrecta de coeficientes de corrección en el hueco '{}'",
                name
            ),
        };
        let gglshwi = attrs.remove_f32("transmisividadJulio")?;

        Ok(Self {
            name,
            gap,
            x,
            y,
            height,
            width,
            setback,
            corrg0,
            corrg1,
            corru0,
            corru1,
            gglshwi,
        })
    }
}

// height, width, glasstype, gap, transmisividadjulio,

// Muro exterior (EXTERIOR-WALL) -----------------------------------
//
// ROOF es igual pero cambia la inclinación por defecto
// (90 para EXTERIOR-WALL y 0 para ROOF)
//
// Ejemplo en BDL:
// ```text
//    "P01_E02_PE006" = EXTERIOR-WALL
//    ABSORPTANCE   =            0.6
//    COMPROBAR-REQUISITOS-MINIMOS = YES
//    TYPE_ABSORPTANCE    = 1
//    COLOR_ABSORPTANCE   = 0
//    DEGREE_ABSORPTANCE   = 2
//    CONSTRUCCION_MURO  = "muro_opaco"
//    CONSTRUCTION  = "muro_opaco0.60"
//    LOCATION      = SPACE-V11
//        ..
//    "muro_opaco0.60" =  CONSTRUCTION
//        TYPE   = LAYERS
//        LAYERS = "muro_opaco"
//        ABSORPTANCE = 0.600000
//        ..
//    "P01_E02_PE006_V" = WINDOW
//        X              =            3.3
//        Y              =            0.1
//        SETBACK        =              0
//        HEIGHT         =            2.6
//        WIDTH          =              5
//        GAP            = "muro_cortina_controlsolar"
//        COEFF = ( 1.000000, 1.000000, 1.000000, 1.000000)
//        transmisividadJulio        = 0.220000
//        GLASS-TYPE     = "Doble baja emisividad argon"
//        FRAME-WIDTH   =      0.1329403
//        FRAME-CONDUCT =       5.299999
//        FRAME-ABS     =            0.7
//        INF-COEF       =              9
//        OVERHANG-A     =              0
//        OVERHANG-B     =              0
//        OVERHANG-W     =              0
//        OVERHANG-D     =              0
//        OVERHANG-ANGLE =              0
//        LEFT-FIN-A     =              0
//        LEFT-FIN-B     =              0
//        LEFT-FIN-H     =              0
//        LEFT-FIN-D     =              0
//        RIGHT-FIN-A    =              0
//        RIGHT-FIN-B    =              0
//        RIGHT-FIN-H    =              0
//        RIGHT-FIN-D    =              0
//        ..
// ```

// Muro interior (INTERIOR-WALL) -------------------------------------
//
// Ejemplo en BDL:
// ```text
//    "P01_E02_Med001" = INTERIOR-WALL
//     INT-WALL-TYPE = STANDARD
//     NEXT-TO       = "P01_E07"
//     COMPROBAR-REQUISITOS-MINIMOS = NO
//     CONSTRUCTION  = "tabique"
//     LOCATION      = SPACE-V1
//           ..
//     "tabique" =  CONSTRUCTION
//           TYPE   = LAYERS
//           LAYERS = "tabique"
//           ..
// ```

// Muro o soleras en contacto con el terreno (UNDERGROUND-WALL) --------
//
// Ejemplo en BDL:
// ```text
//    "P01_E01_FTER001" = UNDERGROUND-WALL
//     Z-GROUND      =              0
//     COMPROBAR-REQUISITOS-MINIMOS = YES
//                    CONSTRUCTION  = "solera tipo"
//                    LOCATION      = BOTTOM
//                     AREA          =        418.4805
//                     PERIMETRO     =        65.25978
//                          ..
//                    "solera tipo" =  CONSTRUCTION
//                          TYPE   = LAYERS
//                          LAYERS = "solera tipo"
//                          ..
// ```

// Cubierta
//
// Ejemplo en BDL:
// ```text
//    "P03_E01_CUB001" = ROOF
//                      ABSORPTANCE   =            0.6
//                      COMPROBAR-REQUISITOS-MINIMOS = YES
//                      TYPE_ABSORPTANCE    = 0
//                      COLOR_ABSORPTANCE   = 0
//                      DEGREE_ABSORPTANCE   = 2
//                      CONSTRUCTION  = "cubierta"
//                      LOCATION      = TOP
//                            ..
//                      "cubierta" =  CONSTRUCTION
//                            TYPE   = LAYERS
//                            LAYERS = "cubierta"
//                            ..
// ```
