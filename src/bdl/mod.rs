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
    // Construcciones (de huecos y opacos)
    // TODO:
    /// Elementos de la envolvente
    pub elements: Vec<BdlElementType>,
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

        let mut blocks = Vec::<BdlBlock>::new();

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
                    BdlBlock {
                        name: name.to_string(),
                        btype: btype.to_string(),
                        parent: None,
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
                // Elementos globales =========================
                // WEEK-SCHEDULE-PD
                // DAY-SCHEDULE-PD
                // SCHEDULE-PD
                // WORK-SPACE (global)
                // SPACE-CONDITIONS (para los espacios)
                // SYSTEM-CONDITIONS (para los espacios)

                // Componentes de la envolvente ===============
                // Materiales y construcciones ----------------
                "MATERIAL" => {
                    bdldata.materials.push(parse_material(block)?);
                }
                "LAYERS" => {
                    bdldata.elements.push(parse_layers(block)?);
                }
                "CONSTRUCTION" => {
                    bdldata.elements.push(parse_construction(block)?);
                }
                // GAP -> equivalente a construction pero para WINDOW (infiltraciones, tipo de marco y vidrio)
                // NAME-FRAME -> Marco (GAP) (conductividad y absortividad)
                // GLASS-TYPE -> tipo de vidrio en WINDOW o GAP (factor de sombra y conductividad)

                // Elementos geométricos y espacios -----------
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
                // Polígonos. Definen la geometría, mediante el atributo POLYGON de:
                // - EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL
                // - FLOOR y SPACE
                "POLYGON" => {
                    bdldata
                        .polygons
                        .insert(block.name.clone(), parse_polygon(block)?);
                }

                // Elementos opacos de la envolvente -----------
                "EXTERIOR-WALL" => {
                    // TODO: no asigna el muro a un espacio
                    bdldata.elements.push(BdlElementType::ExteriorWall(block));
                }
                "INTERIOR-WALL" => {
                    // TODO: no asigna el muro a un espacio
                    bdldata.elements.push(BdlElementType::InteriorWall(block));
                }
                "UNDERGROUND-WALL" => {
                    // TODO: no asigna el muro a un espacio
                    bdldata
                        .elements
                        .push(BdlElementType::UndergroundWall(block));
                }
                // "ROOF" => {
                //     // TODO: no asigna la cubierta a un espacio
                //     bdldata.elements.push(parse_roof(block)?);
                // }
                // "THERMAL-BRIDGE" => {
                //     // TODO: no asigna el elemento a un espacio
                //     bdldata.elements.push(parse_thermalbridge(block)?);
                // }
                // BUILDING-SHADE

                // Elementos transparentes de la envolvente -----
                // Ventana.
                // Puede definirse con GLASS-TYPE, WINDOW-LAYER o GAP
                // y puede pertenecer a un INTERIOR-WALL o EXTERIOR-WALL
                // (trasnmisividadJulio)
                "WINDOW" => {
                    // TODO: no asigna la ventana a un muro y a su vez este a un espacio
                    bdldata.elements.push(BdlElementType::Window(block));
                }
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

fn parse_material(block: BdlBlock) -> Result<Material, Error> {
    let mut el = Material::new(block.name, block.btype);
    el.attrs = block.attrs;
    el.group = el.attrs.get("GROUP")?.to_string();
    Ok(el)
}

fn parse_floor(block: BdlBlock) -> Result<Floor, Error> {
    let mut el = Floor::new(block.name);
    el.attrs = block.attrs;
    el.z = el.attrs.get_f32("Z").unwrap_or_default();
    Ok(el)
}

fn parse_space(block: BdlBlock) -> Result<Space, Error> {
    //TODO: falta el contexto para asignar el espacio a la planta
    let mut el = Space::new(block.name);
    el.attrs = block.attrs;
    el.polygon = el.attrs.get("POLYGON")?.to_string();
    el.height = el.attrs.get_f32("HEIGHT").ok();
    Ok(el)
}

// fn parse_exteriorwall(block: BdlBlock) -> Result<BdlElementType, Error> {
//     //TODO: falta el contexto para asignar el muro al espacio
//     let mut el = ExteriorWall::new(block.name);
//     el.attrs = block.attrs;
//     Ok(BdlElementType::ExteriorWall(el))
// }

fn parse_construction(block: BdlBlock) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = Construction::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::Construction(el))
}

fn parse_layers(block: BdlBlock) -> Result<BdlElementType, Error> {
    //TODO: falta el contexto para asignar el muro al espacio
    let mut el = Layers::new(block.name);
    el.attrs = block.attrs;
    Ok(BdlElementType::Layers(el))
}

fn parse_polygon(block: BdlBlock) -> Result<Polygon, Error> {
    let attrs = block.attrs;
    let mut polygon = Polygon::new(block.name);
    for (_k, v) in &attrs.0 {
        let vec = v.to_string().parse()?;
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
