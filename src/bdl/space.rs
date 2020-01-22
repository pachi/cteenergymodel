//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento espacio (zona) (SPACE)
//! Estos elementos agrupan las superficies (muros) del edificio
//! y definen propiedades como su tipo, pertenencia a la envolvente térmica,
//! potencia de iluminación y VEEI, además de los perfiles de uso y ocupación.

use std::convert::TryFrom;

use super::blocks::BdlBlock;
use super::Data;

use failure::bail;
use failure::Error;

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
    pub fn height(&self, db: &Data) -> Result<f32, Error> {
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
    pub fn area(&self, db: &Data) -> Result<f32, Error> {
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

    /// Calcula el perímetro del espacio
    ///
    /// Usa el perímetro del polígono que define el espacio
    pub fn perimeter(&self, db: &Data) -> Result<f32, Error> {
        Ok(db
            .polygons
            .get(&self.polygon)
            .ok_or_else(|| {
                format_err!(
                    "Polígono del espacio {} no encontrado {}. No se puede calcular el perímetro",
                    self.name,
                    self.polygon
                )
            })?
            .perimeter())
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
        let spaceconds = attrs
            .remove_str("SPACE-CONDITIONS")
            .unwrap_or(spacetype.clone());
        // No existe en LIDER antiguo
        let systemconds = attrs
            .remove_str("SYSTEM-CONDITIONS")
            .unwrap_or(spacetype.clone());
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
