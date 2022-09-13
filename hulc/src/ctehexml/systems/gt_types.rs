// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Tipos de sistemas del .ctehexml - GT

// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105
// https://doe2.com/Download/DOE-23/DOE23Vol2-Dictionary_50h.pdf
//
// Archivo BDLDialogsCALENER-GT_3_4.txt para referencias de variables por tipos de objeto

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
            _ => bail!("Tipo de condensación desconocido"),
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
    /// Combustible
    /// En el caso de enfriadoras por absorción de llama directa o motor de combustión es distinto a electricidad y está en
    /// (FUEL-METER)
    pub fuel: String,

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
        let fuel = match kind {
            ChillerKind::GasAbsor | ChillerKind::Engine => {
                block.attrs.get_str("FUEL-METER").unwrap_or_default()
            }
            _ => "Electricidad".into(),
        };

        Self {
            name: block.name.clone(),
            kind,
            condenser_kind,
            cool_capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            eer: block.attrs.get_f32("C-NUM-OF-UNITS").unwrap_or_default(),
            eer_th: block.attrs.get_f32("C-IPLV").ok(),
            heat_capacity: block.attrs.get_f32("C-DESIGN-KW").ok(),
            fuel,
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
    pub fuel: String,

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

        let fuel = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
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
            fuel,
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
    pub fuel: String,

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

        let fuel = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
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
            fuel,
            dhw_loop: block.attrs.get_str("DHW-LOOP").unwrap_or_default(),
            dhw_pump: block.attrs.get_str("DHW-PUMP").ok(),
            dhw_tank,
        }
    }
}

/// Tipo de torres de refrigeración
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum HeatRejectionKind {
    /// Torre de refrigeración de circuito abierto
    #[default]
    OpenTower,
    /// Torre de refrigeración de circuito cerrado
    ClosedTower,
}

impl FromStr for HeatRejectionKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HeatRejectionKind::*;

        match s {
            "OPEN-TWR" => Ok(OpenTower),
            "FLUID-COOLER" => Ok(ClosedTower),
            _ => bail!("Tipo de condensación desconocido"),
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
    /// - FLUID-COOLER: Torre de refrigeración circuito cerrado
    /// No usados en GT:
    /// - OPEN-TWR&HX:
    /// - DRYCOOLER:
    pub kind: HeatRejectionKind,
    /// Vector energético
    /// Siempre electricidad
    pub fuel: String,

    // --- General
    /// Capacidad nominal de refrigeración en condiciones CTI, kW
    /// (C-C-CAPACITY)
    pub capacity: f32,
    /// Consumo de ventiladores por celda en condiciones nominales, kW
    /// (FAN-KW/CELL)
    pub fan_kw: f32,
    /// Número de celdas
    /// (NUMBER-OF-CELLS)
    pub number_of_cells: f32,

    // --- Conexiones a circuitos
    /// Circuito de condensados
    /// (CW-LOOP)
    pub cw_loop: String,
    /// Bomba del circuito de condensados
    /// (CW-PUMP)
    pub cw_pump: Option<String>,
    /// Potencia de bombas de recirculación, kW
    /// En torres de circuito cerrado
    /// (SPRAY-KW/CELL)
    pub spray_kw_cell: Option<f32>,
}

impl From<BdlBlock> for GtHeatRejection {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let fuel = "Electricidad".into();

        Self {
            name,
            kind,
            fuel,
            capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            fan_kw: block.attrs.get_f32("FAN-KW/CELL").unwrap_or_default(),
            number_of_cells: block.attrs.get_f32("NUMBER-OF-CELLS").unwrap_or(1.0),
            cw_loop: block.attrs.get_str("CW-LOOP").unwrap_or_default(),
            cw_pump: block.attrs.get_str("CW-PUMP").ok(),
            spray_kw_cell: block.attrs.get_f32("SPRAY-KW/CELL").ok(),
        }
    }
}

