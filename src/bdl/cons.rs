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
use super::AttrMap;

/// Interpreta lista de nombres con formato "("mat1", "mat2", "mat3", ...)"
fn extract_namesvec<S: AsRef<str>>(input: S) -> Vec<String> {
    input
        .as_ref()
        .trim_matches(&[' ', '(', ')'] as &[_])
        .split('"')
        .map(str::trim)
        .filter(|v| *v != "," && *v != "")
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
}

/// Interpreta lista de valores con formato "(num1, num2, num3, ...)"
fn extract_f32vec<S: AsRef<str> + std::fmt::Debug>(input: S) -> Result<Vec<f32>, Error> {
    input
        .as_ref()
        .trim_matches(&[' ', '(', ')'] as &[_])
        .split(',')
        .map(|v| {
            v.trim()
                .parse::<f32>()
                .map_err(|_| format_err!("Error al convertir {}", v))
        })
        .collect::<Result<Vec<f32>, _>>()
        .map_err(|_| format_err!("Error en la conversión numérica de {:?}", input))
}

/// BBDD
/// - Opacos
///      - Materiales y productos (MATERIAL (tipo PROPERTIES o RESISTANCE) -> group)
///      - Cerramientos y particiones (LAYERS -> group)
/// - Semitransparentes
///      - Vidrios (GLASS-TYPE) -> group
///      - Marcos (NAME-FRAME) -> group
///      - Huecos y lucernarios (GAP) -> group
///  - Puentes térmicos (THERMAL-BRIDGE)?
///      - grupo

/// Elementos constructivos y de materiales pertenecientes a la base de datos
/// Se organizan por nombre y grupo (tipo)
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
    /// Definición detallada de propiedades
    pub properties: Option<MaterialProperties>,
    /// Definición de resistencia térmica
    pub resistance: Option<MaterialResistance>,
    // Resto de propiedades
    // THICKNESS_CHANGE, THICKNESS_MAX, THICKNESS_MIN, IMAGE, NAME_CALENER, LIBRARY, UTIL, OBSOLETE
    // pub attrs: AttrMap,
}

/// Definición de propiedades termofísicas y grosor
#[derive(Debug, Copy, Clone, Default)]
pub struct MaterialProperties {
    /// Grosor (m)
    pub thickness: f32,
    /// Conductividad térmica
    pub conductivity: f32,
    /// Densidad
    pub density: f32,
    /// Calor específico
    pub specificheat: f32,
    /// Factor de difusividad al vapor de agua
    pub vapourdiffusivity: f32,
}

/// Definición por resistencia térmica
#[derive(Debug, Copy, Clone, Default)]
pub struct MaterialResistance {
    /// Resistencia térmica ???
    pub resistance: f32,
}

