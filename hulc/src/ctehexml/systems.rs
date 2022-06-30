// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Interpretación de la información de sistemas del .ctehexml

// TODO: unificar generadores de calor y/o frío, salvo equipos ideales (o generar dos con ellos)
// TODO: ¿Separar acumuladores de generadores en equipos... llevándolo a otro atributo de los sistemas?
// TODO: Revisar otros tipos de equipos (PV, bombas, ventiladores, etc)
// TODO: Pensar otros componentes como circuitos y distribución
// TODO: Traer sistemas GT
// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105

use std::convert::TryFrom;

use anyhow::{format_err, Error};
use roxmltree::Node;

use super::systems_gt::GtSystems;
use super::xmlhelpers::{
    get_tag_as_f32, get_tag_as_f32_or_default, get_tag_as_str, get_tag_as_u32_or, get_tag_text,
};

/// Sistemas técnicos de climatización, ACS y ventilación
#[derive(Debug, Clone, PartialEq)]
pub enum System {
    /// Sistema sólo de ACS
    /// (sin calefacción, sin refrigeración, sin ventilación)
    Dhw {
        /// Nombre
        name: String,
        /// Lista de equipos
        equipment: Vec<Equipment>,
        /// Multiplicador
        multiplier: u32,
        /// Demanda de ACS
        dhw_demand: Vec<DhwDemand>,
    },

    /// Sistema multizona por agua (calefacción y ACS opcional)
    /// Sistemas mixtos y calefacción por agua
    /// (sin refrigeración, sin ventilación)
    MultizoneHotWater {
        /// Nombre
        name: String,
        /// Lista de equipos
        equipment: Vec<Equipment>,
        /// Multiplicador
        multiplier: u32,
        /// Temperatura de impulsión calefacción (ºC)
        heating_supply_temp: f32,
        /// Demanda de ACS
        dhw_demand: Option<Vec<DhwDemand>>,
        /// Lista de unidades terminales
        /// ZoneEquipment::HotWaterCoil
        zone_equipment: Vec<ZoneEquipment>,
    },

    /// Sistema unizona
    /// (sin ACS, sin ventilación)
    SingleZone {
        /// Nombre
        name: String,
        /// Lista de equipos
        equipment: Vec<Equipment>,
        /// Multiplicador
        multiplier: u32,
        /// Zona atendida
        /// En SingleZone siempre es Some<Zona>
        control_zone: Option<String>,
        // Caudal ventilación (m³/h)
        // Solo se usa en sistemas multizona por conductos y se pone a cero
        // Ponemos un assert en la importación
        // ventilation: f32,
    },

    /// Sistema multizona por conductos o expansión directa
    /// (sin ACS, recuperacción de calor / freecooling opcionales)
    MultizoneAir {
        /// Nombre
        name: String,
        /// Lista de equipos
        equipment: Vec<Equipment>,
        /// Multiplicador
        multiplier: u32,
        /// Zona de control
        /// En equipos de conductos tiene una zona pero no en DX
        control_zone: Option<String>,
        /// Caudal ventilación (m³/h)
        /// En sistemas con autónomos es 0
        outdoor_air_flow: f32,
        /// Caudal de aire retornado desde las zonas acondicionadas (m³/h)
        /// En sistemas con autónomos lo ponemos a 0
        return_air_flow: f32,
        /// Opciones
        /// Recuperación de calor y/o economizador de aire (freecooling)
        options: Vec<SystemOptions>,
        /// Lista de unidades terminales
        /// ZoneEquipment::AirDiffuser | DirectExpansion
        zone_equipment: Vec<ZoneEquipment>,
    },

    /// Sistema exclusivo de ventilación
    /// XXX: Podríamos tener un Option<Zone> y que None sea global del edificio?
    /// ¿Esto podría ser un equipo de zona y no un sistema? ¿en qué sistema iría?
    /// O podría ser un https://bigladdersoftware.com/epx/docs/9-6/input-output-reference/group-hvac-templates.html#hvactemplatesystemdedicatedoutdoorair
    /// y pensar sus opciones
    WholeBuildingDoas {
        /// Nombre
        name: String,
        /// Caudal requerido, m³/h
        /// TODO: Eliminar y dejar solo capacidad y consumo específico?
        required_air_flow: f32,
        /// Caudal máximo, m³/h
        capacity: f32,
        /// Consumo específico
        spf: f32,
        /// Opciones
        /// Recuperación de calor y/o economizador de aire (freecooling)
        options: Vec<SystemOptions>,
        /// Multiplicador
        multiplier: u32,
    },
}

/// Opciones en equipos / sistemas
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SystemOptions {
    /// Recuperador de calor
    HeatRecovery {
        /// Eficiencia del sistema de recuperación de calor, -
        efficiency: f32,
    },
    /// Freecooling
    /// No diferenciamos entre tipos de economizadores o si son de agua o aire, etc
    Economizer { control: EconomizerControl },
    // Humidification { control }
    // Dehumidification { control }
}

/// Tipo de control en economizador de aire
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EconomizerControl {
    /// Temperatura
    Temperature,
    /// Entalpía
    Enthalpy,
    /// Temperatura y entalpía
    TemperatureEnthalpy,
    /// Desconocido
    Unknown,
}

