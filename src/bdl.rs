//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf

use failure::bail;
use failure::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AttrMap(HashMap<String, String>);

impl AttrMap {
    /// Constructor
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inserta valor v en la clave k y devuelve el valor existente o None
    pub fn insert<K: ToString, V: ToString>(&mut self, k: K, v: V) -> Option<String> {
        self.0.insert(k.to_string(), v.to_string())
    }

    /// Devuelve valor como cadena
    pub fn get(&self, attr: &str) -> Result<String, Error> {
        self.0
            .get(attr)
            .map(|v| v.trim().to_string())
            .ok_or_else(|| format_err!("Atributo inexistente: {}", attr))
    }

    /// Devuelve valor como número
    pub fn get_f32(&self, attr: &str) -> Result<f32, Error> {
        self.0
            .get(attr)
            .and_then(|v| v.parse::<f32>().ok())
            .ok_or_else(|| format_err!("Atributo inexistente o con valor incorrecto: {}", attr))
    }
}

// Objetos

#[derive(Debug)]
pub enum BdlType {
    /// Material
    Material,
    // Layers,
    /// Espacio
    Space,
    /// Planta
    Floor,
    /// Polígono
    Polygon,
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
    pub z: f32,
    /// nombres de los espacios que pertenecen a la planta
    pub spaces: Vec<String>,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Floor {
    pub fn new(name: &str) -> Self {
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
    pub height: Option<f32>,
    /// Nombre de polígono que define el espacio
    /// XXX: con SHAPE = POLIGON este valor tiene el polígono
    /// con SHAPE = BOX o BOX = NO-SHAPE se usan otras propiedades
    pub polygon: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Space {
    pub fn new(name: &str) -> Self {
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
    pub fn new(name: &str) -> Self {
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
        dbg!("Vector: {}", s);
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

// ------------------------- BDL ----------------------------

/// Datos del edificio
#[derive(Debug, Default)]
pub struct BdlBuildingData {
    /// Lista de plantas
    pub floors: Vec<Floor>,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Lista de polígonos
    pub polygons: HashMap<String, Polygon>,
    /// Materiales
    pub materials: Vec<Material>,
    // Construcciones
    // pub constructions: Vec<Construction>
}

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct BdlData {
    pub building: BdlBuildingData,
}

impl BdlData {
    pub fn new(input: &str) -> Self {
        // Datos
        let mut bdldata: BdlBuildingData = Default::default();

        // Elimina líneas en blanco y comentarios, y luego separa por bloques
        let cleanlines = input
            .replace("\r\n", "\n")
            .lines()
            .map(str::trim)
            .filter(|l| *l != "" && !l.starts_with("$"))
            .collect::<Vec<&str>>()
            .join("\n");

        // TODO: parsear y guardar _lider_part en algún lado
        let [_lider_part, bdl_part] = match cleanlines
            .splitn(2, "TEMPLARY = USER")
            .collect::<Vec<_>>()
            .as_slice() {
                [lider_part, bdl_part] => [*lider_part, *bdl_part],
                _ => panic!("Error en la estructura de datos. No se han encontrado los datos de LIDER y de USARIO")
            };

        // Parsea bloques
        for block in bdl_part.split("..").map(str::trim) {
            let stanza: Vec<_> = block.splitn(2, '\n').map(str::trim).collect();
            let (bheadline, bdata) = match stanza.as_slice() {
                [bheadline, bdata] => (bheadline, bdata),
                [""] => continue,
                _ => panic!("Error al parsear el bloque: '{:?}'", stanza),
            };

            let (bname, btype) = match bheadline.rsplitn(2, '=').collect::<Vec<_>>().as_slice() {
                [bname, btype] => (bname.trim_matches(|c| c == ' ' || c == '"'), btype.trim()),
                _ => panic!("Error al parsear encabezado: {}", bheadline),
            };

            match btype {
                "MATERIAL" => {
                    bdldata
                        .materials
                        .push(parse_material(bname, bdata).unwrap());
                }
                "FLOOR" => {
                    bdldata.floors.push(parse_floor(bname, bdata).unwrap());
                }
                "SPACE" => {
                    bdldata.spaces.push(parse_space(bname, bdata).unwrap());
                    // Asigna el espacio a la planta actual
                    if let Some(mut lastfloor) = bdldata.floors.pop() {
                        lastfloor.spaces.push(bname.to_string());
                        bdldata.floors.push(lastfloor);
                    };
                }
                "POLYGON" => {
                    bdldata
                        .polygons
                        .insert(bname.to_string(), parse_polygon(bname, bdata).unwrap());
                }
                _ => {
                    eprintln!("Tipo desconocido");
                }
            };
        }

        Self { building: bdldata }
    }
}

fn parse_attributes(data: &str) -> AttrMap {
    let mut attributes = AttrMap::new();
    data.lines().for_each(|l| {
        if let [key, value] = l.split('=').map(str::trim).collect::<Vec<_>>().as_slice() {
            attributes.insert(key, value);
        }
    });
    attributes
}

fn parse_material(name: &str, data: &str) -> Result<Material, Error> {
    dbg!("Material: {}", name);
    let attr = parse_attributes(data);

    let name = attr.get("NAME")?;
    let mtype = attr.get("TYPE")?;

    let mut material = Material::new(name, mtype);
    material.group = attr.get("GROUP")?;
    material.attrs = attr;
    Ok(material)
}

fn parse_floor(name: &str, data: &str) -> Result<Floor, Error> {
    dbg!("Floor: {}", name);
    let mut floor = Floor::new(name);
    let attr = parse_attributes(data);
    floor.z = attr.get_f32("Z").unwrap_or_default();
    floor.attrs = attr;
    Ok(floor)
}

fn parse_space(name: &str, data: &str) -> Result<Space, Error> {
    dbg!(name);
    let mut space = Space::new(name);
    let attr = parse_attributes(data);
    space.polygon = attr.get("POLYGON")?;
    space.height = attr.get_f32("HEIGHT").ok();
    space.attrs = attr;
    Ok(space)
}

fn parse_polygon(name: &str, data: &str) -> Result<Polygon, Error> {
    dbg!("Polygon: {}", name);
    let mut polygon = Polygon::new(name);
    for (_k, v) in &parse_attributes(data).0 {
        let vec = v.parse()?;
        polygon.vectors.push(vec)
    }
    Ok(polygon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctehexml::parse;

    #[test]
    fn test_bdl() {
        let data = parse("tests/00_plurif_s3_v0_d3/00_plurif_s3_v0_d3.ctehexml").unwrap();
        let bdldata = BdlData::new(&data.entrada_grafica_lider);
        println!("{:#?}", bdldata.building);
    }
}
