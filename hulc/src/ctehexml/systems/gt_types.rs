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
// Ver Manual Técnico GT

// TODO: THERMAL-STORAGE (Cold-water-tank, Hot-water-tank)

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

/// Bomba de GT. En circuitos o equipos (como enfriadoras)
/// (PUMP)
/// Potencia de la bomba: P = rho ·  g · Q · H / n
/// Eficiencia del motor (MOTOR-EFF) = 0.8
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

/// Tipo de circuito hidráulico
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CirculationLoopKind {
    /// Dos tubos (agua fría o agua caliente pero no simultáneamente)
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// Temperatura de cambio estacional (SNAP-T) o Disponibilidad en función de horario ()
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para calor (HEAT-SETPT-T) (defecto 80ºC)
    /// caudal máximo (C-C-PROCESS-FLOW)
    /// consigna para frío (COOL-SETPT-T) (defecto 7ºC)
    /// Control AF y AC
    #[default]
    Pipe2,
    /// ACS
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 35ºC)
    /// consigna para calor (HEAT-SETPT-T) (defecto 50ºC)
    /// caudal máximo, requerido (C-C-PROCESS-FLOW)
    /// temperatura de agua de red, requerido (DHW-INLET-T)
    /// horario demanda ACS (PROCESS-SCH)
    Dhw,
    /// Agua fría
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (COOL-SETPT-T) (defecto 7ºC)
    ///
    Chw,
    /// Agua bruta (intercambio con el terreno)
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (COOL-SETPT-T) (defecto 30ºC) y calor (HEAT-SETPT-T) (defecto 20ºC)
    /// bomba del circuito, requerido (LOOP-PUMP)
    LakeWell,
    /// Agua caliente
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 20ºC)
    /// consigna para calor (defecto 80ºC)
    Hw,
    /// Bomba de calor circuito cerrado
    /// usa caudal recirculado (C-C-LP-REC-FLOW1) (defecto 0 l/h)
    /// salto de temperatura (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (COOL-SETPT-T) (defecto 30ºC) y calor (HEAT-SETPT-T) (defecto 20ºC)
    /// bomba del circuito, requerido (LOOP-PUMP)
    Whlp,
    /// Circuito de agua de condensación
    /// salto de temperatura de diseño (LOOP-DESIGN-DT) (defecto 5ºC)
    /// consigna para frío (COOL-SETPT-T) (defecto 30ºC)
    Cw,
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
    /// (C-IPLV) // HIR = 1 / C-IPLV
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
    /// (C-THERM-EFF-MAX || ( conv 0.85, bajatemp 0.90, condens 0.95, biomasa 0.75)
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