// Equipos ------------------------------------------------------------
//

/// Parámetros de rendimiento de calefacción
#[derive(Debug, Clone, PartialEq)]
pub struct HeatingParams {
    /// Vector energético para generar calor
    fuel: String,
    /// Capacidad calorífica máxima nominal (kW)
    /// En equipos ideales se recomienda 9_999_999_999.99 (1.0e11 - 0.01 > 1e10)
    capacity: f32,
    /// Rendimiento nominal (-)
    /// Relación entre la capacidad nominal y el consumo nominal
    efficiency: f32,
}

/// Parámetros de rendimiento de refrigeración
#[derive(Debug, Clone, PartialEq)]
pub struct CoolingParams {
    /// Vector energético para generar frío
    fuel: String,
    /// Capacidad total refrigeración nominal (kW)
    /// En equipos ideales se recomienda 9_999_999_999.99 (1.0e11 - 0.01 > 1e10)
    capacity: f32,
    /// Rendimiento nominal (kW)
    /// Relación entre la capacidad nominal y el consumo nominal
    efficiency: f32,
    /// Fracción de capacidad sensible de refrigeración respecto a la capacidad total (-)
    /// En unidades aire-agua o aire-refrigerante toma valor 1.0
    shr: f32,
}

/// Tipos de equipos
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EquipmentType {
    CalderaConvencional,
    CalderaElectrica,
    CalderaBajaTemperatura,
    CalderaCondensacion,
    CalderaBiomasa,
    CalderaAcsElectrica,
    CalderaAcsConvencional,
    CalefaccionElectrica,
    ExpansionDirectaAireAireSf,
    ExpansionDirectaAireAireBdc,
    ExpansionDirectaAireAguaBdc,
    ExpansionDirectaUnidadExterior,
    RendimientoConstante,
    AcumuladorAguaCaliente,
}

impl Default for EquipmentType {
    fn default() -> Self {
        Self::CalderaConvencional
    }
}

impl TryFrom<&str> for EquipmentType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Convencional" => Ok(Self::CalderaConvencional),
            "Electrica" => Ok(Self::CalderaElectrica),
            "BajaTemperatura" => Ok(Self::CalderaBajaTemperatura),
            "Condensacion" => Ok(Self::CalderaCondensacion),
            "Biomasa" => Ok(Self::CalderaBiomasa),
            "ACS-Electrica" => Ok(Self::CalderaAcsElectrica),
            "ACS-Convencional" => Ok(Self::CalderaAcsConvencional),
            "EQ_CalefaccionElectrica" => Ok(Self::CalefaccionElectrica),
            "EQ_ED_AireAire_SF" => Ok(Self::ExpansionDirectaAireAireSf),
            "EQ_ED_AireAire_BDC" => Ok(Self::ExpansionDirectaAireAireBdc),
            "EQ_ED_AireAgua_BDC" => Ok(Self::ExpansionDirectaAireAguaBdc),
            "EQ_ED_UnidadExterior" => Ok(Self::ExpansionDirectaUnidadExterior),
            "EQ_RendimientoCte" => Ok(Self::RendimientoConstante),
            "EQ_Acumulador_AC" => Ok(Self::AcumuladorAguaCaliente),
            _ => Err(format_err!("Tipo de equipo desconocido {}", value)),
        }
    }
}

/// Equipos primarios y de generación
#[derive(Debug, Clone, PartialEq)]
pub enum Equipment {
    /// Boiler, Hot water or electric baseboard heating system
    HeatingGenerator {
        /// Nombre
        name: String,
        /// Tipo
        /// Calderas: Convencional, Electrica, BajaTemperatura, Condensación, Biomasa, ACS-Electrica, ACS-Convencional
        /// + CalefaccionElectrica
        kind: EquipmentType,
        /// Parámetros de la generación de calor
        heating: Option<HeatingParams>,
        /// Multiplicador
        multiplier: u32,
        /// Curvas de rendimiento
        curves: Vec<(String, String)>,
    },

    /// Air to air, air to refrigerant or air to water heat pump or dx system
    HeatingAndCoolingGenerator {
        /// Nombre
        name: String,
        /// Tipo
        /// Aire-aire: ExpansionDirectaAireAireSf, ExpansionDirectaAireAireBdc,
        /// Aire-fluido: EQ_ED_AireAgua_BDC, EQ_ED_UnidadExterior
        kind: EquipmentType,
        /// Parámetros de la generación de calor
        heating: Option<HeatingParams>,
        /// Parámetros de la generación de frío
        cooling: Option<CoolingParams>,
        /// Caudal de aire de impulsión nominal (m³/h)
        /// Solo en equipos aire-aire
        supply_air_flow: Option<f32>,
        /// Multiplicador
        multiplier: u32,
        /// Curvas de rendimiento
        curves: Vec<(String, String)>,
    },
    /// Ideal constant efficiency heating and/or cooling system
    IdealGenerator {
        /// Nombre
        name: String,
        /// Tipo
        /// EQ_RendimientoCte
        kind: EquipmentType,
        /// Parámetros de la generación de calor
        heating: Option<HeatingParams>,
        /// Parámetros de la generación de frío
        cooling: Option<CoolingParams>,
        /// Multiplicador
        multiplier: u32,
    },

