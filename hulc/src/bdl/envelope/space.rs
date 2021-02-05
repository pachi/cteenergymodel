// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento espacio (zona) (SPACE)
//! Estos elementos agrupan las superficies (muros) del edificio
//! y definen propiedades como su tipo, pertenencia a la envolvente térmica,
//! potencia de iluminación y VEEI, además de los perfiles de uso y ocupación.

use std::convert::TryFrom;

use anyhow::{bail, format_err, Error};

use super::super::BdlBlock;
use super::super::Data;
use super::geom::Polygon;
use super::walls::{Tilt, Wall};

/// Espacio
#[derive(Debug, Clone, Default)]
pub struct Space {
    /// Nombre del espacio
    pub name: String,
    /// Tipo de espacio (CONDITIONED, UNHABITED, ¿UNCONDITIONED?, ¿PLENUM?)
    pub stype: String,
    /// Nombre de polígono que define el espacio
    /// HULC únicamente usa espacios definidos por polígono (no usa SHAPE = BOX o NO-SHAPE)
    pub polygon: Polygon,
    /// Altura (suelo a suelo) del espacio
    /// (HULC solo permite que los espacios tengan la altura de la planta)
    pub height: f32,
    /// Cota X del espacio en el espacio de coordenadas de la planta
    pub x: f32,
    /// Cota Y del espacio en el espacio de coordenadas de la planta
    pub y: f32,
    /// Cota Z del espacio en el espacio de coordenadas de la planta
    /// XXX: HULC no usa esta coordenada en los espacios, solo los sitúa en el plano de la planta
    pub z: f32,
    /// Azimuth del espacio en el sistema coordenado de la planta (grados) [0-360]
    /// Ángulo del eje +Y respecto al eje +Y del edificio (eje de rotación Z).
    /// Sentido horario (x+/E+)
    pub azimuth: f32,
    /// Pertenencia a la envolvente térmica
    pub insidete: bool,
    /// Planta a la que pertenece el espacio
    pub floor: String,
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
    /// Multiplicador de planta
    pub floor_multiplier: f32,
    /// Multiplicador de espacio
    pub multiplier: f32,
    /// Si es un espacio multiplicado
    pub ismultiplied: bool,
    /// Tasa de renovación de aire (ventilación), en renh
    /// En edificios residenciales no se guarda (es None) y se usa el global, repartiendo por volumen
    pub airchanges_h: Option<f32>,
}

impl Space {
    /// Altura (suelo a techo) del espacio (m)
    ///
    /// Usa la altura bruta y resta espesores de cubiertas y forjados
    pub fn space_height(&self, db: &Data) -> Result<f32, Error> {
        let topwall = &self.top_wall(db)?;
        let topheight = db
            .db
            .wallcons
            .get(&topwall.cons)
            .ok_or_else(|| {
                format_err!(
                    "No se encuentra la composición de capas \"{}\"",
                    &topwall.cons
                )
            })?
            .total_thickness();
        Ok(self.height - topheight)
    }

    /// Superficie del espacio (m2)
    ///
    /// Usa el área del polígono que define el espacio
    pub fn area(&self) -> f32 {
        self.polygon.area()
    }

    /// Calcula el perímetro del espacio (m)
    ///
    /// Usa el perímetro del polígono que define el espacio
    pub fn perimeter(&self) -> f32 {
        self.polygon.perimeter()
    }

    /// Calcula el perímetro expuesto del espacio (m)
    ///
    /// El perímetro expuesto es el que separa el espacio del exterior o de un espacio no calefactado fuera de la estructura aislada
    /// Excluye las parte que separan el espacio con otros espacios calefactados
    pub fn exposed_perimeter(&self, db: &Data) -> f32 {
        use super::BoundaryType::{ADIABATIC, EXTERIOR, GROUND, INTERIOR};
        // Solo se computa el de los espacios acondicionados
        if self.stype != "CONDITIONED" {
            return 0.0;
        };

        // Muros exteriores (verticales)
        let vertical_walls_for_space = db
            .walls
            .iter()
            .filter(|w| w.space == self.name && w.position() == Tilt::SIDE);

        // Area bruta total de muros y área bruta de muros exteriores
        let (total_vwalls_area, exterior_vwalls_area) = vertical_walls_for_space
            .map(|w| {
                let area = w.gross_area(db).unwrap_or(0.0);
                match w.bounds {
                    // Contactos con el exterior o el terreno
                    EXTERIOR | GROUND => (area, area),
                    // Contactos con otros espacios no acondicionados o no habitables
                    INTERIOR => {
                        w.nextto
                            .as_deref()
                            .and_then(|nxts| db.spaces.iter().find(|s| s.name == nxts))
                            .and_then(|nextspace| {
                                if nextspace.stype != "CONDITIONED" {
                                    // tenemos en cuenta el contacto de espacios acondicionados con otros tipos
                                    Some((area, area))
                                } else {
                                    None
                                }
                            })
                            // El resto no se considera contacto con el exterior
                            .unwrap_or((area, 0.0))
                    }
                    ADIABATIC => (area, 0.0),
                }
            })
            .fold((0.0, 0.0), |(acc_tot, acc_ext), (el_tot, el_ext)| {
                (acc_tot + el_tot, acc_ext + el_ext)
            });

        if total_vwalls_area < 0.01 {
            0.0
        } else {
            self.polygon.perimeter() * exterior_vwalls_area / total_vwalls_area
        }
    }

