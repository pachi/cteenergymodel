// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Tipos de sistemas del .ctehexml - GT

// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105
// https://doe2.com/Download/DOE-23/DOE23Vol2-Dictionary_50h.pdf

use std::str::FromStr;

use anyhow::{bail, Error};

pub use crate::bdl::BdlBlock;

/// Tipo de bomba hidráulica
/// (CAP-CTRL)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum PumpKind {
    /// Bomba de caudal constante
    #[default]
    CaudalConstante,
    /// Bomba de dos velocidades
    DosVelocidades,
    /// Bomba de caudal variable
    CaudalVariable,
}

impl FromStr for PumpKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONE_SPEED-PUMP" => Ok(Self::CaudalConstante),
            "TWO-SPEED-PUMP" => Ok(Self::DosVelocidades),
            "VAR-SPEED-PUMP" => Ok(Self::CaudalVariable),
            _ => bail!("Tipo de bomba hidráulica desconocido"),
        }
    }
}

/// Bomba de GT. En circuitos o equipos (como enfriadoras)
/// (PUMP)
/// Potencia de la bomba: P = rho ·  g · Q · H / n
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtPump {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de control
    /// (CAP-CTRL)
    pub kind: PumpKind,
    /// Caudal Q, l/h
    /// (C-C-FLOW)
    pub flow: f32,
    /// Altura manométrica H, m
    /// (HEAD)
    pub head: f32,
    /// Rendimiento total de la bomba n, -
    /// Producto del rendimiento hidráulico, mecánico y eléctrico
    pub eff: f32,
    // Otros parámetros menos habituales y curvas de comportamiento
}

impl From<BdlBlock> for GtPump {
    fn from(block: BdlBlock) -> Self {
        let eff = block.attrs.get_f32("MECH-EFF").unwrap_or(0.77)
            * block.attrs.get_f32("MOTOR-EFF").unwrap_or(0.80);
        Self {
            name: block.name.clone(),
            kind: block
                .attrs
                .get_str("CAP-CTRL")
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
            flow: block.attrs.get_f32("C-C-FLOW").unwrap_or_default(),
            head: block.attrs.get_f32("HEAD").unwrap_or_default(),
            eff,
        }
    }
}

/// Tipo de circuito hidráulico
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CirculationLoopKind {
    /// Dos tubos (agua fría o agua caliente pero no simultáneamente)
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// Temperatura de cambio estacional (SNAP-T) o Disponibilidad en función de horario ()
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para calor (defecto 80ºC)
    /// caudal máximo (C-C-PROCESS-FLOW)
    /// consigna para frío (defecto 7ºC)
    /// Control AF y AC
    #[default]
    Pipe2,
    /// ACS
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 35ºC)
    /// consigna para calor (defecto 50ºC)
    /// caudal máximo (C-C-PROCESS-FLOW)
    /// temperatura de agua de red (DHW-INLET-T)
    /// horario ACS (PROCESS-SCH)
    Dhw,
    /// Agua fría
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (defecto 7ºC)
    Chw,
    /// Agua bruta (intercambio con el terreno)
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (defecto 30ºC) y calor (defecto 20ºC)
    LakeWell,
    /// Agua caliente
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 20ºC)
    /// consigna para calor (defecto 80ºC)
    Hw,
    /// Bomba de calor circuito cerrado
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (defecto 30ºC) y calor (defecto 20ºC)
    Whlp,
    /// Circuito de agua de condensación
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (defecto 30ºC)
    Cw,
}

impl FromStr for CirculationLoopKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PIPE2" => Ok(Self::Pipe2),
            "DHW" => Ok(Self::Dhw),
            "CHW" => Ok(Self::Chw),
            "LAKE / WELL" => Ok(Self::LakeWell),
            "HW" => Ok(Self::Hw),
            "WLHP" => Ok(Self::Whlp),
            "CW" => Ok(Self::Cw),
            _ => bail!("Tipo de circuito hidráulico desconocido"),
        }
    }
}