    HotWaterStorageTank {
        /// Nombre
        name: String,
        /// Tipo
        /// EQ_RendimientoCte
        kind: EquipmentType,
        /// Volumen, m³
        volume: f32,
        /// Coeficiente de pérdidas global del depósito, UA (W/ºC)
        ua: f32,
        /// Temperatura de consigna baja del depósito (ºC=80) (tConsignaBaja)
        temp_low: f32,
        /// Temperatura de consigna alta del depósito (ºC=60) (tConsignaAlta)
        temp_high: f32,
        /// Temperatura de entrada del agua de red (temperaturaEntrada = según climas)
        input_temp: f32,
        /// Temperatura del ambiente exterior (temperaturaAmbiente = 25ºC)
        space_temp: f32,
    },
}

/// Demanda de ACS
/// XXX: Esto es más bien un perfil/carga y no tanto un sistema
#[derive(Debug, Clone, PartialEq)]
pub struct DhwDemand {
    /// Nombre
    pub name: String,
    /// Demanda de ACS, l/dia
    pub demand: f32,
    /// Temperatura de utilización, ºC
    pub dhw_temp: f32,
    /// Temperatura del agua de red, ºC
    /// Se puede calcular
    /// Ver también E+ reference "Water Mains Temperatures"
    pub water_mains_temp: f32,
    /// Horario para consumo diario
    pub schedule: String,
}

/// Equipos terminales (de zona) con refrigerante, agua o aire
#[derive(Debug, Clone, PartialEq)]
pub enum ZoneEquipment {
    /// Direct Expansion Equipment (heating, cooling, ventilation)
    AirTerminalUnit {
        /// Nombre
        name: String,
        /// Zona abastecida
        zone: String,
        /// Caudal de impulsión (de aire) nominal de la unidad interior (m³/h) (vImpulsionNom),
        supply_flow_rated: f32,
        /// Caudal de aire exterior impulsado por la unidad interior (m³/h) (vVentilacion = 0 en vivienda),
        outdoor_air_flow: f32,
        /// Capacidad total máxima de refrigeración nominal (kW) (capTotRefNom),
        cooling_cap_rated: f32,
        /// Capacidad sensible máxima de refrigeración nominal (kW) (capSenRefNom),
        cooling_sh_cap_rated: f32,
        /// Capacidad calorífica máxima nominal (kW) (capCalNom),
        heating_cap_rated: f32,
        /// Multiplicador
        multiplier: u32,
    },
    /// Hot Water coil (only heating)
    HotWaterCoil {
        /// Nombre
        name: String,
        /// Zona abastecida
        zone: String,
        /// Capacidad calorífica máxima nominal (kW) (capCalNom),
        capacity_rated: f32,
        /// Multiplicador
        multiplier: u32,
    },
    /// Air diffuser (only ventilation)
    AirDiffuser {
        /// Nombre
        name: String,
        /// Zona abastecida
        zone: String,
        /// Caudal de impulsión (de aire) nominal de la unidad interior (m³/h) (vImpulsionNom),
        supply_flow_rated: f32,
        /// Multiplicador
        multiplier: u32,
    },
}

pub fn parse_systems(doc: &roxmltree::Document) -> (Vec<String>, Vec<System>) {
    // Definición de sistemas en VyP - Solución temporal sin descender en elementos
    let systems = doc
        .descendants()
        .find(|n| n.has_tag_name("Definicion_Sistema"));

    // Curvas de corrección de sistemas. Guardamos solo los nombres
    let factores_correccion_sistemas = match systems {
        Some(sis_node) => sis_node
            .descendants()
            .filter(|n| n.has_tag_name("CurvaComportamiento"))
            .filter_map(|n| n.attribute("nombre").map(str::to_string))
            .collect(),
        None => vec![],
    };

    // println!("Factores:\n{:#?}\n\n", factores_correccion_sistemas);

    // Definición de sistemas
    let mut sistemas = match systems {
        Some(sis_def_node) => sis_def_node
            .descendants()
            .find(|sis_node| sis_node.has_tag_name("Sistemas"))
            .map(|n| {
                n.children()
                    .filter(roxmltree::Node::is_element)
                    .map(build_system)
                    .collect()
            })
            .unwrap_or_default(),
        None => vec![],
    };

    // Sistema exclusivo de ventilación
    if let Some(doas) = build_doas(doc) {
        sistemas.push(doas)
    };

    // Sistemas GT
    let gt_systems_str = doc
        .descendants()
        .find(|n| n.has_tag_name("Definicion_Sistema_CALENER_GT"))
        .and_then(|e| e.text())
        .unwrap_or("")
        .trim();
    let gt_systems = GtSystems::new(&gt_systems_str).unwrap();

    // TODO: eliminar
    println!("Sistemas:\n{:#?}", gt_systems);

    // TODO: eliminar
    println!("Sistemas VyP:\n{:#?}", sistemas);

    // TODO: completar sistemas GT
    (factores_correccion_sistemas, sistemas)
}

