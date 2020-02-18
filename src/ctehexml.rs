/* -*- coding: utf-8 -*-

Copyright (c) 2018 Rafael Villar Burke <pachi@ietcc.csic.es>

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

use std::collections::HashMap;

use failure::Error;

use crate::bdl::Data;
use crate::utils::read_file;

#[derive(Debug)]
pub struct CtehexmlData {
    // Datos buenos
    pub datos_generales: String,
    pub bdldata: Data,
    pub definicion_sistemas: String,
    // Datos legacy - a eliminar en refactorización
    pub climate: String,
    pub gglshwi: HashMap<String, f32>,
}

// Lee estructura de datos desde cadena con formato de archivo .ctehexml
pub fn parse<T: AsRef<str>>(path: T) -> Result<CtehexmlData, Error> {
    let utf8buf = read_file(path.as_ref())?;

    // Localiza datos en XML
    let doc = roxmltree::Document::parse(&utf8buf)?;

    let datos_generales = doc
        .descendants()
        .find(|n| n.tag_name().name() == "DatosGenerales")
        .ok_or_else(|| format_err!("Etiqueta <DatosGenerales> no encontrada en el XML"))?;

    let entrada_grafica_lider = doc
        .descendants()
        .find(|n| n.tag_name().name() == "EntradaGraficaLIDER")
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
        .to_string();

    // TODO: solución temporal sin descender en elementos
    let definicion_sistemas = doc
        .descendants()
        .find(|n| n.tag_name().name() == "Definicion_Sistema")
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
        .to_string();

    let bdldata = Data::new(&entrada_grafica_lider)?;

    // gglshwi de huecos
    let gglshwi: HashMap<String, f32> = bdldata
        .windows
        .iter()
        // TODO: usamos el default pero deberíamos adaptarnos a los distintos formatos:
        // TODO: - información pública -> gglshwi en WINDOW
        // TODO: - versión final -> gglshwi en GAP
        // TODO: - LIDER antiguo -> no existe
        .map(|w| {
            (
                w.name.to_string(),
                bdldata
                    .db
                    .windows
                    .get(&w.gap)
                    .unwrap()
                    .gglshwi
                    .unwrap_or_default(),
            )
        })
        .collect();

    // Zona climática
    let climate = datos_generales
        .descendants()
        .find(|n| n.tag_name().name() == "zonaClimatica")
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
        .to_string();

    Ok(CtehexmlData {
        datos_generales: datos_generales.text().unwrap_or("").trim().to_string(),
        bdldata,
        definicion_sistemas,
        gglshwi,
        climate,
    })
}

// TODO: convertir a DB sin datos innecesarios del catálogo
static LIDERCATSTR: &'static str = include_str!("BDCatalogo.bdc.utf8");

/// Carga archivo .ctehexml y extiende con BBDD por defecto de HULC
pub fn parse_with_catalog<T: AsRef<str>>(path: T) -> Result<CtehexmlData, Error> {
    // Carga archivo .ctehexml
    let mut ctehexmldata = parse(path.as_ref())?;
    let mut db = ctehexmldata.bdldata.db;
    // Carga datos del catálogo
    let catdb = Data::new(&LIDERCATSTR)?.db;
    db.materials.extend(catdb.materials.clone());
    db.layers.extend(catdb.layers.clone());
    db.windows.extend(catdb.windows.clone());
    db.glasses.extend(catdb.glasses.clone());
    db.frames.extend(catdb.frames.clone());
    ctehexmldata.bdldata.db = db;
    Ok(ctehexmldata)
}
