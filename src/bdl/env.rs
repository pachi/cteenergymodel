//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos de la envolvente térmica:
//!
//! - Huecos (WINDOW)
//! - muros (EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL)
//! - cubiertas (ROOF)
//!
//! Todos menos el hueco tienen una construcción y pertenecen a un espacio (location)
//! 
//! Otros elementos:
//! - Sombra (BUILDING-SHADE)

use failure::Error;
use std::convert::TryFrom;

use super::{extract_f32vec, BdlBlock};

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlEnvType {
    Window(Window),
    ExteriorWall(Wall),
    InteriorWall(InteriorWall),
    UndergroundWall(UndergroundWall),
    Roof(Wall),
}

// TODO: ver sisgen/libreria_sisgen/claseEdificio.py

// Hueco (WINDOW) -------------------------------------------------

/// Hueco o lucernario (WINDOW)
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// Nombre
    pub name: String,
    /// Muro, cubierta o suelo en el que se sitúa
    pub parent: String,
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
            name,
            parent,
            mut attrs,
            ..
        } = value;
        let parent = parent.ok_or_else(|| format_err!("Hueco sin muro asociado '{}'", &name))?;
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
            parent,
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

// Muros (EXTERIOR-WALL, ROOF, INTERIOR-WALL, UNDERGROUND-WALL) ------------------

/// Definición geométrica de un muro (EXTERIOR-WALL, ROOF o INTERIOR-WALL)
/// Se usa cuando no se define respecto a un vértice del espacio padre sino por polígono
#[derive(Debug, Clone, Default)]
pub struct WallGeometry {
    /// Nombre del polígono que define la geometría
    pub name: String,
    /// Coordenada X de la esquina inferior izquierda
    pub x: f32,
    /// Coordenada Y de la esquina inferior izquierda
    pub y: f32,
    /// Coordenada Z de la esquina inferior izquierda
    pub z: f32,
    /// Acimut (grados sexagesimales)
    /// Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del muro
    pub azimuth: f32,
    /// Inclinación
    /// Ángulo entre el eje Z y la normal exterior del muro
    /// Por defecto es 90 para ExteriorWall y 0 (hacia arriba) para Roof
    pub tilt: f32,
}

impl WallGeometry {
    pub fn get_wallgeometry(mut attrs: super::AttrMap) -> Result<Option<Self>, Error> {
        if let Some(name) = attrs.remove_str("POLYGON").ok() {
            let x = attrs.remove_f32("X")?;
            let y = attrs.remove_f32("Y")?;
            let z = attrs.remove_f32("Z")?;
            let azimuth = attrs.remove_f32("AZIMUTH")?;
            // XXX: se podría identificar la inclinación por defecto según el btype
            let tilt = attrs.remove_f32("TILT")?;
            Ok(Some(WallGeometry {
                name,
                x,
                y,
                z,
                azimuth,
                tilt,
            }))
        } else {
            Ok(None)
        }
    }
}

// Muro exterior (EXTERIOR-WALL) o cubierta (ROOF) ------------------------------
// ROOF es igual pero con inclinación por defecto = 0 en vez de 90

/// Muro exterior (EXTERIOR-WALL) o cubierta (ROOF)
/// Puede definirse su configuración geométrica por polígono
/// o por localización respecto al espacio padre.
#[derive(Debug, Clone, Default)]
pub struct Wall {
    /// Nombre
    pub name: String,
    /// Tipo (EXTERIOR-WALL o ROOF)
    pub wtype: String,
    /// Espacio en al que pertenece el muro o cubierta
    pub parent: String,
    /// Definición de la composición del cerramiento (Construction)
    pub construction: String,
    /// Absortividad definida por usuario
    pub absorptance: f32,
    /// Posición respecto al espacio asociado (TOP, BOTTOM, nombreespacio)
    pub location: Option<String>,
    /// Posición definida por polígono
    pub geometry: Option<WallGeometry>,
}

impl TryFrom<BdlBlock> for Wall {
    type Error = Error;

