// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Tipos de sistemas del .ctehexml - GT

// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105
// https://doe2.com/Download/DOE-23/DOE23Vol2-Dictionary_50h.pdf

/// Bomba de GT. En circuitos o equipos (como enfriadoras)
/// (PUMP)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtPump {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de control
    /// (CAP-CTRL)
    /// - ONE-SPEED-PUMP: velocidad constante*
    /// - ONE-SPEED-PUMP: dos velocidades
    /// - VAR-SPEED-PUMP : velocidad variable
    pub kind: String,
    /// Caudal, l/h
    /// (C-C-FLOW)
    pub flow: f32,
    /// Altura manométrica, m
    /// (HEAD)
    pub head: f32,
    // Otros parámetros menos habituales y curvas de comportamiento
}

/// Circuitos hidráulicos de GT
/// (CIRCULATION-LOOP)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtCirculationLoop {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// (TYPE)
    /// - PIPE2: Dos tubos (agua fría o agua caliente pero no simultáneamente)
    /// - DHW: ACS
    /// - CHW : Agua fría
    /// - LAKE / WELL? : Agua bruta (intercambio con el terreno)
    /// - HW : Agua caliente
    /// - WLHP : Bomba de calor circuito cerrado
    /// - CW : Circuito de agua de condensación
    pub kind: String,
    /// Temperatura de diseño para ACS
    /// Se supone igual al del horario de temperaturas del circuito
    /// Si es menor que este se hace mezcla con agua fría
    pub process_temperature: f32,
    /// Consigna de cal, ºC (por defecto 80)
    /// (HEAT-SETPT-T)
    pub heat_setpoint_temp: f32,
    /// Consigna de ref, ºC (por defecto 7)
    /// (COOL-SETPT-T)
    pub cool_setpoint_temp: f32,
    /// Temperatura del agua de red, ºC
    /// (DHW-INLET-T)
    pub dhw_inlet_temp: f32,
    /// Caudal máximo de ACS (a la temperatura de salida), l/h
    /// (C-C-PROCESS-FLOW)
    pub process_flow: f32,
    /// Bomba asociada a este circuito
    /// (LOOP-PUMP)
    pub loop_pump: String,
    // Horario de cal/ACS HEATING-SCHEDULE
    // pub heating_schedule: String,
    // Horario de ref COOLING-SCHEDULE
    // pub cooling_schedule: String,
    // Tipo de control T agua
    // C-C-LOOP-OPER-AF ==4 (Disponibilidad en función de horario), ==3 (Cambio estacional por temperatura + SNAP-T, Temperatura cambio estacional)
    // Subtipo: SUBTYPE primario, secundario
}

/// Planta enfriadora de GT
/// Puede incluir plantas enfriadoras reversibles tipo BdC y de otros tipos
/// (CHILLER)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtChiller {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// - ELEC-HERM-REC: compresor eléctrico
    /// - ELEC-HTREC: eléctrico con recuperación de calor
    /// - ABSOR-1: absorción simple etapa
    /// - ABSOR-2: absorción doble etapa
    /// - GAS-ABSOR: absorción por llama directa
    /// - ENGINE: motor combustión interna
    /// - HEAT-PUMP: bomba de calor 2T
    /// - LOOP-TO-LOOP-HP: bomba de calor 4T
    /// - otros de compresor eléctrico (ELEC-OPEN-CENT) y agua (WATER-ECONOMIZER)...
    pub kind: String,

    /// Capacidad nominal de refrigeración (C-C-CAPACITY), kW
    pub capacity_ref: f32,
    // Rendimiento en refrigeración, ERR (C-NUM-OF-UNITS)
    pub eer: f32,
    /// Capacidad nominal de calefacción en enfriadoras reversibles tipo BdC
    /// (C-DESIGN-KW), kW
    pub capacity_cal: f32,
    /// Rendimiento en calefacción para enfriadoras reversibles, COP (C-COP)
    pub cop: f32,

    /// Tipo de condensación para BdC y compresión eléctrica
    /// - AIR-COOLED: Por aire
    /// - WATER-COOLED: por agua
    /// - otros? REMOTE-AIR-COOLED, REMOTE-EVAP-COOLED
    pub condenser_type: String,

    // -- Conexiones a circuitos --
    // Circuito agua fría ---
    /// Circuito de agua enfriada que alimenta
    pub chw_loop: String,
    // Salto de temperatura, ºC (CHW-DT)
    pub chw_dt: f32,
    // -- Curvas comportamiento --
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
    pub kind: String,
    /// Subtipo
    /// (C-C-SUBTYPE)
    /// 1 - Convencional
    /// 2 - Baja temperatura
    /// 3 - Condensación
    /// 4 - Biomasa
    pub subkind: String,

    /// Potencia nominal (C-C-CAPACITY), kW
    pub capacity: f32,
    /// Rendimiento térmico, ratio
    /// En calderas de combustible
    /// (C-THERM-EFF-MAX || 0.85)
    pub eff: f32,
    /// Eficiencia eléctrica, nu
    /// En calderas eléctricas
    /// (C-AFUE)
    pub elec_eff: f32,

    // -- Conexiones a circuitos --
    // Circuito agua caliente ---
    /// Circuito de agua caliente que alimenta
    pub hw_loop: String,
    // Temperatura de consigna, ºC (45ºC)
    // ?
    // Salto de temperatura, ºC (HW-DT) (5ºC)
    pub hw_dt: f32,

    /// Tipo de combustible
    /// - Gas Natural*
    /// - Gasóleo
    /// - ...
    pub fuel_meter: String,

    /// Consumo nominal / consumo eléctrico, ratio (400)
    /// (C-C-KN)
    pub kn: f32, // -- Curvas comportamiento --
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
    pub kind: String,
    /// Combustible
    /// - GasNatural*
    /// - Gasóleo
    /// - ...
    /// (FUEL-METER)
    pub carrier: String,
    /// Circuito de ACS que alimenta
    /// (DHW-LOOP)
    pub dwh_loop: String,
    /// Capacidad nominal, kW
    /// C-C-CAPACITY
    pub capacity: f32,
    /// Rendimiento eléctrico, COP
    /// en BdC y calderas eléctricas
    /// (C-STBY-LOSS-FRAC)
    pub cop: f32,
    /// Eficiencia térmica, nu
    /// En calderas de combustible
    /// (C-ENERGY-FACTOR)
    pub thermal_eff: f32,
    /// Presencia de depósito
    /// (C-CATEGORY)
    /// 0 - nada*
    /// 1 - con
    /// 2 - sin
    // Solo con depósito si es == 1
    /// Volumen del depósito de acumulación, l
    /// (TANK-VOLUME)
    pub tank_volume: f32,
    /// Pérdidas térmicas del depósito de acumulación, W/K
    /// (TANK-UA)
    pub tank_ua: f32,
    // TODO: HP-SUPP-CAPACITY=0 - capacidad del sistema de respaldo en BdC, por defecto es igual que capacity
    // C-C-SUBTYPE -> 2 = Tiene panel solar,
    // C-C-AREA-PS      = 10 superficie de paneles solares, m²
    // C-C-PORC-PS      = 30 % de demanda cubierta, %
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
#[derive(Debug, Default, Clone, PartialEq)]
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