/// Equipos de cogeneración de GT
/// (ELEC-GENERATOR)
///
/// "Equipo de cogeneración 1" = ELEC-GENERATOR  
///    TYPE             = ENGINE-GENERATOR
///    CAPACITY         = 123
///    FUEL-METER       = "Gasóleo"
///    EXH-LOOP         = "CIRCUITO ACS"
///    JAC-LOOP         = "CIRCUITO ACS"
///    CW-LOOP          = "Circuito hidraúlico 4"
///    CW-DT            = 20
///    C-C-HIR          = 0.35
///    ..
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtElectricGenerator {
    /// Nombre / descripción
    pub name: String,
    // Tipo de sistema
    // (TYPE)
    // - ENGINE-GENERATOR: Motor de combustión
    // No usados por GT
    // - GAS-TURBINE-GENERATOR (no usado en GT)
    // - STEAM-TURBINE-GENERATOR (no usado en GT)
    // - PV-ARRAY (no usado en GT)
    // pub kind: String,
    /// Combustible usado
    /// (FUEL-METER) | "Gas Natural"
    pub fuel: String,
    /// Potencia nominal, kW
    /// (CAPACITY)
    pub capacity: f32,
    /// Rendimiento térmico nominal, ratio
    /// Relación entre electricidad producida y el consumo de combustible en condiciones nominales
    /// Usa poder calorífico superior (las tablas suelen venir en PCI)
    /// (C-C-HIR) || 0.35
    pub eff: f32,
    /// Circuito de energía térmica sobrante de las camisas del motor
    /// (CW-LOOP)
    pub cw_loop: Option<String>,
    /// Circuito para recuperación de calor de los gases
    /// (EXH-LOOP)
    pub exh_loop: Option<String>,
    /// Circuito para recuperación de calor de la camisa
    /// (JAC-LOOP)
    pub jac_loop: Option<String>,
}

impl From<BdlBlock> for GtElectricGenerator {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let fuel = block
            .attrs
            .get_str("FUEL-METER")
            .unwrap_or_else(|_| "Gas Natural".into());

        Self {
            name,
            fuel,
            capacity: block.attrs.get_f32("CAPACITY").unwrap_or_default(),
            eff: block.attrs.get_f32("C-C-HIR").unwrap_or(0.35),
            cw_loop: block.attrs.get_str("CW-LOOP").ok(),
            exh_loop: block.attrs.get_str("EXH-LOOP").ok(),
            jac_loop: block.attrs.get_str("JAC-LOOP").ok(),
        }
    }
}

/// Tipos de intercambiadores con agua bruta o el terreno
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GroundLoopHxKind {
    /// Intercambiador con agua bruta
    #[default]
    LakeWell,
    Ground,
}

impl FromStr for GroundLoopHxKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GroundLoopHxKind::*;

        match s {
            "LAKE/WELL" => Ok(LakeWell),
            "VERT-WELL-NEW" | "HORIZ-STRAIGHT-LOOP" | "HORIZ-SLINKY-LOOP" => Ok(Ground),
            _ => bail!("Tipo de condensación desconocido"),
        }
    }
}

/// Intercambiado de calor de agua con el terreno/agua/lago/pozo (alimentación de agua bruta) de GT
/// (GROUND-LOOP-HX)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GtGroundLoopHx {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - LAKE/WELL: intercambio con agua subterránea
    /// - otros tipos no usados en GT para suelo horizontal o vert., etc
    pub kind: GroundLoopHxKind,
    /// Circuito de agua bruta (condensados)
    /// (CIRCULATION-LOOP)
    pub circ_loop: String,
    /// Horario de temperatura del circuito
    /// (LOOP-TEMP-SCH)
    pub loop_temp_sch: String,
}

impl From<BdlBlock> for GtGroundLoopHx {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();

        Self {
            name,
            kind,
            circ_loop: block.attrs.get_str("CIRCULATION-LOOP").unwrap_or_default(),
            loop_temp_sch: block.attrs.get_str("LOOP-TEMP-SCH").unwrap_or_default(),
        }
    }
}

/// Tipos de sistemas secundarios de GT
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GtSystemKind {
    // -- SISTEMA VENTILACIÓN
    /// Solo ventilación (packaged multizone, doble conducto)
    Pmzs,
    // -- SISTEMAS TODO AIRE
    /// Autónomo caudal constante (packaged single zone, simple conducto, 1 termostato)
    #[default]
    Psz,
    /// Autónomo caudal variable (packaged variable-air volume, simple conducto)
    Pvavs,
    /// Autónomo caudal variable temperatura variable (packaged variable volume variable temperature, simple conducto)
    Pvvt,
    /// Todo aire caudal constante unizona (variable temperature (single zone reheat?, simple conducto)
    Szrh,
    /// Todo aire caudal variable (variable volume fan, simple conducto)
    Vavs,
    /// Todo aire caudal constante (constant-volume reheat fan)
    Rhfs,
    // SISTEMA TODO AIRE
    /// Enfriamiento evaporativo (evaporative cooling)
    EvapCool,
    // SISTEMA DE DOBLE CONDUCTO
    /// Todo aire doble conducto (dual-duct fan)
    Dds,
    // -- SISTEMAS ZONALES
    /// Autónomo mediante unidades terminales (packaged terminal aire conditioner)
    Ptac,
    /// Autónomo BdC en circuito cerrado (¿water loop? heat pump)
    Hp,
    /// Fancoil (ventiloconvector) (fan coil)
    Fc,
    /// Termoventilación (unit ventilator)
    Uvt,
    /// Solo calefacción por efecto Joule (unit heater)
    Uht,
    /// Solo calefacción por agua (floor panel heating)
    Fph,
    // -- SISTEMA AIRE PRIMARIO
    /// Climatizadora de aire primario (ceiling bypass)
    Cbvav,
}

