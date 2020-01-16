//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos geométricos o de zonificación:
//! - Plantas (FLOOR)
//! - Espacios (SPACE)
//! - Polígono (POLYGON)
//! - Vector

use std::convert::TryFrom;

use super::blocks::BdlBlock;
use super::BdlData;

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

impl Space {
    /// Calcula la altura del espacio
    ///
    /// Usa el valor definido como propiedad o la altura por defecto para los espacios
    /// definida en la planta
    pub fn height(&self, db: &BdlData) -> Result<f32, Error> {
        if let Some(height) = self.height {
            Ok(height)
        } else {
            Ok(db.floors
                .iter()
                .find(|f| f.name == self.parent)
                .map(|f| f.spaceheight)
                .ok_or_else(|| {
                    format_err!(
                        "Polígono del espacio {} no encontrado {}. No se puede calcular la superficie",
                        self.name,
                        self.polygon
                    )
                })?)
        }
    }

    /// Calcula el área del espacio
    ///
    /// Usa el área del polígono que define el espacio
    pub fn area(&self, db: &BdlData) -> Result<f32, Error> {
        Ok(db
            .polygons
            .get(&self.polygon)
            .ok_or_else(|| {
                format_err!(
                    "Polígono del espacio {} no encontrado {}. No se puede calcular la superficie",
                    self.name,
                    self.polygon
                )
            })?
            .area())
    }
}

impl TryFrom<BdlBlock> for Space {
    type Error = Error;

    /// Convierte de Bloque BDL a espacio
    ///
    /// Ejemplo:
    /// ```text
    ///     "P01_E01" = SPACE
    ///         nCompleto = "P01_E01"
    ///         HEIGHT        =            3.5
    ///         SHAPE             = POLYGON
    ///         POLYGON           = "P01_E01_Pol2"
    ///         TYPE              = CONDITIONED
    ///         SPACE-TYPE        = "Residencial"
    ///         SYSTEM-CONDITIONS = "Residencial"
    ///         SPACE-CONDITIONS  = "Residencial"
    ///         FLOOR-WEIGHT      =              0
    ///         MULTIPLIER        = 1
    ///         MULTIPLIED        = 0
    ///         PILLARS-NUMBERS   = 0
    ///         FactorSuperficieUtil   = 1.0
    ///         perteneceALaEnvolventeTermica   = SI
    ///         INTERIOR-RADIATION  = FIXED
    ///         POWER     = 4.4
    ///         VEEI-OBJ  = 7.000000
    ///         VEEI-REF  = 10.000000
    ///         ..
    /// 
    ///     $ LIDER antiguo
    ///     "P01_E01" = SPACE
    ///         HEIGHT        =              3
    ///         SHAPE             = POLYGON
    ///         POLYGON           = "P01_E01_Poligono002"
    ///         TYPE              = CONDITIONED
    ///         SPACE-TYPE        = "Residencial"
    ///         FLOOR-WEIGHT      =              0
    ///         MULTIPLIER        = 1            
    ///         MULTIPLIED        = 0
    ///         PILLARS-NUMBERS   = 0
    ///         INTERIOR-RADIATION  = FIXED
    ///         POWER     = 4.4
    ///         VEEI-OBJ  = 7.000000
    ///         VEEI-REF  = 10.000000
    ///         AIR-CHANGES/HR        = 1.000000
    ///         ..
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
        // No existe en LIDER antiguo
        let spaceconds = attrs.remove_str("SPACE-CONDITIONS").unwrap_or(spacetype.clone());
        // No existe en LIDER antiguo
        let systemconds = attrs.remove_str("SYSTEM-CONDITIONS").unwrap_or(spacetype.clone());
        let multiplier = attrs.remove_f32("MULTIPLIER")?;
        // XXX: Es un booleano codificado como entero que se parse como número
        let ismultiplied = (attrs.remove_f32("MULTIPLIED")? - 1.0).abs() < 0.1;

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
    /// Área del polígono definido por vértices (m2)
    pub fn area(&self) -> f32 {
        // https://www.mathopenref.com/coordpolygonarea2.html
        // https://www.mathopenref.com/coordpolygonarea.html
        // 0.5 * ( \SUM( x_i * y_i+1 - y_i * x_i+1)_(i = de 1 a n) + (x_n * y_1 - y_n * x_1) )
        let vertices = &self.vertices;
        let nverts = vertices.len();

        let mut area = 0.0;
        for i in 0..nverts {
            let nexti = (i + 1) % nverts; // el último vértice vuelve a cero
            let vi = &vertices[i].vector;
            let vj = &vertices[nexti].vector;
            area += vi.x * vj.y - vi.y * vj.x;
        }

        f32::abs(0.5 * area)
    }