/// Circuitos hidráulicos de GT
/// (CIRCULATION-LOOP)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtCirculationLoop {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de circuito
    /// (TYPE)
    pub kind: CirculationLoopKind,
    /// Bomba asociada a este circuito
    /// (LOOP-PUMP)
    pub loop_pump: Option<String>,
    // Subtipo: (SUBTYPE) primario, secundario (se conecta a uno primario)
    // Otros datos: salto temperatura de diseño, caudal recirculado, circuito primario, porcentaje caudal primario

    // --- Cal / ACS
    /// Consigna de calefacción/ACS, ºC
    /// (HEAT-SETPT-T)
    /// Por defecto:
    /// - 80ºC en PIPE2, HW
    /// - 50ºC en DHW
    /// - 20ºC en WHLP, LAKEWELL
    pub heat_setpoint_temp: Option<f32>,
    /// Caudal máximo de calefacción/ACS (a la temperatura de salida), l/h
    /// (C-C-PROCESS-FLOW)
    pub dhw_flow: Option<f32>,
    /// Temperatura del agua de red en Cal/ACS, ºC
    /// (DHW-INLET-T)
    pub dhw_inlet_temp: Option<f32>,
    // Horario de calefacción/ACS (HEATING-SCHEDULE)
    // pub heating_schedule: String,

    // --- Refrigeración/Condensación
    /// Consigna de refrigeración, ºC (por defecto 7)
    /// (COOL-SETPT-T)
    /// Por defecto:
    /// - 7ºC en PIPE2, CHW
    /// - 30ºC en LAKEWELL, WHLP, CW
    pub cool_setpoint_temp: Option<f32>,
    // Horario de ref (COOLING-SCHEDULE)
    // pub cooling_schedule: String,
    // Tipo de control T agua
    // (C-C-LOOP-OPER-AF) ==4 (Disponibilidad en función de horario), ==3 (Cambio estacional por temperatura + SNAP-T, Temperatura cambio estacional)
}

impl From<BdlBlock> for GtCirculationLoop {
    fn from(block: BdlBlock) -> Self {
        use CirculationLoopKind::*;

        let kind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let heat_setpoint_temp = block.attrs.get_f32("HEAT-SETPT-T").ok().or(match kind {
            Pipe2 | Hw => Some(80.0),
            Dhw => Some(50.0),
            Whlp | LakeWell => Some(20.0),
            _ => None,
        });
        let cool_setpoint_temp = block.attrs.get_f32("COOL-SETPT-T").ok().or(match kind {
            Pipe2 | Chw => Some(7.0),
            Dhw | Whlp | LakeWell => Some(30.0),
            _ => None,
        });
        Self {
            name: block.name.clone(),
            kind,
            dhw_flow: block.attrs.get_f32("C-C-FLOW").ok(),
            dhw_inlet_temp: block.attrs.get_f32("DHW-INLET-T").ok(),
            loop_pump: block.attrs.get_str("LOOP-PUMP").ok(),
            heat_setpoint_temp,
            cool_setpoint_temp,
        }
    }
}

/// Tipo de enfriadora
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ChillerKind {
    /// Compresor eléctrico
    #[default]
    ElecHermRec,
    /// Compresor eléctrico con recuperación de calor
    ElecHeatRec,
    /// Absorción de simple etapa
    Absor1,
    /// Absorción de doble etapa
    Absor2,
    /// Absorción por llama directa
    GasAbsor,
    /// Motor de combustión interna
    Engine,
    /// Bomba de calor 2T
    HeatPump,
    /// Bomba de calor 4T
    LoopToLoopHeatPump,
}