/// Genera sistema a partir de su nodo XML
/// Podrían equivaler en E+ a:
/// https://bigladdersoftware.com/epx/docs/9-6/input-output-reference/group-hvac-templates.html#group----hvac-templates
/// - HVACTemplate:System:UnitarySystem (el unizona y multizona de aire)
/// - HVACTemplate:System:DualDuct (el multizona de conductos)
/// - ¿para ACS?
fn build_system(node: roxmltree::Node) -> System {
    let kind = node.tag_name().name().to_string();
    let name = get_tag_as_str(&node, "nombre_usuario").to_string();
    let multiplier = get_tag_as_u32_or(&node, "multiplicador", 1);

    // Equipos primarios o de almacenamiento
    let equipment = node
        .children()
        .find(|n| n.has_tag_name("equipos"))
        .map(|n| {
            n.children()
                .filter(Node::is_element)
                .map(build_equipment)
                .collect()
        })
        .unwrap_or_default();

    // Secundarios, en sistemas que no son unizona
    let zone_equipment = node
        .children()
        .find(|n| n.has_tag_name("unidades_terminales"))
        .map(|n| {
            n.children()
                .filter(|n| n.is_element())
                .map(build_zone_equipment)
                .collect()
        });
    // Solo en sistemas de ACS o mixtos
    let dhw_demand = node
        .children()
        .find(|n| n.has_tag_name("demandas"))
        .map(|n| {
            n.children()
                .filter(|n| n.has_tag_name("DemandaACS"))
                .map(build_dhwdemand)
                .collect()
        });

    match kind.as_str() {
        "SIS_Acs" => System::Dhw {
            name,
            multiplier,
            // ignoramos este dato ya que es redundante con el de la demanda
            // dhw_supply_temp: get_tag_as_f32(&node, "tImpulsion").unwrap(),
            dhw_demand: dhw_demand.unwrap(),
            equipment,
        },
        "SIS_Mixto" | "SIS_CalefaccionPorAgua" => {
            // Ignoramos el dato tImpulsionACS porque es redundante con el de la demanda de ACS
            // Temperatura de impulsión de calefacción
            let heating_supply_temp =
            // Es sistema mixto, con tImpulsionCal + tImpulsionACS
            // O es un sistema de calefacción por agua, con tImpulsion
            get_tag_as_f32(&node, "tImpulsionCal").or_else(|_| get_tag_as_f32(&node, "tImpulsion")).unwrap_or_default();

            System::MultizoneHotWater {
                name,
                multiplier,
                heating_supply_temp,
                dhw_demand,
                equipment,
                zone_equipment: zone_equipment.unwrap(),
            }
        }
        "SIS_ClimatizacionUnizona" => {
            assert!(get_tag_as_f32_or_default(&node, "vVentilacion") == 0.0);
            System::SingleZone {
                name,
                multiplier,
                control_zone: get_tag_text(&node, "zona").map(str::to_string),
                equipment,
            }
        }
        "SIS_Conductos" | "SIS_Conductos2" | "SIS_Autonomo" | "SIS_Autonomo2" => {
            // Conductos 2 y Autonomo2
            let has_heat_recovery = ["Sí tiene", "Si", "Sí"].contains(
                &get_tag_text(&node, "recuperacionCalor")
                    .map(|s| s.trim().trim_matches('"'))
                    .unwrap(),
            );
            // Solo conductos 2
            let heat_recovery_eff = get_tag_as_f32_or_default(&node, "eficienciaRecuperador");
            // Solo conductos 2
            // Control por temperatura | Control por entalpía
            let freecooling = get_tag_text(&node, "enfriamientoGratuito").map(|s| {
                match s.trim().trim_matches('"') {
                    "Control por temperatura" => EconomizerControl::Temperature,
                    "Control por entalpía" => EconomizerControl::Enthalpy,
                    _ => EconomizerControl::Unknown,
                }
            });
            // Solo conductos
            let oa_flow = get_tag_as_f32_or_default(&node, "vVentilacion");
            // Solo conductos
            let return_air_flow = get_tag_as_f32_or_default(&node, "vRetorno");
            let control_zone =
                get_tag_text(&node, "zonaControl").map(|s| s.trim().trim_matches('"').to_string());
            let mut options = vec![];
            if has_heat_recovery {
                options.push(SystemOptions::HeatRecovery {
                    efficiency: heat_recovery_eff,
                });
            }
            if let Some(control) = freecooling {
                options.push(SystemOptions::Economizer { control });
            }

            System::MultizoneAir {
                name,
                multiplier,
                control_zone,
                outdoor_air_flow: oa_flow,
                return_air_flow,
                options,
                equipment,
                zone_equipment: zone_equipment.unwrap(),
            }
        }
        _ => panic!("Sistema de tipo desconocido: {}", kind),
    }
}

