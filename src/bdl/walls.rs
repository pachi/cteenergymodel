//! Parser del Building Description Language (BDL) de DOE
//!
//! Cerramientos opacos de la envolvente térmica:
//! - EXTERIOR-WALL
//! - ROOF
//! - INTERIOR-WALL
//! - UNDERGROUND-WALL
//!
//! Todos tienen una construcción y pertenecen a un espacio (location)

use failure::Error;
use std::convert::TryFrom;

use crate::utils::normalize;
use super::{BdlBlock, Data};

// Cerramientos opacos (EXTERIOR-WALL, ROOF, INTERIOR-WALL, UNDERGROUND-WALL) ------------------

/// Cerramiento exterior o interior
/// Puede definirse su configuración geométrica por polígono
/// o por localización respecto al espacio padre.
#[derive(Debug, Clone, Default)]
pub struct Wall {
    /// Nombre
    pub name: String,
    /// Espacio en al que pertenece el cerramiento
    pub space: String,
    /// Definición de la composición del cerramiento (Construction)
    pub construction: String,
    /// Posición respecto al espacio asociado (TOP, BOTTOM, nombreespacio)
    pub location: Option<String>,
    /// Inclinación (grados sexagesimales)
    /// Ángulo entre el eje Z y la normal exterior del muro
    pub tilt: f32,
    /// Posición definida por polígono
    pub geometry: Option<WallGeometry>,
    /// Tipo de cerramiento:
    /// - EXTERIOR-WALL: Muro en contacto con el aire exterior
    /// - ROOF: Cubierta en contacto con el aire exterior
    /// - STANDARD: cerramiento interior entre dos espacios
    /// - ADIABATIC: cerramiento que no conduce calor (a otro espacio) pero lo almacena
    /// - INTERNAL: cerramiento interior a un espacio (no comunica espacios)
    /// - AIR: superficie interior a un espacio, sin masa, pero que admite convección
    pub wtype: String,
    // --- Propiedades exclusivas -----------------------
    // XXX: Absortividad definida por usuario -> Se debe consultar en la construcción
    // XXX: (solo en cerramientos en contacto con el aire)
    // XXX: pub absorptance: Option<f32>,
    /// Espacio adyacente que conecta con el espacio padre
    /// (solo en algunos tipos de cerramientos interiores (no adiabático o superficie interior))
    pub nextto: Option<String>,
    /// Profundidad del elemento en el terreno (m)
    /// (solo en cerramientos en contacto con el terreno)
    pub zground: Option<f32>,
}

impl Wall {
    /// Superficie bruta (incluyendo huecos) del muro (m2)
    ///
    /// TODO: la búsqueda de polígonos y espacios no es óptima (se podría cachear)
    pub fn gross_area(&self, db: &Data) -> Result<f32, Error> {
        if let Some(geom) = &self.geometry {
            // Superficie para muros definidos por polígono
            let geom_polygon = db.polygons.get(&geom.polygon).ok_or_else(|| {
                format_err!(
                    "Polígono del cerramiento {} no encontrado {}. No se puede calcular la superficie",
                    self.name,
                    geom.polygon
                )
            })?;
            Ok(geom_polygon.area())
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
                space.area(&db)
            // Elementos definidos por vértice (location contiene el nombre del vértice)
            } else {
                let poly = db.polygons.get(&space.polygon).ok_or_else(|| {
                    format_err!(
                        "Polígono {} del espacio {} del cerramiento {} no encontrado. No se puede calcular la superficie",
                        space.polygon,
                        self.space,
                        self.name
                    )
                })?;
                let height = space.height(&db)?;
                let length = poly.edge_length(&location);
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
        if let Some(geom) = &self.geometry {
            // 1. Muros definidos por geometría (polígono)
            let geom_polygon = db.polygons.get(&geom.polygon).ok_or_else(|| {
                format_err!(
                    "Polígono del cerramiento {} no encontrado {}. No se puede calcular el perímetro",
                    self.name,
                    geom.polygon
                )
            })?;
            Ok(geom_polygon.perimeter())
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
                space.perimeter(&db)
            // 2.2 Elementos definidos por vértice (location contiene el nombre del vértice)
            } else {
                let poly = db.polygons.get(&space.polygon).ok_or_else(|| {
                    format_err!(
                        "Polígono {} del espacio {} del cerramiento {} no encontrado. No se puede calcular el perímetro",
                        space.polygon,
                        self.space,
                        self.name
                    )
                })?;
                let height = space.height(&db)?;
                let length = poly.edge_length(&location);
                Ok(2.0 * (height + length))
            }
        } else {
            bail!("Formato de cerramiento incorrecto. No se define por polígono ni por vértice")
        }
    }