impl FromStr for ChillerKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChillerKind::*;

        match s {
            "ELEC-HERM-REC" => Ok(ElecHermRec),
            "ELEC-HTREC" => Ok(ElecHeatRec),
            "ABSOR-1" => Ok(Absor1),
            "ABSOR-2" => Ok(Absor2),
            "GAS-ABSOR" => Ok(GasAbsor),
            "ENGINE" => Ok(Engine),
            "HEAT-PUMP" => Ok(HeatPump),
            "LOOP-TO-LOOP-HP" => Ok(LoopToLoopHeatPump),
            // No usados en GT
            // ELEC-OPEN-CENT y WATER-ECONOMIZER
            _ => bail!("Tipo de enfriadora desconocido"),
        }
    }
}

/// Tipo de condensación
/// (CONDENSER-TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CondenserKind {
    /// Condensación por aire
    #[default]
    Air,
    /// Condensación por agua
    Water,
    /// Condensación remota por aire
    RemoteAir,
    /// Condensación remota por evaporación
    RemoteEvap,
}

impl FromStr for CondenserKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CondenserKind::*;

        match s {
            "AIR-COOLED" => Ok(Air),
            "WATER-COOLED" => Ok(Water),
            // No usados en GT?
            "REMOTE-AIR-COOLED" => Ok(RemoteAir),
            "REMOTE-EVAP-COOLED" => Ok(RemoteEvap),
            _ => bail!("Tipo de condensadora desconocido"),
        }
    }
}

/// Planta enfriadora de GT
/// Puede incluir plantas enfriadoras reversibles tipo BdC y de otros tipos
/// (CHILLER)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtChiller {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    pub kind: ChillerKind,

    /// Tipo de condensación para BdC y compresión eléctrica
    /// (CONDENSER-TYPE)???
    pub condenser_kind: CondenserKind,

    /// Capacidad nominal de refrigeración, kW
    /// (C-C-CAPACITY)
    pub cool_capacity: f32,
    /// Rendimiento (eléctrico) en refrigeración, ERR, -
    /// (C-NUM-OF-UNITS)
    pub eer: f32,
    /// Rendimiento térmico en refrigeración, EER, -
    /// (C-IPLV)
    /// En equipos que consumen electricidad y otro combustible, como absorción con llama directa
    pub eer_th: Option<f32>,
    /// Capacidad nominal de calefacción en enfriadoras reversibles tipo BdC
    /// (C-DESIGN-KW), kW
    pub heat_capacity: Option<f32>,
    /// Rendimiento en calefacción para enfriadoras reversibles, COP, -
    /// (C-COP)
    pub cop: Option<f32>,
    /// Combustible (adicional a electricidad)
    /// (FUEL-METER)
    pub carrier: Option<String>,

    // -- Conexiones a circuitos --
    // Circuito agua fría ---
    /// Circuito de agua enfriada que alimenta
    /// (CHW-LOOP)
    pub chw_loop: String,

    // Circuito condensación ---
    /// Circuito de agua condensada (en casos que lo usen, absorción y motor combustión)
    /// (CW-LOOP)
    pub cw_loop: Option<String>,

    // Circuito agua caliente ---
    /// Circuito de agua caliente (en casos que lo usen)
    /// (HW-LOOP)
    pub hw_loop: Option<String>,

    // Circuito recuperación calor ---
    /// Circuito recuperación calor (en casos que lo usen)
    /// (HTREC-LOOP)
    pub htrec_loop: Option<String>,
    // Salto de temperatura, ºC
    // (CHW-DT)
    // pub chw_dt: f32,
    // -- Curvas comportamiento --

    // TODO: ver combustible para casos no eléctricos
}

