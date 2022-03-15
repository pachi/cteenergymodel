// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Composiciones constructivas de cerramientos opacos (LAYERS)

use std::convert::TryFrom;

use anyhow::{format_err, Error};

use crate::bdl::{extract_f32vec, extract_namesvec, BdlBlock};

/// Definición de elemento a través de sus capas
#[derive(Debug, Clone, Default)]
pub struct WallCons {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Lista de nombres de materiales de las capas ([mat1, mat2, ...])
    pub material: Vec<String>,
    /// Lista de espesores de las capas [m] ([e1, e2, ...])
    pub thickness: Vec<f32>,
    /// Absortividad (a la radiación solar) (-)
    /// NOTE: esta propiedad se toma del objeto Construction de BDL y no de Layers pero lo recogemos aquí y evitamos definir ese objeto
    pub absorptance: f32,
}

impl WallCons {
    /// Espesor total de una composición de capas [m]
    pub fn thickness(&self) -> f32 {
        self.thickness.iter().sum()
    }
}

impl TryFrom<BdlBlock> for WallCons {
    type Error = Error;

    /// Conversión de bloque BDL a definición de capas
    ///
    /// Ejemplo:
    /// ```text
    ///     "muro_opaco" = LAYERS
    ///         GROUP        = "envolvente"
    ///         NAME_CALENER = ""
    ///         NAME         = "muro_opaco"
    ///         TYPE-DEFINITION = 1
    ///         MATERIAL     = ("Mortero de cemento o cal para albañilería y para revoco/enlucido 1000 < d < 1250","EPS Poliestireno Expandido [ 0.029 W/[mK]]","1/2 pie LP métrico o catalán 80 mm< G < 100 mm","MW Lana mineral [0.031 W/[mK]]","Placa de yeso laminado [PYL] 750 < d < 900")
    ///         THICKNESS = (          0.015,           0.06,          0.115,           0.04,           0.02)
    ///         LIBRARY       =  NO
    ///         UTIL          =  YES
    ///         IMAGE = ""
    ///         DEFAULT = NO
    ///         ..
    ///
    ///     $ LIDER antiguo
    ///     "CONST_referencia-5" = LAYERS
    ///         MATERIAL = ( "PlaquetaREF","MorteroREF","ForjadoREF" )
    ///         THICKNESS = ( 0.015, 0.020, 0.250 )
    ///         ..
    /// ```
    /// XXX: Propiedades de Layers no convertidas:
    /// XXX: IMAGE, NAME_CALENER, LIBRARY, UTIL, TYPE-DEFINITION, DEFAULT
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        // En LIDER antiguo no se guarda el grupo
        let group = attrs
            .remove_str("GROUP")
            .unwrap_or_else(|_| "Capas".to_string());
        let material = extract_namesvec(attrs.remove_str("MATERIAL")?);
        let thickness = extract_f32vec(attrs.remove_str("THICKNESS")?)?;
        if material.len() != thickness.len() {
            return Err(format_err!(
                "El material {} tiene una lista de materiales y de grosores de distinta longitud",
                name
            ));
        }
        // Los grosores de las capas no son correctas en el caso de Cámaras de aire,
        // ya que HULC no está definido el grosor en los materiales y las considera con un grosor por defecto de 5cm
        // Aquí corregimos los grosores
        let fixed_thickness: Vec<f32> = material
            .iter()
            .zip(thickness.iter())
            .map(|(name, thickness)| {
                if name.starts_with("Cámara de aire ") {
                    match &name[name.len() - 5..] {
                        " 1 cm" => 0.01,
                        " 2 cm" => 0.02,
                        " 5 cm" => 0.05,
                        "10 cm" => 0.10,
                        _ => *thickness,
                    }
                } else {
                    *thickness
                }
            })
            .collect();

        Ok(Self {
            name,
            group,
            material,
            thickness: fixed_thickness,
            absorptance: 0.0,
        })
    }
}