/// Genera demanda de ACS a partir de su nodo XML
fn build_dhwdemand(node: roxmltree::Node) -> DhwDemand {
    // DEMANDAACS - "Demanda ACS" - ✔
    // - Nombre equipo (Nombre)
    //   Consumo total de ACS diario (l/dia) (conACSDiario),
    //   Temperatura de uso (ºC) (TUso=45),
    //   Temperatura (media) del agua de red (ºC) (Tred, según zona climática),
    //   Perfil diario (perfilDiario="1/24"|"Demanda_Hor"),
    //   // multiplicador (viene del sistema)
    let name = get_tag_as_str(&node, "nombre_usuario").to_string();
    let demand = get_tag_as_f32_or_default(&node, "conACSDiario");
    // XXX: Llevamos esto al sistema y lo dejamos fuera de aquí?
    let dhw_temp = get_tag_as_f32_or_default(&node, "TUso");
    let water_mains_temp = get_tag_as_f32_or_default(&node, "TRed");
    let schedule = get_tag_as_str(&node, "perfilDiario").to_string();

    DhwDemand {
        name,
        demand,
        dhw_temp,
        water_mains_temp,
        schedule,
    }
}

/// Secundarios - equipos de zona (unidades terminales) genera a partir de nodo XML
fn build_zone_equipment(node: roxmltree::Node) -> ZoneEquipment {
    let kind = node.tag_name().name();
    let name = get_tag_as_str(&node, "nombre_usuario").to_string();
    let zone = get_tag_as_str(&node, "zona").trim_matches('"').to_string();
    let multiplier = get_tag_as_u32_or(&node, "multiplicador", 1);

    match kind {
        "UT_AguaCaliente" => {
            // UT_AGUACALIENTE (UT_RADIADOR?) - "U.T. De Agua Caliente" (Calefacción) ✔
            //    - Nombre,
            //      Zona abastecida (Zona),
            //      Capacidad nominal o potencia máxima (kW) (capNom),
            //      // Ancho de banda del termostato (ºC) (fijo, dtTermostato = 50.0)
            let name = node.attribute("nombre").unwrap().to_string();
            ZoneEquipment::HotWaterCoil {
                name,
                zone,
                capacity_rated: get_tag_as_f32_or_default(&node, "capNom"),
                multiplier,
            }
        }
        "UT_ED_UnidadInterior" => {
            // UT_ED_UNIDADINTERIOR - "U.T. Unidad Interior" (Climatiza aire y ventila) - ✔
            //    - Nombre (nombre + nombre usuario),
            //      Zona abastecida (Zona),
            //      Capacidad total máxima de refrigeración nominal (kW) (capTotRefNom),
            //      Capacidad sensible máxima de refrigeración nominal (kW) (capSenRefNom),
            //      Capacidad calorífica máxima nominal (kW) (capCalNom),
            //      Caudal nominal de aire impulsado por la unidad interior (m³/h) (vImpulsionNom),
            //      Caudal de aire exterior impulsado por la unidad interior (m³/h) (vVentilacion = 0 en vivienda),
            //      // ¿Ancho de banda del termostato (ºC) (dtTermostato=1)?,
            //      // multiplicador (multiEspacio * multiPlanta)
            ZoneEquipment::AirTerminalUnit {
                name,
                zone,
                supply_flow_rated: get_tag_as_f32_or_default(&node, "vImpulsionNom"),
                outdoor_air_flow: get_tag_as_f32_or_default(&node, "vVentilacion"),
                cooling_cap_rated: get_tag_as_f32_or_default(&node, "capTotRefNom"),
                cooling_sh_cap_rated: get_tag_as_f32_or_default(&node, "capSenRefNom"),
                heating_cap_rated: get_tag_as_f32_or_default(&node, "capCalNom"),
                multiplier,
            }
        }
        "UT_ImpulsionAire" => {
            // UT_IMPULSIONAIRE - "U.T. De impulsión de aire" (Solo impulsa aire, sin vent ni clima) - ✔
            //    - Nombre (nombre + nombre usuario)
            //      Caudal nominal de aire impulsado por la unidad interior (m³/h) (vImpulsionNom),
            //      Zona abastecida (Zona),
            //      // ¿Proporcion ventilación (proporcionvVentilacion=0)?,
            //      // ¿Ancho de banda del termostato (ºC) (dtTermostato=0)?,
            //      // multiplicador
            ZoneEquipment::AirDiffuser {
                name,
                zone,
                supply_flow_rated: get_tag_as_f32_or_default(&node, "vImpulsionNom"),
                multiplier,
            }
        }
        _ => panic!("Equipo de zona desconocido: {}", kind),
    }
}

