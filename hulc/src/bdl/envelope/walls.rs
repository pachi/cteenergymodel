// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Cerramientos opacos de la envolvente térmica:
//! - EXTERIOR-WALL
//! - ROOF
//! - INTERIOR-WALL
//! - UNDERGROUND-WALL
//!
//! Todos tienen una construcción y pertenecen a un espacio (location)
#![allow(clippy::upper_case_acronyms)]

use std::convert::TryFrom;

use anyhow::{bail, format_err, Error};

use crate::bdl::{envelope::Polygon, BdlBlock, Data};

// Cerramientos opacos (EXTERIOR-WALL, ROOF, INTERIOR-WALL, UNDERGROUND-WALL) ------------------

/// Posiciones de los cerramientos según su inclinación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tilt {
    /// Suelo (inclinación < 60º)
    BOTTOM,
    /// Cubierta (inclinación > 120º)
    TOP,
    /// Muro (inclinación entre 60 y 120º)
    SIDE,
}

/// Condiciones de contorno de los cerramientos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundaryType {
    /// Cerramiento en contacto con el aire exterior
    EXTERIOR,
    /// Cerramiento en contacto con el aire de otro espacio
    INTERIOR,
    /// Cerramiento en contacto con el terreno
    GROUND,
    /// Cerramiento sin transmisión térmica
    ADIABATIC,
}

impl Default for BoundaryType {
    fn default() -> Self {
        BoundaryType::EXTERIOR
    }
}

/// Cerramiento exterior o interior
/// Puede definirse su configuración geométrica por polígono
/// o por localización respecto al espacio padre.
#[derive(Debug, Clone, Default)]
pub struct Wall {
    /// Nombre
    pub name: String,
    /// Espacio en al que pertenece el cerramiento
    pub space: String,
    /// Definición de la composición de capas del cerramiento
    /// Incialmente contiene un elemento CONSTRUCTION y se sustituye en el postproceso por LAYERS
    pub cons: String,
    /// Posición respecto al espacio asociado (TOP | BOTTOM | SPACE-V{vertexnum})
    /// Cuando se define por top, también tiene POLYGON, cuando es BOTTOM no lo define
    /// Cuando LOCATION es SPACE-Vn remite al polígono del espacio (POLYGON), empezando en el vértice n
    pub location: Option<String>,
    /// Coordenada X de la esquina inferior izquierda
    /// usa coordenadas del espacio y es el cerramiento visto desde fuera
    pub x: f32,
    /// Coordenada Y de la esquina inferior izquierda
    /// usa coordenadas del espacio y es el cerramiento visto desde fuera
    pub y: f32,
    /// Coordenada Z de la esquina inferior izquierda
    /// usa coordenadas del espacio y es el cerramiento visto desde fuera
    pub z: f32,
    /// Desviación de la proyeccción horizontal de la normal del muro con la Y+ del espacio (grados sexagesimales, sentido horario, 0-360)
    /// 0 -> orientación norte, 90 -> orientación este, 180 -> orientación sur y 270 -> orientación oeste
    pub angle_with_space_north: f32,
    /// Inclinación (grados sexagesimales)
    /// Ángulo entre el eje Z y la normal exterior del muro
    pub tilt: f32,
    /// Nombre del polígono que define la geometría
    /// Definición geométrica de un muro (EXTERIOR-WALL, ROOF o INTERIOR-WALL)
    /// Se usa cuando no se define respecto a un vértice del espacio padre sino por polígono
    pub polygon: Option<Polygon>,
    /// Tipos de cerramiento:
    /// - UNDERGROUND: cerramiento en contacto con el terreno (UNDERGROUND-WALL)
    /// - EXTERIOR: cerramiento en contacto con el aire exterior (EXTERIOR-WALL, ROOF)
    /// - INTERIOR: cerramiento interior entre dos espacios (STANDARD en BDL)
    /// - ADIABATIC: cerramiento que no conduce calor (a otro espacio) pero lo almacena
    /// Existen otros tipos en BDL pero HULC no los admite:
    /// - INTERNAL: cerramiento interior a un espacio (no comunica espacios)
    /// - AIR: superficie interior a un espacio, sin masa, pero que admite convección
    pub bounds: BoundaryType,
    // --- Propiedades exclusivas -----------------------
    /// Espacio adyacente que conecta con el espacio padre
    /// (solo en algunos tipos de cerramientos interiores (no adiabático o superficie interior))
    pub nextto: Option<String>,
}