impl FromStr for GtSystemKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GtSystemKind::*;

        match s {
            "PSZ" => Ok(Psz),
            "PMZS" => Ok(Pmzs),
            "PVAVS" => Ok(Pvavs),
            "PVVT" => Ok(Pvvt),
            "SZRH" => Ok(Szrh),
            "VAVS" => Ok(Vavs),
            "RHFS" => Ok(Rhfs),
            "DDS" => Ok(Dds),
            "PTAC" => Ok(Ptac),
            "HP" => Ok(Hp),
            "FC" => Ok(Fc),
            "UVT" => Ok(Uvt),
            "UHT" => Ok(Uht),
            "FPH" => Ok(Fph),
            "EVAP-COOL" => Ok(EvapCool),
            "CBVAV" => Ok(Cbvav),
            _ => bail!("Tipo de sistema secundario desconocido"),
        }
    }
}

/// Sistema (subsistema secundario) de GT
/// (SYSTEM)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtSystem {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema (lado del aire)
    /// p.339 https://doe2.com/Download/DOE-22/DOE22Vol2-Dictionary.pdf
    /// (TYPE)
    /// Tratamiento de aire
    /// - PSZ: Autónomo caudal constante (packaged single zone, simple conducto)
    /// - PMZS: Solo ventilación (packaged multizone, doble conducto)
    /// - PVAVS: Autónomo caudal variable (packaged variable-air volume, simple conducto)
    /// - PVVT: Autónomo caudal variable temperatura variable (packaged variable volume variable temperature, simple conducto)
    /// - SZRH: Todo aire caudal constante unizona (variable temperature (single zone reheat?, simple conducto)
    /// - VAVS: Todo aire caudal variable (variable volume fan, simple conducto)
    /// - RHFS: Todo aire caudal constante (constant-volume reheat fan)
    /// - DDS: Todo aire doble conducto (dual-duct fan)
    ///
    /// Sistemas unitarios
    /// * Sistemas zonales
    /// - PTAC: Autónomo mediante unidades terminales (packaged terminal aire conditioner)
    /// - HP: Autónomo BdC en circuito cerrado (¿water loop? heat pump)
    /// - FC: Fancoil (ventiloconvector) (fan coil)
    /// - UVT: Termoventilación (unit ventilator)
    /// - UHT: Solo calefacción por efecto Joule (unit heater)
    /// - FPH: Solo calefacción por agua (floor panel heating)
    ///
    /// - EVAP-COOL: Enfriamiento evaporativo (evaporative cooling)
    /// - CBVAV: Climatizadora de aire primario (ceiling bypass)
    ///
    /// Subtipo:
    /// (C-C-SUBTYPE-E1) ¿y otros?
    pub kind: GtSystemKind,
    // Parámetros generales ---
    /// Zona de control
    /// (CONTROL-ZONE)
    pub control_zone: Option<String>,

    // Tipo de retorno
    // (RETURN-AIR-PATH)
    // DIRECT | PLENUM-ZONES | DUCT | None
    // pub return_air_path: Option<String>,
    // Control de humedad ---
    // Tipo Control de Humedad (C-C-HUM-CONTROL)
    // Humedad máxima (C-C-HUM-MAX)
    // Humedad mínima (C-C-HUM-MIN)
    /// Ventiladores
    pub fans: Option<SysFans>,

    /// Refrigeración
    pub cooling: Option<SysCooling>,

    /// Calefacción
    pub heating: Option<SysHeating>,

    /// Control
    pub system: Option<SysControl>,

    /// Técnicas de recuperación
    pub recovery: Option<SysRecovery>,
    // -- Curvas de comportamiento
    // ...
}

