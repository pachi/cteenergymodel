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

mod gt_sys;
mod gt_types;
mod vyp_sys;
mod vyp_types;

pub use vyp_types::*;
pub use gt_types::*;

pub fn parse_systems(doc: &roxmltree::Document) -> (Vec<String>, Vec<System>) {
    let (factores_correccion_sistemas, sistemas) = vyp_sys::parse_systems(doc);
    let gt_systems = gt_sys::parse_systems(doc);

    // TODO: eliminar
    println!("Sistemas  GT:\n{:#?}", gt_systems);

    // TODO: eliminar
    println!("Sistemas VyP:\n{:#?}", sistemas);

    // TODO: completar sistemas GT
    (factores_correccion_sistemas, sistemas)
}
