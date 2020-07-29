/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento espacio (zona) (SPACE)
//! Estos elementos agrupan las superficies (muros) del edificio
//! y definen propiedades como su tipo, pertenencia a la envolvente térmica,
//! potencia de iluminación y VEEI, además de los perfiles de uso y ocupación.

use std::convert::TryFrom;

use super::geom::Polygon;
use super::walls::Tilt;
use crate::bdl::BdlBlock;
use crate::bdl::Data;

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
    /// HULC únicamente usa espacios definidos por polígono (no usa SHAPE = BOX o NO-SHAPE)
    pub polygon: Polygon,
    /// Altura (suelo a suelo) del espacio
    /// (HULC solo permite que los espacios tengan la altura de la planta)
    pub height: f32,
    /// Cota Z del espacio
    /// Se define en la planta inicialmente
    pub z: f32,
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
    /// Multiplicador
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
        let topwall = db
            .walls
            .iter()
            .find(|w| {
                // Muros exteriores o cubiertas sobre el espacio
                if w.position() == Tilt::TOP {
                    true
                } else {
                    w.nextto.as_ref().map(|s| 
                        // Es un cerramiento interior sobre este espacio
                        s == &self.name
                    ).unwrap_or(false)
                }
            })
            .ok_or_else(|| {
                format_err!(
                    "Cerramiento superior del espacio {} no encontrado. No se puede calcular la altura libre",
                    self.name
                )
            })?;
        let topheight = db.db.wallcons.get(&topwall.construction)
            .ok_or_else(|| format_err!("No se encuentra la composición de capas \"{}\"", &topwall.construction))?.total_thickness();
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

        let stype = attrs.remove_str("TYPE")?;
        // Generamos un polígono por defecto, ya que se inserta en el postproceso de bloques
        let polygon = Polygon::default();
        // HULC no define a veces la altura pero para el cálculo de volúmenes y alturas
        // usa la altura de la planta
        // XXX: podríamos ver si esos casos se corresponden a espacios con cubierta inclinada
        // XXX: y calcular la altura media en función de la geometría de la cubierta
        let height = attrs.remove_f32("HEIGHT").unwrap_or_default();
        // La cota Z se define en los objetos FLOOR, pero la guardamos luego aquí para eliminarlos
        let z = 0.0;
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
        let floor = parent.ok_or_else(|| {
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
            .unwrap_or_else(|_| spacetype.clone());
        // No existe en LIDER antiguo
        let systemconds = attrs
            .remove_str("SYSTEM-CONDITIONS")
            .unwrap_or_else(|_| spacetype.clone());
        let multiplier = attrs.remove_f32("MULTIPLIER")?;
        // XXX: Es un booleano codificado como entero que se parse como número
        let ismultiplied = (attrs.remove_f32("MULTIPLIED")? - 1.0).abs() < 0.1;
        let airchanges_h = match (stype.as_str(), spaceconds.as_str()) {
            // Usamos la ventilación según niveles de estanqueidad de la UNE-EN ISO 13789:2017 (HULC usa 0 renh para nivel1)
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_1") => {Some(0.1)},
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_2") => {Some(0.5)},
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_3") => {Some(1.0)},
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_4") => {Some(3.0)},
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_5") => {Some(10.0)},
            _ => attrs.remove_f32("AIR-CHANGES/HR").map(|v| Some(v)).unwrap_or( None)
        };

        Ok(Self {
            name,
            stype,
            polygon,
            height,
            z,
            insidete,
            floor,
            power,
            veeiobj,
            veeiref,
            spacetype,
            spaceconds,
            systemconds,
            multiplier,
            ismultiplied,
            airchanges_h
        })
    }
}