/// Primarios + acumulación - equipos de generación a partir del nodo XML
fn build_equipment(node: roxmltree::Node) -> Equipment {
    use EquipmentType::*;

    let name = get_tag_as_str(&node, "nombre_usuario").to_string();
    let kind = {
        let kind_str = node.tag_name().name();
        if kind_str == "EQ_Caldera" {
            name.split_once('-')
                .and_then(|s| s.1.rsplit_once('-').map(|s| s.0))
                .unwrap_or("")
        } else {
            kind_str
        }
        .try_into()
        .unwrap_or_else(|e| panic!("ERROR: {:?}", e))
    };

    let multiplier = get_tag_as_u32_or(&node, "multiplicador", 1);
    let fuel = get_tag_as_str(&node, "tipoEnergia")
        .trim_matches('"')
        .to_string();
    let curves = node
        .children()
        .filter(|n| {
            [
                "cap_T",
                "ren_T",
                "ren_FCP_Potencia",
                "ren_FCP_Tiempo",
                "con_FCP",
                "capTotRef_T",
                "capTotRef_FCP",
                "capSenRef_T",
                "conRef_T",
                "conRef_FCP",
                "conCal_T",
                "conCal_FCP",
            ]
            .contains(&n.tag_name().name())
        })
        .map(|n| {
            (
                n.tag_name().name().to_string(),
                n.text().unwrap().trim().trim_matches('"').to_string(),
            )
        })
        .collect();

    match kind {
        CalderaConvencional
        | CalderaElectrica
        | CalderaBajaTemperatura
        | CalderaCondensacion
        | CalderaBiomasa
        | CalderaAcsElectrica
        | CalderaAcsConvencional => {
            // Calderas: Convencional, Electrica, BajaTemperatura, Condensación,
            // Biomasa, ACS-Electrica, ACS-Convencional
            // <tipoCaldera> no se usa para el tipo y está vacío se puede deducir del nombre

            // EQ_CALDERA - "Caldera eléctrica o de combustible" - ✔
            //    - Nombre,
            //      Capacidad total nominal (kW) (capNom),
            //      Rendimiento Nominal (basado en PCI) (renNom),
            //      Tipo de energía (tipoEnergia),
            //      // multiplicador=1
            //    - Capacidad en función de la temperatura de impulsión (cap_T),
            //      Rendimiento nominal en función de la temperatura de impulsión (ren_T),
            //      Rendimiento en funcion de la carga parcial en términos de potencia (ren_FCP_Potencia),
            //      Rendimiento en funcion de la carga parcial en términos de tiempo (ren_FCP_Tiempo)
            let heating = Some(HeatingParams {
                fuel,
                capacity: get_tag_as_f32(&node, "capNom").unwrap_or_default(),
                efficiency: get_tag_as_f32(&node, "renNom").unwrap_or_default(),
            });

            Equipment::HeatingGenerator {
                name,
                kind,
                heating,
                multiplier,
                curves,
            }
        }
        CalefaccionElectrica => {
            // EQ_CALEFACCIONELECTRICA - "Calefacción eléctrica unizona" - "Electricidad" - ✔
            //    - Nombre (nombre + nombre usuario)
            //      Tipo de energía (tipoEnergia="Electricidad"),
            //      Capacidad nominal (kW) (capNom),
            //      Consumo nominal (kW) (conNom),
            //      // ¿Dif. temperatura del termostato (ºC) (dtTermostato = 1)?,
            //      // multiplicador
            //    - Consumo a carga parcial (con_FCP)
            let capacity = get_tag_as_f32(&node, "capNom").unwrap_or_default();
            let consumption = get_tag_as_f32(&node, "conNom").unwrap_or(capacity);
            let heating = if consumption > 0.0 {
                Some(HeatingParams {
                    fuel,
                    capacity,
                    efficiency: capacity / consumption,
                })
            } else {
                Some(HeatingParams {
                    fuel,
                    capacity,
                    efficiency: 0.0,
                })
            };
            Equipment::HeatingGenerator {
                name,
                kind,
                heating,
                multiplier,
                curves,
            }
        }
        ExpansionDirectaAireAireSf
        | ExpansionDirectaAireAireBdc
        | ExpansionDirectaAireAguaBdc
        | ExpansionDirectaUnidadExterior => {
            // Aire-aire: EQ_ED_AireAire_SF, EQ_ED_AireAire_BDC,
            // Aire-fluido: EQ_ED_AireAgua_BDC, EQ_ED_UnidadExterior

            // EQ_ED_AIREAIRE_SF - "Expansión directa aire-aire sólo frio" - "Electricidad" - ✔
            //    - Nombre,
            //      Tipo de energía (tipoEnergia="Electricidad"),
            //      Capacidad total refrigeración nomminal (kW) (capTotRefNom),
            //      Capacidad sensible refrigeración nominal (kW) (capSenRefNom),
            //      Consumo refrigeración nominal (kW) (conRefNom),
            //      Caudal de aire impulsión nominal (m³/h) (vImpulsionNom),
            //      Tipo energía (tipoEnergia="Electricidad"),
            //      // Dif. temperatura termostato (dtTermostato),
            //      // multiplicador=1
            //    - Capacidad total refrigeración en función de la tempratura (capTotRef_T),
            //      Capacidad total de refrigeración en función de la carga parcial (capTotRef_FCP),
            //      Carga sensible refrigeración en función de temperaturas (capSenRef_T),
            //      Consumo de refrigeración en función de la temperatura (conRef_T),
            //      Consumo de refrigeración en función de la carga parcial (conRef_FCP)
            // EQ_ED_AIREAIRE_BDC - "Expansión directa aire-aire bomba de calor" - "Electricidad" - ✔
            //    - Nombre (nombre + nombre usuario),
            //      Tipo energía (tipoEnergia ="Electricidad"),
            //      Capacidad total refrigeración nominales (kW) (capTotRefNom),
            //      Capacidad sensible refrigeración nominal (kW) (capSenRefNom),
            //      Consumo refrigeración nominal (kW) (conRefNom),
            //      Capacidad calefacción nominal (kW) (capCalNom),
            //      Consumo calefacción nominal (kW) (conCalNom),
            //      Caudal aire impulsión nominal (m³/h) (vImpulsionNom),
            //      // ¿Volumen de ventilación? (vVentilacion=0),
            //      // Dif. temperatura termostato (dtTermostato=1),
            //      // multiplicador
            //    - Capacidad total refrigeración en función temperaturas (capTotRef_T),
            //      Capacidad total de refrigeración en función de la carga parcial (capTotRef_FCP),
            //      Carga sensible refrigeración en función de temperaturas (capSenRef_T),
            //      Capacidad de calefacción en función de la temperatura (capCal_T),
            //      Consumo de refrigeración en función de la temperatura (conRef_T),
            //      Consumo de refrigeración en función de la carga parcial (conRef_FCP),
            //      Consumo calefacción en función de la temperatura (conCal_T),
            //      Consumo de calefacción en función de la carga parcial (conCal_FCP),
            // EQ_ED_AIREAGUA_BDC - "Expansión directa bomba de calor aire-agua" - "Electricidad" - ✔
            //    - Nombre (nombre + nombre usuario)
            //      Tipo energía (tipoEnergia ="Electricidad"),
            //      Capacidad nominal (kW) (capNom),
            //      Consumo nominal (kW) (conNom),
            //      // ¿Temperatura de impulsión nominal? (tImpulsionNom),
            //      // multiplicador = 1
            //    - Capacidad en función de la T (cap_T),
            //      Consumo en función de la T (con_T),
            //      Consumo en función de la carga parcial (con_FCP)
            // EQ_ED_UNIDADEXTERIOR - "Unidad exterior en expansión directa" - "Electricidad" - ✔
            //    - Nombre (nombre + nombre usuario),
            //      Tipo energía (tipoEnergia ="Electricidad"),
            //      Capacidad total refrigeración en condiciones nominales (kW) (capTotRefNom),
            //      // Capacidad sensible refrigeración nominal (capSenRefNom),
            //      Consumo refrigeración nominal (kW) (conRefNom),
            //      Capacidad calefacción nominal (kW) (capCalNom),
            //      Consumo calefacción nominal (kW) (conCalNom),
            //      // multiplicador
            //    - Capacidad total refrigeración en función temperaturas (capTotRef_T),
            //      Capacidad total de refrigeración en función de la carga parcial (capTotRef_FCP),
            //      Carga sensible refrigeración en función de temperaturas (capSenRef_T),
            //      Consumo de refrigeración en función de la temperatura (conRef_T),
            //      Consumo de refrigeración en función de la carga parcial (conRef_FCP),
            //      Capacidad de calefacción en función de la temperatura (capCal_T),
            //      Consumo calefacción en función de la temperatura (conCal_T),
            //      Consumo de calefacción en función de la carga parcial (conCal_FCP),

            let heating = if kind == ExpansionDirectaAireAireSf {
                None
            } else {
                let capacity = get_tag_as_f32(&node, "capCalNom").unwrap_or_default();
                let consumption = get_tag_as_f32(&node, "conCalNom").unwrap_or_default();
                let efficiency = if consumption > 0.0 {
                    capacity / consumption
                } else {
                    0.0
                };
                Some(HeatingParams {
                    fuel: "Electricidad".to_string(),
                    capacity,
                    efficiency,
                })
            };

            let cooling = {
                let capacity = get_tag_as_f32(&node, "capTotRefNom").unwrap_or_default();
                let capacity_sensible_heat =
                    get_tag_as_f32(&node, "capSenRefNom").unwrap_or_default();
                let consumption = get_tag_as_f32(&node, "conRefNom").unwrap_or_default();
                let efficiency = if consumption > 0.0 {
                    capacity / consumption
                } else {
                    0.0
                };
                let shr = if capacity > 0.0 {
                    capacity_sensible_heat / capacity
                } else {
                    0.0
                };
                Some(CoolingParams {
                    fuel: "Electricidad".to_string(),
                    capacity,
                    shr,
                    efficiency,
                })
            };
            let supply_air_flow = match kind {
                ExpansionDirectaAireAireSf | ExpansionDirectaAireAireBdc => {
                    Some(get_tag_as_f32(&node, "vImpulsionNom").unwrap_or_default())
                }
                _ => None,
            };

            Equipment::HeatingAndCoolingGenerator {
                name,
                kind,
                heating,
                cooling,
                supply_air_flow,
                multiplier,
                curves,
            }
        }
        RendimientoConstante => {
            // EQ_RENDIMIENTOCTE - "Rendimiento Constante" - ✔
            //    - Nombre (nombre + nombre usuario)
            //      Suministra Calefacción ? (daCal),
            //      Tipo energía de calefacción (tipoEnergiaCal, "Gasoleo")
            //      Rendimiento de calefacción (basado en PCI, para combustibles) (renCal, 0.9),
            //      Tipo energía de refrigeración (tipoEnergiaRef, "Electricidad")
            //      Suministra Refrigeración ? (daRef),
            //      Rendimiento de refrigeración (renRef, 2.52),
            //      // ¿Volumen ventilación? (vVentilacion),
            //      // multiplicador
            let da_cal = get_tag_as_str(&node, "daCal") == "true";
            let da_ref = get_tag_as_str(&node, "daRef") == "true";

            let heating = if da_cal {
                let fuel = get_tag_as_str(&node, "tipoEnergiaCal").to_string();
                let efficiency = get_tag_as_f32(&node, "renCal").unwrap_or_default();
                Some(HeatingParams {
                    fuel,
                    capacity: 1.0e10 - 0.01,
                    efficiency,
                })
            } else {
                None
            };

            let cooling = if da_ref {
                let fuel = get_tag_as_str(&node, "tipoEnergiaRef").to_string();
                let efficiency = get_tag_as_f32(&node, "renRef").unwrap_or_default();
                Some(CoolingParams {
                    fuel,
                    capacity: 1.0e11 - 0.01,
                    /// Suponemos fracción sensible = 1.0 (¿sería mejor 0.7?)
                    shr: 1.0,
                    efficiency,
                })
            } else {
                None
            };

            Equipment::IdealGenerator {
                name,
                kind,
                heating,
                cooling,
                multiplier,
            }
        }
        AcumuladorAguaCaliente => {
            // EQ_ACUMULADOR_AC - "Acumulador Agua Caliente" - ✔
            //    - Nombre (nombre + nombre usuario)
            //      Volumen del depósito en litros (volumen),
            //      Coeficiente de pérdidas global del depósito, UA (W/ºC) (UA),
            //      Temperatura de consigna baja del depósito (ºC=80) (tConsignaBaja),
            //      Temperatura de consigna alta del depósito (ºC=60) (tConsignaAlta),
            //      Temperatura de entrada del agua de red (temperaturaEntrada = según climas),
            //      Temperatura del ambiente exterior (temperaturaAmbiente = 25ºC)?
            //      multiplicador
            let volume = get_tag_as_f32(&node, "Volumen").unwrap_or_default();
            let ua = get_tag_as_f32(&node, "UA").unwrap_or_default();
            let temp_low = get_tag_as_f32(&node, "tConsignaBaja").unwrap_or_default();
            let temp_high = get_tag_as_f32(&node, "tConsignaAlta").unwrap_or_default();
            let input_temp = get_tag_as_f32(&node, "temperaturaEntrada").unwrap_or_default();
            let space_temp = get_tag_as_f32(&node, "temperaturaAmbiente").unwrap_or_default();

            Equipment::HotWaterStorageTank {
                name,
                kind,
                volume,
                ua,
                temp_low,
                temp_high,
                input_temp,
                space_temp,
            }
        }
    }
}

