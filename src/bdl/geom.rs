//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos geométricos o de zonificación:
//! - Plantas (FLOOR)
//! - Espacios (SPACE)
//! - Polígono (POLYGON)
//! - Vector

use std::convert::TryFrom;

use super::blocks::BdlBlock;

use failure::bail;
use failure::Error;

/// Planta (agrupación de espacios)
#[derive(Debug, Clone, Default)]
pub struct Floor {
    /// Nombre de la planta
    pub name: String,
    /// Cota de la planta en el sistema coordenado del edificio
    /// XXX: podría no aparecer y ser la de la del edificio
    pub z: f32,
    /// Polígono que define la geometría
    pub polygon: String,
    /// Altura suelo a suelo, incluyendo los plenum
    pub floorheight: f32,
    /// Altura libre (la altura de los espacios de tipo PLENUM es floorheight - spaceheight)
    pub spaceheight: f32,
    /// Planta anterior (inferior)
    pub previous: String,
}

impl TryFrom<BdlBlock> for Floor {
    type Error = Error;

    /// Convierte bloque BDL a planta (agrupación de espacios)
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
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        // De los tipos de definición de la geometría: POLYGON, BOX, NO-SHAPE
        // solamente manejamos definiciones por polígono
        if attrs.remove_str("SHAPE")? != "POLYGON" {
            bail!("Planta '{}' de tipo distinto a POLYGON, no soportado");
        };
        let z = attrs.remove_f32("Z").unwrap_or_default();
        let polygon = attrs.remove_str("POLYGON")?;
        let floorheight = attrs.remove_f32("FLOOR-HEIGHT")?;
        let spaceheight = attrs.remove_f32("SPACE-HEIGHT")?;
        let previous = attrs.remove_str("PREVIOUS")?;
        Ok(Self {
            name,
            z,
            polygon,
            floorheight,
            spaceheight,
            previous,
        })
    }
}

/// Espacio
#[derive(Debug, Clone, Default)]
pub struct Space {
    /// Nombre del espacio
    pub name: String,
    /// Tipo de espacio (CONDITIONED, UNHABITED, ¿UNCONDITIONED?, ¿PLENUM?)
    pub stype: String,
    /// Nombre de polígono que define el espacio
    /// XXX: Solo vale para SHAPE = POLIGON (no vale con BOX o NO-SHAPE)
    pub polygon: String,
    /// Altura del espacio, (o None si es la de la planta)
    pub height: Option<f32>,
    /// Pertenencia a la envolvente térmica
    pub insidete: bool,
    /// Planta a la que pertenece el espacio
    pub parent: String,
    /// Potencia de iluminación (W/m2)
    pub power: f32,
    /// VEEI del edificio objeto W/m2/100lux
    pub veeiobj: f32,
    /// VEEI del edificio de referencia W/m2/100lux
    pub veeiref: f32,
    /// Tipo de espacio
    pub spacetype: String,
    /// Condiciones de uso del espacio
    pub spaceconds: String,
    /// Condiciones de operación de los sistemas
    pub systemconds: String,
    /// Multiplicador
    pub multiplier: f32,
    /// Si es un espacio multiplicado
    pub ismultiplied: bool,
}

impl TryFrom<BdlBlock> for Space {
    type Error = Error;

