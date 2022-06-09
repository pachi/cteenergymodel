// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Interpretación de los datos generales del .ctehexml

use anyhow::{format_err, Error};

use super::xmlhelpers::{get_tag_as_f32, get_tag_as_i32, get_tag_as_str};

#[derive(Debug, Clone)]
/// Datos del bloque DatosGenerales
/// hay algunos campos que aparentemente no se usan/cambian como: tipoUso
pub struct DatosGenerales {
    /// Nombre del proyecto
    pub nombre_proyecto: String,
    /// Tipo de edificio:
    /// - Vivienda unifamiliar: Unifamiliar
    /// - Viviendas en bloque: Bloque
    /// - Una sola vivienda de un bloque: UnaBloque
    /// - Terciario pequeño o mediano (PMT): Terciario
    /// - Gran tericario (GT): Gran
    pub tipo_vivienda: String,
    /// Edificios nuevos
    /// - Nuevo
    /// Edificios Existentes:
    /// - Ampliacion
    /// - Cambio (cambio de uso)
    /// - Reformas:
    ///     - Más del 25% de envolvente
    ///         - CambioMas25ConSistemas (con cambio de sistemas de clima y ACS)
    ///         - CambioMas25ConSistemasClim (con cambio de sistemas de clima)
    ///         - CambioMas25ConSistemasACS (con cambio de sistemas de ACS)
    ///         - CambioMas25SinSistemas (sin cambio de sistemas)
    ///     - Menos del 25% de envolvente
    ///         - CambioMenos25ConSistemas
    ///         - CambioMenos25ConSistemasClim
    ///         - CambioMenos25ConSistemasACS
    ///         - CambioMenos25SinSistemas
    /// Sólo certificación de edificios existentes:
    /// - Certificacion
    pub tipo_definicion: String,
    /// Número de viviendas del edificio
    pub num_viviendas_bloque: i32,
    /// Caudal de ventilación l/s
    pub valor_impulsion_aire: f32,
    /// Zona climática (alfa1, A1, ...). Este valor no indica si el clima es canario o no
    pub zona_climatica: String,
    /// Zona climática según archivo climático (alfa1c, A1, A1c, ...)
    pub archivo_climatico: String,
    /// Valor del ensayo de permeabilidad (solo residencial)
    pub valor_n50_medido: Option<f32>,
    /// Contenido del bloque en texto, sin parsear
    pub bloque_raw: String,
}

impl Default for DatosGenerales {
    fn default() -> Self {
        Self {
            nombre_proyecto: "Proyecto nuevo".into(),
            tipo_vivienda: "Unifamiliar".into(),
            tipo_definicion: "Nuevo".into(),
            num_viviendas_bloque: 1,
            valor_impulsion_aire: 0.0,
            zona_climatica: "D3".into(),
            archivo_climatico: "D3".into(),
            valor_n50_medido: None,
            bloque_raw: "".into(),
        }
    }
}

pub fn parse_datos_generales(doc: &roxmltree::Document) -> Result<DatosGenerales, Error> {
    let datos_generales = doc
        .descendants()
        .find(|n| n.tag_name().name() == "DatosGenerales")
        .ok_or_else(|| format_err!("Etiqueta <DatosGenerales> no encontrada en el XML"))?;

    let valor_n50_medido = match get_tag_as_str(&datos_generales, "ensayoPermeabilidad") {
        "SI" => Some(get_tag_as_f32(&datos_generales, "ValorN50Medido")?),
        _ => None,
    };

    // XXX: No usamos zona_climatica puesto que no diferencia climas canarios
    let zona_climatica = get_tag_as_str(&datos_generales, "zonaClimatica").to_string();
    let archivo_climatico =
        get_tag_as_str(&datos_generales, "pathArchivoMeteorologicoSeleccionado")
            .split(".bin")
            .take(1)
            .collect::<Vec<_>>()
            .get(0)
            .map(|s| {
                s.split("zona")
                    .take(2)
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap_or(&"")
                    .to_string()
            })
            .unwrap_or_else(|| "".to_string());

    Ok(DatosGenerales {
        nombre_proyecto: get_tag_as_str(&datos_generales, "nomPro").to_string(),
        tipo_vivienda: get_tag_as_str(&datos_generales, "tipoVivienda").to_string(),
        tipo_definicion: get_tag_as_str(&datos_generales, "tipoDefinicion").to_string(),
        num_viviendas_bloque: get_tag_as_i32(&datos_generales, "numViviendasBloque")?,
        valor_impulsion_aire: get_tag_as_f32(&datos_generales, "valorImpulsionAire")?,
        zona_climatica,
        archivo_climatico,
        valor_n50_medido,
        bloque_raw: datos_generales.text().unwrap_or("").trim().to_string(),
    })
}