/// Hot Water Storage
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtHotWaterStorageTank {
    /// Nombre
    pub name: String,
    /// Volumen, m³
    /// Por defecto, capacidad de la caldera * 65
    pub volume: f32,
    /// Coeficiente de pérdidas global del depósito, UA (W/ºC)
    /// Por defecto, volumen * 0.042
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
    /// en BdC (2.7) y calderas eléctricas (1.0) -> (C-STBY-LOSS-FRAC)
    /// Eficiencia térmica, nu
    /// En calderas de combustible
    /// gas 0.80,
    /// (C-ENERGY-FACTOR)
    pub eff: f32,
    /// Combustible
    /// - Gas Natural
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
    pub number_of_cells: u32,

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

/// Tipos de intercambiadores con agua bruta o el terreno
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GroundLoopHxKind {
    /// Intercambiador con agua bruta
    #[default]
    LakeWell,
    Ground,
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

/// Tipos de sistemas secundarios de GT
/// (TYPE)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GtSystemKind {
    // -- SISTEMA VENTILACIÓN, CENTRALIZADO
    /// Solo ventilación (packaged multizone, doble conducto)
    /// Sin producción de frío/calor y tratamiento del aire centralizado
    Pmzs,
    // -- SISTEMAS TODO AIRE, CENTRALIZADO
    /// Autónomo caudal constante (packaged single zone, simple conducto, 1 termostato)
    /// Producción de frío con autónomo y tratamiento del aire centralizado
    #[default]
    Psz,
    /// Autónomo caudal variable (packaged variable-air volume, simple conducto)
    /// Producción de frío con autónomo y tratamiento del aire centralizado
    Pvavs,
    /// Autónomo caudal variable temperatura variable (packaged variable volume variable temperature, simple conducto)
    /// Producción de frío con autónomo y tratamiento del aire centralizado
    Pvvt,
    /// Todo aire caudal constante unizona (variable temperature (single zone reheat?, simple conducto)
    /// Producción de frío con agua fría y tratamiento del aire centralizado
    Szrh,
    /// Todo aire caudal variable (variable volume fan, simple conducto)
    /// Producción de frío con agua fría y tratamiento del aire centralizado
    Vavs,
    /// Todo aire caudal constante (constant-volume reheat fan)
    /// Producción de frío con agua fría y tratamiento del aire centralizado
    Rhfs,
    // SISTEMA TODO AIRE
    /// Enfriamiento evaporativo (evaporative cooling)
    /// Producción de frío con enfriamiento evaporativo y tratamiento del aire centralizado
    EvapCool,
    // SISTEMA DE DOBLE CONDUCTO, CENTRALIZADO
    /// Todo aire doble conducto (dual-duct fan)
    /// Producción de frío con agua fría y tratamiento del aire centralizado
    Dds,
    // -- SISTEMAS ZONALES
    /// Autónomo mediante unidades terminales (packaged terminal aire conditioner)
    /// Producción de frío con autónomos y tratamiento del aire zonal
    /// Subtipos: Convencional / Caudal de refrigerante variable
    Ptac,
    /// Autónomo BdC en circuito cerrado (¿water loop? heat pump)
    /// Producción de frío con autónomos y tratamiento del aire zonal
    Hp,
    /// Fancoil (ventiloconvector) (fan coil)
    /// Producción de frío con agua fría y tratamiento del aire zonal
    Fc,
    /// Termoventilación (unit ventilator)
    /// Solo calefacción y tratamiento del aire zonal
    Uvt,
    /// Solo calefacción por efecto Joule (unit heater)
    /// Solo calefacción y tratamiento del aire zonal
    /// Subtipos: Con aire de impulsión / Sin aire de impulsión
    Uht,
    /// Solo calefacción por agua (floor panel heating)
    /// Solo calefacción y tratamiento del aire zonal
    /// Subtipos: Paneles radiantes / radiadores
    Fph,
    // -- SISTEMA AIRE PRIMARIO, CENTRALIZADO
    /// Climatizadora de aire primario (ceiling bypass)
    /// Producción de frío con agua fría y tratamiento del aire zonal
    Cbvav,
}

/// Sistema (subsistema secundario) de GT
///
/// Son los equipos y dispositivos encargados del tratamiento y distribución del
/// aire a los locales.
///
/// Incluye las UTA (sección de baterías (frío y calor), sección de humidificación,
/// de los ventiladores, las zonas térmicas, los termostatos, las unidades
/// terminales, etc.
///
/// Las instalaciones que no utilicen el agua como fluido caloportador solo está
/// formada por sistemas secundarios, sin necesidad de circuitos, como,
/// por ejemplo, es el caso de todos los equipos autónomos que enfrían aire con
/// expansión directa de un refrigerante.
///
/// En general, los subsistemas secundarios se dividen a nivel de sistema (o de UTA)
/// y de zona.
///
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
    /// - PTAC: Autónomo mediante unidades terminales (packaged terminal air conditioner)
    /// - HP: Autónomo BdC en circuito cerrado (¿water loop? heat pump)
    /// - FC: Fancoil (ventiloconvector) (fan coil)
    /// - UVT: Termoventilación (unit ventilator)
    /// - UHT: Solo calefacción por efecto Joule (unit heater)
    /// - FPH: Solo calefacción por agua (floor panel heating)
    ///
    /// - EVAP-COOL: Enfriamiento evaporativo (evaporative cooling)
    /// - CBVAV: Climatizadora de aire primario (ceiling bypass)
    ///
    /// Tipo:
    /// (KIND)
    /// Subtipo:
    /// (C-C-SUBTYPE-E1) (C-C-SUBTYPE-E2)
    pub kind: GtSystemKind,

    // Parámetros generales ---
    /// Zona de control
    /// Solo en sistemas:
    /// - Todo aire caudal constante unizona (SZRH)
    /// - Autónomo caudal constante (PSZ)
    /// (CONTROL-ZONE)
    pub control_zone: Option<String>,

    // Tipo de retorno
    // (RETURN-AIR-PATH)
    // DIRECT | PLENUM-ZONES | DUCT | None
    // pub return_air_path: Option<String>,
    // Control de humedad ---
    // TODO:
    // Tipo Control de Humedad (C-C-HUM-CONTROL)
    // Humedad máxima (C-C-HUM-MAX)
    // Humedad mínima (C-C-HUM-MIN)

    // Aire ----------------
    // -- Ventiladores --
    /// Horario de funcionamiento de los ventiladores de impulsión
    /// (FAN-SCHEDULE)
    pub fan_schedule: Option<String>,
    // Tipo de control
    // (C-C-FAN-CONTROL)
    // Posición del ventilador
    // (C-C-FAN-PLACEMENT)

    // Ventilador de impulsión ---
    /// Caudal del ventilador de impulsión, m³/h
    /// (C-C-SUPPLY-FLOW)
    /// Potencia del ventilador de impulsión, kW (C-C-SUPPLY-KW)
    /// En sistemas zonales (PTAC, HP, FC, UVT, UHT, FPH) se usa el factor de transporte W/(m³/h) (C-C-SUP-KW/FLOW)
    /// default: 0.10
    pub supply_fan: Option<Fan>,

    // Ventilador de retorno ---
    /// ¿Existe ventilador retorno?
    /// (C-C-RETURN-FAN)
    /// Caudal de retorno, m³/h
    /// (RETURN-FLOW)
    /// Potencia de ventilador de retorno, kW
    /// (C-C-RETURN-KW)
    pub return_fan: Option<Fan>,
    // Caja de caudal variable  o caja de mezcla en doble conducto DDS ---
    // Caudal mínimo para caja de mezcla (en caudal variable), -
    // (C-C-MIN-FLOW-RAT)
    // pub min_flow_ratio: Option<f32>,

    // -- Ventilación --
    /// Ventilación: Técnicas de recuperación de un subsistema secundario de GT
    /// No aplicable a sistemas de solo ventilación PMZS
    /// Free cooling
    // ¿Existe enfriamiento gratuito?
    // (C-C-ENF-GRAT)
    // Tipo de control de enfriamiento gratuito
    // (C-C-OA-CONTROL)
    // 0 (default): "Por temperatura"
    // 1: "Por entalpía"
    pub airside_economizer: Option<EconomizerControl>,
    /// Recuperación de calor del aire de expulsión
    /// Efectividad de la recuperación de calor (calor sensible)
    /// Valor por defecto, si hay = 0.76
    // ¿Existe recuperación de calor?
    // (RECOVER-EXHAUST)
    // YES
    // NO
    // Tipo de recuperación de calor (no usado en GT?)
    // (ERV-RECOVER-TYPE)
    // pub exhaust_recovery_type: Option<String>,
    // Potencia recuperador de calor, kW
    // (ERV-HX-KW)
    // Efectividad de la recuperación de calor (calor sensible)
    // (ERV-SENSIBLE-EFF)
    // Si hay, valor por defecto = 0.76
    pub exhaust_recovery: Option<f32>,

    // Equipos ----------------
    // -- Refrigeración

    // -- Fuentes de Calefacción y Refrigeración --
    // - Fuente de refrigeración a nivel del sistema
    // - Fuente de calefacción a nivel del sistema
    // - Fuente de calor/frío zonal

    // Parámetros para refrigeración (circuito baterías, enf. evaporativo, etc)
    pub cooling: Option<SysCooling>,

    // Fuentes de calor ---
    // https://doe2.com/Download/DOE-22/DOE22Vol2-Dictionary.pdf p.391
    // Ver fuentes de calor en Manual Técnico de Calener GT
    // Ver tablas de Manual Técnico de GT
    // Fuentes de calor a nivel de sistema de un subsistema secundario de GT
    // No existe en sistemas solo ventilación (PMZS)
    /// Potencia total de calefacción (baterias del sistema), kW
    /// (C-C-HEAT-CAP)
    /// En unidades terminales se definen en la zona
    /// Fuente de calor a nivel de sistema (baterías principales del sistema)
    /// 0=n/a, 1=eléctrica, 2=circuito agua caliente, 3=circuito ACS, 4=BdC eléctrica, 5=BdC gas, 6=generador aire, 7=ninguna
    /// (C-C-HEAT-SOURCE)
    /// No existe en sistemas zonales FC, PTAC, HP, UVT, UHT, FPH
    /// y en EVAP-COOL y CBVAV
    pub heating_sys: Option<HeatSource>,

    /// Fuente de calor (o frío) para las baterías de zona (recalentamiento) en unidades terminales
    /// Potencia total de calefacción (baterias del sistema), kW
    /// (C-C-HEAT-CAP)
    /// En unidades terminales se definen en la zona
    /// (C-C-ZONE-H-SOUR)
    /// 0=n/a, 1=eléctrica, 2=circuito agua caliente, 3=circuito ACS, 4=BdC eléctrica, 5=BdC gas, 6=generador aire, 7=Ninguna
    /// No existe en todo aire doble conducto (DDS), Climatizadora aire primario (CBVAV), y Solo ventilación (PMZS)
    /// La generación de aire solo está en sistemas autónomos (en zonales solo PTAC, no HP)
    /// La capacidad se define en la zona con (C-C-HEAT-CAP)
    pub heating_zone: Option<HeatSource>,

    // Precalentamiento ---
    // GT no exporta al XML la potencia del precalentamiento
    // Opcional en Sistemas todo aire (SZRH, VAVS, RHFS, DDS) y
    // autónomos caudal constante y variable (PSZ, PVAVS) (no en PVVT ni sistemas zonales)
    pub pre_heating: Option<SysPreHeating>,

    // Calefacción auxiliar ---
    // Opcional en sistemas Todo aire caudal variable (VAVS)
    pub aux_heating: Option<SysAuxHeating>,

    // ...
    /// Control
    pub control: Option<SysControl>,
    // -- Curvas de comportamiento
}