    /// Perímetro de un polígono (m)
    pub fn perimeter(&self) -> f32 {
        let nlen = self.vertices.len();
        match nlen {
            0 => 0.0,
            1 => 0.0,
            _ => {
                let first = self.vertices.get(0).unwrap().clone();
                let mut vecs = self.vertices.clone();
                vecs.push(first);
                vecs.as_slice().windows(2).map(|win| {
                    let vn = &win[0].vector;
                    let vm = &win[1].vector;
                    (vn.x - vm.x).hypot(vn.y - vm.y)
                }).sum()
            }
        }
    }

    /// Longitud del lado que empieza en el vértice indicado
    pub fn edge_length(&self, vertexname: &str) -> f32 {
        let vv = &self.vertices;
        let [n, m] = self.edge_indices(vertexname).unwrap_or([0, 0]);
        let Vector2D { x: xn, y: yn } = unsafe { vv.get_unchecked(n).vector };
        let Vector2D { x: xm, y: ym } = unsafe { vv.get_unchecked(m).vector };
        (xn - xm).hypot(yn - ym)
    }

    /// Índices del lado que empieza en el vértice dado
    /// El lado que empieza en el último vértice continua en el vértice inicial
    pub fn edge_indices(&self, vertexname: &str) -> Option<[usize; 2]> {
        let vv = &self.vertices;
        let nvertsmax = vv.len() - 1;
        let maybepos = vv.iter().position(|v| v.name == vertexname);
        match maybepos {
            Some(pos) if pos == nvertsmax => Some([pos, 0]),
            Some(pos) if pos < nvertsmax => Some([pos, pos + 1]),
            _ => None,
        }
    }

    /// Ángulo con el sur de la normal del lado definido por el vértice y con desviación global respecto al norte
    /// Los ángulos se dan en grados sexagesimales
    pub fn edge_orient(&self, vertexname: &str, northangle: f32) -> f32 {
        let vv = &self.vertices;
        let [n, m] = self.edge_indices(vertexname).unwrap_or([0, 0]);
        let Vector2D { x: xn, y: yn } = unsafe { vv.get_unchecked(n).vector };
        let Vector2D { x: xm, y: ym } = unsafe { vv.get_unchecked(m).vector };
        // vector director del lado
        let dx = xm - xn;
        let dy = ym - yn;
        // normal al vector director (hay dos, (dy, -dx) y (-dy, dx))
        let nx = dy;
        let ny = -dx;
        // vector del sur (0, -1)
        let sx = 0.0;
        let sy = -1.0;
        // ángulo entre la normal y el sur
        let dot = sx * nx + sy * ny;
        let mag_n = nx.hypot(ny);
        let mag_s = 1.0;
        // Para las normales en el semiplano nx <= 0 cogemos el ángulo largo
        let sign = f32::signum(nx);
        normalize(
            sign * f32::acos(dot / (mag_n * mag_s)).to_degrees() - northangle,
            0.0,
            360.0,
        )
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
            if let Ok(vdata) = attrs.remove_str(&name) {
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

// Normalize number to an arbitrary range
// by assuming the range wraps around when going below min or above max
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    let width = end - start;
    let offset = value - start; // value relative to 0
                                // + start to reset back to start of original range
    (offset - (f32::floor(offset / width) * width)) + start
}

