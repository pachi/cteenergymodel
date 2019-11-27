//! Parser del Building Description Language (BDL) de DOE
//!
//! Referencias:
//! - http://doe2.com/DOE2/
//! - http://doe2.com/download/DOE-22/DOE22Vol2-Dictionary.pdf
//! - http://doe2.com/download/doe-23/DOE23Vol3-Topics_50h.pdf (ver Building Description Language)
//!
//! Curioso: https://github.com/protodave/bdl_viz

use std::collections::HashMap;

use failure::bail;
use failure::Error;

mod types;
pub use types::*;

// ------------------------- BDL ----------------------------

pub struct Block {
    /// Tipo de bloque
    pub btype: String,
    /// Nombre del material
    pub name: String,
    // Conjunto de propiedades
    pub attrs: AttrMap,
}

/// Datos del edificio
#[derive(Debug, Default)]
pub struct BdlBuildingData {
    /// Lista de plantas
    pub floors: Vec<Floor>,
    /// Lista de espacios
    pub spaces: Vec<Space>,
    /// Lista de polígonos
    pub polygons: HashMap<String, Polygon>,
    /// Materiales
    pub materials: Vec<Material>,
    // Elementos de la envolvente
    pub elements: Vec<BdlElementType>
}

/// Datos del archivo BDL
#[derive(Debug, Default)]
pub struct BdlData {
    pub building: BdlBuildingData,
}

impl BdlData {
    pub fn new(input: &str) -> Result<Self, Error> {
        // Datos
        let mut bdldata: BdlBuildingData = Default::default();

        // Elimina líneas en blanco y comentarios, y luego separa por bloques
        let cleanlines = input
            .replace("\r\n", "\n")
            .lines()
            .map(str::trim)
            .filter(|l| *l != "" && !l.starts_with("$"))
            .collect::<Vec<&str>>()
            .join("\n");

        // TODO: parsear y guardar _lider_part en algún lado
        let [_lider_part, bdl_part] = match cleanlines
            .splitn(2, "TEMPLARY = USER")
            .collect::<Vec<_>>()
            .as_slice() {
                [lider_part, bdl_part] => [*lider_part, *bdl_part],
                _ => panic!("Error en la estructura de datos. No se han encontrado los datos de LIDER y de USARIO")
            };

        let mut blocks = Vec::<Block>::new();

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
                    Block {
                        name: name.to_string(),
                        btype: btype.to_string(),
                        attrs,
                    }
                } else {
                    panic!("Error al parsear encabezado: {}", bheadline);
                }
            } else {
                panic!("Error al parsear el bloque: '{:?}'", stanza);
            };

            blocks.push(bdlblock);
        }

        // Parsea bloques
        for block in blocks {
            match block.btype.as_ref() {
                "MATERIAL" => {
                    bdldata.materials.push(parse_material(block)?);
                }
                "FLOOR" => {
                    bdldata.floors.push(parse_floor(block)?);
                }
                "SPACE" => {
                    // Asigna el espacio a la planta actual
                    // Genera planta por defecto si no hay una
                    if bdldata.floors.len() == 0 {
                        bdldata.floors.push(Floor::new("Default"));
                    };
                    bdldata
                        .floors
                        .last_mut()
                        .map(|f| f.spaces.push(block.name.clone()));
                    bdldata.spaces.push(parse_space(block)?);
                }
                "POLYGON" => {
                    bdldata
                        .polygons
                        .insert(block.name.clone(), parse_polygon(block)?);
                }
                "WINDOW" => {
                    // TODO: no asigna la ventana a un muro y a su vez este a un espacio
                    bdldata.elements.push(parse_window(block)?);
                }
                "EXTERIOR-WALL" => {
                    // TODO: no asigna el muro a un espacio
                    bdldata.elements.push(parse_exteriorwall(block)?);
                }
                "INTERIOR-WALL" => {
                    // TODO: no asigna el muro a un espacio
                    bdldata.elements.push(parse_interiorwall(block)?);
                }
                "UNDERGROUND-WALL" => {
                    // TODO: no asigna el muro a un espacio
                    bdldata.elements.push(parse_undergroundwall(block)?);
                }
                "LAYERS" => {
                    bdldata.elements.push(parse_layers(block)?);
                }
                "CONSTRUCTION" => {
                    bdldata.elements.push(parse_construction(block)?);
                }
                // ROOF, BUILDING-SHADE, GAP, GLASS-TYPE, NAME-FRAME, WORK-SPACE,
                // SPACE-CONDITIONS, SYSTEM-CONDITIONS, THERMAL-BRIDGE,
                // WEEK-SCHEDULE-PD, DAY-SCHEDULE-PD, SCHEDULE-PD
                _ => {
                    eprintln!(
                        "Tipo desconocido. bname: {}, btype: {}",
                        block.name, block.btype
                    );
                }
            };
        }

        Ok(Self { building: bdldata })
    }
}

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
            attributes.insert(key, value);
        } else {
            bail!("No se ha podido extraer clave y atributo de '{}'", l)
        }
    }
    Ok(attributes)
}

fn parse_material(block: Block) -> Result<Material, Error> {
    let mut el = Material::new(block.name, block.btype);
    el.attrs = block.attrs;
    el.group = el.attrs.get("GROUP")?;
    Ok(el)
}

fn parse_floor(block: Block) -> Result<Floor, Error> {
    let mut el = Floor::new(block.name);
    el.attrs = block.attrs;
    el.z = el.attrs.get_f32("Z").unwrap_or_default();
    Ok(el)
}

fn parse_space(block: Block) -> Result<Space, Error> {
    //TODO: falta el contexto para asignar el espacio a la planta
    let mut el = Space::new(block.name);
    el.attrs = block.attrs;
    el.polygon = el.attrs.get("POLYGON")?;
    el.height = el.attrs.get_f32("HEIGHT").ok();
    Ok(el)
}

fn parse_window(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar la ventana a un muro
    let mut el = Window::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::Window(el))
}

fn parse_exteriorwall(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = ExteriorWall::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::ExteriorWall(el))
}


fn parse_interiorwall(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = InteriorWall::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::InteriorWall(el))
}

fn parse_undergroundwall(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = UndergroundWall::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::UndergroundWall(el))
}

fn parse_construction(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = Construction::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::Construction(el))
}

fn parse_layers(block: Block) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = Layers::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::Layers(el))
}

fn parse_polygon(block: Block) -> Result<Polygon, Error> {
    let attrs = block.attrs;
    let mut polygon = Polygon::new(block.name);
    for (_k, v) in &attrs.0 {
        let vec = v.parse()?;
        polygon.vectors.push(vec)
    }
    Ok(polygon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctehexml::parse;

    #[test]
    fn test_bdl() {
        let data = parse("tests/00_plurif_s3_v0_d3/00_plurif_s3_v0_d3.ctehexml").unwrap();
        let bdldata = BdlData::new(&data.entrada_grafica_lider).unwrap();
        println!("{:#?}", bdldata.building);
    }
}