    /// Conversión de bloque BDL a muro exterior (o cubierta)
    ///
    /// Ejemplos en BDL:
    /// ```text
    ///    "P01_E02_PE006" = EXTERIOR-WALL
    ///         ABSORPTANCE   =            0.6
    ///         COMPROBAR-REQUISITOS-MINIMOS = YES
    ///         TYPE_ABSORPTANCE    = 1
    ///         COLOR_ABSORPTANCE   = 0
    ///         DEGREE_ABSORPTANCE   = 2
    ///         CONSTRUCCION_MURO  = "muro_opaco"
    ///         CONSTRUCTION  = "muro_opaco0.60"
    ///         LOCATION      = SPACE-V11
    ///         ..
    ///     "P02_E01_FE001" = EXTERIOR-WALL
    ///         ABSORPTANCE   =           0.95
    ///         COMPROBAR-REQUISITOS-MINIMOS = YES
    ///         TYPE_ABSORPTANCE    = 0
    ///         COLOR_ABSORPTANCE   = 7
    ///         DEGREE_ABSORPTANCE   = 2
    ///         CONSTRUCCION_MURO  = "muro_opaco"  
    ///         CONSTRUCTION  = "muro_opaco0.95"  
    ///         X             =       -49.0098
    ///         Y             =              0
    ///         Z             =              0
    ///         AZIMUTH       =             90
    ///         TILT          =            180
    ///         POLYGON       = "P02_E01_FE001_Poligono3"
    ///         ..
    ///     "P03_E01_CUB001" = ROOF
    ///         ABSORPTANCE   =            0.6
    ///         COMPROBAR-REQUISITOS-MINIMOS = YES
    ///         TYPE_ABSORPTANCE    = 0
    ///         COLOR_ABSORPTANCE   = 0
    ///         DEGREE_ABSORPTANCE   = 2
    ///         CONSTRUCTION  = "cubierta"
    ///         LOCATION      = TOP
    ///         ..
    /// ```
    /// TODO: atributos no trasladados:
    /// TODO: propiedades para definir el estado de la interfaz para la selección de la absortividad:
    /// TODO: TYPE_ABSORPTANCE, COLOR_ABSORPTANCE, DEGREE_ABSORPTANCE,
    /// TODO: CONSTRUCCION_MURO, COMPROBAR-REQUISITOS-MINIMOS
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            btype,
            parent,
            mut attrs,
            ..
        } = value;
        let parent = parent
            .ok_or_else(|| format_err!("Muro o cubierta sin espacio asociado '{}'", &name))?;
        let construction = attrs.remove_str("CONSTRUCTION")?;
        let absorptance = attrs.remove_f32("ABSORPTANCE")?;
        let location = attrs.remove_str("LOCATION").ok();
        let geometry = WallGeometry::get_wallgeometry(attrs)?;
        Ok(Self {
            name,
            wtype: btype,
            parent,
            construction,
            absorptance,
            location,
            geometry,
        })
    }
}

// Muro interior (INTERIOR-WALL) -------------------------------------

/// Muro interior
#[derive(Debug, Clone, Default)]
pub struct InteriorWall {
    /// Nombre
    pub name: String,
    /// Tipo de muro interior
    /// - STANDARD: muro entre dos espacios
    /// - ADIABATIC: muro que no conduce calor (a otro espacio) pero lo almacena
    /// - INTERNAL: muro interior a un espacio (no comunica espacios)
    /// - AIR: superficie interior sin masa pero que admite convección
    pub wtype: String,
    /// Espacio adyacente que conecta con el espacio padre (salvo que sea adiabático o interior)
    pub nextto: Option<String>,
    /// Espacio en al que pertenece el muro
    pub parent: String,
    /// Definición de la composición del cerramiento (Construction)
    pub construction: String,
    /// Posición respecto al espacio asociado (TOP, BOTTOM, nombreespacio)
    pub location: Option<String>,
    /// Posición definida por polígono
    pub geometry: Option<WallGeometry>,
}

impl TryFrom<BdlBlock> for InteriorWall {
    type Error = Error;

    /// Conversión de bloque BDL a muro exterior (o cubierta)
    ///
    /// Ejemplos en BDL:
    /// ```text
    ///    "P01_E02_Med001" = INTERIOR-WALL
    ///         INT-WALL-TYPE = STANDARD
    ///         NEXT-TO       = "P01_E07"
    ///         COMPROBAR-REQUISITOS-MINIMOS = NO
    ///         CONSTRUCTION  = "tabique"
    ///         LOCATION      = SPACE-V1
    ///         ..
    ///     "P02_E01_FI002" = INTERIOR-WALL
    ///         INT-WALL-TYPE = STANDARD  
    ///         NEXT-TO       = "P01_E04"  
    ///         COMPROBAR-REQUISITOS-MINIMOS = NO
    ///         CONSTRUCTION  = "forjado_interior"                 
    ///         X             =         -38.33
    ///         Y             =           3.63
    ///         Z             =              0
    ///         AZIMUTH       =             90
    ///         TILT          =            180
    ///         POLYGON       = "P02_E01_FI002_Poligono2"
    ///         ..
    /// ```
    /// TODO: atributos no trasladados:
    /// TODO: COMPROBAR-REQUISITOS-MINIMOS
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            parent,
            mut attrs,
            ..
        } = value;
        let parent =
            parent.ok_or_else(|| format_err!("Muro interior sin espacio asociado '{}'", &name))?;
        let wtype = attrs.remove_str("INT-WALL-TYPE")?;
        let nextto = attrs.remove_str("NEXT-TO").ok();
        let construction = attrs.remove_str("CONSTRUCTION")?;
        let location = attrs.remove_str("LOCATION").ok();
        let geometry = WallGeometry::get_wallgeometry(attrs)?;
        Ok(Self {
            name,
            wtype,
            nextto,
            parent,
            construction,
            location,
            geometry,
        })
    }
}

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