/// Genera el sistema exclusivo de ventilación, si existe
/// Podría ser algo similar a https://bigladdersoftware.com/epx/docs/9-6/input-output-reference/group-hvac-templates.html#hvactemplatesystemdedicatedoutdoorair
/// y pensar sus opciones.
fn build_doas(doc: &roxmltree::Document) -> Option<System> {
    // Equipo exclusivo ventilación
    //    - en <DatosGenerales><datosVentilacion>1;1882.800;1858.73;0;0.00;0.00;0.000;0.00;1;4000;3200;8000;4800;12000;5600;16000;6100;0;0;0;0;0;0.0000;0.00;1882.800;0.00;0.00;0.0000</datosVentilacion>
    //
    doc.descendants()
        .find(|n| n.has_tag_name("datosVentilacion"))
        .and_then(|n| {
            n.text()
                .unwrap_or_default()
                .split(';')
                .map(str::parse::<f32>)
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .and_then(|data| {
            // Si no están todos los datos o el primero es 0 (no hay ventilador) no devolvemos nada
            if data.len() != 28 || data[0] < 0.1 {
                return None;
            };
            let required_air_flow = data[1];

            let capacity = if data[8] < 0.1 {
                // Si data[8] es 0 se usa una curva característica
                // Cogemos el máximo caudal de la curva o el caudal de cálculo
                let max = [data[9], data[11], data[13], data[15], data[17], data[19]]
                    .iter()
                    .fold(data[1], |a, b| a.max(*b));
                max
            } else {
                data[4]
            };

            // SPF W/(m³/h)
            let spf = if required_air_flow < f32::EPSILON {
                0.0
            } else {
                // Potencia consumida W / Caudal de cálculo m³/h
                (10000.0 * data[2] / required_air_flow).round() / 10000.0
            };

            // sin recuperación = 0, con recuperación y bypass térmico = 10, con recuperación sin bypass térmico = 11
            let opts = data[21];
            let has_heat_recovery = opts > 9.0;
            // options = 10 es recuperación con bypass
            let economizer = has_heat_recovery && opts < 10.5;
            let heat_recovery_efficiency = if economizer {
                (10000.0 * data[27]).round() / 10000.0
            } else {
                0.0
            };

            let mut options = vec![];
            if has_heat_recovery {
                options.push(SystemOptions::HeatRecovery {
                    efficiency: heat_recovery_efficiency,
                });
            }
            if economizer {
                options.push(SystemOptions::Economizer {
                    control: EconomizerControl::Unknown,
                });
            }

            let fan = System::WholeBuildingDoas {
                name: "Sistema exclusivo de ventilación".to_string(),
                required_air_flow,
                capacity,
                spf,
                options,
                multiplier: 1,
            };
            Some(fan)
        })
}
