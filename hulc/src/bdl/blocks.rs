// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Bloques genéricos de BDL

use super::AttrMap;

use anyhow::{bail, Error};

// Objetos ----------------------------------------------------------------

/// Bloque de datos de BDL
#[derive(Clone, Debug, Default)]
pub struct BdlBlock {
    /// Tipo de bloque
    pub btype: String,
    /// Nombre del elemento o material
    pub name: String,
    // Elemento madre, referenciado por nombre
    pub parent: Option<String>,
    /// Conjunto de propiedades
    pub attrs: AttrMap,
}

impl std::str::FromStr for BdlBlock {
    type Err = Error;
    /// Convierte de cadena a bloque
    ///
    /// Ejemplo:
    /// ```text
    ///     "P01_E01_Pol2" = POLYGON
    ///     V1   =( 14.97, 11.39 )
    ///     V2   =( 10.84, 11.39 )
    ///     V3   =( 10.86, 0 )
    ///     V4   =( 18.22, 0 )
    ///     V5   =( 18.22, 9.04 )
    ///     V6   =( 14.97, 9.04 )
    ///     ..
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Separa encabezado del resto
        let stanza: Vec<_> = s.splitn(2, '\n').map(str::trim).collect();

        // Algunos bloques pueden estar vacíos y no tener name, como
        // "LOADS-REPORT", "SYSTEMS-REPORT", "PLANT-REPORT"
        if stanza.len() == 1
        {
            return Ok(BdlBlock {
                name: stanza[0].to_string(),
                btype: stanza[0].to_string(),
                parent: None,
                attrs: AttrMap::new(),
            });
        }

        let [bheadline, bdata] = if let [bheadline, bdata] = stanza.as_slice() {
            [bheadline, bdata]
        } else {
            bail!("Error al interpretar el bloque: '{:?}'", s);
        };
        // Interpreta encabezado como nombre = tipo
        let headlineparts = bheadline
            .splitn(2, '=')
            .map(str::trim)
            .map(|s| s.trim_matches('"'))
            .collect::<Vec<_>>();
        let [name, btype] = if let [name, btype] = headlineparts.as_slice() {
            [*name, *btype]
        } else if !headlineparts.is_empty() && headlineparts[0].ends_with("-REPORT") {
            [headlineparts[0], headlineparts[0]]
        } else {
            bail!(
                "Error al parsear el encabezado '{}'\ndel bloque:\n{:?}",
                bheadline,
                s
            );
        };
        // Lee atributos
        let attrs = parse_attributes(bdata)?;
        let name = name.trim().to_string();
        // Construye el objeto
        Ok(BdlBlock {
            name,
            btype: btype.to_string(),
            parent: None,
            attrs,
        })
    }
}