/// Ventiladores
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Fan {
    /// Caudal, m³/h
    pub flow: f32,
    /// Potencia de ventilador, kW
    pub kw: f32,
}

/// Fuentes de calor - Tipos de sistemas secundarios de GT
/// (TYPE)
/// Fuente de calor a nivel de sistema o zona
/// 0=n/a, 1=eléctrica, 2=circuito agua caliente, 3=circuito ACS, 4=BdC eléctrica, 5=BdC gas, 6=generador aire, 7=ninguna
#[derive(Debug, Clone, PartialEq)]
pub enum HeatSource {
    /// Electricidad
    Electric { heating_cap: Option<f32> },
    /// Batería de circuito agua caliente
    HotWaterLoop {heating_cap: Option<f32>, w_loop: String },
    /// Batería de circuito de ACS
    /// En GT solo en calentamiento principal (sistema), y recalentamiento terminal (zona)
    DhwLoop { heating_cap: Option<f32>, w_loop: String },
    /// BdC eléctrica
    /// En GT solo en calentamiento principal (sistema)
    HeatPump { heating_cap: Option<f32>, cooling_cap: Option<f32>, cooling_sh_cap: Option<f32>, cop: f32 },
    /// BdC gas
    /// En GT solo en calentamiento principal (sistema)
    GasHeatPump { heating_cap: Option<f32>, cooling_cap: Option<f32>, cooling_sh_cap: Option<f32>, cop: f32 },
    /// Generador aire (con combustible)
    /// En GT solo en calentamiento principal (sistema)
    /// eff: eficiencia del sistema
    /// aux_kw: consumo auxiliar, kW
    Furnace { heating_cap: Option<f32>, eff: f32, aux_kw: f32 },
}