impl From<BdlBlock> for GtChiller {
    fn from(block: BdlBlock) -> Self {
        let kind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let condenser_kind = block
            .attrs
            .get_str("CONDENSER-TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        Self {
            name: block.name.clone(),
            kind,
            condenser_kind,
            cool_capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            eer: block.attrs.get_f32("C-NUM-OF-UNITS").unwrap_or_default(),
            eer_th: block.attrs.get_f32("C-IPLV").ok(),
            heat_capacity: block.attrs.get_f32("C-DESIGN-KW").ok(),
            carrier: block.attrs.get_str("FUEL-METER").ok(),
            cop: block.attrs.get_f32("C-COP").ok(),
            chw_loop: block.attrs.get_str("CHW-LOOP").unwrap_or_default(),
            cw_loop: block.attrs.get_str("CW-LOOP").ok(),
            hw_loop: block.attrs.get_str("HW-LOOP").ok(),
            htrec_loop: block.attrs.get_str("HTREC-LOOP").ok(),
        }
    }
}

/// Tipo de caldera
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum BoilerKind {
    /// Caldera convencional
    #[default]
    Conventional,
    /// Caldera de baja temperatura
    LowTemp,
    /// Caldera de condensación
    Condensing,
    /// Biomasa
    Biomass,
    /// Caldera eléctrica
    Electric,
}

/// Caldera de GT
/// (BOILER)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtBoiler {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - HW-BOILER-W/DRAFT: caldera de combustible (con ventilador)
    /// - ELEC-HW-BOILER: caldera eléctrica
    /// Subtipo
    /// (C-C-SUBTYPE)
    /// 1 - Convencional
    /// 2 - Baja temperatura
    /// 3 - Condensación
    /// 4 - Biomasa
    /// 5 - Eléctrica
    pub kind: BoilerKind,

    /// Potencia nominal (C-C-CAPACITY), kW
    pub capacity: f32,
    /// Rendimiento, -
    /// En calderas de combustible, Rendimiento térmico, ratio
    /// (C-THERM-EFF-MAX || 0.85)
    /// En calderas eléctricas, Eficiencia eléctrica, nu
    /// (C-AFUE || 0.98)
    pub eff: f32,
    /// Tipo de combustible
    /// - Gas Natural*
    /// - Gasóleo
    /// - ...
    pub carrier: String,

    // -- Conexiones a circuitos --
    // Circuito agua caliente ---
    /// Circuito de agua caliente que alimenta
    /// (HW-LOOP)
    pub hw_loop: String,
    /// Bomba agua caliente
    /// (HW-PUMP)
    pub hw_pump: Option<String>,
    // Temperatura de consigna, ºC (45ºC)
    // La toma del circuito?
    // Salto de temperatura, ºC
    // (HW-DT || 5ºC)
    // pub hw_dt: f32,
    // Consumo nominal / consumo eléctrico, ratio (400)
    // (C-C-KN)
    // pub kn: f32,
    // -- Curvas comportamiento --
}

impl From<BdlBlock> for GtBoiler {
    fn from(block: BdlBlock) -> Self {
        use BoilerKind::*;

        let kind = match block.attrs.get_str("TYPE").unwrap_or_default().as_str() {
            "ELEC-HW-BOILER" => Electric,
            _ => match block
                .attrs
                .get_str("C-C-SUBTYPE")
                .unwrap_or_default()
                .as_str()
            {
                "2" => LowTemp,
                "3" => Condensing,
                "4" => Biomass,
                "5" => Electric,
                // "1" => Conventional,
                _ => Default::default(),
            },
        };

        let carrier = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
            Electric => "Electricidad".into(),
            Biomass => "Biomasa".into(),
            _ => "Gas Natural".into(),
        });

        let eff = match kind {
            Electric => block.attrs.get_f32("C-AFUE").unwrap_or(0.98),
            // TODO: ver si los subtipos tienen rendimientos por defecto diferentes
            _ => block.attrs.get_f32("C-THERM-EFF-MAX").unwrap_or(0.85),
        };

        Self {
            name: block.name.clone(),
            kind,
            capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            eff,
            carrier,
            hw_loop: block.attrs.get_str("HW-LOOP").unwrap_or_default(),
            hw_pump: block.attrs.get_str("HW-PUMP").ok(),
        }
    }
}

