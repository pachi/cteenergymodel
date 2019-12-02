//! Parser del Building Description Language (BDL) de DOE
//!
//! Composiciones constructivas y de materiales
//!
//! - Material (MATERIAL)
//! - Acristalamiento (GLASS-TYPE)
//! - Marco (NAME-FRAME)
//! - Cerramiento (LAYERS)
//! - Hueco (GAP)

use std::convert::TryFrom;

use failure::Error;

use super::blocks::BdlBlock;
use super::types::AttrMap;

// BBDD
// - Opacos
//      - Materiales y productos (MATERIAL (tipo PROPERTIES o RESISTANCE) -> group)
//      - Cerramientos y particiones (LAYERS -> group)
// - Semitransparentes
//      - Vidrios (GLASS-TYPE) -> group
//      - Marcos (NAME-FRAME) -> group
//      - Huecos y lucernarios (GAP) -> group
//  - Puentes térmicos (THERMAL-BRIDGE)?
//      - grupo

/// Elementos constructivos y de materiales pertenecientes a la base de datos
#[derive(Debug)]
pub enum BdlDB {
    /// Material o producto
    Material(Material),
    /// Composición por capas (opacos)
    Layers(Layers),
    /// Composición por capas (huecos)
    Gap(Gap),
    /// Vidrio
    Glass(Glass),
    /// Marco
    Frame(Frame),
    /// Puente térmico
    ThermalBridge(ThermalBridge),
}

// TODO: dudas sobre si incluir o no cosas que no tienen grupo en la BBDD de construcciones

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

impl TryFrom<BdlBlock> for Material {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let group = attrs.get("GROUP")?.to_string();
        let mtype = attrs.get("TYPE")?.to_string();
        Ok(Self {
            name,
            group,
            mtype,
            attrs,
        })
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
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Layers {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let group = attrs.get("GROUP")?.to_string();
        Ok(Self { name, group, attrs })
    }
}

// Hueco o lucernario (GAP)
//
// Grupo y tipo vidrio: GROUP-GLASS y GLASS-TYPE
// Grupo y tipo marco: GROUP-FRAME y NAME-FRRAME
//
// Porcentaje cubierto por el marco en PORCENTAGE (%)
// Porcentaje de incremento de U en porcentajeIncrementoU (%)
// Permeabilidad al aire en INF-COEF (m3/hm2)
//
// Ejemplo:
// ```text
//      "ventana estandar" = GAP
//           NAME           = "ventana estandar"
//           TYPE           = 1
//           GROUP          = "muro_cortina"
//           GROUP-GLASS         = "Vidrios"
//           GLASS-TYPE          = "Doble baja emisividad argon"
//           GROUP-FRAME       = "Metálicos en posición vertical"
//           NAME-FRAME        = "VER_Con rotura de puente térmico mayor de 12 mm"
//           PORCENTAGE        = 20.000000
//           INF-COEF          = 9.000000
//           porcentajeIncrementoU = 10.000000
//           NAME_CALENER      = ""
//          VIGENCIA = ( "A", "B", "C", "D", "E", "F")
//          IMAGE = ""
//           TRANSMITANCIA       =            5.7
//           SHADING-COEF        =           0.86
//           SHADE-COEF-SUMMER   =              1
//           SHADE-COEF-WINTER   =              1
//           MARKER-SUMMER       =              1
//           MARKER-WINTER       =              1
//           LIBRARY           =  NO
//           UTIL              =  YES
//           ISDOOR            = NO
//           DEFAULT           = NO
//          ..
// ```
#[derive(Debug, Clone, Default)]
pub struct Gap {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Acristalamiento (GLASS-TYPE)
    pub glass: String,
    /// Marco (NAME-FRAME)
    pub frame: String,
    /// Porcentaje de hueco cubierto por el marco (%)
    pub framepct: f32,
    /// Permeabilidad al aire (m3/hm2 a 100Pa??)
    pub inf: f32,
    /// Porcentaje de U debido a intercalarios y cajón de persiana (%)
    pub deltau: f32,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Gap {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let group = attrs.get("GROUP")?.to_string();
        let glass = attrs.get("GLASS-TYPE")?.to_string();
        let frame = attrs.get("NAME-FRAME")?.to_string();
        let framepct = attrs.get_f32("PORCENTAGE")?;
        let inf = attrs.get_f32("INF-COEF")?;
        let deltau = attrs.get_f32("porcentajeIncrementoU").unwrap_or_default();
        Ok(Self {
            name,
            group,
            glass,
            frame,
            framepct,
            inf,
            deltau,
            attrs,
        })
    }
}