    /// Convierte de Bloque BDL a espacio
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
    /// TODO: propiedades no convertidas:
    /// TODO: PILLARS-NUMBERS (número de pilares en el espacio, como PTs),
    /// TODO: FactorSuperficieUtil, INTERIOR-RADIATION, nCompleto, FLOOR-WEIGHT
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            mut attrs,
            parent,
            ..
        } = value;
        // XXX: por ahora solo soportamos definición del espacios por polígono
        if attrs.remove_str("SHAPE")? != "POLYGON" {
            bail!(
                "Tipo de espacio desconocido (no definido por polígno): {}",
                name
            )
        };

        let stype = attrs.remove_str("TYPE")?;
        let polygon = attrs.remove_str("POLYGON")?;
        let height = attrs.remove_f32("HEIGHT").ok();
        let insidete = attrs
            .remove_str("perteneceALaEnvolventeTermica")
            .ok()
            .and_then(|v| if v == "SI" { Some(true) } else { Some(false) })
            // TODO: En archivos antiguos, sin ese parámetro miramos si es acondicionado
            // TODO: En teoría también podría haber habitables no acondicionados
            .or_else(|| match stype.as_ref() {
                "CONDITIONED" => Some(true),
                _ => Some(false),
            })
            .unwrap_or(false);
        let parent = parent.ok_or_else(|| {
            format_err!(
                "No se encuentra la referencia de la planta en el espacio {}",
                name
            )
        })?;
        let power = attrs.remove_f32("POWER")?;
        let veeiobj = attrs.remove_f32("VEEI-OBJ")?;
        let veeiref = attrs.remove_f32("VEEI-REF")?;
        let spacetype = attrs.remove_str("SPACE-TYPE")?;
        let spaceconds = attrs.remove_str("SPACE-CONDITIONS")?;
        let systemconds = attrs.remove_str("SYSTEM-CONDITIONS")?;
        let multiplier = attrs.remove_f32("MULTIPLIER")?;
        let ismultiplied = attrs.remove_f32("MULTIPLIED")? == 1.0;

        Ok(Self {
            name,
            stype,
            polygon,
            height,
            insidete,
            parent,
            power,
            veeiobj,
            veeiref,
            spacetype,
            spaceconds,
            systemconds,
            multiplier,
            ismultiplied,
        })
    }
}

/// Polígono
/// TODO: ver sisgen/libreria_sisgen/claseEdificio.py para áreas, normal, etc.
#[derive(Debug, Clone, Default)]
pub struct Polygon {
    /// Nombre del polígono
    pub name: String,
    /// Lista de vectores que definen el polígono
    pub vertices: Vec<Vertex2D>,
}

impl Polygon {
    /// Calcula área del polígono definido por vértices
    pub fn area(&self) -> f32 {
        // https://www.mathopenref.com/coordpolygonarea2.html
        // https://www.mathopenref.com/coordpolygonarea.html
        // 0.5 * ( \SUM( x_i * y_i+1 - y_i * x_i+1)_(i = de 1 a n) + (x_n * y_1 - y_n * x_1) )
        let vertices = &self.vertices;
        let nverts = vertices.len();

        let mut area = 0.0;
        for i in 0..nverts {
            let nexti = (i+1) % nverts; // el último vértice vuelve a cero
            let vi = &vertices[i].vector;
            let vj = &vertices[nexti].vector;
            area += vi.x * vj.y - vi.y * vj.x;
        }

        f32::abs(0.5 * area)
    }
}

impl TryFrom<BdlBlock> for Polygon {
    type Error = Error;

    /// Convierte de bloque BDL a polígono
    ///
    /// Define la geometría, mediante el atributo POLYGON de:
    /// - EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL, FLOOR y SPACE
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
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let mut vertices = Vec::new();
        for i in 1.. {
            let name = format!("V{}", i);
            if let Some(vdata) = attrs.remove_str(&name).ok() {
                vertices.push(Vertex2D {
                    name,
                    vector: vdata.parse()?,
                });
            } else {
                break;
            }
        }
        Ok(Self { name, vertices })
    }
}

/// Vertex2D - Vértice, conjunto de nombre y vector 2d (x, y)
#[derive(Debug, Clone, Default)]
pub struct Vertex2D {
    /// Nombre del vértice
    pub name: String,
    /// Coordenadas del vértice
    pub vector: Vector2D,
}

