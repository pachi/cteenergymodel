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

// Funciones relacionadas con la interpretación de archivos .ctehexml

use std::path::{Path, PathBuf};

use failure::Error;

use crate::bdl::Data;
use crate::utils::{find_file_in_basedir, read_file};

/// Datos del archivo .ctehexml
#[derive(Debug, Clone)]
pub struct CtehexmlData {
    /// Datos generales
    pub datos_generales: DatosGenerales,
    /// Datos del BDL
    pub bdldata: Data,
    /// Bloque de definición de sistemas
    pub definicion_sistemas: String,
}

#[derive(Debug, Clone)]
/// Datos del bloque DatosGenerales
/// hay algunos campos que aparentemente no se usan/cambian como: tipoUso
pub struct DatosGenerales {
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

/// Localiza archivo .ctehexml en el directorio de proyecto basedir
pub fn find_ctehexml<T: AsRef<str>>(basedir: T) -> Result<Option<PathBuf>, Error> {
    find_file_in_basedir(basedir, "*.ctehexml")
}

/// Devuelve contenido de la etiqueta como texto
fn get_tag_as_str<'a>(parent: &'a roxmltree::Node, tag: &str) -> &'a str {
    parent
        .descendants()
        .find(|n| n.tag_name().name() == tag)
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
}

/// Devuelve contenido de la etiqueta como f32
fn get_tag_as_f32(parent: &roxmltree::Node, tag: &str) -> Result<f32, Error> {
    get_tag_as_str(parent, tag)
        .parse::<f32>()
        .map_err(|_e| format_err!("Error al convertir número"))
}

/// Devuelve contenido de la etiqueta como i32
fn get_tag_as_i32(parent: &roxmltree::Node, tag: &str) -> Result<i32, Error> {
    get_tag_as_str(parent, tag)
        .parse::<i32>()
        .map_err(|_e| format_err!("Error al convertir número"))
}

/// Lee estructura de datos desde cadena con formato de archivo .ctehexml
pub fn parse<T: AsRef<Path>>(path: T) -> Result<CtehexmlData, Error> {
    let utf8buf = read_file(path.as_ref())?;

    // Localiza datos en XML
    let doc = roxmltree::Document::parse(&utf8buf)?;

    // Datos generales
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
            .unwrap_or("".to_string());

    let datos_generales = DatosGenerales {
        tipo_vivienda: get_tag_as_str(&datos_generales, "tipoVivienda").to_string(),
        tipo_definicion: get_tag_as_str(&datos_generales, "tipoDefinicion").to_string(),
        num_viviendas_bloque: get_tag_as_i32(&datos_generales, "numViviendasBloque")?,
        valor_impulsion_aire: get_tag_as_f32(&datos_generales, "valorImpulsionAire")?,
        zona_climatica,
        archivo_climatico,
        valor_n50_medido,
        bloque_raw: datos_generales.text().unwrap_or("").trim().to_string(),
    };

    // BDL Lider
    let entrada_grafica_lider = doc
        .descendants()
        .find(|n| n.tag_name().name() == "EntradaGraficaLIDER")
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
        .to_string();
    let bdldata = Data::new(&entrada_grafica_lider)?;

    // Definición de sistemas - Solución temporal sin descender en elementos
    let definicion_sistemas = doc
        .descendants()
        .find(|n| n.tag_name().name() == "Definicion_Sistema")
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
        .to_string();

    Ok(CtehexmlData {
        datos_generales,
        bdldata,
        definicion_sistemas,
    })
}

// TODO: convertir a DB sin datos innecesarios del catálogo
static LIDERCATSTR: &str = include_str!("BDCatalogo.bdc.utf8");

/// Carga archivo .ctehexml y extiende con BBDD por defecto de HULC
pub fn parse_with_catalog<T: AsRef<Path>>(path: T) -> Result<CtehexmlData, Error> {
    // Carga archivo .ctehexml
    let mut ctehexmldata = parse(path.as_ref())?;
    let mut db = ctehexmldata.bdldata.db;
    // Carga datos del catálogo
    let catdb = Data::new(&LIDERCATSTR)?.db;
    db.materials.extend(catdb.materials);
    db.wallcons.extend(catdb.wallcons);
    db.windowcons.extend(catdb.windowcons);
    db.glasses.extend(catdb.glasses);
    db.frames.extend(catdb.frames);
    ctehexmldata.bdldata.db = db;
    Ok(ctehexmldata)
}