// Tipo de marco (NAME-FRAME)
//
// Conductividad en FRAME-CONDUCT (W/m2K)
// Absortividad(alpha) en FRAME-ABS (-)
//
// Ejemplo:
// ```text
//      "Marco PVC_1" = NAME-FRAME
//      GROUP         = "Marcos HULC2020"
//      FRAME-WIDTH   =            0.1
//      FRAME-CONDUCT =            1.3
//      FRAME-ABS     =            0.7
//      NAME_CALENER  = ""
//                LIBRARY       = NO
//      UTIL          =  NO
//      ..
// ```
#[derive(Debug, Clone, Default)]
pub struct Frame {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Conductividad W/m2K
    pub conductivity: f32,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Frame {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let group = attrs.get("GROUP")?.to_string();
        let conductivity = attrs.get_f32("FRAME-CONDUCT")?;
        Ok(Self {
            name,
            group,
            conductivity,
            attrs,
        })
    }
}

// Tipo de vidrio (GLASS-TYPE)
//
// Conductividad en GLASS-CONDUCTANCE (W/m2K)
// Factor solar (g) en SHADING-COEF * 0.85 (-)
//
// Ejemplo:
// ```text
//      "Vidrio Triple Bajo Emisivo" = GLASS-TYPE
//           GROUP             = "Vidrios HULC2020"
//           TYPE              = SHADING-COEF
//           SHADING-COEF      =      0.5882353
//           GLASS-CONDUCTANCE =           1.25
//           NAME_CALENER      = ""
//           LIBRARY       =  NO
//          UTIL          =  NO
//          ..
// ```
#[derive(Debug, Clone, Default)]
pub struct Glass {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Conductividad W/m2K (GLASS-CONDUCTANCE)
    pub conductivity: f32,
    /// Factor solar a incidencia normal - (SHADING-COEF)
    pub shadingcoef: f32,
    /// Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Glass {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let group = attrs.get("GROUP")?.to_string();
        let conductivity = attrs.get_f32("GLASS-CONDUCTANCE")?;
        // TODO: Guardamos esto o el valor a incidencia normal?
        // TODO: comprobar el 0.85 (el Fw en la 52016 es 0.90)
        let shadingcoef = attrs.get_f32("SHADING-COEF")? * 0.85;
        Ok(Self {
            name,
            group,
            conductivity,
            shadingcoef,
            attrs,
        })
    }
}

// Puente térmico (THERMAL-BRIDGE)
//
// Se pueden de definir (DEFINICION) por defecto (1), por usuario (2) o por catálogo (3?)
//
// Ejemplo:
// ```text
//      "LONGITUDES_CALCULADAS" = THERMAL-BRIDGE
//            LONG-TOTAL = 0.000000
//            DEFINICION = 1
//          ..
//      "FRENTE_FORJADO" = THERMAL-BRIDGE
//            LONG-TOTAL = 171.629913
//            DEFINICION = 2
//            TTL    = 0.080000
//            FRSI        = 0.45
//            ANGLE-MIN   = 135
//            ANGLE-MAX   = 225
//            TYPE        = SLAB
//            PARTITION   = YES
//          ..
// ```
#[derive(Debug, Clone, Default)]
pub struct ThermalBridge {
    /// Nombre
    pub name: String,
    // XXX: no existen grupos en puentes térmicos
    // Grupo al que pertenece (biblioteca)
    //pub group: String,
    /// Definición: por defecto (1), usuario (2), catálogo (3) ???
    pub definition: String,
    /// Longitud total (m)
    pub length: f32,
    /// Transmitancia térmica W/mK
    pub psi: Option<f32>,
    /// Tipo de puente térmico (SLAB, ...)
    pub tbtype: Option<String>,
    /// Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for ThermalBridge {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        // let group = attrs.get("GROUP")?.to_string();
        let definition = attrs.get("DEFINICION")?.to_string();
        let length = attrs.get_f32("LONG-TOTAL")?;
        let psi = attrs.get_f32("TTL").ok();
        let tbtype = attrs.get("TYPE").ok().map(|v| v.to_string());
        Ok(Self {
            name,
            // group,
            definition,
            length,
            psi,
            tbtype,
            attrs,
        })
    }
}
