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
    pub btype: BdlBlockType,
    /// Nombre del elemento o material
    /// En BDL en teoría no puede tener más de 32 caracteres (DOE-2.2)
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
        if stanza.len() == 1 {
            return Ok(BdlBlock {
                name: stanza[0].to_string(),
                btype: stanza[0].parse()?,
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
            btype: btype.parse()?,
            parent: None,
            attrs,
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum BdlBlockType {
    Floor,
    Zone,
    #[default]
    Space,
    UndergroundWall,
    UndergroundFloor,
    InteriorWall,
    ExteriorWall,
    Window,
    Roof,
    Door,
    ThermalBridge,
    Construction,
    Material,
    NameFrame,
    GlassType,
    Layers,
    Gap,
    BuildingShade,
    Polygon,
    RunPeriodPd,
    BuildParameters,
    DaySchedulePd,
    WeekSchedulePd,
    SchedulePd,
    ScheduleDay,
    ScheduleWeek,
    // Schedule,
    SystemConditions,
    SpaceConditions,
    Defectos,
    GeneralData,
    WorkSpace,
    AuxLine,
    ParteLider,
    DescriptionCondiction,
    Description,
    //
    System,
    Pump,
    CirculationLoop,
    Chiller,
    Boiler,
    DwHeater,
    HeatRejection,
    ElecGenerator,
    GroundLoopHx,
    // No implementados
    ElecMeter,
    FuelMeter,
    MasterMeters,
    Plane,
    LoadsReport,
    SystemsReport,
    PlantReport,
    ReportBlock,
    HourlyReport,
}

impl std::str::FromStr for BdlBlockType {
    type Err = Error;
    /// Convierte de cadena a bloque
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BdlBlockType::*;

        Ok(match s {
            "FLOOR" => Floor,
            "ZONE" => Zone,
            "SPACE" => Space,
            "UNDERGROUND-WALL" => UndergroundWall,
            "UNDERGROUND-FLOOR" => UndergroundFloor,
            "INTERIOR-WALL" => InteriorWall,
            "EXTERIOR-WALL" => ExteriorWall,
            "WINDOW" => Window,
            "ROOF" => Roof,
            "DOOR" => Door,
            "THERMAL-BRIDGE" => ThermalBridge,
            "CONSTRUCTION" => Construction,
            "MATERIAL" => Material,
            "NAME-FRAME" => NameFrame,
            "GLASS-TYPE" => GlassType,
            "LAYERS" => Layers,
            "GAP" => Gap,
            "BUILDING-SHADE" => BuildingShade,
            "POLYGON" => Polygon,
            "RUN-PERIOD-PD" => RunPeriodPd,
            "BUILD-PARAMETERS" => BuildParameters,
            "DAY-SCHEDULE-PD" => DaySchedulePd,
            "WEEK-SCHEDULE-PD" => WeekSchedulePd,
            "SCHEDULE-PD" => SchedulePd,
            "SCHEDULE-DAY" => ScheduleDay,
            "SCHEDULE-WEEK" => ScheduleWeek,
            // "SCHEDULE" => Schedule,
            "SYSTEM-CONDITIONS" => SystemConditions,
            "SPACE-CONDITIONS" => SpaceConditions,
            "DEFECTOS" => Defectos,
            "GENERAL-DATA" => GeneralData,
            "WORK-SPACE" => WorkSpace,
            "AUX-LINE" => AuxLine,
            "PARTELIDER" => ParteLider,
            "DESCRIPTION-CONDICTION" => DescriptionCondiction,
            "DESCRIPTION" => Description,
            "SYSTEM" => System,
            "PUMP" => Pump,
            "CIRCULATION-LOOP" => CirculationLoop,
            "CHILLER" => Chiller,
            "BOILER" => Boiler,
            "DW-HEATER" => DwHeater,
            "HEAT-REJECTION" => HeatRejection,
            "ELEC-GENERATOR" => ElecGenerator,
            "GROUND-LOOP-HX" => GroundLoopHx,
            // No implementados
            "ELEC-METER" => ElecMeter,
            "FUEL-METER" => FuelMeter,
            "MASTER-METERS" => MasterMeters,
            "PLANE" => Plane,
            "LOADS-REPORT" => LoadsReport,
            "SYSTEMS-REPORT" => SystemsReport,
            "PLANT-REPORT" => PlantReport,
            "REPORT-BLOCK" => ReportBlock,
            "HOURLY-REPORT" => HourlyReport,
            _ => bail!("Tipo de bloque desconocido {}", s),
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
    use BdlBlockType::*;

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
        let parent = match bdlblock.btype {
            // Las plantas no cuelgan de ningún elemento
            Floor => {
                currentfloor = bdlblock.name.clone();
                None
            }
            // Los espacios cuelgan de las plantas
            Space => {
                currentspace = bdlblock.name.clone();
                Some(currentfloor.clone())
            }
            // Los muros cuelgan de los espacios
            ExteriorWall | InteriorWall | Roof | UndergroundWall | UndergroundFloor => {
                currentwall = bdlblock.name.clone();
                Some(currentspace.clone())
            }
            // Las construcciones y ventanas cuelgan de los muros
            Construction | Window | Door => Some(currentwall.clone()),
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
