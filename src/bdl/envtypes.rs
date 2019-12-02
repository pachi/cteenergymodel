//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos de la envolvente térmica
//!
//! - Huecos (WINDOW)
//! - muros (EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL)
//! - cubiertas (ROOF)
//! 
//! Todos menos el hueco tienen una construcción y pertenecen a un espacio (location)

use super::blocks::BdlBlock;

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlEnvType {
    Window(BdlBlock),
    ExteriorWall(BdlBlock),
    InteriorWall(BdlBlock),
    UndergroundWall(BdlBlock),
    Roof(BdlBlock),
}

// Hueco (WINDOW) -------------------------------------------------
//
// Puede definirse con GLASS-TYPE, WINDOW-LAYER o GAP
// y puede pertenecer a un INTERIOR-WALL o EXTERIOR-WALL
// (trasnmisividadJulio)
//
// Ejemplo en BDL:
// ```text
//     "P01_E02_PE005_V" = WINDOW
//     X              =            0.2
//     Y              =            0.1
//     SETBACK        =              0
//     HEIGHT         =            2.6
//     WIDTH          =              5
//     GAP            = "muro_cortina_controlsolar"
//     COEFF = ( 1.000000, 1.000000, 1.000000, 1.000000)
//     transmisividadJulio        = 0.220000
//     GLASS-TYPE     = "Doble baja emisividad argon"
//     FRAME-WIDTH   =      0.1329403
//     FRAME-CONDUCT =       5.299999
//     FRAME-ABS     =            0.7
//     INF-COEF       =              9
//     OVERHANG-A     =              0
//     OVERHANG-B     =              0
//     OVERHANG-W     =              0
//     OVERHANG-D     =              0
//     OVERHANG-ANGLE =              0
//     LEFT-FIN-A     =              0
//     LEFT-FIN-B     =              0
//     LEFT-FIN-H     =              0
//     LEFT-FIN-D     =              0
//     RIGHT-FIN-A    =              0
//     RIGHT-FIN-B    =              0
//     RIGHT-FIN-H    =              0
//     RIGHT-FIN-D    =              0
//     ..
// ```

// Muro exterior (EXTERIOR-WALL) -----------------------------------
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