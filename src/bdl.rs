//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

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

// Objetos ----------------------------------------------------------------

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

/// Hueco
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
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// Nombre del material
    pub name: String,
    // Resto de propiedades
    pub attrs: AttrMap,
}

impl Window {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Elementos del modelo BDL
#[derive(Debug)]
pub enum BdlType {
    /// Material
    Material(Material),
    // Layers,
    /// Espacio
    Space(Space),
    /// Planta
    Floor(Floor),
    /// Polígono
    Polygon(Polygon),
    /// Ventana
    Element(BdlElementType)
}

/// Elementos de envolvente
#[derive(Debug)]
pub enum BdlElementType {
    Window(Window),
    // ExteriorWall(ExteriorWall),
    // InteriorWall(InteriorWall),
}

// ------------------------- BDL ----------------------------

pub struct Block {
    /// Tipo de bloque
    pub btype: String,
    /// Nombre del material
    pub name: String,
    // Conjunto de propiedades
    pub attrs: AttrMap,
}

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
    // Elementos de la envolvente
    pub elements: Vec<BdlElementType>
}

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct BdlData {
    pub building: BdlBuildingData,
}

impl BdlData {
    pub fn new(input: &str) -> Result<Self, Error> {
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

        let mut blocks = Vec::<Block>::new();

        for block in bdl_part
            .split("..")
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            let stanza: Vec<_> = block.splitn(2, '\n').map(str::trim).collect();

            let bdlblock = if let [bheadline, bdata] = stanza.as_slice() {
                if let [name, btype] = bheadline
                    .splitn(2, '=')
                    .map(str::trim)
                    .map(|s| s.trim_matches('"'))
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    let attrs = parse_attributes(bdata)?;
                    Block {
                        name: name.to_string(),
                        btype: btype.to_string(),
                        attrs,
                    }
                } else {
                    panic!("Error al parsear encabezado: {}", bheadline);
                }
            } else {
                panic!("Error al parsear el bloque: '{:?}'", stanza);
            };

            blocks.push(bdlblock);
        }

        // Parsea bloques
        for block in blocks {
            match block.btype.as_ref() {
                "MATERIAL" => {
                    bdldata.materials.push(parse_material(block)?);
                }
                "FLOOR" => {
                    bdldata.floors.push(parse_floor(block)?);
                }
                "SPACE" => {
                    // Asigna el espacio a la planta actual
                    // Genera planta por defecto si no hay una
                    if bdldata.floors.len() == 0 {
                        bdldata.floors.push(Floor::new("Default"));
                    };
                    bdldata
                        .floors
                        .last_mut()
                        .map(|f| f.spaces.push(block.name.clone()));
                    bdldata.spaces.push(parse_space(block)?);
                }
                "POLYGON" => {
                    bdldata
                        .polygons
                        .insert(block.name.clone(), parse_polygon(block)?);
                }
                // "CONSTRUCTION" => {
                //     eprintln!("CONSTRUCTION. bname: {}, btype: {}", bname, btype);
                // }
                "WINDOW" => {
                    // TODO: no asigna la ventana a un muro y a su vez este a un espacio
                    bdldata.elements.push(parse_window(block)?);
                }
                // "EXTERIOR-WALL" => {
                //     eprintln!("EXTERIOR-WALL. bname: {}, btype: {}", bname, btype);
                // }
                // "INTERIOR-WALL" => {
                //     eprintln!("INTERIOR-WALL. bname: {}, btype: {}", bname, btype);
                // }
                // "UNDERGROUND-WALL" => {
                //     eprintln!("UNDERGROUND-WALL. bname: {}, btype: {}", bname, btype);
                // }
                _ => {
                    eprintln!(
                        "Tipo desconocido. bname: {}, btype: {}",
                        block.name, block.btype
                    );
                }
            };
        }

        Ok(Self { building: bdldata })
    }
}

fn parse_attributes(data: &str) -> Result<AttrMap, Error> {
    let mut attributes = AttrMap::new();
    let mut lines = data.lines();
    while let Some(l) = lines.next() {
        if let [key, value] = l.split('=').map(str::trim).collect::<Vec<_>>().as_slice() {
            // Valores simples o con paréntesis
            let value = if value.starts_with('(') && !value.ends_with(')') {
                let mut values = vec![*value];
                while let Some(newvalueline) = lines.next() {
                    let val = newvalueline.trim();
                    values.push(val);
                    if val.ends_with(')') {
                        break;
                    };
                }
                values.join("").to_string()
            } else {
                value.trim_matches('"').to_string()
            };
            attributes.insert(key, value);
        } else {
            bail!("No se ha podido extraer clave y atributo de '{}'", l)
        }
    }
    Ok(attributes)
}

fn parse_material(block: Block) -> Result<Material, Error> {
    let attrs = block.attrs;
    let mut material = Material::new(block.name, block.btype);
    material.group = attrs.get("GROUP")?;
    material.attrs = attrs;
    Ok(material)
}

fn parse_floor(block: Block) -> Result<Floor, Error> {
    let attrs = block.attrs;
    let mut floor = Floor::new(block.name);
    floor.z = attrs.get_f32("Z").unwrap_or_default();
    floor.attrs = attrs;
    Ok(floor)
}

fn parse_space(block: Block) -> Result<Space, Error> {
    //TODO: falta el contexto para asignar el espacio a la planta
    let attrs = block.attrs;
    let mut space = Space::new(block.name);
    space.polygon = attrs.get("POLYGON")?;
    space.height = attrs.get_f32("HEIGHT").ok();
    space.attrs = attrs;
    Ok(space)
}

fn parse_window(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar la ventana a un muro
    let attrs = block.attrs;
    let mut window = Window::new(block.name);
    window.attrs = attrs;
    Ok(BdlElementType::Window(window))
}

fn parse_polygon(block: Block) -> Result<Polygon, Error> {
    let attrs = block.attrs;
    let mut polygon = Polygon::new(block.name);
    for (_k, v) in &attrs.0 {
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
        let bdldata = BdlData::new(&data.entrada_grafica_lider).unwrap();
        println!("{:#?}", bdldata.building);
    }
}