/// Tipo de calentador de ACS
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum DwHeaterKind {
    /// Caldera de combustible
    #[default]
    Conventional,
    /// Caldera eléctrica
    Electric,
    /// Bomba de calor
    HeatPump,
}

impl FromStr for DwHeaterKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DwHeaterKind::*;

        match s {
            "HEAT-PUMP" => Ok(HeatPump),
            "GAS" => Ok(Conventional),
            "ELEC" => Ok(Electric),
            _ => bail!("Tipo de calentador de ACS desconocido"),
        }
    }
}

/// Hot Water Storage
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtHotWaterStorageTank {
    /// Nombre
    pub name: String,
    /// Volumen, m³
    pub volume: f32,
    /// Coeficiente de pérdidas global del depósito, UA (W/ºC)
    pub ua: f32,
}

/// Calderas de ACS de GT
/// (DW-HEATER)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtDwHeater {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - HEAT-PUMP: bomba de calor
    /// - GAS: caldera de combustible
    /// - ELEC: caldera eléctrica
    pub kind: DwHeaterKind,

    /// Capacidad nominal, kW
    /// C-C-CAPACITY
    pub capacity: f32,
    /// Rendimiento
    /// Rendimiento eléctrico, COP
    /// en BdC y calderas eléctricas
    /// (C-STBY-LOSS-FRAC)
    /// Eficiencia térmica, nu
    /// En calderas de combustible
    /// (C-ENERGY-FACTOR)
    pub eff: f32,
    /// Combustible
    /// - GasNatural
    /// - Gasóleo
    /// - ...
    /// (FUEL-METER)
    pub carrier: String,

    /// Circuito de ACS que alimenta
    /// (DHW-LOOP)
    pub dhw_loop: String,
    /// Bomba agua caliente sanitaria
    /// (DHW-PUMP)
    pub dhw_pump: Option<String>,

    /// Depósito de agua caliente
    /// Presencia de depósito
    /// (C-CATEGORY)
    /// 0 - nada*
    /// 1 - con
    /// 2 - sin
    // Solo con depósito si es == 1
    /// Volumen del depósito de acumulación, l
    /// (TANK-VOLUME)
    /// Pérdidas térmicas del depósito de acumulación, W/K
    /// (TANK-UA)
    pub dhw_tank: Option<GtHotWaterStorageTank>,
    // TODO: HP-SUPP-CAPACITY=0 - capacidad del sistema de respaldo (resistencia elec.) en BdC, por defecto es igual que capacity
    // C-C-SUBTYPE -> 2 = Tiene panel solar,
    // C-C-AREA-PS      = 10 superficie de paneles solares, m²
    // C-C-PORC-PS      = 30 % de demanda cubierta, %
}

impl From<BdlBlock> for GtDwHeater {
    fn from(block: BdlBlock) -> Self {
        use DwHeaterKind::*;

        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let capacity = block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default();

        let carrier = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
            Electric | HeatPump => "Electricidad".into(),
            _ => "Gas Natural".into(),
        });

        let eff = match kind {
            Electric => block.attrs.get_f32("C-STBY-LOSS-FRAC").unwrap_or(1.00),
            HeatPump => block.attrs.get_f32("C-STBY-LOSS-FRAC").unwrap_or(2.70),
            _ => block.attrs.get_f32("C-ENERGY-FACTOR").unwrap_or(0.80),
        };

        let has_tank = &block.attrs.get_str("C-CATEGORY").unwrap_or_default() == "1";
        let dhw_tank = if has_tank {
            let volume = block
                .attrs
                .get_f32("TANK-VOLUME")
                .unwrap_or(65.0 * capacity);
            Some(GtHotWaterStorageTank {
                name: format!("Deposito - {}", name),
                volume,
                ua: block.attrs.get_f32("TANK-UA").unwrap_or(0.042 * volume),
            })
        } else {
            None
        };

        Self {
            name,
            kind,
            capacity,
            eff,
            carrier,
            dhw_loop: block.attrs.get_str("DHW-LOOP").unwrap_or_default(),
            dhw_pump: block.attrs.get_str("DHW-PUMP").ok(),
            dhw_tank,
        }
    }
}

