// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elemento espacio (zona) (SPACE)
//! Estos elementos agrupan las superficies (muros) del edificio
//! y definen propiedades como su tipo, pertenencia a la envolvente térmica,
//! potencia de iluminación y VEEI, además de los perfiles de uso y ocupación.

use std::convert::TryFrom;

use anyhow::{bail, format_err, Error};

use super::super::BdlBlock;
use super::geom::Polygon;

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
    /// Desviación de la Y+ del espacio respecto al sistema coordenado del edificio (grados) [0-360]
    /// Ángulo del eje +Y respecto al eje +Y del edificio (eje de rotación Z).
    /// Sentido horario (x+/E+)
    pub angle_with_building_north: f32,
    /// Pertenencia a la envolvente térmica
    pub insidete: bool,
    /// Planta a la que pertenece el espacio
    pub floor: String,
    /// Potencia de iluminación (W/m²)
    /// En vivienda es 4.4W/m²
    /// En terciario se introduce en la interfaz
    pub power: f32,
    /// VEEI del edificio objeto W/m²/100lux
    /// En terciario se introduce en la interfaz
    /// En vivienda es 7W/m²·100lux
    pub veei_obj: f32,
    /// VEEI del edificio de referencia W/m²/100lux
    /// En vivienda es 10W/m²·100lux
    /// En terciario se introduce en la interfaz
    pub veei_ref: f32,
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
        let bdl_azimuth = attrs.remove_f32("AZIMUTH").unwrap_or_default();
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
        let veei_obj = attrs.remove_f32("VEEI-OBJ")?;
        let veei_ref = attrs.remove_f32("VEEI-REF")?;
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
        // En espacios no habitables se usa en SPACE-CONDITIONS el método AIR-CHANGE que usa
        // el parámetro INF-FLOW/AREA para indicar las infiltraciones (¡en renh!) del espacio.
        // Se indican los no habitables según perfiles de nivel de estanqueidad que tienen los valores de AIR-CHANGES/HR en
        // las SPACE-CONDITIONS que se señalan más abajo.
        // En otros tipos de espacios se usa en sus SPACE-CONDTIONS el método CRACK-AIR-CHANGE que
        // calcula las infiltraciones durante el tiempo de ventilación (con un valor de infiltraciones de m³/h·m² de espacio,
        // con INF-FLOW/AREA del SPACE-CONDITIONS y que vamos a ignorar aquí, ya que tiene un valor fijo de 7.2m³/hm² en todos los perfiles tipo
        // aunque es editable en GT, que da caudal=INF-FLOW/AREA/SPACE-HEIGHT) y un valor fijo de renovaciones hora del SPACE el resto del
        // tiempo, que se obtiene de AIR-CHANGES/HR del SPACE (en renh)
        // Para el método CRACK se calculan las infiltraciones a partir de los elementos de la envolvente y
        // no se consideran infiltraciones constantes, así que usamos None
        let airchanges_h = match (stype.as_str(), spaceconds.as_str()) {
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_1") => Some(0.1), // 0 renh en GT y 13789:1999 y 0.1 renh en 13789:2017
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_2") => Some(0.5), // no cambia entre 13789:1999 y 2017
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_3") => Some(1.0), // no cambia entre 13789:1999 y 2017
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_4") => Some(3.0), // 5 renh en GT y 13789:1999 y 3 renh en 13789:2017
            ("UNHABITED", "NIVEL_ESTANQUEIDAD_5") => Some(10.0), // no cambia entre 13789:1999 y 2017
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
            angle_with_building_north: bdl_azimuth,
            insidete,
            floor,
            power,
            veei_obj,
            veei_ref,
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