impl Wall {
    /// Superficie bruta (incluyendo huecos) del muro (m2)
    ///
    /// TODO: la búsqueda de polígonos y espacios no es óptima (se podría cachear)
    pub fn gross_area(&self, db: &Data) -> Result<f32, Error> {
        if let Some(polygon) = &self.polygon {
            // Superficie para muros definidos por polígono
            Ok(polygon.area())
        } else if let Some(location) = self.location.as_deref() {
            // Superficie para muros definidos por posición, en un espacio
            let space = db.get_space(self.space.as_str()).ok_or_else(|| {
                format_err!(
                    "Espacio {} del cerramiento {} no encontrado. No se puede calcular la superficie",
                    self.space,
                    self.name
                )
            })?;
            // Elementos de suelo o techo
            if ["TOP", "BOTTOM"].contains(&location) {
                Ok(space.area())
            // Elementos definidos por vértice (location contiene el nombre del vértice)
            } else {
                let poly = &space.polygon;
                let height = space.height;
                let length = poly.edge_length(location);
                Ok(height * length)
            }
        } else {
            bail!("Formato de cerramiento incorrecto. No se define por polígono ni por vértice")
        }
    }

    /// Superficie neta (sin huecos) del cerramiento (m2)
    pub fn net_area(&self, db: &Data) -> Result<f32, Error> {
        let wall_gross_area = self.gross_area(db)?;
        let windows_area = db
            .windows
            .iter()
            .filter(|w| w.wall == self.name)
            .map(|w| w.area())
            .sum::<f32>();
        Ok(wall_gross_area - windows_area)
    }

    /// Perímetro del cerramiento (m)
    pub fn perimeter(&self, db: &Data) -> Result<f32, Error> {
        // 1. Elementos definidos por geometría -> perímetro del polígono
        // 2. Elementos definidos por posición TOP, BOTTOM o SPACE-Vxx
        // 2.1 Elementos TOP o BOTTOM -> perímetro del polígono del espacio
        // 2.2 Elementos definidos por vértice en el espacio -> longitud de lado * altura
        if let Some(polygon) = &self.polygon {
            // 1. Muros definidos por geometría (polígono)
            Ok(polygon.perimeter())
        } else if let Some(location) = self.location.as_deref() {
            // 2. Muros definidos por posición, en un espacio (polígono del espacio)
            let space = db.get_space(self.space.as_str()).ok_or_else(|| {
                format_err!(
                    "Espacio {} del cerramiento {} no encontrado. No se puede calcular el perímetro",
                    self.space,
                    self.name
                )
            })?;
            // 2.1 Elementos de suelo o techo
            if ["TOP", "BOTTOM"].contains(&location) {
                Ok(space.perimeter())
            // 2.2 Elementos definidos por vértice (location contiene el nombre del vértice)
            } else {
                let poly = &space.polygon;
                let height = space.height;
                let length = poly.edge_length(location);
                Ok(2.0 * (height + length))
            }
        } else {
            bail!("Formato de cerramiento incorrecto. No se define por polígono ni por vértice")
        }
    }

    /// Posición del elemento (TOP, BOTTOM, SIDE) según su inclinación
    /// Los elementos con inclinación > 60º Con la horizontal son verticales.
    pub fn position(&self) -> Tilt {
        if self.tilt <= 60.0 {
            Tilt::TOP
        } else if self.tilt < 120.0 {
            Tilt::SIDE
        } else if self.tilt < 240.0 {
            Tilt::BOTTOM
        } else if self.tilt < 300.0 {
            Tilt::SIDE
        } else {
            Tilt::TOP
        }
    }
}

impl TryFrom<BdlBlock> for Wall {
    type Error = Error;