/// Torre de refrigeración de GT
/// (HEAT-REJECTION)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtHeatRejection {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// - OPEN-TWR: Torre de refrigeración circuito abierto
    /// - OPEN-TWR&HX:
    /// - FLUID-COOLER: Torre de refrigeración circuito cerrado
    /// - DRYCOOLER:
    pub kind: String,
    /// Capacidad nominal de refrigeración en condiciones CTI, kW
    /// (C-C-CAPACITY)
    pub capacity: f32,
    /// Consumo de ventiladores por celda en condiciones nominales, kW
    /// (FAN-KW/CELL)
    pub fan_kw: f32,
    /// Número de celdas
    /// (NUMBER-OF-CELLS)
    pub number_of_cells: f32,
    /// Circuito de condensados
    /// (CW-LOOP)
    pub cw_loop: String,
    /// Potencia de bombas de recirculación, kW
    /// Solo en circuito cerrado
    /// (SPRAY-KW/CELL)
    pub spray_kw_cell: f32,
}

/// Equipos cogeneración de GT
/// (ELEC-GENERATOR)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtElectricGenerator {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - ENGINE-GENERATOR: Motor de combustión
    /// - GAS-TURBINE-GENERATOR (no usado en GT)
    /// - STEAM-TURBINE-GENERATOR (no usado en GT)
    /// - PV-ARRAY (no usado en GT)
    pub kind: String,
    /// Potencia nominal, kW
    /// (CAPACITY)
    pub capacity: f32,
    /// Rendimiento nominal, ratio
    /// Consumo de combustible para la producción eléctrica, ambas en iguales unidades
    /// Usa poder calorífico superior (las tablas suelen venir en PCI)
    /// (C-C-HIR) - Heat input ratio
    pub hir: f32,
    /// Combustible usado
    /// (FUEL-METER)
    pub carrier: String,
}

/// Intercambiado de calor con el terreno (alimentación de agua bruta) de GT
/// (GROUND-LOOP-HX)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GtGroundLoopHx {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - LAKE/WELL: temperatura del circuito dada por horario
    /// - otros tipos no usados en GT para suelo horizontal o vert., etc
    pub kind: String,
    /// Circuito de agua bruta
    /// (CIRCULATION-LOOP)
    pub circulation_loop: String,
    /// Horario de temperatura del circuito
    /// (LOOP-TEMP-SCH)
    pub loop_temp_sch: String,
}