/// Vertex3D - Vértice, conjunto de nombre y vector 3d (x, y, z)
#[derive(Debug, Clone, Default)]
pub struct Vertex3D {
    /// Nombre del vértice
    pub name: String,
    /// Coordenadas del vértice
    pub vector: Vector3D,
}

/// Vector 2D (x,y)
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector2D {
    /// Coordenada x
    pub x: f32,
    /// Coordenada y
    pub y: f32,
}

impl std::str::FromStr for Vector2D {
    type Err = Error;

    /// Convierte de cadena a vector de coordenadas
    ///
    /// Ejemplo:
    /// ```text
    ///     ( 14.97, 11.39 )
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [x, y] = s
            .split(',')
            .map(|v| v.trim_matches(&[' ', '(', ')'] as &[_]))
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(Self {
                x: x.parse::<f32>()?,
                y: y.parse::<f32>()?,
            })
        } else {
            bail!("Fallo al generar vector 2D con los datos '{}'", s)
        }
    }
}

/// Vector 3D (x,y,z)
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3D {
    /// Coordenada x
    pub x: f32,
    /// Coordenada y
    pub y: f32,
    /// Coordenada z
    pub z: f32,
}

impl std::str::FromStr for Vector3D {
    type Err = Error;

    /// Convierte de cadena a vector de coordenadas
    ///
    /// Ejemplo:
    /// ```text
    ///     ( 14.97, 11.39, 2.0 )
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [x, y, z] = s
            .split(',')
            .map(|v| v.trim_matches(&[' ', '(', ')'] as &[_]))
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(Self {
                x: x.parse::<f32>()?,
                y: y.parse::<f32>()?,
                z: z.parse::<f32>()?,
            })
        } else {
            bail!("Fallo al generar vector 3D con los datos '{}'", s)
        }
    }
}
// - Composición de cerramiento (CONSTRUCTION) =================

/// Construcción - Remite a LAYERS (¿y otras opciones?)
#[derive(Debug, Clone, Default)]
pub struct Construction {
    /// Nombre
    pub name: String,
    /// Tipo de definición de la construcción (LAYERS o U-VALUE)
    pub ctype: String,
    /// Elemento vinculado (muro, etc)
    pub parent: String,
    /// Definición de capas, cuando ctype es LAYERS
    pub layers: Option<String>,
    /// Transmitancia, cuando ctype es U-VALUE (W/m2K)
    pub uvalue: Option<f32>,
    /// Absortividad (a la radiación solar) (-)
    pub absorptance: Option<f32>,
    /// Rugosidad (1 a 6)
    pub roughness: Option<f32>,
    /// Nombre de parámetros de muro (WALL-PARAMETERS)
    pub wallparameters: Option<String>,
}

impl TryFrom<BdlBlock> for Construction {
    type Error = Error;

    /// Convierte de bloque BDL a construcción - Remite a LAYERS (¿y otras opciones?)
    ///
    /// Ejemplo:
    /// ```text
    ///     "muro_opaco0.40" =  CONSTRUCTION
    ///     TYPE   = LAYERS  
    ///     LAYERS = "muro_opaco"
    ///     ABSORPTANCE = 0.400000
    ///     ..
    /// ```
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            mut attrs,
            parent,
            ..
        } = value;
        let ctype = attrs.remove_str("TYPE")?;
        let layers = attrs.remove_str("LAYERS").ok();
        let uvalue = attrs.remove_f32("U-VALUE").ok();
        let absorptance = attrs.remove_f32("ABSORPTANCE").ok();
        let roughness = attrs.remove_f32("ROUGHNESS").ok();
        let wallparameters = attrs.remove_str("WALL-PARAMETERS").ok();
        let parent = parent.ok_or_else(|| {
            format_err!(
                "No se encuentra la referencia al elemento en la construcción {}",
                name
            )
        })?;
        Ok(Self {
            name,
            ctype,
            parent,
            layers,
            uvalue,
            absorptance,
            roughness,
            wallparameters,
        })
    }
}