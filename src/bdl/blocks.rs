//! Parser del Building Description Language (BDL) de DOE
//!
//! Bloques genéricos de BDL

use super::types::AttrMap;

use failure::bail;
use failure::Error;

// Objetos ----------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct BdlBlock {
    /// Tipo de bloque
    pub btype: String,
    /// Nombre del material
    pub name: String,
    // Elemento madre, referenciado por nombre
    pub parent: Option<String>,
    /// Conjunto de propiedades
    pub attrs: AttrMap,
}

pub fn build_blocks(bdl_part: &str) -> Result<Vec<BdlBlock>, Error> {
    let mut blocks = Vec::<BdlBlock>::new();
    let mut currentfloor = "Default".to_string();
    let mut currentspace = String::new();
    let mut currentwall = String::new();

    for block in bdl_part
        .split("..")
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        let stanza: Vec<_> = block.splitn(2, '\n').map(str::trim).collect();

        let bdlblock = if let [bheadline, bdata] = stanza.as_slice() {
            if let [name, btype] = bheadline
                .splitn(2, '=')
                .map(str::trim)
                .map(|s| s.trim_matches('"'))
                .collect::<Vec<_>>()
                .as_slice()
            {
                let attrs = parse_attributes(bdata)?;
                let name = name.to_string();
                let parent = match *btype {
                    // Las plantas no cuelgan de ningún elemento
                    "FLOOR" => {
                        currentfloor = name.clone();
                        None
                    }
                    // Los espacios cuelgan de las plantas
                    "SPACE" => {
                        currentspace = name.clone();
                        Some(currentfloor.clone())
                    }
                    // Los muros cuelgan de los espacios
                    "EXTERIOR-WALL" | "INTERIOR-WALL" | "ROOF" | "UNDERGROUND-WALL"
                    | "UNDERGROUND-FLOOR" => {
                        currentwall = name.clone();
                        Some(currentspace.clone())
                    }
                    // Las construcciones y ventanas cuelgan de los muros
                    "CONSTRUCTION" | "WINDOW" | "DOOR" => Some(currentwall.clone()),
                    _ => None,
                };

                BdlBlock {
                    name: name.clone(),
                    btype: btype.to_string(),
                    parent,
                    attrs,
                }
            } else {
                bail!(
                    "Error al parsear el encabezado '{}'\ndel bloque:\n{:?}",
                    bheadline,
                    stanza
                );
            }
        } else {
            bail!("Error al parsear el bloque: '{:?}'", stanza);
        };

        blocks.push(bdlblock);
    }
    Ok(blocks)
}

/// Lee atributos de bloque BDL
fn parse_attributes(data: &str) -> Result<AttrMap, Error> {
    let mut attributes = AttrMap::new();
    let mut lines = data.lines();
    while let Some(l) = lines.next() {
        if let [key, value] = l.split('=').map(str::trim).collect::<Vec<_>>().as_slice() {
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
            bail!("No se ha podido extraer clave y atributo de '{}'", l)
        }
    }
    Ok(attributes)
}
