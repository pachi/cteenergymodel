//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos geométricos o de zonificación:
//! - Plantas (FLOOR)
//! - Espacios (SPACE)
//! - Polígono (POLYGON)
//! - Vector

use std::convert::TryFrom;

use super::blocks::BdlBlock;
use super::types::AttrMap;

use failure::bail;
use failure::Error;

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
    /// TODO: usar parent en su lugar
    pub z: f32,
    /// nombres de los espacios que pertenecen a la planta
    /// TODO: podríamos eliminarlo y marcar en el espacio la planta a la que pertenece
    pub spaces: Vec<String>,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Floor {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let z = attrs.get_f32("Z").unwrap_or_default();
        Ok(Self {
            name,
            z,
            attrs,
            ..Default::default()
        })
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

impl TryFrom<BdlBlock> for Space {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let height = attrs.get_f32("HEIGHT").ok();
        let polygon = attrs.get("POLYGON")?.to_string();
        Ok(Self {
            name,
            height,
            polygon,
            attrs,
        })
    }
}

/// Polígono
/// 
/// Define la geometría, mediante el atributo POLYGON de:
/// - EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL
/// - FLOOR y SPACE
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
    pub vertices: Vec<Vertex>,
}

impl TryFrom<BdlBlock> for Polygon {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let mut vertices = Vec::new();
        for (name, vals) in &attrs.0 {
            let vector = vals.to_string().parse()?;
            vertices.push(Vertex {
                name: name.clone(),
                vector,
            })
        }
        Ok(Self { name, vertices })
    }
}

/// Vertex - Vértice, conjunto de nombre y vector
///
/// Ejemplo:
/// ```text
///     V1   =( 14.97, 11.39 )
/// ```
#[derive(Debug, Clone, Default)]
pub struct Vertex {
    name: String,
    vector: Vector,
}

/// Vector
///
/// Ejemplo:
/// ```text
///     ( 14.97, 11.39 )
/// ```
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

// - Composición de cerramiento (CONSTRUCTION) =================

/// Construcción - Remite a LAYERS (¿y otras opciones?)
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
    /// Tipo de definición de la construcción (LAYERS o U-VALUE)
    pub ctype: String,
    /// Definición de capas
    pub layers: Option<String>,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl TryFrom<BdlBlock> for Construction {
    type Error = Error;

    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { name, attrs, .. } = value;
        let ctype = attrs.get("TYPE")?.to_string();
        let layers = attrs.get("LAYERS").ok().map(|v| v.to_string());
        Ok(Self { name, ctype, layers, attrs })
    }
}
