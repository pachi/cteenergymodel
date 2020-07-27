//! Parser del Building Description Language (BDL) de DOE
//!
//! Composiciones constructivas de cerramientos opacos (LAYERS)

use failure::Error;
use std::{collections::HashMap, convert::TryFrom};

use crate::bdl::{extract_f32vec, extract_namesvec, BdlBlock, Boundaries, Material, Tilt};
use crate::utils::fround2;

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
}

// TODO: estas implementaciones deberían llevarse a WallConstruction de types cuando se creen
// y dejar las clases BDL como meros contenedores de datos
impl WallCons {
    /// Espesor total de una composición de capas [m]
    pub fn total_thickness(&self) -> f32 {
        self.thickness.iter().sum()
    }

    /// Transmitancia térmica de una composición de capas [W/m2K]
    pub fn u_intrinsic(&self, materialsdb: &HashMap<String, Material>) -> Result<f32, Error> {
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
            // Transmitancia térmica
            .and_then(|resvec| {
                if resvec != 0.0 {
                    Some(1.0 / resvec)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                format_err!(
                    "Error al calcular la transmitancia de la composición \"{}\"",
                    self.name
                )
            })
    }

    /// Transmitancia térmica del cerramiento, en W/m2K
    /// Tiene en cuenta la posición del elemento para fijar las resistencias superficiales
    /// Notas:
    /// - en particiones interiores no se considera el factor b, reductor de temperatura
    /// - NO se ha implementado el cálculo de elementos en contacto con espacios no habitables
    /// - NO se ha implementado el cálculo de cerramientos en contacto con el terreno
    ///     - en HULC los valores por defecto de Ra y D se indican en las opciones generales de
    ///       las construcciones por defecto
    /// - los elementos adiabáticos se reportan con valor 0.0
    /// - las particiones interiores horizontales
    pub fn u(
        &self,
        bounds: Boundaries,
        position: Tilt,
        materialsdb: &HashMap<String, Material>,
    ) -> f32 {
        use Boundaries::*;
        use Tilt::*;
        let u = self.u_intrinsic(&materialsdb).unwrap_or_default();

        // Resistencias superficiales [m2·K/W]
        // Revisar según DA-DB-HE/1 tabla 1
        const RSE: f32 = 0.04;
        const RSI_ASCENDENTE: f32 = 0.10;
        const RSI_HORIZONTAL: f32 = 0.13;
        const RSI_DESCENDENTE: f32 = 0.17;

        let u_noround = match bounds {
            UNDERGROUND => match position {
                // TODO: implementar soleras en contacto con el terreno
                BOTTOM => Default::default(),
                // TODO: implementar muros enterrados
                SIDE => Default::default(),
                // Cubiertas enterradas: el terreno debe estar definido como una capa de tierra con lambda = 2 W/K
                TOP => 1.0 / (1.0 / u + RSI_ASCENDENTE + RSE),
            },
            // Tomamos valor 0.0. Siempre se podría consultar la resistencia intrínseca
            ADIABATIC => 0.0,
            // HULC no diferencia entre posiciones para elementos interiores
            // TODO: Detectar el caso de contacto con espacios no habitables, con cálculo de b, e implementar
            // TODO: tal vez esto debería recibir el valor b como parámetro
            INTERIOR => 1.0 / (1.0 / u + 2.0 * RSI_HORIZONTAL),
            // Elementos en contacto con el exterior
            EXTERIOR => match position {
                BOTTOM => 1.0 / (1.0 / u + RSI_DESCENDENTE + RSE),
                TOP => 1.0 / (1.0 / u + RSI_ASCENDENTE + RSE),
                SIDE => 1.0 / (1.0 / u + RSI_HORIZONTAL + RSE),
            },
        };
        fround2(u_noround)
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
    /// TODO: Propiedades de Layers no convertidas:
    /// TODO: IMAGE, NAME_CALENER, LIBRARY, UTIL, TYPE-DEFINITION, DEFAULT
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
        })
    }
}
