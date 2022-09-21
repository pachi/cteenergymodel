// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Tipos de sistemas del .ctehexml

// TODO: ¿Separar acumuladores de generadores en equipos... llevándolo a otro atributo de los sistemas?
// TODO: Revisar otros tipos de equipos (PV, bombas, ventiladores, etc)
// TODO: Pensar otros componentes como circuitos y distribución
// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105

/// Sistemas técnicos de climatización, ACS y ventilación
#[derive(Debug, Clone, PartialEq)]
pub enum VypSystem {
    /// Sistema sólo de ACS
    /// (sin calefacción, sin refrigeración, sin ventilación)
    Dhw {
        /// Nombre
        name: String,
        /// Lista de equipos
        equipment: Vec<GenerationEquipment>,
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
        equipment: Vec<GenerationEquipment>,
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
        equipment: Vec<GenerationEquipment>,
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
    /// (sin ACS, recuperación de calor / freecooling opcionales)
    MultizoneAir {
        /// Nombre
        name: String,
        /// Lista de equipos
        equipment: Vec<GenerationEquipment>,
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
    /// Sistema de generación solar térmica
    SolarThermalGenerator(SolarThermalGenerator),
    /// Sistema de generación solar fotovoltaica
    PhotovoltaicGenerator(PhotovoltaicGenerator),
    /// Sistema de generación solar térmica
    WindGenerator(WindGenerator),
    /// Sistema de generación solar térmica
    CHPGenerator(CHPGenerator),
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
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum EconomizerControl {
    /// Temperatura
    #[default]
    Temperature,
    /// Entalpía
    Enthalpy,
    /// Temperatura y entalpía
    TemperatureEnthalpy,
    /// Desconocido
    Unknown,
}

/// Tipos de sistemas de generación solar térmica
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum SolarThermalKind {
    ///PlanoSelectivo
    #[default]
    FlatPlateSelective,
    /// PlanoNoSelectivo
    FlatPlateNonSelective,
    /// TubosDeVacio, ETC
    EvacuatedTube,
    /// Termodinámico
    Thermodynamic,
    /// Colector de aire
    AirCollector,
    /// Otro
    Other,
}

/// Sistema de generación solar térmica
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SolarThermalGenerator {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de colector usado
    pub kind: SolarThermalKind,
    // (PotenciaNominal(0,7kW/m² guía IDAE)?),
    // potencia_nominal: f32,
    /// RendimientoOptico [-]
    pub optical_performance: f32,
    /// CoeficientePerdidas [W/K·m²]
    pub losses_coeff: f32,
    /// SuperficieApertura, [m²]
    pub surface: f32,
    /// Orientación (...)
    pub orientation: f32,
    /// Inclinación (....)
    pub tilt: f32,
    /// Volumen de acumulación (l)
    pub storage_capacity: f32,
    /// PerdidasPct (%, por sombras)
    pub losses_pct: f32,
}

/// Sistema de generación solar fotovoltaica
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PhotovoltaicGenerator {
    /// Nombre / descripción
    pub name: String,
    /// PotenciaNominal (kW, pico)
    pub capacity: f32,
    /// Superficie, m²
    pub surface: f32,
    /// Orientación
    pub orientation: f32,
    /// Inclinación
    pub tilt: f32,
    /// Capacidad nominal de acumulación, C_20 (Ah)
    pub storage_capacity: f32,
    /// Pérdidas (%)
    pub losses_pct: f32,
}

/// Sistema de generación eólica
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WindGenerator {
    /// Nombre / descripción
    pub name: String,
    /// Potencia máxima, kW
    pub capacity: f32,
    /// Velocidad de viento de arranque, m/s
    pub wind_speed_start: f32,
    /// Velocidad de viento de parada, m/s
    pub wind_speed_stop: f32,
}

/// Sistema de cogeneración
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CHPGenerator {
    /// Nombre / descripción
    pub name: String,
    // TODO: Ver qué parámetros necesitamos
    // Tipo de cogenerador: CogenDiesel, CogenFuelCell, CogenGasTurbine
    // kind: String
    // Combustible
    // fuel: String,
    // Potencia eléctrica máxima, kW
    // capacity: f32,
    // Eficiencia térmica, -
    // efficiency_th: f32,
    // Eficiencia eléctrica, -
    // efficiency_el: f32,
    // curves: xxx
}

// Equipos ------------------------------------------------------------
//

/// Parámetros de rendimiento de calefacción
#[derive(Debug, Clone, PartialEq)]
pub struct HeatingParams {
    /// Vector energético para generar calor
    pub fuel: String,
    /// Capacidad calorífica máxima nominal (kW)
    /// En equipos ideales se recomienda 9_999_999_999.99 (1.0e11 - 0.01 > 1e10)
    pub capacity: f32,
    /// Rendimiento nominal (-)
    /// Relación entre la capacidad nominal y el consumo nominal
    pub efficiency: f32,
}

/// Parámetros de rendimiento de refrigeración
#[derive(Debug, Clone, PartialEq)]
pub struct CoolingParams {
    /// Vector energético para generar frío
    pub fuel: String,
    /// Capacidad total refrigeración nominal (kW)
    /// En equipos ideales se recomienda 9_999_999_999.99 (1.0e11 - 0.01 > 1e10)
    pub capacity: f32,
    /// Rendimiento nominal (kW)
    /// Relación entre la capacidad nominal y el consumo nominal
    pub efficiency: f32,
    /// Fracción de capacidad sensible de refrigeración respecto a la capacidad total (-)
    /// En unidades aire-agua o aire-refrigerante toma valor 1.0
    pub shr: f32,
}

/// Tipos de equipos
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum EquipmentKind {
    #[default]
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

/// Equipos primarios y de generación
#[derive(Debug, Clone, PartialEq)]
pub enum GenerationEquipment {
    /// Boiler, Hot water or electric baseboard heating system
    /// Air to air, air to refrigerant or air to water heat pump or dx system
    /// Ideal (constant efficiency) heating and/or cooling system
    ThermalGenerator(ThermalGenerator),
    /// Hot Water Generation
    /// TODO: Debería esto estar en otro lado?
    HotWaterStorageTank(HotWaterStorageTank),
}

/// Boiler, Hot water or electric baseboard heating system
/// Air to air, air to refrigerant or air to water heat pump or dx system
/// Ideal (constant efficiency) heating and/or cooling system
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ThermalGenerator {
    /// Nombre / descripción
    pub name: String,
    /// Tipo
    /// Calderas: Convencional, Electrica, BajaTemperatura, Condensación, Biomasa, ACS-Electrica, ACS-Convencional
    /// Calefacción eléctrica: CalefaccionElectrica
    /// Sistema ideal de rendimiento constante: EQ_RendimientoCte
    /// Sistemas aire-aire, aire-refrigerante, aire-agua o expansión directa
    /// Aire-aire: ExpansionDirectaAireAireSf, ExpansionDirectaAireAireBdc,
    /// Aire-fluido: EQ_ED_AireAgua_BDC, EQ_ED_UnidadExterior
    pub kind: EquipmentKind,
    /// Parámetros de la generación de calor
    pub heating: Option<HeatingParams>,
    /// Parámetros de la generación de frío
    pub cooling: Option<CoolingParams>,
    /// Caudal de aire de impulsión nominal (m³/h)
    /// Solo en equipos aire-aire
    pub supply_air_flow: Option<f32>,
    /// Multiplicador
    pub multiplier: u32,
    /// Curvas de rendimiento
    pub curves: Vec<(String, String)>,
}

/// Hot Water Storage
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HotWaterStorageTank {
    /// Nombre
    pub name: String,
    /// Tipo: AcumuladorAguaCaliente
    pub kind: EquipmentKind,
    /// Volumen, m³
    pub volume: f32,
    /// Coeficiente de pérdidas global del depósito, UA (W/ºC)
    pub ua: f32,
    /// Temperatura de consigna baja del depósito (ºC=80) (tConsignaBaja)
    pub temp_low: f32,
    /// Temperatura de consigna alta del depósito (ºC=60) (tConsignaAlta)
    pub temp_high: f32,
    /// Temperatura de entrada del agua de red (temperaturaEntrada = según climas)
    pub input_temp: f32,
    /// Temperatura del ambiente exterior (temperaturaAmbiente = 25ºC)
    pub space_temp: f32,
}

/// Demanda de ACS
/// XXX: Esto es más bien un perfil/carga y no tanto un sistema
#[derive(Debug, Default, Clone, PartialEq)]
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
    /// XXX: Se podría unificar con AirTerminalUnit usando valores nulos de capacidad?
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
