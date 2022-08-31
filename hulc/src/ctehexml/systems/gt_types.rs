// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Tipos de sistemas del .ctehexml - GT

// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105
// https://doe2.com/Download/DOE-23/DOE23Vol2-Dictionary_50h.pdf


/// Sistema de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GtSystem {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// - FC: Fancoil
    pub kind: String,
    /// Circuito de agua caliente que alimenta el sistema
    /// Es el circuito por defecto para zonas salvo que se indique
    pub hw_loop: Option<String>,
    /// Circuito de agua caliente que alimenta las unidades de zona
    pub zone_hw_loop: Option<String>,
    /// Circuito de agua enfriada que alimenta el sistema
    /// Es el circuito por defecto para zonas salvo que se indique
    pub chw_loop: Option<String>,
    /// Circuito de agua enfriada que alimenta las unidades de zona
    pub zone_chw_loop: Option<String>,
    /// Horario de funcionamiento de los ventiladores
    pub fan_schedule: String,
    // TODO: faltan capacidades de cal / ref y flujos (C-C-COOL-CAP, C-C-HEAT-CAP, ...)
}

/// Calderas de ACS de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DwHeater {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// - HEAT-PUMP: bomba de calor
    pub kind: String,
    /// Circuito de ACS que alimenta
    pub dwh_loop: String,
    // Pérdidas en standby
    // stby_loss_frac
    /// Capacidad nominal
    pub capacity: f32,
    // TODO: HP-SUPP-CAPACITY=0 - capacidad del sistema de respaldo en BdC, por defecto es igual que capacity
}


/// Enfriadora de GT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Chiller {
    /// Nombre / descripción
    pub name: String,
    /// Tipo de sistema
    /// - HEAT-PUMP: bomba de calor
    pub kind: String,
    /// Circuito de agua enfriada que alimenta
    pub chw_loop: String,
    /// Tipo de condensador
    pub condenser_type: String,
    /// Capacidad nominal
    pub capacity: f32,
    /// Capacidad de diseño ??? (igual a capacity)
    pub design_kw: f32,
    /// Rendimiento
    pub c_cop: f32,
    // Número de unidades ???
    // pub num_of_units: f32 
    // TODO: HP-SUPP-CAPACITY=0 - capacidad del sistema de respaldo en BdC, por defecto es igual que capacity
}

// TODO: circuitos