impl Default for HeatSource {
    fn default() -> Self {
        Self::Electric { heating_cap: Some(10.0) }
    }
}

/// Datos específicos de refrigeración de un subsistema secundario de GT
/// No existen en sistemas de solo ventilación PMZS
/// TODO: reorganizar como enumeración según tipo...
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysCooling {
    /// Potencia total batería frío, kW
    /// (C-C-COOL-CAP)
    pub cooling_cap: f32,
    /// Potencia sensible batería frío, kW
    /// Para la mayor parte de equipos prefabricados es 0.8 * cool_cap
    /// (C-C-COOL-SH-CAP)
    pub cooling_sh_cap: f32,

    // ## Chilled-Water Coils
    // Baterías  de agua fría ---
    /// Circuito de agua fría que alimenta el sistema
    /// Es el circuito por defecto para zonas salvo que se indique
    /// (CHW-LOOP)
    pub chw_loop: Option<String>,
    /// Caudal agua fría, l/h
    /// (C-C-CHW-COIL-Q)
    /// default potencia * 0.86 / salto_térmico
    pub chw_coil_q: Option<f32>,
    // Salto térmico batería de agua fría (CHW-COIL-DT)
    // Tipo de válvula batería de agua fría (C-C-CHW-VALVE)

    // Circuito de agua fría que alimenta las unidades de zona
    // (ZONE-CHW-LOOP)
    // pub zone_chw_loop: Option<String>,

    // Autónomos ---
    // Parámetros para sistemas de generación autónomos
    // No utilizan circuitos de agua, sean sistemas con tratamiento del aire
    // centralizado o zonal
    // refrigeración con dx (bomba de calor), condensación por agua, enf. evaporativo, etc

    // ## DX Cooling
    /// Rendimiento, EER
    /// (C-C-EER)
    /// Default: Autónomos 2.80
    pub eer: Option<f32>,
    // Tipo de condensación
    // (C-C-COND-TYPE)
    // Solo en sistemas zonales
    // No existe en sistemas todo aire o doble conducto
    // Por aire (0), por agua (1), preenfriamiento evaporativo (2)
    // Default: por aire
    // pub cond_type: Option<String>,

    // ## Water-Cooled condensers
    // Refrigeración autónomos, condensación por agua ---
    // Circuito condensación
    // (CW-LOOP)
    // pub cw_loop: Option<String>,
    // Salto térmico condensación, ºC
    // (CW-COIL-DT)

    // Refrigeración autónomos, preenfriamiento evaporativo ---
    // Varios preenfriamiento evaporativo (frío):
    // Efectividad kWh/kWh (EVAP-PCC-EFF)
    // Horario (EVAP-PCC-SCH)
    // Consumo W/W (EVAP-PCC-ELEC)

    // TODO: Enfriamiento evaporativo (batería frío)---
    // Tipo (C-C-PROP-SR-2)
    // Consumo/Caudal (EVAP-CL-KW/FLOW)
    // Fracción aire impulsión (EVAP-CL-AIR)
    // Efectividad enfriamiento directo (DIRECT-EFF)
    // Efectividad enfriamiento indirecto (INDIR-EFF)

    // TODO: Economizador agua (batería frío) ---
    // Existe? (WS-ECONO)
    // Nombre circuito agua (WSE-LOOP)
    // pub wse_loop: Option<String>,
    // Salto térmico agua (WSE-COIL-DT)
    // pub wse_coil_dt: Option<f32>,
}