    /// Volumen bruto del espacio (m3)
    ///
    /// Usa el área y la altura total (suelo a suelo) del espacio
    pub fn gross_volume(&self) -> f32 {
        self.area() * self.height
    }

    /// Volumen neto del espacio (m3)
    ///
    /// Usa el área y la altura libre (suelo a techo) del espacio
    pub fn net_volume(&self, db: &Data) -> Result<f32, Error> {
        Ok(self.area() * self.space_height(db)?)
    }

    /// Muro superior de un espacio
    pub fn top_wall<'a>(&self, db: &'a Data) -> Result<&'a Wall, Error> {
        db
            .walls
            .iter()
            .find(|w| {
                match w.position() {
                    // Muros exteriores o cubiertas sobre el espacio
                    Tilt::TOP => w.space == self.name,
                    // Es un cerramiento interior sobre este espacio
                    Tilt::BOTTOM => {
                        w.nextto.as_ref().map(|s| s == &self.name).unwrap_or(false)}
                    _ => false
                }
            })
            .ok_or_else(|| {
                format_err!(
                    "Cerramiento superior del espacio {} no encontrado. No se puede calcular la altura libre",
                    self.name
                )
            })
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
    /// NOTE: La propiedad POLYGON se trata en el postproceso y en la conversión incial se usa
    /// un valor por defecto.
    /// XXX: propiedades no convertidas:
    /// XXX: PILLARS-NUMBERS (número de pilares en el espacio, como PTs),
    /// XXX: FactorSuperficieUtil, INTERIOR-RADIATION, nCompleto, FLOOR-WEIGHT
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
        // CONDITIONED|UNHABITED|No acondiconado
        let stype = attrs.remove_str("TYPE")?;
        // Generamos un polígono por defecto, ya que se inserta en el postproceso de bloques
        let polygon = Polygon::default();
        // HULC no define a veces la altura pero para el cálculo de volúmenes y alturas
        // usa la altura de la planta
        // XXX: podríamos ver si esos casos se corresponden a espacios con cubierta inclinada
        // XXX: y calcular la altura media en función de la geometría de la cubierta
        let height = attrs.remove_f32("HEIGHT").unwrap_or_default();
        let x = attrs.remove_f32("X").unwrap_or_default();
        let y = attrs.remove_f32("Y").unwrap_or_default();
        let z = attrs.remove_f32("Z").unwrap_or_default();
        let azimuth = attrs.remove_f32("AZIMUTH").unwrap_or_default();
        let insidete = attrs
            .remove_str("perteneceALaEnvolventeTermica")
            .ok()
            .map(|v| v == "SI")
            // TODO: En archivos antiguos, sin ese parámetro miramos si es acondicionado
            // TODO: En teoría también podría haber habitables no acondicionados
            .or_else(|| match stype.as_ref() {
                "CONDITIONED" => Some(true),
                _ => Some(false),
            })
            .unwrap_or(false);
        let floor = parent.ok_or_else(|| {
            format_err!(
                "No se encuentra la referencia de la planta en el espacio {}",
                name
            )
        })?;
        // Potencia de iluminación
        let power = attrs.remove_f32("POWER")?;
        let veeiobj = attrs.remove_f32("VEEI-OBJ")?;
        let veeiref = attrs.remove_f32("VEEI-REF")?;
        // Condiciones operacionales Nombre o #n
        let spacetype = attrs.remove_str("SPACE-TYPE")?;
        // No existe en LIDER antiguo
        let spaceconds = attrs
            .remove_str("SPACE-CONDITIONS")
            .unwrap_or_else(|_| spacetype.clone());
        // No existe en LIDER antiguo
        let systemconds = attrs
            .remove_str("SYSTEM-CONDITIONS")
            .unwrap_or_else(|_| spacetype.clone());
        // XXX: Usamos por defecto un valor 1.0, ya que se obtiene de la planta
        let floor_multiplier = 1.0;
        let multiplier = attrs.remove_f32("MULTIPLIER")?;
        // XXX: Es un booleano codificado como entero que se parse como número
        let ismultiplied = (attrs.remove_f32("MULTIPLIED")? - 1.0).abs() < 0.1;
        let airchanges_h = match (stype.as_str(), spaceconds.as_str()) {
            // Usamos la ventilación según niveles de estanqueidad de la UNE-EN ISO 13789:2017 (HULC usa 0 renh para nivel1)
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_1") => Some(0.1),
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_2") => Some(0.5),
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_3") => Some(1.0),
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_4") => Some(3.0),
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_5") => Some(10.0),
            _ => attrs.remove_f32("AIR-CHANGES/HR").map(Some).unwrap_or(None),
        };

        Ok(Self {
            name,
            stype,
            polygon,
            height,
            x,
            y,
            z,
            azimuth,
            insidete,
            floor,
            power,
            veeiobj,
            veeiref,
            spacetype,
            spaceconds,
            systemconds,
            floor_multiplier,
            multiplier,
            ismultiplied,
            airchanges_h,
        })
    }
}