/// Ventiladores de un subsistema secundario de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysFans {
    /// Horario de funcionamiento de los ventiladores de impulsión
    /// (FAN-SCHEDULE)
    pub fan_schedule: String,
    // Tipo de control
    // (C-C-FAN-CONTROL)
    // Posición del ventilador
    // (C-C-FAN-PLACEMENT)

    // Ventilador de impulsión ---
    /// Caudal del ventilador de impulsión, m³/h
    /// (C-C-SUPPLY-FLOW)
    pub supply_flow: f32,
    /// Potencia del ventilador de impulsión, kW
    /// (C-C_SUPPLY-KW)
    pub supply_kw: f32,

    // Ventilador de retorno ---
    /// ¿Existe ventilador retorno?
    /// (C-C-RETURN-FAN)
    /// Caudal de retorno, m³/h
    /// (RETURN-FLOW)
    pub return_flow: Option<f32>,
    /// Potencia de ventilador de retorno, kW
    /// (C-C-RETURN-KW)
    pub return_kw: Option<f32>,
    // Caja de caudal variable  o caja de mezcla en doble conducto DDS ---
    // Caudal mínimo
    // (C-C-MIN-FLOW-RAT)
}

/// Refrigeración de un subsistema secundario de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysCooling {
    // Baterías ---
    /// Potencia total batería, kW
    /// (C-C-COOL-CAP)
    pub cool_cap: f32,
    /// Potencia sensible batería, kW
    /// (C-C-COOL-SH-CAP)
    pub cool_sh_cap: f32,

    /// Circuito de agua fría que alimenta el sistema
    /// Es el circuito por defecto para zonas salvo que se indique
    /// (CHW-LOOP)
    pub chw_loop: Option<String>,
    /// Caudal agua fría, l/h
    /// (C-C-CHW-COIL-Q)
    pub chw_coil_q: Option<f32>,
    /// Circuito de agua enfriada que alimenta las unidades de zona
    /// (ZONE-CHW-LOOP)
    pub zone_chw_loop: Option<String>,
    // Salto térmico agua (CHW-COIL-DT), tipo de válvula (C-C-CHW-VALVE)...

    // Autónomos ---
    /// Tipo de condensación
    /// (C-C-COND-TYPE)
    /// Default autónomos: por aire
    pub tipo_condensacion: Option<String>,
    /// Rendimiento, EER
    /// (C-C-EER)
    /// Default: Autónomos 2.80
    pub eer: Option<f32>,
    /// Rendimiento, COP
    /// (C-C-COP)
    /// Default: ??
    pub cop: Option<f32>,
    // Varios preenfriamiento evaporativo:
    // Efectividad kWh/kWh (EVAP-PCC-EFF)
    // Horario (EVAP-PCC-SCH)
    // Consumo W/W (EVAP-PCC-ELEC)
    // Circuito condensación
    // (CW-LOOP)
    pub cw_loop: Option<String>,
    // Salto térmico condensación, ºC
    // (CW-COIL-DT)
    // Generador de aire:
    // Rendimiento térmico generador de aire
    // (C-C-FURNACE-HIR)
    // Consumo auxiliar generador de aire, kW
    // (C-C-FURNACE-AUX)

    // Enfriamiento evaporativo ---
    // Tipo (C-C-PROP-SR-2)
    // Consumo/Caudal (EVAP-CL-KW/FLOW)
    // Fracción aire impulsión (EVAP-CL-AIR)
    // Efectividad enfriamiento directo (DIRECT-EFF)
    // Efectividad enfriamiento indirecto (INDIR-EFF)

    // Economizador agua ---
    // Existe? (WS-ECONO)
    // Nombre circuito agua (WSE-LOOP)
    // Salto térmico agua (WSE-COIL-DT)
}