/// Sistema (subsistema secundario) de GT
/// (SYSTEM)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtSystem {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - PSZ: Autónomo caudal constante
    /// - PMZS: Solo ventilación
    /// - PVAVS: Autónomo caudal variable
    /// - PVVT: Autónomo caudal variable temperatura variable
    /// - PTAC: Autónomo mediante unidades terminales
    /// - HP: Autónomo BdC en circuito cerrado
    /// - SZRH: Todo aire caudal constante unizona
    /// - VAVS: Todo aire caudal variable
    /// - RHFS: Todo aire caudal constante
    /// - DDS: Todo aire doble conducto
    /// - FC: Fancoil (ventiloconvector)
    /// - UVT: Termoventilación
    /// - UHT: Solo calefacción por efecto Joule
    /// - EVAP-COOL: Enfriamiento evaporativo
    /// - CBVAV: Climatizadora de aire primario
    /// - FPW: Solo calefacción por agua
    pub kind: String,
    /// Zona de control
    /// (CONTROL-ZONE)
    pub control_zone: String,

    // -- Ventiladores --
    // Ventilador de impulsión ---
    /// Horario de funcionamiento de los ventiladores de impulsión
    /// (FAN-SCHEDULE)
    pub fan_schedule: String,
    /// Caudal del ventilador de impulsión, m³/h
    /// (C-C-SUPPLY-FLOW)
    pub supply_flow: f32,
    /// Potencia del ventilador de impulsión, kW
    /// (??)
    pub supply_kw: f32,
    // Tipo de control
    // ??
    // Ventilador de retorno ---
    // Caudal
    // Potencia
    // Caja de caudal variable ---
    // Caudal mínimo

    // -- Refrigeración --
    // Baterías ---
    /// Circuito de agua enfriada que alimenta el sistema
    /// Es el circuito por defecto para zonas salvo que se indique
    /// (CHW-LOOP)
    pub chw_loop: Option<String>,
    /// Circuito de agua enfriada que alimenta las unidades de zona
    /// (??)
    pub zone_chw_loop: Option<String>,
    /// Potencia total batería zonal, kW
    /// (C-C-COOL-CAP)
    pub cool_cap: f32,
    /// Potencia sensible batería zonal, kW
    /// (C-C-COOL-SH-CAP)
    pub cool_sh_cap: f32,
    // Salto térmico, tipo de válvula...

    // Autónomos ---

    // Enfriamiento evaporativo ---

    // Economizador agua ---

    // -- Calefacción --
    // Fuentes de calor ---
    // Fuente de calor a nivel de zona
    // (C-C-ZONE-H-SOUR)
    // 0=n/a, 1=1=electrica, 2=agua caliente, 3= circuito ACS, 4=Recuperaci ón BdC gas, 5=Ninguna
    // Fuente de calor a nivel de sistema
    // (C-C-HEAT-SOURCE)
    // 0 = n/a, 1=electrica, 2=agua caliente, 3= circuito ACS, 4=BdC elec, 5=BdC gas, 6=generador aire, 7=ninguna
    // Baterías ---
    /// Circuito de agua caliente que alimenta el sistema
    /// Es el circuito por defecto para zonas salvo que se indique
    pub hw_loop: Option<String>,
    /// Circuito de agua caliente que alimenta las unidades de zona
    /// (ZONE-HW-LOOP)
    pub zone_hw_loop: Option<String>,
    /// Potencia total batería zonal, kW
    /// (C-C-HEAT-CAP)
    pub heat_cap: f32,
    // Salto térmico, tipo de válvula...
    // Precalentamiento/Calef. auxiliar ---
    // Autónomos ---
    // Bomba de calor ---
    /// Circuito de condensación
    /// (CW-LOOP)
    pub cw_loop: String,
    // -- Control --
    // Temperatura de impulsión min, max
    // Horarios de disponibilidad
    // Control UTA

    // -- Técnicas de recuperación --
    // Enfriamiento gratuito + control
    // Recuperación de calor + efectividad

    // -- Curvas de comportamiento
    // ...
}

/// Zona de GT
/// (ZONE)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtZone {
    /// Nombre / descripción
    pub name: String,
    /// Tipo
    /// (TYPE)
    /// - CONDITIONED: acondicionada
    /// - PLENUM: plenum
    /// - UNCONDITIONED: no acondicionada
    pub kind: String,
    /// Espacio asociado
    /// (SPACE)
    pub space: String,

    // -- Aire impulsión --
    /// Caudal de impulsión de la zona, m³/h
    /// (C-C-ASSIG-FLOW)
    pub assigned_flow: f32,

    // -- Ventilador de extracción
    // Caudal
    // Potencia

    // -- Aire exterior --
    // Método para asignar caudal (C-C-OA-MET-DEF): 0 ==por persona, 1=total
    /// Caudal de aire primario mínimo por persona con máxima ocupación, m³/h
    /// (C-C-OA-FLOW/PER)
    pub oa_flow_per: f32,
    /// Caudal de aire primario total, m³/h
    /// (C-C-OA-FLOW)
    pub oa_flow: f32,
}
