// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Composiciones constructivas de cerramientos opacos (LAYERS)

use std::{collections::HashMap, convert::TryFrom};

use anyhow::{format_err, Error};

use crate::bdl::{extract_f32vec, extract_namesvec, BdlBlock, Material};

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
    /// XXX: esta no es una definición del BDL pero lo usaremos para tomarla de Construction y evitar ese objeto
    pub absorptance: f32,
}

impl WallCons {
    /// Espesor total de una composición de capas [m]
    pub fn total_thickness(&self) -> f32 {
        self.thickness.iter().sum()
    }

    /// Resistencia térmica de una composición de capas [W/m2K]
    pub fn r_intrinsic(&self, materialsdb: &HashMap<String, Material>) -> Result<f32, Error> {
        let materials = &self
            .material
            .iter()
            .map(|m| {
                materialsdb.get(m).ok_or_else(|| {
                    format_err!(
                        "No se encuentra el material \"{}\" de la composición de capas \"{}\"",
                        m,
                        self.name
                    )
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        materials
            .iter()
            .zip(&self.thickness)
            // Resistencias térmicas de las capas
            .map(|(mat, thk)| match mat.properties {
                Some(props) if props.conductivity != 0.0 => Some(thk / props.conductivity),
                None => mat.resistance,
                _ => None,
            })
            // Resistencia térmica total
            .try_fold(0.0_f32, |acc, x| x.map(|res| res + acc))
            .ok_or_else(|| {
                format_err!(
                    "Error al calcular la resistencia térmica de la composición \"{}\"",
                    self.name
                )
            })
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
        Ok(Self {
            name,
            group,
            material,
            thickness,
            absorptance: 0.0,
        })
    }
}