/// Parámetros de sistemas de calefacción de un subsistema secundario de GT
/// No existen en sistemas de solo ventilación PMZS
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysHeatingDetail {
    // Baterías ----
    /// Caudal batería, l/h
    /// (C-C-HW-COIL-Q)
    pub hw_coil_q: Option<f32>,
    // Recalentamiento
    // Potencia batería recalentamiento
    // (C-C-REHEAT)
    // pub reheat_cap: bool,
    /// Circuito de agua caliente que alimenta la UTA
    /// Existe en sistema todo aire: PSZ, PVAVS, PVVT, SZRH, VAVS, RHFS, EVAP-COOL
    /// y doble conducto DDS
    /// No existe en sistemas zonales (PTAC, HP, FC, UVT, UHT, FPH)
    /// Es el circuito por defecto para zonas salvo que se indique
    /// (HW-LOOP)
    pub hw_loop: Option<String>,
    /// Circuito de agua caliente que alimenta las unidades de zona en sistemas
    /// de tratamiento de aire centralizado cuando difiera del del sistema y no se especifique
    /// en la zona
    /// No existe en sistema de doble conducto DDS
    /// (ZONE-HW-LOOP)
    pub zone_hw_loop: Option<String>,
    /// Circuito de agua caliente ¿para algunos equipos de zona?
    /// (DHW-LOOP)
    pub dhw_loop: Option<String>,
    // Salto térmico agua batería calefacción, ºC (HW-COIL-DT)
    // Tipo de válvula batería calefacción (C-C-HW-VALVE)...
    // Tipo de control en sistemas zonales (C-C-CONDENSER-TYPE)

    // Autónomos ---
    // Parámetros para sistemas de generación autónomos
    // No utilizan circuitos de agua, sean sistemas con tratamiento del aire
    // centralizado o zonal

    // calefacción (dx o generador de aire) de un subsistema secundario de GT

    // Opciones cuando HeatSource es Generador de aire / furnace (calor) ---
    // Combustible usado
    // (MSTR-FUEL-METER)
    pub heat_fuel: Option<String>,
    // Rendimiento térmico del generador de aire
    // (C-C-FURNACE-HIR)
    // Consumo auxiliar del generador de aire, kW
    // (C-C-FURNACE-AUX)

    // Opciones cuando el HeatSource es BdC ---
    /// Rendimiento, COP
    /// (C-C-COP)
    pub cop: Option<f32>,
    // Datos adicionales cuando la fuente de calor es una bomba de calor
    // Fuente de calor de apoyo de la BdC
    // (C-C-HP-SUPP-SOUR)
    // Eléctrica, Agua caliente, Recuperación BdC gas, Ninguna?
    // pub heat_source: Option<String>,
    // Potencia de apoyo, kW
    // (C-C-HP-SUPP-CAP)
    // pub aux_capacity: Option<f32>,

    // Desescarche BdC ---
    // Tipo de desescarche (DEFROST-TYPE)
    // Control desescarche (DEFROST-CTRL)
    // Temperatura desescarche (DEFROST-T)
    // Pot. resist. / Pot. BdC, (RESIST-CAP-RATIO)
}

