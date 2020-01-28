//! Parser del Building Description Language (BDL) de DOE
//!
//! Material (MATERIAL) (tipo PROPERTIES o RESISTANCE))

use failure::Error;
use std::convert::TryFrom;

use crate::bdl::BdlBlock;

/// Material definido por sus propiedades térmicas o por resistencia
#[derive(Debug, Clone, Default)]
pub struct Material {
    /// Nombre del material
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Definición detallada de propiedades (labda, rho, C_p, mu, ...)
    pub properties: Option<MaterialProperties>,
    /// Definición de resistencia térmica R (m2K/W)
    pub resistance: Option<f32>,
}

/// Definición de propiedades termofísicas y grosor
#[derive(Debug, Copy, Clone, Default)]
pub struct MaterialProperties {
    /// Espesor, d (m)
    /// En LIDER antiguo no se define este valor
    pub thickness: Option<f32>,
    /// Conductividad térmica, lambda (W/mK)
    pub conductivity: f32,
    /// Densidad, rho (kg/m3)
    pub density: f32,
    /// Calor específico, C_p (J/kg K)
    pub specificheat: f32,
    /// Factor de difusividad al vapor de agua, mu (-)
    /// En archivos de LIDER antiguo se pone por defecto 0.0 (no definido)
    pub vapourdiffusivity: Option<f32>,
}

impl TryFrom<BdlBlock> for Material {
    type Error = Error;

    /// Conversión de bloque BDL a material
    ///
    /// Ejemplo en BDL:
    /// ```text
    ///     "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm" = MATERIAL
    ///     TYPE              = PROPERTIES
    ///     THICKNESS         =           0.45
    ///     THICKNESS_CHANGE         = YES
    ///     THICKNESS_MAX         =              2
    ///     THICKNESS_MIN         =          0.001
    ///     CONDUCTIVITY      =      0.4787234
    ///     DENSITY           =           1280
    ///     SPECIFIC-HEAT     =           1000
    ///     VAPOUR-DIFFUSIVITY-FACTOR =             60
    ///     NAME          = "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm"
    ///     GROUP         = "Forjados reticulares"
    ///     IMAGE          = "ladrillo.bmp"
    ///     NAME_CALENER   = "oldeado descolgado -Canto 450 "
    ///     LIBRARY       = NO
    ///     UTIL          =  NO
    ///     OBSOLETE      = NO
    ///     ..
    ///     
    ///     $ LIDER antiguo
    ///     "AislanteREF" = MATERIAL
    ///         TYPE = PROPERTIES
    ///         CONDUCTIVITY = 0.036
    ///         DENSITY = 30
    ///         SPECIFIC-HEAT = 1800
    ///         ..
    /// ```
    /// TODO: Propiedades no convertidas:
    /// TODO: THICKNESS_CHANGE, THICKNESS_MAX, THICKNESS_MIN, IMAGE, NAME_CALENER, LIBRARY, UTIL, OBSOLETE
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        // El LIDER antiguo no guardaba grupos
        let group = attrs
            .remove_str("GROUP")
            .unwrap_or("Materiales".to_string());
        let (properties, resistance) = match attrs.remove_str("TYPE")?.as_ref() {
            "PROPERTIES" => {
                // XXX: En LIDER antiguo no se define este valor
                let thickness = attrs.remove_f32("THICKNESS").ok();
                let conductivity = attrs.remove_f32("CONDUCTIVITY")?;
                let density = attrs.remove_f32("DENSITY")?;
                let specificheat = attrs.remove_f32("SPECIFIC-HEAT")?;
                // XXX: En LIDER antiguo no se define este valor
                let vapourdiffusivity = attrs.remove_f32("VAPOUR-DIFFUSIVITY-FACTOR").ok();
                (
                    Some(MaterialProperties {
                        thickness,
                        conductivity,
                        density,
                        specificheat,
                        vapourdiffusivity,
                    }),
                    None,
                )
            }
            _ => {
                let resistance = attrs.remove_f32("RESISTANCE")?;
                (None, Some(resistance))
            }
        };
        Ok(Self {
            name,
            group,
            properties,
            resistance,
        })
    }
}
