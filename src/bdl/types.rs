//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::HashMap;

use failure::bail;
use failure::Error;

#[derive(Debug, Clone)]
pub enum BdlValue {
    String(String),
    Number(f32),
}

impl From<String> for BdlValue {
    fn from(val: String) -> Self {
        BdlValue::String(val.to_string())
    }
}

impl From<&str> for BdlValue {
    fn from(val: &str) -> Self {
        BdlValue::String(val.to_string())
    }
}

impl From<f32> for BdlValue {
    fn from(val: f32) -> Self {
        BdlValue::Number(val)
    }
}

impl std::fmt::Display for BdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            BdlValue::String(val) => write!(f, "{}", val),
            BdlValue::Number(val) => write!(f, "{}", val),
        }
    }
}

impl std::convert::TryFrom<BdlValue> for f32 {
    type Error = Error;

    fn try_from(value: BdlValue) -> Result<Self, Self::Error> {
        match value {
            BdlValue::Number(num) => Ok(num),
            _ => bail!("Valor numérico incorrecto: {:?}", value),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttrMap(pub HashMap<String, BdlValue>);

impl AttrMap {
    /// Constructor
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inserta valor v en la clave k y devuelve el valor existente o None
    pub fn insert<K: ToString>(&mut self, k: K, v: &str) -> Option<BdlValue> {
        let val: BdlValue = match v.parse::<f32>() {
            Ok(num) => BdlValue::Number(num),
            _ => BdlValue::String(v.trim().to_string()),
        };
        self.0.insert(k.to_string(), val)
    }

    /// Devuelve valor
    pub fn get(&self, attr: &str) -> Result<BdlValue, Error> {
        self.0
            .get(attr)
            .map(|v| v.to_owned())
            .ok_or_else(|| format_err!("Atributo inexistente: {}", attr))
    }

    /// Devuelve valor como número
    pub fn get_f32(&self, attr: &str) -> Result<f32, Error> {
        self.0
            .get(attr)
            .and_then(|v| match v {
                BdlValue::Number(num) => Some(*num),
                _ => None,
            })
            .ok_or_else(|| format_err!("Atributo inexistente o con valor incorrecto: {}", attr))
    }
}

// Objetos ----------------------------------------------------------------

// TODO: definir BdlBlock genérico y evitar redefinir tantos bloques iguales

#[derive(Clone, Debug, Default)]
pub struct BdlBlock {
    /// Tipo de bloque
    pub btype: String,
    /// Nombre del material
    pub name: String,
    // Elemento madre, referenciado por nombre
    pub parent: Option<String>,
    /// Conjunto de propiedades
    pub attrs: AttrMap,
}

/// Material definido por sus propiedades térmicas o por resistencia
///
/// Ejemplo en BDL:
/// ```text
///     "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm" = MATERIAL
///     TYPE              = PROPERTIES
///     THICKNESS         =           0.45
///     THICKNESS_CHANGE         = YES
///     THICKNESS_MAX         =              2
///     THICKNESS_MIN         =          0.001
///     CONDUCTIVITY      =      0.4787234
///     DENSITY           =           1280
///     SPECIFIC-HEAT     =           1000
///     VAPOUR-DIFFUSIVITY-FACTOR =             60
///     NAME          = "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm"
///     GROUP         = "Forjados reticulares"
///     IMAGE          = "ladrillo.bmp"
///     NAME_CALENER   = "oldeado descolgado -Canto 450 "
///     LIBRARY       = NO
///     UTIL          =  NO
///     OBSOLETE      = NO
///     ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Material {
    /// Nombre del material
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Tipo de material "RESISTANCE" o "PROPERTIES"
    pub mtype: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Material {
    pub fn new<N: ToString, T: ToString>(name: N, mtype: T) -> Self {
        Self {
            name: name.to_string(),
            mtype: mtype.to_string(),
            ..Default::default()
        }
    }
}

/// Construcción
///
/// Ejemplo:
/// ```text
///     "muro_opaco0.40" =  CONSTRUCTION
///     TYPE   = LAYERS  
///     LAYERS = "muro_opaco"
///     ABSORPTANCE = 0.400000
///     ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Construction {
    /// Nombre
    pub name: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Construction {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Definición de capas
///
/// Ejemplo:
/// ```text
///     "muro_opaco" = LAYERS
///         GROUP        = "envolvente"
///         NAME_CALENER = ""
///         NAME         = "muro_opaco"
///         TYPE-DEFINITION = 1
///         MATERIAL     = ("Mortero de cemento o cal para albañilería y para revoco/enlucido 1000 < d < 1250","EPS Poliestireno Expandido [ 0.029 W/[mK]]","1/2 pie LP métrico o catalán 80 mm< G < 100 mm","MW Lana mineral [0.031 W/[mK]]","Placa de yeso laminado [PYL] 750 < d < 900")
///         THICKNESS = (          0.015,           0.06,          0.115,           0.04,           0.02)
///         LIBRARY       =  NO
///         UTIL          =  YES
///         IMAGE = ""
///         DEFAULT = NO
///     ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Layers {
    /// Nombre
    pub name: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Layers {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Planta
///
/// Ejemplo:
/// ```text
///     "P01" = FLOOR
///     POLYGON       =  "P01_Poligono1"
///     FLOOR-HEIGHT  =            3.5
///     SPACE-HEIGHT  =            3.5
///     SHAPE         =  POLYGON
///     PREVIOUS      =  "Ninguna"
///     ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Floor {
    /// nombre de la planta
    pub name: String,
    /// cota de la planta, salvo que no se indique planta y se usa la del edificio
    /// TODO: deberíamos acceder a esto a través de una función que consulte la planta y el edificio
    pub z: f32,
    /// nombres de los espacios que pertenecen a la planta
    /// TODO: podríamos eliminarlo y marcar en el espacio la planta a la que pertenece
    pub spaces: Vec<String>,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Floor {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Espacio
///
/// Ejemplo:
/// ```text
///     "P01_E01" = SPACE
///     nCompleto = "P01_E01"
///     HEIGHT        =            3.5
///     SHAPE             = POLYGON
///     POLYGON           = "P01_E01_Pol2"
///     TYPE              = CONDITIONED
///     SPACE-TYPE        = "Residencial"
///     SYSTEM-CONDITIONS = "Residencial"
///     SPACE-CONDITIONS  = "Residencial"
///     FLOOR-WEIGHT      =              0
///     MULTIPLIER        = 1
///     MULTIPLIED        = 0
///     PILLARS-NUMBERS   = 0
///     FactorSuperficieUtil   = 1.0
///     perteneceALaEnvolventeTermica   = SI
///     INTERIOR-RADIATION  = FIXED
///     POWER     = 4.4
///     VEEI-OBJ  = 7.000000
///     VEEI-REF  = 10.000000
///     ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Space {
    /// Nombre del espacio
    pub name: String,
    /// Altura del espacio, si difiere de la de la planta
    /// TODO: deberíamos acceder a esto a través de una función que consulte el espacio y la planta
    pub height: Option<f32>,
    /// Nombre de polígono que define el espacio
    /// XXX: con SHAPE = POLIGON este valor tiene el polígono
    /// con SHAPE = BOX o BOX = NO-SHAPE se usan otras propiedades
    pub polygon: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Space {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Polígono
///
/// Ejemplo:
/// ```text
///     "P01_E01_Pol2" = POLYGON
///     V1   =( 14.97, 11.39 )
///     V2   =( 10.84, 11.39 )
///     V3   =( 10.86, 0 )
///     V4   =( 18.22, 0 )
///     V5   =( 18.22, 9.04 )
///     V6   =( 14.97, 9.04 )
///     ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Polygon {
    /// Nombre del polígono
    pub name: String,
    /// Lista de vectores que definen el polígono
    pub vectors: Vec<Vector>,
}

impl Polygon {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Vector
///
/// Ejemplo:
/// ```text
///     V1   =( 14.97, 11.39 )
/// ```text
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector {
    /// x
    pub x: f32,
    /// y
    pub y: f32,
}

impl std::str::FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Vector, Self::Err> {
        let x: &[_] = &[' ', '(', ')'];
        if let [x, y] = s
            .split(',')
            .map(|v| v.trim_matches(x))
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(Vector {
                x: x.parse::<f32>()?,
                y: y.parse::<f32>()?,
            })
        } else {
            bail!("Fallo al generar vector")
        }
    }
}

// Hueco
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

// Muro exterior
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

// Muro interior
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

// Muro o soleras en contacto con el terreno
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

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlElementType {
    Window(BdlBlock),
    ExteriorWall(BdlBlock),
    InteriorWall(BdlBlock),
    UndergroundWall(BdlBlock),
    Construction(Construction),
    Layers(Layers),
}