impl TryFrom<BdlBlock> for Material {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let group = attrs.remove_str("GROUP")?;
        let (properties, resistance) = match attrs.remove_str("TYPE")?.as_ref() {
            "PROPERTIES" => {
                let thickness = attrs.remove_f32("THICKNESS")?;
                let conductivity = attrs.remove_f32("CONDUCTIVITY")?;
                let density = attrs.remove_f32("DENSITY")?;
                let specificheat = attrs.remove_f32("SPECIFIC-HEAT")?;
                let vapourdiffusivity = attrs.remove_f32("VAPOUR-DIFFUSIVITY-FACTOR")?;
                (
                    Some(MaterialProperties {
                        thickness,
                        conductivity,
                        density,
                        specificheat,
                        vapourdiffusivity,
                    }),
                    None,
                )
            }
            _ => {
                let resistance = attrs.remove_f32("RESISTANCE")?;
                (None, Some(MaterialResistance { resistance }))
            }
        };
        Ok(Self {
            name,
            group,
            properties,
            resistance,
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
    /// Lista de nombres de materiales de las capas
    pub material: Vec<String>,
    /// Lista de espesores de las capas
    pub thickness: Vec<f32>,
    // Resto de propiedades
    // IMAGE, NAME_CALENER, LIBRARY, UTIL, TYPE-DEFINITION, DEFAULT
    // pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Layers {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let group = attrs.remove_str("GROUP")?;
        let material = extract_namesvec(attrs.remove_str("MATERIAL")?);
        let thickness = extract_f32vec(attrs.remove_str("THICKNESS")?)?;
        Ok(Self {
            name,
            group,
            material,
            thickness,
        })
    }
}

/// Hueco o lucernario (GAP)
///
/// Grupo y tipo vidrio: GROUP-GLASS y GLASS-TYPE
/// Grupo y tipo marco: GROUP-FRAME y NAME-FRRAME
///
/// Porcentaje cubierto por el marco en PORCENTAGE (%)
/// Porcentaje de incremento de U en porcentajeIncrementoU (%)
/// Permeabilidad al aire en INF-COEF (m3/hm2)
///
/// Ejemplo:
/// ```text
///      "ventana estandar" = GAP
///           NAME           = "ventana estandar"
///           TYPE           = 1
///           GROUP          = "muro_cortina"
///           GROUP-GLASS         = "Vidrios"
///           GLASS-TYPE          = "Doble baja emisividad argon"
///           GROUP-FRAME       = "Metálicos en posición vertical"
///           NAME-FRAME        = "VER_Con rotura de puente térmico mayor de 12 mm"
///           PORCENTAGE        = 20.000000
///           INF-COEF          = 9.000000
///           porcentajeIncrementoU = 10.000000
///           NAME_CALENER      = ""
///          VIGENCIA = ( "A", "B", "C", "D", "E", "F")
///          IMAGE = ""
///           TRANSMITANCIA       =            5.7
///           SHADING-COEF        =           0.86
///           SHADE-COEF-SUMMER   =              1
///           SHADE-COEF-WINTER   =              1
///           MARKER-SUMMER       =              1
///           MARKER-WINTER       =              1
///           LIBRARY           =  NO
///           UTIL              =  YES
///           ISDOOR            = NO
///           DEFAULT           = NO
///          ..
/// ```
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
    /// Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Gap {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let group = attrs.remove_str("GROUP")?;
        let glass = attrs.remove_str("GLASS-TYPE")?;
        let frame = attrs.remove_str("NAME-FRAME")?;
        let framepct = attrs.remove_f32("PORCENTAGE")?;
        let inf = attrs.remove_f32("INF-COEF")?;
        let deltau = attrs
            .remove_f32("porcentajeIncrementoU")
            .unwrap_or_default();
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

/// Tipo de marco (NAME-FRAME)
///
/// Conductividad en FRAME-CONDUCT (W/m2K)
/// Absortividad(alpha) en FRAME-ABS (-)
///
/// Ejemplo:
/// ```text
///      "Marco PVC_1" = NAME-FRAME
///      GROUP         = "Marcos HULC2020"
///      FRAME-WIDTH   =            0.1
///      FRAME-CONDUCT =            1.3
///      FRAME-ABS     =            0.7
///      NAME_CALENER  = ""
///      LIBRARY       = NO
///      UTIL          =  NO
///      ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct Frame {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Conductividad W/m2K
    pub conductivity: f32,
    /// Absortividad del marco -
    pub absorptivity: f32,
    /// Ancho del marco
    pub width: f32,
    // Resto de propiedades
    // NAME-CALENER, LIBRRARY, UTIL
    // pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Frame {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let group = attrs.remove_str("GROUP")?;
        let conductivity = attrs.remove_f32("FRAME-CONDUCT")?;
        let absorptivity = attrs.remove_f32("FRAME-ABS")?;
        let width = attrs.remove_f32("FRAME-WIDTH")?;
        Ok(Self {
            name,
            group,
            conductivity,
            absorptivity,
            width,
        })
    }
}

/// Tipo de vidrio (GLASS-TYPE)
///
/// Conductividad en GLASS-CONDUCTANCE (W/m2K)
/// Factor solar (g) en SHADING-COEF * 0.85 (-)
///
/// Ejemplo:
/// ```text
///      "Vidrio Triple Bajo Emisivo" = GLASS-TYPE
///           GROUP             = "Vidrios HULC2020"
///           TYPE              = SHADING-COEF
///           SHADING-COEF      =      0.5882353
///           GLASS-CONDUCTANCE =           1.25
///           NAME_CALENER      = ""
///           LIBRARY       =  NO
///           UTIL          =  NO
///           ..
/// ```
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
    // Resto de propiedades
    // pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Glass {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        if attrs.remove_str("TYPE")? != "SHADING-COEF".to_string() {
            bail!(
                "Definición de vidrio por código no soportada en '{}'",
                &name
            );
        };
        let group = attrs.remove_str("GROUP")?;
        let conductivity = attrs.remove_f32("GLASS-CONDUCTANCE")?;
        // TODO: Guardamos esto o el valor a incidencia normal?
        // TODO: comprobar el 0.85 (el Fw en la 52016 es 0.90)
        let shadingcoef = attrs.remove_f32("SHADING-COEF")? * 0.85;
        Ok(Self {
            name,
            group,
            conductivity,
            shadingcoef,
        })
    }
}

/// Puente térmico (THERMAL-BRIDGE)
///
/// Se pueden de definir (DEFINICION) por defecto (1), por usuario (2) o por catálogo (3?)
///
/// Ejemplo:
/// ```text
///      "LONGITUDES_CALCULADAS" = THERMAL-BRIDGE
///            LONG-TOTAL = 0.000000
///            DEFINICION = 1
///          ..
///      "FRENTE_FORJADO" = THERMAL-BRIDGE
///            LONG-TOTAL = 171.629913
///            DEFINICION = 2
///            TTL    = 0.080000
///            FRSI        = 0.45
///            ANGLE-MIN   = 135
///            ANGLE-MAX   = 225
///            TYPE        = SLAB
///            PARTITION   = YES
///          ..
/// ```
#[derive(Debug, Clone, Default)]
pub struct ThermalBridge {
    /// Nombre
    pub name: String,
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
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let definition = attrs.remove_str("DEFINICION").unwrap_or_default();
        let length = attrs.remove_f32("LONG-TOTAL")?;
        let psi = attrs.remove_f32("TTL").ok();
        let tbtype = attrs.remove_str("TYPE").ok();
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
