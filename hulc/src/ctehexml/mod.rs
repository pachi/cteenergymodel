// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Funciones relacionadas con la interpretación de archivos .ctehexml

mod datosgenerales;
mod systems;

use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Error;
use flate2::read::GzDecoder;

use crate::bdl::Data;
use crate::utils::file::{find_file_in_basedir, read_file};

use datosgenerales::{parse_datos_generales, DatosGenerales};
use systems::{parse_systems, System};

static LIDERCATSTRZ: &[u8] = include_bytes!("BDCatalogo.bdc.utf8.gz");

/// Datos del archivo .ctehexml
#[derive(Debug, Default, Clone)]
pub struct CtehexmlData {
    /// Datos generales
    pub datos_generales: DatosGenerales,
    /// Datos del BDL
    pub bdldata: Data,
    /// Definiciones de factores de corrección de sistemas
    pub factores_correccion_sistemas: Vec<String>,
    /// Bloques de definición de sistemas
    pub sistemas: Vec<System>,
}

/// Localiza archivo .ctehexml en el directorio de proyecto basedir
pub fn find_ctehexml<T: AsRef<str>>(basedir: T) -> Result<Option<PathBuf>, Error> {
    find_file_in_basedir(basedir, "*.ctehexml")
}

/// Carga archivo .ctehexml y extiende con BBDD por defecto de HULC
pub fn parse_with_catalog_from_path<T: AsRef<Path>>(path: T) -> Result<CtehexmlData, Error> {
    // Carga archivo .ctehexml
    let data = read_file(path.as_ref())?;
    parse_with_catalog(&data)
}

/// Lee estructura de datos desde patch de archivo .ctehexml
pub fn parse_from_path<T: AsRef<Path>>(path: T) -> Result<CtehexmlData, Error> {
    let utf8data = read_file(path.as_ref())?;
    parse(&utf8data)
}

/// Carga archivo .ctehexml y extiende con BBDD por defecto de HULC
pub fn parse_with_catalog(data: &str) -> Result<CtehexmlData, Error> {
    // Carga datos del .ctehexml
    let mut ctehexmldata = parse(data)?;
    let mut db = ctehexmldata.bdldata.db;
    // Carga datos del catálogo comprimido
    let catdb = load_lider_catalog()?;
    db.materials.extend(catdb.materials);
    db.wallcons.extend(catdb.wallcons);
    db.wincons.extend(catdb.wincons);
    db.glasses.extend(catdb.glasses);
    db.frames.extend(catdb.frames);
    ctehexmldata.bdldata.db = db;
    Ok(ctehexmldata)
}

/// Carga datos del catálogo comprimido de LIDER
pub fn load_lider_catalog() -> Result<crate::bdl::DB, Error> {
    let mut gz = GzDecoder::new(LIDERCATSTRZ);
    let mut dbstring = String::new();
    gz.read_to_string(&mut dbstring)?;
    Ok(Data::new(&dbstring)?.db)
}

/// Lee estructura de datos desde cadena con formato de archivo .ctehexml
pub fn parse(data: &str) -> Result<CtehexmlData, Error> {
    // Localiza datos en XML
    let doc = roxmltree::Document::parse(data)?;

    // Datos generales
    let datos_generales = parse_datos_generales(&doc)?;

    // BDL Lider
    let entrada_grafica_lider = doc
        .descendants()
        .find(|n| n.has_tag_name("EntradaGraficaLIDER"))
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim()
        .to_string();
    let bdldata = Data::new(&entrada_grafica_lider)?;

    let (factores_correccion_sistemas, sistemas) = parse_systems(&doc);

    Ok(CtehexmlData {
        datos_generales,
        bdldata,
        factores_correccion_sistemas,
        sistemas,
    })
}