/// Calefacción de un subsistema secundario de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysHeating {
    // -- Calefacción --
    // Fuentes de calor ---
    // Fuente de calor a nivel de zona
    // (C-C-ZONE-H-SOUR)
    // 0=n/a, 1=1=eléctrica, 2=agua caliente, 3= circuito ACS, 4=Recuperación BdC gas, 5=Ninguna
    // Fuente de calor a nivel de sistema
    // (C-C-HEAT-SOURCE)
    // 0 = n/a, 1=eléctrica, 2=agua caliente, 3= circuito ACS, 4=BdC eléctrica, 5=BdC gas, 6=generador aire, 7=ninguna
    // Combustible
    // (MSTR-FUEL-METER)
    pub heat_fuel: Option<String>,

    // Baterías ---
    /// Potencia total batería, kW
    /// (C-C-HEAT-CAP)
    pub heat_cap: f32,
    /// Caudal batería, l/h
    pub hw_coil_q: Option<f32>,
    // Potencia batería recalentamiento
    // (C-C-REHEAT)
    // pub reheat_cap: Option<f32>,
    /// Circuito de agua caliente que alimenta la UTA
    /// Es el circuito por defecto para zonas salvo que se indique
    pub hw_loop: Option<String>,
    /// Circuito de agua caliente que alimenta las unidades de zona
    /// (ZONE-HW-LOOP)
    pub zone_hw_loop: Option<String>,
    // Circuito de ACS
    // (DHW-LOOP)
    // pub dhw_loop: Option<String>,

    // Salto térmico agua (HW-COIL-DT), tipo de válvula (C-C-HW-VALVE)...

    // Precalentamiento ---
    /// Fuente de calor
    /// (C-C-PREHEAT-SOURCE)
    pub preheat_source: Option<String>,
    /// Potencia batería, kW
    /// (C-C-PREHEAT-CAP)
    pub preheat_cap: Option<String>,
    // Min temperatura salida (PREHEAT-T)
    /// Circuito batería precalentamiento
    /// (PHW-LOOP)
    pub preheat_loop: Option<String>,
    /// Caudal batería precalentamiento, l/h
    /// (C-C-PHW-COIL-Q)
    pub preheat_coil_q: Option<f32>,
    // Salto térmico batería precalentamiento, ºC
    // (PHW-COIL-DT)
    // pub preheat_coil_dt: Option<f32>,
    // Tipo válvula batería precalentamiento
    // (PHW-VALVE-TYPE)
    // pub preheating_valve_type: Option<String>

    // Calef. auxiliar ---
    /// Fuente de calor calefacción auxiliar
    /// (C-C-BBRD-SOUR)
    pub aux_heat_source: Option<String>,
    // Tipo de control de calefacción auxiliar
    // (C-C-BBRD-CONTROL)
    // pub aux_heat_control: Option<String>,
    // Circuito unidad terminal
    // (BBRD-LOOP)
    // pub aux_heat_loop: Option<String>,
    // Salto térmico unidad terminal, ºC
    // (BBRD-COIL-DT)
    // pub aux_heat_dt: Option<f32>

    // Bomba de calor ---
    /// Fuente de calor
    /// (C-C-HP-SUPP-SOUR)
    pub hp_heat_source: Option<String>,
    /// Potencia apoyo, kW
    /// (C-C-HP-SUPP-CAP)
    pub hp_supp_capacity: Option<f32>,
    // Tipo de desescarche (DEFROST-TYPE)
    // Control desescarche (DEFROST-CTRL)
    // Temperatura desescarche (DEFROST-T)
    // Pot. resist. / Pot. BdC, (RESIST-CAP-RATIO)
}

/// Control de un subsistema secundario de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysControl {
    /// Temperatura de impulsión min
    /// (MIN-SUPPLY-T)
    /// Default: autónomos, 15ºC
    pub min_supply_t: Option<f32>,
    /// Temperatura de impulsión max
    /// (MAX-SUPPLY-T)
    pub max_supply_t: Option<f32>,
    // Horarios de disponibilidad
    // calefacción (HEATING-SCHEDULE) y refrigeración (COOLING-SCHEDULE)
    // Control UTA (C-C-COOL-CONTROL)
    // Consigna del termostato (COOL-SET-T)
    // Horario de temperatura (COOL-SET-SCH)
    // Ley de correspondencia (COOL-RESET-SCH)
}

/// Técnicas de recuperación de un subsistema secundario de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysRecovery {
    // Enfriamiento gratuito ---
    /// ¿Existe enfriamiento gratuito?
    /// (C-C-ENF-GRAT)
    pub free_cooling: bool,
    /// Tipo de control de enfriamiento gratuito
    /// (C-C-OA-CONTROL)
    pub oa_control: Option<String>,

    // Recuperación de calor ---
    /// ¿Existe recuperación de calor?
    /// (RECOVER-EXHAUST)
    pub exhaust_recovery: bool,
    /// Tipo de recuperación de calor
    /// (ERV-RECOVER-TYPE)
    pub exhaust_recovery_type: Option<String>,
    // Potencia recuperador de calor, kW
    // (ERV-HX-KW)
    /// Efectividad recuperación de calor (sensible)
    /// (ERV-SENSIBLE-EFF)
    pub exhaust_recovery_eff: Option<f32>,
}