    /// Azimut, ángulo del muro respecto al norte (grados sexagesimales)
    /// 
    /// Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del muro
    /// Se puede indicar una desviación del norte geográfico respecto al geométrico (northangle)
    /// 
    /// Se calcula:
    /// 1. Los elementos definidos por geometría ya tiene definido su azimut
    /// 2. Los elementos horizontales se definen con azimut igual a 0.0
    /// 3. Los elementos definidos por vértice de polígono del espacio madre deben consultar su azimuth con el polígono del espacio
    pub fn azimuth(&self, northangle: f32, db: &Data) -> Result<f32, Error> {
        if let Some(geom) = &self.geometry {
            // Elementos definidos por polígono
            // Elementos horizontales (hacia arriba o hacia abajo)
            if self.tilt == 0.0 || self.tilt == 180.0 {
                Ok(0.0)
            } else {
                // Se guarda el ángulo respecto al eje Y del espacio (norte, si la desviación global es cero)
                Ok(geom.azimuth)
            }
        } else {
            match (self.wtype.as_str(), self.location.as_deref()) {
                // Elementos horizontales
                ("ROOF", _) | (_, Some("TOP")) | (_, Some("BOTTOM")) => Ok(0.0),
                // Elementos definidos por vértice en polígono
                (_, Some(vertex)) => {
                    // Superficie para muros definidos por vértice del polígono de su espacio
                    let space = db
                        .get_space(self.space.as_str())
                        .ok_or_else(|| {
                            format_err!(
                                "Espacio {} del cerramiento {} no encontrado. No se puede calcular el azimut",
                                self.space,
                                self.name
                            )
                        })?;
                    let polygon = db.polygons.get(&space.polygon).ok_or_else(|| {
                        format_err!(
                            "Polígono del espacio {} del cerramiento {} no encontrado. No se puede calcular el azimut",
                            space.name,
                            self.name,
                        )
                    })?;
                    let azimuth = normalize(180.0 - polygon.edge_orient(vertex, northangle), 0.0, 360.0);
                    Ok(azimuth)
                }
                // Resto de casos
                _ => bail!("Imposible calcular azimut de elemento {}", self.name),
            }
        }
    }
}

/// Definición geométrica de un muro (EXTERIOR-WALL, ROOF o INTERIOR-WALL)
/// Se usa cuando no se define respecto a un vértice del espacio padre sino por polígono
#[derive(Debug, Clone, Default)]
pub struct WallGeometry {
    /// Nombre del polígono que define la geometría
    pub polygon: String,
    /// Coordenada X de la esquina inferior izquierda
    /// usa coordenadas del espacio ??
    pub x: f32,
    /// Coordenada Y de la esquina inferior izquierda
    /// usa coordenadas del espacio ??
    pub y: f32,
    /// Coordenada Z de la esquina inferior izquierda
    /// usa coordenadas del espacio ??
    pub z: f32,
    /// Acimut (grados sexagesimales)
    /// Ángulo entre el eje Y (norte) del espacio y la proyección horizontal de la normal exterior del muro
    pub azimuth: f32,
}

impl WallGeometry {
    pub fn parse_wallgeometry(mut attrs: super::AttrMap) -> Result<Option<Self>, Error> {
        if let Ok(polygon) = attrs.remove_str("POLYGON") {
            // XXX: en LIDER antiguo pueden no aparecer algunas de estas coordenadas
            let x = attrs.remove_f32("X").unwrap_or_default();
            let y = attrs.remove_f32("Y").unwrap_or_default();
            let z = attrs.remove_f32("Z").unwrap_or_default();
            let azimuth = attrs.remove_f32("AZIMUTH")?;

            Ok(Some(WallGeometry {
                polygon,
                x,
                y,
                z,
                azimuth,
            }))
        } else {
            Ok(None)
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
    /// XXX: propiedades cacheadas de la CONSTRUCTION:
    /// XXX: ABSORPTANCE
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
        let construction = attrs.remove_str("CONSTRUCTION")?;
        let location = match attrs.remove_str("LOCATION").ok() {
            // Solo soportamos algunos subtipos de location: TOP, BOTTOM, SPACE-x
            Some(loc) if ["TOP", "BOTTOM"].contains(&loc.as_str()) => Some(loc),
            // Para los elementos definidos como vértices de espacios guardamos el vértice directamente
            Some(loc) if loc.starts_with("SPACE-") => Some(loc["SPACE-".len()..].to_string()),
            // Para el resto fallamos
            Some(loc) => bail!("Elemento {} con localización desconocida {}", name, loc),
            _ => None,
        };

        // Tipos
        // TODO: Convertir a enum
        let wtype = match btype.as_str() {
            "INTERIOR-WALL" => attrs.remove_str("INT-WALL-TYPE")?,
            "UNDERGROUND-WALL" => "UNDERGROUND-WALL".to_string(),
            "ROOF" => "ROOF".to_string(),
            "EXTERIOR-WALL" => "EXTERIOR-WALL".to_string(),
            _ => bail!("Elemento {} con tipo desconocido {}", name, btype),
        };

        // Si la inclinación es None (se define location)
        // Solamente se define explícitamente cuando se define el cerramiento por geometría
        // TODO: dado que siempre definimos el tilt no nos haría falta tener un subtipo ROOF
        let tilt = match attrs.remove_f32("TILT").ok() {
            Some(tilt) => tilt,
            _ => match (wtype.as_str(), location.as_deref()) {
                // Cubiertas y cerramientos en location top (techos)
                ("ROOF", _) | (_, Some("TOP")) => 0.0,
                // cerramientos en location bottom (suelos y soleras)
                (_, Some("BOTTOM")) => 180.0,
                // Cerramientos verticales
                _ => 90.0,
            },
        };

        // Propiedades específicas
        // XXX: La absortividad debe consultarse en la construcción, esto parece una cache de HULC
        // let absorptance = match wtype.as_str() {
        //     "EXTERIOR-WALL" | "ROOF" => Some(attrs.remove_f32("ABSORPTANCE")?),
        //     _ => None,
        // };
        let nextto = match wtype.as_str() {
            "STANDARD" | "AIR" => attrs.remove_str("NEXT-TO").ok(),
            _ => None,
        };
        let zground = match wtype.as_str() {
            "UNDERGROUND-WALL" => Some(attrs.remove_f32("Z-GROUND")?),
            _ => None,
        };

        let geometry = WallGeometry::parse_wallgeometry(attrs)?;
        Ok(Self {
            name,
            wtype,
            space,
            construction,
            location,
            tilt,
            geometry,
            nextto,
            zground,
        })
    }
}