/// Muro (UNDERGROUND-WALL) o suelo (UNDEGROUND-FLOOR) en contacto con el terreno
#[derive(Debug, Clone, Default)]
pub struct UndergroundWall {
    /// Nombre
    pub name: String,
    /// Espacio en al que pertenece el muro o suelo
    pub parent: String,
    /// Definición de la composición del cerramiento (Construction)
    pub construction: String,
    /// Profundidad del elemento (m)
    pub zground: f32,
    /// Superficie (m2)
    pub area: f32,
    /// Perímetro (m)
    pub perimeter: f32,
    /// Posición respecto al espacio asociado (TOP, BOTTOM, nombreespacio)
    pub location: Option<String>,
    /// Posición definida por polígono
    pub geometry: Option<WallGeometry>,
}

impl TryFrom<BdlBlock> for UndergroundWall {
    type Error = Error;

    /// Conversión de bloque BDL a muro exterior (o cubierta)
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///    "P01_E01_FTER001" = UNDERGROUND-WALL
    ///         Z-GROUND      =              0
    ///         COMPROBAR-REQUISITOS-MINIMOS = YES
    ///         CONSTRUCTION  = "solera tipo"
    ///         LOCATION      = BOTTOM
    ///         AREA          =        418.4805
    ///         PERIMETRO     =        65.25978
    ///         ..
    /// ```
    /// TODO: atributos no trasladados:
    /// TODO: COMPROBAR-REQUISITOS-MINIMOS
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            parent,
            mut attrs,
            ..
        } = value;
        let zground = attrs.remove_f32("Z-GROUND")?;
        let area = attrs.remove_f32("AREA")?;
        let perimeter = attrs.remove_f32("PERIMETRO")?;
        let parent =
            parent.ok_or_else(|| format_err!("Muro interior sin espacio asociado '{}'", &name))?;
        let construction = attrs.remove_str("CONSTRUCTION")?;
        let location = attrs.remove_str("LOCATION").ok();
        let geometry = WallGeometry::get_wallgeometry(attrs)?;
        Ok(Self {
            name,
            zground,
            area,
            perimeter,
            parent,
            construction,
            location,
            geometry,
        })
    }
}

// Muro o soleras en contacto con el terreno (UNDERGROUND-WALL) --------

/// Sombra (BUILDING-SHADE)
#[derive(Debug, Clone, Default)]
pub struct Shade {
    /// Nombre
    pub name: String,
    /// Transmisividad de la radiación solar de la superficie (-)
    pub tran: f32,
    /// Reflectividad visible de la superficie (-)
    pub refl: f32,
    /// Coordenada X de la esquina inferior izquierda
    pub x: f32,
    /// Coordenada Y de la esquina inferior izquierda
    pub y: f32,
    /// Coordenada Z de la esquina inferior izquierda
    pub z: f32,
    /// Alto, en eje Y local de la superficie (m)
    pub height: f32,
    /// Ancho, en eje X local de la superficie (m)
    pub width: f32,
    /// Acimut (grados sexagesimales)
    /// Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del plano
    pub azimuth: f32,
    /// Inclinación (grados sexagesimales)
    /// Ángulo entre el eje Z del edificio y la proyección de la normal exterior del plano
    pub tilt: f32,
}

impl TryFrom<BdlBlock> for Shade {
    type Error = Error;

    /// Conversión de bloque BDL a sombra
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///     "patio1_lateral2" = BUILDING-SHADE
    ///         BULB-TRA = "Default.bulb"
    ///         BULB-REF = "Default.bulb"
    ///         TRAN     =              0
    ///         REFL     =            0.7
    ///         X        = 18.200001
    ///         Y        = 9.030000
    ///         Z        = 0.000000
    ///         HEIGHT   = 12.500000
    ///         WIDTH    = 3.500000
    ///         TILT     = 90.000000
    ///         AZIMUTH  = 180.000000
    ///         ..
    /// ```
    /// TODO: atributos no trasladados:
    /// TODO: BULB-TRA, BULB-REF
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let tran = attrs.remove_f32("TRAN")?;
        let refl = attrs.remove_f32("REFL")?;
        let x = attrs.remove_f32("X")?;
        let y = attrs.remove_f32("Y")?;
        let z = attrs.remove_f32("Z")?;
        let height = attrs.remove_f32("HEIGHT")?;
        let width = attrs.remove_f32("WIDTH")?;
        let azimuth = attrs.remove_f32("AZIMUTH")?;
        let tilt = attrs.remove_f32("TILT")?;
        Ok(Self {
            name,
            tran,
            refl,
            x,
            y,
            z,
            height,
            width,
            azimuth,
            tilt,
        })
    }
}