/// Precalentamiento de un subsistema secundario de GT
/// Su definición depende de la fuente de calor
/// No existen en sistemas de solo ventilación PMZS
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysPreHeating {
    // Precalentamiento (si no es con batería) ---
    // Calienta el aire cuando está por debajo de la temperatura de congelación
    /// Fuente de calor
    /// (C-C-PREHEAT-SOURCE)
    /// Solamente admite fuentes: eléctrica, agua caliente y circuito de ACS
    pub source: HeatSource,
    /// Potencia batería, kW
    /// (C-C-PREHEAT-CAP)
    pub capacity: f32,
    // Min temperatura salida (PREHEAT-T)

    // Batería de precalentamiento ---
    // Calienta el aire cuando está por debajo de la temperatura de congelación
    /// Circuito batería precalentamiento, si el source es un circuito (y no eléctrico o de gas)
    /// (PHW-LOOP)
    pub loop_name: Option<String>,
    // Caudal batería precalentamiento, l/h
    // (C-C-PHW-COIL-Q)
    // pub preheat_coil_q: Option<f32>,
    // Salto térmico batería precalentamiento, ºC
    // (PHW-COIL-DT)
    // pub preheat_coil_dt: Option<f32>,
    // Tipo válvula batería precalentamiento
    // (PHW-VALVE-TYPE)
    // pub preheating_valve_type: Option<String>
}

/// Calefacción auxiliar de un subsistema secundario de GT
/// Se puede definir según la fuente de calor
/// No existen en sistemas de solo ventilación PMZS
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysAuxHeating {
    // Calefacción auxiliar ---
    /// Fuente de calor calefacción auxiliar
    /// (C-C-BBRD-SOUR)
    /// Solo en sistemas todo aire caudal variable VAVS
    /// Se admiten como fuentes solo eléctrica y agua caliente
    pub source: HeatSource,
    // Tipo de control de calefacción auxiliar
    // (C-C-BBRD-CONTROL)
    // pub aux_heat_control: Option<String>,
    /// Unidad terminal para la calefacción auxiliar ---
    /// Circuito unidad terminal
    /// (BBRD-LOOP)
    pub loop_name: Option<String>,
    // Salto térmico unidad terminal, ºC
    // (BBRD-COIL-DT)
    pub dt: Option<f32>,
    // TODO: potencia cuando es eléctrica?
}

/// Control de un subsistema secundario de GT
/// No aplicable a sistemas de solo ventilación PMZS
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SysControl {
    // Temperaturas de impulsión ---
    /// Temperatura de impulsión min
    /// (MIN-SUPPLY-T)
    /// Default: autónomos, 15ºC
    pub min_supply_t: Option<f32>,
    /// Temperatura de impulsión max
    /// (MAX-SUPPLY-T)
    pub max_supply_t: Option<f32>,
    // Horarios de disponibilidad ---
    /// Horario de disponibilidad de calefacción
    /// (HEATING-SCHEDULE)
    pub heating_schedule: Option<String>,
    /// Horario de disponibilidad de refrigeración
    /// (COOLING-SCHEDULE)
    pub cooling_schedule: Option<String>,
    // Control de la UTA ---
    // Conducto frío:
    // Control UTA (C-C-COOL-CONTROL)
    // Consigna del termostato (COOL-SET-T)
    // Horario de temperatura (COOL-SET-SCH)
    // Ley de correspondencia (COOL-RESET-SCH)
    // Conducto caliente (solo en sistemas doble conducto DDS):
    // Control UTA (HEAT-CONTROL)
    // Consigna del termostato (HEAT-SET-T)
    // Horario de temperatura (HEAT-SET-SCH)
    // Ley de correspondencia (HEAT-RESET-SCH)
}