/// Tipos de zonas
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ZoneKind {
    /// Acondicionada
    #[default]
    Conditioned,
    Plenum,
    Unconditioned,
}

impl FromStr for ZoneKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ZoneKind::*;

        match s {
            "CONDITIONED" => Ok(Conditioned),
            "PLENUM" => Ok(Plenum),
            "UNCONDITIONED" => Ok(Unconditioned),
            _ => bail!("Tipo de zona desconocido"),
        }
    }
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
    pub kind: ZoneKind,
    /// Espacio asociado
    /// (SPACE)
    pub space: String,
    /// Sistema secundario asignado a la zona
    /// (PARENT)
    pub system: Option<String>,

    // --- Caudales
    // -- Aire impulsión de diseño --
    /// Caudal de impulsión de diseño de la zona, m³/h
    /// Si no se define usa la disponible por el sistema
    /// (C-C-ASSIG-FLOW)
    pub design_flow: Option<f32>,

    // -- Ventilador de extracción
    /// Caudal de extracción, m³/h
    /// (C-C-EXH-FLOW)
    pub exh_flow: Option<f32>,
    /// Potencia de extracción, kW
    /// (C-C-EXHAUST-KW)
    pub exh_kw: Option<f32>,

    // -- Aire exterior --
    // Método para asignar caudal (C-C-OA-MET-DEF): 0 ==por persona, 1=total
    /// Caudal de aire primario mínimo por persona con máxima ocupación, m³/h
    /// (C-C-OA-FLOW/PER)
    pub oa_flow_per: Option<f32>,
    /// Caudal de aire primario total, m³/h
    /// (C-C-OA-FLOW)
    pub oa_flow: Option<f32>,
    // --- Unidades terminales
    // -- Refrigeración --
    /// Potencia nominal total de refrigeración (sensible + latente) de la unidad terminal, kW
    /// Si no se define usa la disponible por el sistema
    /// (C-C-COOL-CAP)
    pub cool_cap: Option<f32>,
    /// Potencia nominal sensible de refrigeración de la unidad terminal, kW
    /// Si no se define usa la disponible por el sistema
    /// (C-C-COOL-SH-CAP)
    pub cool_sh_cap: Option<f32>,

    // -- Calefacción --
    /// Potencia nominal de calefacción de la unidad terminal, kW
    /// Si no se define usa la disponible por el sistema
    /// (C-C-HEAT-CAP)
    pub heat_cap: Option<f32>,
    // -- Calefacción auxiliar --
    // TODO: ?? Generar ejemplo para ver variables usadas
}

impl From<BdlBlock> for GtZone {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();

        let has_exhaust_fan = block.attrs.get_str("C-C-PROP-ZR-1").unwrap_or_default() == "1";
        let exh_flow = if has_exhaust_fan {
            block.attrs.get_f32("C-C-EXH-FLOW").ok()
        } else {
            None
        };
        let exh_kw = if has_exhaust_fan {
            block.attrs.get_f32("C-C-EXH-KW").ok()
        } else {
            None
        };

        let (oa_flow_per, oa_flow) = match block
            .attrs
            .get_str("C-C-OA-MET-DEF")
            .unwrap_or_default()
            .as_str()
        {
            // Caudal total
            "1" => (None, block.attrs.get_f32("C-C-OA-FLOW").ok()),
            // Caudal por persona
            _ => (block.attrs.get_f32("C-C-OA-FLOW/PER").ok(), None),
        };

        Self {
            name,
            kind,
            space: block.attrs.get_str("SPACE").unwrap_or_default(),
            // El sistema se asigna tras la construcción
            system: None,
            design_flow: block.attrs.get_f32("C-C-ASSIG-FLOW").ok(),
            exh_flow,
            exh_kw,
            oa_flow_per,
            oa_flow,
            cool_cap: block.attrs.get_f32("C-C-COOL-CAP").ok(),
            cool_sh_cap: block.attrs.get_f32("C-C-COOL-SH-CAP").ok(),
            heat_cap: block.attrs.get_f32("C-C-HEAT-CAP").ok(),
        }
    }
}