    /// Conversión de bloque BDL a cerramiento
    /// (cerramiento exterior, interior, cubierta o elemento en contacto con el terreno)
    ///
    /// Ejemplos en BDL de EXTERIOR-WALL y ROOF:
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
    ///         CONSTRUCTION  = "SATE"  
    ///         X             =          2.496
    ///         Y             =         -4.888
    ///         Z             =              3
    ///         AZIMUTH       =            180
    ///         LOCATION      = TOP  
    ///         POLYGON       = "P03_E01_FE004_Poligono3"
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
    /// XXX: atributos no trasladados:
    /// XXX: propiedades para definir el estado de la interfaz para la selección de la absortividad:
    /// XXX: TYPE_ABSORPTANCE, COLOR_ABSORPTANCE, DEGREE_ABSORPTANCE
    /// XXX: Atributos no trasladados: COMPROBAR-REQUISITOS-MINIMOS, CONSTRUCCION_MURO
    ///
    /// Ejemplos en BDL de INTERIOR-WALL:
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
    /// XXX: atributos no trasladados: Ninguno
    ///
    /// Ejemplos en BDL de UNDERGROUND-WALL:
    /// ```text
    ///    "P01_E01_FTER001" = UNDERGROUND-WALL
    ///     Z-GROUND      =              0
    ///     COMPROBAR-REQUISITOS-MINIMOS = YES
    ///                    CONSTRUCTION  = "solera tipo"
    ///                    LOCATION      = BOTTOM
    ///                    AREA          =        418.4805
    ///                    PERIMETRO     =        65.25978
    ///                          ..
    ///                    "solera tipo" =  CONSTRUCTION
    ///                          TYPE   = LAYERS
    ///                          LAYERS = "solera tipo"
    ///                          ..
    /// ```
    /// XXX: No se han trasladado las variables de AREA y PERIMETRO porque se pueden calcular
    /// y los valores comprobados en algunos archivos no son correctos
    /// XXX: Ignoramos "Z-GROUND", la profundidad del elemento en el terreno, ya que no es del BDL y es menos fiable que min(space.z, 0.0)
    ///
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name,
            btype,
            parent,
            mut attrs,
            ..
        } = value;
        let space =
            parent.ok_or_else(|| format_err!("Cerramiento sin espacio asociado '{}'", &name))?;
        // XXX: incialmente guardamos la referencia al elemento CONSTRUCTION (agrupa wallcons y absorptance)
        // XXX: y se sustituye en un postproceso por el elemento LAYERS, que ampliamos con el ABSORPTANCE de CONSTRUCTION
        let cons = attrs.remove_str("CONSTRUCTION")?;
        // let absorptance = attrs.remove_f32("ABSORPTANCE").ok();

        let location = match attrs.remove_str("LOCATION").ok() {
            // Solo soportamos algunos subtipos de location: TOP, BOTTOM, SPACE-x
            Some(loc) if ["TOP", "BOTTOM"].contains(&loc.as_str()) => Some(loc),
            // Para los elementos definidos como vértices de espacios guardamos el vértice directamente
            Some(loc) if loc.starts_with("SPACE-") => Some(loc["SPACE-".len()..].to_string()),
            // Para el resto fallamos
            Some(loc) => bail!("Elemento {} con localización desconocida {}", name, loc),
            _ => None,
        };

        // Tipos de cerramientos
        let bounds = match btype.as_str() {
            "INTERIOR-WALL" => {
                let int_wall = attrs.remove_str("INT-WALL-TYPE")?;
                match int_wall.as_str() {
                    "STANDARD" => BoundaryType::INTERIOR,
                    "ADIABATIC" => BoundaryType::ADIABATIC,
                    // AIR, INTERNAL
                    _ => bail!(
                        "Cerramiento interior {} con subtipo desconocido {} / {}",
                        name,
                        btype,
                        int_wall
                    ),
                }
            }
            "UNDERGROUND-WALL" => BoundaryType::GROUND,
            "EXTERIOR-WALL" | "ROOF" => BoundaryType::EXTERIOR,
            _ => bail!("Elemento {} con tipo desconocido {}", name, btype),
        };

        // Si la inclinación es None (se define location)
        // Solamente se define explícitamente cuando se define el cerramiento por geometría
        let tilt = match attrs.remove_f32("TILT").ok() {
            Some(tilt) => tilt,
            _ => match (btype.as_str(), location.as_deref()) {
                // Cubiertas y cerramientos en location top (techos)
                ("ROOF", _) | (_, Some("TOP")) => 0.0,
                // cerramientos en location bottom (suelos y soleras)
                (_, Some("BOTTOM")) => 180.0,
                // Cerramientos verticales
                _ => 90.0,
            },
        };

        // En LIDER antiguo pueden no aparecer algunas de las coordenadas
        let x = attrs.remove_f32("X").unwrap_or_default();
        let y = attrs.remove_f32("Y").unwrap_or_default();
        let z = attrs.remove_f32("Z").unwrap_or_default();

        // Detectamos si se define la geometría por polígono
        // Como guardaremos el polígono no por su nombre sino como objeto aquí usamos un default
        // y lo corregimos en el postproceso
        let polygon = attrs.remove_str("POLYGON").ok().map(|_| Default::default());

        let bdl_azimuth = match location.as_deref() {
            Some("BOTTOM") => {
                // En los elementos bottom, que giramos el suelo del espacio queremos dejar la orientación sin mover respecto al sur
                180.0
            }
            _ => attrs.remove_f32("AZIMUTH").unwrap_or_default(),
        };

        // Propiedades específicas
        let nextto = match bounds {
            BoundaryType::INTERIOR => attrs.remove_str("NEXT-TO").ok(),
            _ => None,
        };
        Ok(Self {
            name,
            bounds,
            space,
            cons,
            location,
            x,
            y,
            z,
            // Temporalmente guardamos el ángulo con el norte definido explícitamente
            // Debe corregirse en el bucle global (para acceder a los polígonos) cuando
            // location se defina por polígono o sea top o bottom
            angle_with_space_north: bdl_azimuth,
            tilt,
            polygon,
            nextto,
        })
    }
}