/// Tipos de control para el enfriamiento gratuito por aire
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum EconomizerControl {
    /// Temperatura
    #[default]
    Temperature,
    /// Entalpía
    Enthalpy,
    /// Temperatura y entalpía
    // No usado en GT
    TemperatureEnthalpy,
    /// Desconocido
    // No usado en GT
    Unknown,
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

/// Sistemas secundarios de distribución específicos de una zona térmica
///
/// Datos de las zonas térmicas abastecidas por los sistemas:
/// - Termostato (consignas, tipo, etc) de zona
/// - Caudales de zona (impulsión, ventilación y extracción)
/// - Capacidades de calefacción / refrigeración
///
/// (ZONE)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtZoneSystem {
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

    // --- Termostatos
    /// Consigna de calefacción
    /// Si no se define se supone que no hay control de temperatura activado por la zona
    /// En este caso se debe mirar el bloque SYSTEM-CONDITIONS y SPACE-CONDITIONS
    /// del SPACE, con (HEAT-TEMP-SCH) y (COOL-TEMP-SCH) para definir las consignas
    /// (HEAT-TEMP-SCH)
    pub heat_temp_sch: Option<String>,
    /// Consigna de refrigeración
    /// Si no se define se supone que no hay control de temperatura activado por la zona
    /// En este caso se debe mirar el bloque SYSTEM-CONDITIONS y SPACE-CONDITIONS
    /// del SPACE, con (HEAT-TEMP-SCH) y (COOL-TEMP-SCH) para definir las consignas
    /// (COOL-TEMP-SCH)
    pub cool_temp_sch: Option<String>,
    // Tipo de termostato
    // (C-C-THERM-TYPE)

    // --- Caudales
    // -- Aire impulsión de diseño --
    /// Caudal de impulsión de diseño de la zona, m³/h
    /// Si no se define usa la disponible por el sistema
    /// (C-C-ASSIG-FLOW)
    pub design_flow: Option<f32>,

    // -- Ventilador de extracción
    /// Caudal de extracción, m³/h
    /// (C-C-EXH-FLOW)
    /// Potencia de extracción, kW
    /// (C-C-EXHAUST-KW)
    pub exhaust_fan: Option<Fan>,

    // -- Aire exterior --
    // Método para asignar caudal (C-C-OA-MET-DEF): 0 ==por persona, 1=total
    /// Caudal de aire primario mínimo por persona con máxima ocupación, m³/h
    /// (C-C-OA-FLOW/PER)
    /// Caudal de aire primario total, m³/h
    /// (C-C-OA-FLOW)
    pub oa_flow: Option<OutdoorAirFlow>,

    // --- Unidades terminales
    // -- Refrigeración --
    /// Potencia nominal total de refrigeración (sensible + latente) de la unidad terminal, kW
    /// Si no se define usa la disponible por el sistema
    /// Solo en sistemas zonales (UHT, UVT, FC, IU, HP, and PTAC)
    /// (C-C-COOL-CAP)
    pub cool_cap: Option<f32>,
    /// Potencia nominal sensible de refrigeración de la unidad terminal, kW
    /// Si no se define usa la disponible por el sistema
    /// (C-C-COOL-SH-CAP)
    pub cool_sh_cap: Option<f32>,

    // -- Calefacción --
    /// Potencia nominal de calefacción de la unidad terminal, kW
    /// Si no se define usa la disponible por el sistema
    /// Solo en sistemas zonales (UHT, UVT, FC, IU, HP, and PTAC)
    /// (C-C-HEAT-CAP)
    pub heat_cap: Option<f32>,
    // -- Calefacción auxiliar --
    // TODO: ?? Generar ejemplo para ver variables usadas
}

/// Definición del flujo de aire primario
#[derive(Debug, Clone, PartialEq)]
pub enum OutdoorAirFlow {
    /// Caudal de aire primario mínimo por persona con máxima ocupación, m³/h
    PerPerson(f32),
    /// Caudal de aire primario total, m³/h
    /// TODO: confirmar si es constante o con máxima ocupación
    Total(f32),
}

impl Default for OutdoorAirFlow {
    fn default() -> Self {
        OutdoorAirFlow::Total(0.0)
    }
}