/// Elimina líneas en blanco y comentarios
fn clean_lines(input: &str) -> String {
    input
        .replace("\r\n", "\n") // Normalizar saltos de línea
        .replace('ÿ', "") // Marcador de LIDER (antiguo)
        .lines()
        .map(str::trim)
        .filter(|l| {
            !l.is_empty() // Líneas en blanco
            && !l.starts_with('$') // Comentarios
            && !l.starts_with('+') // Encabezados de LIDER (antiguo)
            && !l.starts_with("TEMPLARY") // Separador de parte de lider del BDL "estándar"
            && *l != "MARCOS"
            && *l != "HUECOS"
            && *l != "PUENTES TERMICOS"
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Limpia y corrige datos de LIDER para tener bloques BDL bien formateados
///
/// Elimina comentarios y líneas en blanco
/// Corrige bloque de datos de LIDER mal formados
fn sanitize_lider_data(input: &str) -> String {
    // Elimna comentarios y líneas innecesarias
    let cleanlines = clean_lines(input);

    // Si existe, separamos una parte inicial de atributos sueltos de LIDER,
    // sin bloque, del resto de contenido, que es BDL válido:
    // CAMBIO = SI
    // CAMBIO-CALENER = NO
    // EEGeneradaAutoconsumida        = "0"
    // PANELFOTOVOLTAICOAUTOCONSUMIDO =              0
    // CONTRIBUCIONRESACS             =           1800
    // ENERGIAGT  = YES
    let (_lider_part, bdl_part) =
        if let Some(pos) = cleanlines.find("\"DATOS GENERALES\" = GENERAL-DATA") {
            cleanlines.split_at(pos)
        } else if let Some(pos) = cleanlines.find("\"Defecto\" = DESCRIPTION") {
            cleanlines.split_at(pos)
        } else {
            return cleanlines;
        };
    format!(
        "\"PARTELIDER\" = PARTELIDER\n{}\n..\n{}",
        _lider_part, bdl_part
    )
}

pub fn build_blocks<T: AsRef<str>>(input: T) -> Result<Vec<BdlBlock>, Error> {
    let cleandata = sanitize_lider_data(input.as_ref());

    let blockstrs = cleandata
        .split("..")
        .map(str::trim)
        .filter(|v| !v.is_empty());

    let mut blocks = Vec::<BdlBlock>::new();
    let mut currentfloor = "Default".to_string();
    let mut currentspace = String::new();
    let mut currentwall = String::new();

    for block in blockstrs {
        // Ignoramos bloques SET-DEFAULT del antiguo LIDER
        // Ignora bloques "END", "COMPUTE", "STOP"
        if block.starts_with("SET-DEFAULT")
            || block.starts_with("END")
            || block.starts_with("COMPUTE")
            || block.starts_with("STOP")
        {
            continue;
        };
        let mut bdlblock: BdlBlock = block.parse()?;
        // Corrige el elemento madre
        let parent = match bdlblock.btype.as_str() {
            // Las plantas no cuelgan de ningún elemento
            "FLOOR" => {
                currentfloor = bdlblock.name.clone();
                None
            }
            // Los espacios cuelgan de las plantas
            "SPACE" => {
                currentspace = bdlblock.name.clone();
                Some(currentfloor.clone())
            }
            // Los muros cuelgan de los espacios
            "EXTERIOR-WALL" | "INTERIOR-WALL" | "ROOF" | "UNDERGROUND-WALL"
            | "UNDERGROUND-FLOOR" => {
                currentwall = bdlblock.name.clone();
                Some(currentspace.clone())
            }
            // Las construcciones y ventanas cuelgan de los muros
            "CONSTRUCTION" | "WINDOW" | "DOOR" => Some(currentwall.clone()),
            _ => None,
        };
        bdlblock.parent = parent;
        blocks.push(bdlblock);
    }
    Ok(blocks)
}

/// Lee atributos de bloque BDL
fn parse_attributes(data: &str) -> Result<AttrMap, Error> {
    let mut attributes = AttrMap::new();
    let mut lines = data.lines().map(str::trim);
    while let Some(l) = lines.next() {
        // Continua con marca de fin de bloque o con comillas aisladas, que hemos visto en algún caso raro
        if l == ".." || l == "\"" {
            continue;
        };
        if let [key, value] = l
            .splitn(2, '=')
            .map(str::trim)
            .collect::<Vec<_>>()
            .as_slice()
        {
            // Valores simples o con paréntesis
            let value = if value.starts_with('(') && !value.ends_with(')') {
                let mut values = vec![*value];
                while let Some(newvalueline) = lines.next() {
                    let val = newvalueline.trim();
                    values.push(val);
                    if val.ends_with(')') {
                        break;
                    };
                }
                values.join("").to_string()
            } else {
                value.trim_matches('"').to_string()
            };
            attributes.insert(key, &value);
        } else {
            bail!(
                "No se ha podido extraer clave y atributo de la línea '{}' en '{:#?}'",
                l,
                lines
            )
        }
    }
    Ok(attributes)
}
