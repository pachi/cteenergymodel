use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::parsers::bdl::{Frame, Glass, Material, MaterialProperties, WallCons, WindowCons, DB};

pub static BDCATALOG: Lazy<DB> = Lazy::new(|| DB {
    materials: hashmap! {
    "Bloque de hormigon AL-P 250 mm".into() => Material {
        name: "Bloque de hormigon AL-P 250 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.83)
    },
    "cnv_ver_10".into() => Material {
        name: "cnv_ver_10".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.19)
    },
    "Bloque de hormigon AL-P 290 mm".into() => Material {
    name: "Bloque de hormigon AL-P 290 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.95)
    },
     "FReps400_mold".into() => Material {
    name: "FReps400_mold".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 0.43956, density: 992.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "arenisca".into() => Material {
    name: "arenisca".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.0, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "Tableros de fibras incluyendo MDF 200 < d < 350".into() => Material {
    name: "Tableros de fibras incluyendo MDF 200 < d < 350".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 275.0, specificheat: 1700.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Aluminio".into() => Material {
    name: "Aluminio".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 230.0, density: 2700.0, specificheat: 880.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BHalm170".into() => Material {
    name: "BHalm170".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.17), conductivity: 0.30909, density: 1450.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "tejaPor".into() => Material {
    name: "tejaPor".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 2300.0, specificheat: 840.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "Hoal800".into() => Material {
    name: "Hoal800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.65, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "PYE".into() => Material {
    name: "PYE".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 825.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "FRho300".into() => Material {
    name: "FRho300".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 2.0, density: 1285.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Con capa de compresión -Canto 400 mm".into() => Material {
    name: "Con capa de compresión -Canto 400 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.8, density: 1320.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "PVC".into() => Material {
    name: "PVC".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1390.0, specificheat: 900.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "Espuma elastomérica-flexible".into() => Material {
    name: "Espuma elastomérica-flexible".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 70.0, specificheat: 1500.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "HC2100".into() => Material {
    name: "HC2100".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.44, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "HAuto800".into() => Material {
    name: "HAuto800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "losaAlv350cc".into() => Material {
    name: "losaAlv350cc".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.667, density: 1440.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1500".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.52, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FR Entrevigado de EPS mecanizado enrasado -Canto 450 mm".into() => Material {
    name: "FR Entrevigado de EPS mecanizado enrasado -Canto 450 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 0.769, density: 1360.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Asfalto arenoso".into() => Material {
    name: "Asfalto arenoso".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "Hormigón convencional d 2400".into() => Material {
    name: "Hormigón convencional d 2400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.9, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "mCl".into() => Material {
    name: "mCl".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 390.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "tc2000".into() => Material {
    name: "tc2000".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 1.2, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "Bloque de picón de 80 mm".into() => Material {
    name: "Bloque de picón de 80 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.19)
    },
     "Hormigón con otros áridos ligeros d 700".into() => Material {
    name: "Hormigón con otros áridos ligeros d 700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.74, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Poliamida [nylon] [PA]".into() => Material {
    name: "Poliamida [nylon] [PA]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1150.0, specificheat: 1600.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into() => Material {
    name: "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.04), conductivity: 0.228, density: 670.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BPt250".into() => Material {
    name: "BPt250".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.5, density: 1000.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FR Entrevigado de hormigón -Canto 400 mm".into() => Material {
    name: "FR Entrevigado de hormigón -Canto 400 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 2.043, density: 1570.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Balsa d < 200".into() => Material {
    name: "Balsa d < 200".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.057, density: 180.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FUhal250".into() => Material {
    name: "FUhal250".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.0, density: 1230.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FUhal350_d12".into() => Material {
    name: "FUhal350_d12".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.296, density: 990.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Paneles de fibras con conglomerante hidráulico 250 < d < 350".into() => Material {
    name: "Paneles de fibras con conglomerante hidráulico 250 < d < 350".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 300.0, specificheat: 1700.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "FUhal300".into() => Material {
    name: "FUhal300".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.111, density: 1140.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "BH aligerado hueco espesor 300 mm".into() => Material {
    name: "BH aligerado hueco espesor 300 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.454, density: 1050.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "losaAlv400".into() => Material {
    name: "losaAlv400".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.818, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Caliza dura [2000 < d < 2190]".into() => Material {
    name: "Caliza dura [2000 < d < 2190]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.7, density: 2095.0, specificheat: 1000.0, vapourdiffusivity: Some(150.0) }), resistance: None
    },
     "PURhfc_per_030".into() => Material {
    name: "PURhfc_per_030".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.03, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "aceroInox".into() => Material {
    name: "aceroInox".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 17.0, density: 7900.0, specificheat: 460.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Polipropileno [PP]".into() => Material {
    name: "Polipropileno [PP]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.22, density: 910.0, specificheat: 1800.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FRhal400_d12".into() => Material {
    name: "FRhal400_d12".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.90476, density: 1115.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Yeso de alta dureza 900 < d < 1200".into() => Material {
    name: "Yeso de alta dureza 900 < d < 1200".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.43, density: 1050.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "laton".into() => Material {
    name: "laton".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 120.0, density: 8400.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "losaAlv300".into() => Material {
    name: "losaAlv300".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.579, density: 1290.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Asperón [1900 < d < 2500]".into() => Material {
    name: "Asperón [1900 < d < 2500]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.8, density: 2200.0, specificheat: 1000.0, vapourdiffusivity: Some(40.0) }), resistance: None
    },
     "Material_Base".into() => Material {
    name: "Material_Base".into(), group: "Materiales".into(), properties: Some(MaterialProperties { thickness: Some(0.075), conductivity: 1.0, density: 3000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "cchapdo350".into() => Material {
    name: "cchapdo350".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.11, density: 300.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "titanio".into() => Material {
    name: "titanio".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 21.9, density: 4500.0, specificheat: 522.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "MW_05".into() => Material {
    name: "MW_05".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 40.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "asfal".into() => Material {
    name: "asfal".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.7, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "Caliza blanda [1600 < d < 1790]".into() => Material {
    name: "Caliza blanda [1600 < d < 1790]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.1, density: 1695.0, specificheat: 1000.0, vapourdiffusivity: Some(25.0) }), resistance: None
    },
     "HMlt2300".into() => Material {
    name: "HMlt2300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.65, density: 2150.0, specificheat: 1000.0, vapourdiffusivity: Some(70.0) }), resistance: None
    },
     "BH aligerado macizo espesor 300 mm".into() => Material {
    name: "BH aligerado macizo espesor 300 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.317, density: 860.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "BH aligerado macizo espesor 200 mm".into() => Material {
    name: "BH aligerado macizo espesor 200 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 0.287, density: 840.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada vertical 10 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada vertical 10 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.095)
    },
     "FR Entrevigado de hormigón aligerado - Canto 450 mm".into() => Material {
    name: "FR Entrevigado de hormigón aligerado - Canto 450 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 1.986, density: 1455.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "HC2200".into() => Material {
    name: "HC2200".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.57, density: 2200.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "PTFE".into() => Material {
    name: "PTFE".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 2200.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "MORgt1800".into() => Material {
    name: "MORgt1800".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 1900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHad80".into() => Material {
    name: "BHad80".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.08), conductivity: 0.8, density: 1514.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHalp80".into() => Material {
    name: "BHalp80".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.08), conductivity: 0.17778, density: 1220.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "tc1000".into() => Material {
    name: "tc1000".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.4, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "clv_ver_10".into() => Material {
    name: "clv_ver_10".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.095)
    },
     "Tablero cerámico".into() => Material {
    name: "Tablero cerámico".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.18)
    },
     "XPS Expandido con hidrofluorcarbonos HFC [ 0.025 W/[mK]]".into() => Material {
    name: "XPS Expandido con hidrofluorcarbonos HFC [ 0.025 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.025, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "Hormigón con arcilla expandida sin otros áridos d 700".into() => Material {
    name: "Hormigón con arcilla expandida sin otros áridos d 700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.22, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "espElast".into() => Material {
    name: "espElast".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 70.0, specificheat: 1500.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 1000".into() => Material {
    name: "Hormigón celular curado en autoclave d 1000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.29, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Cuarzo".into() => Material {
    name: "Cuarzo".into(), group: "Vidrios".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.4, density: 2200.0, specificheat: 750.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BCais190".into() => Material {
    name: "BCais190".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.19), conductivity: 0.30159, density: 910.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Cámara de aire sin ventilar horizontal 2 cm".into() => Material {
    name: "Cámara de aire sin ventilar horizontal 2 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.16)
    },
     "FUhal400".into() => Material {
    name: "FUhal400".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.29, density: 1030.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "HC2400".into() => Material {
    name: "HC2400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.9, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada vertical 1 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada vertical 1 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.075)
    },
     "BC con mortero convencional espesor 290 mm".into() => Material {
    name: "BC con mortero convencional espesor 290 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.421, density: 1080.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Plomo".into() => Material {
    name: "Plomo".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 35.0, density: 11300.0, specificheat: 130.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Mortero de yeso".into() => Material {
    name: "Mortero de yeso".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.8, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Subcapa fieltro".into() => Material {
    name: "Subcapa fieltro".into(), group: "Textiles".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 120.0, specificheat: 1300.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "polisulfuro".into() => Material {
    name: "polisulfuro".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.4, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Poliestireno [PS]".into() => Material {
    name: "Poliestireno [PS]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.16, density: 1050.0, specificheat: 1300.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "losaAlv400cc".into() => Material {
    name: "losaAlv400cc".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.818, density: 1320.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "1 pie LM métrico o catalán 40 mm< G < 50 mm".into() => Material {
    name: "1 pie LM métrico o catalán 40 mm< G < 50 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 1.03, density: 2140.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "clv_hor_5".into() => Material {
    name: "clv_hor_5".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.08)
    },
     "BC con mortero aislante espesor 240 mm".into() => Material {
    name: "BC con mortero aislante espesor 240 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.298, density: 920.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Conífera muy pesada d >610".into() => Material {
    name: "Conífera muy pesada d >610".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 620.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "HAuto1000".into() => Material {
    name: "HAuto1000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.29, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "tabFib350".into() => Material {
    name: "tabFib350".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 275.0, specificheat: 1700.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Teja plástico".into() => Material {
    name: "Teja plástico".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Sodocálcico [inc. Vidrio flotado]".into() => Material {
    name: "Sodocálcico [inc. Vidrio flotado]".into(), group: "Vidrios".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 2500.0, specificheat: 750.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "PURhfc".into() => Material {
    name: "PURhfc".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.028, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(105.0) }), resistance: None
    },
     "BHalp240".into() => Material {
    name: "BHalp240".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.28916, density: 850.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "PUR Plancha con HFC o Pentano y rev. impermeable a gases [ 0.025 W/[mK]]".into() => Material {
    name: "PUR Plancha con HFC o Pentano y rev. impermeable a gases [ 0.025 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.025, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FReps450_mold".into() => Material {
    name: "FReps450_mold".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 0.47872, density: 985.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Hoal1300".into() => Material {
    name: "Hoal1300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.42, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "LHGFt".into() => Material {
    name: "LHGFt".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.105), conductivity: 0.219, density: 620.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Gres cuarzoso 2600 < d < 2800".into() => Material {
    name: "Gres cuarzoso 2600 < d < 2800".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.6, density: 2700.0, specificheat: 1000.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "Haep1700".into() => Material {
    name: "Haep1700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.76, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "losaAlv350".into() => Material {
    name: "losaAlv350".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.667, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "sliex".into() => Material {
    name: "sliex".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.6, density: 2700.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "neopreno".into() => Material {
    name: "neopreno".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 1240.0, specificheat: 2140.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 2000".into() => Material {
    name: "Hormigón con otros áridos ligeros d 2000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.5, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Corcho Comprimido ".into() => Material {
    name: "Corcho Comprimido ".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 450.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "LPmp_90".into() => Material {
    name: "LPmp_90".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1225), conductivity: 0.533, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "losaMa350".into() => Material {
    name: "losaMa350".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 2.5, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "LHorADper".into() => Material {
    name: "LHorADper".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.12), conductivity: 1.09091, density: 1258.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BC190".into() => Material {
    name: "BC190".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.19), conductivity: 0.43182, density: 1080.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Latón".into() => Material {
    name: "Latón".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 120.0, density: 8400.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Placas de corcho".into() => Material {
    name: "Placas de corcho".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.065, density: 450.0, specificheat: 1500.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "acero".into() => Material {
    name: "acero".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 50.0, density: 7800.0, specificheat: 450.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FReps250_mold".into() => Material {
    name: "FReps250_mold".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.30488, density: 1140.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Hormigón convencional d 1700".into() => Material {
    name: "Hormigón convencional d 1700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.03, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Arcilla Expandida [árido suelto]".into() => Material {
    name: "Arcilla Expandida [árido suelto]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.148, density: 537.5, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Frondosa pesada 750 < d < 870".into() => Material {
    name: "Frondosa pesada 750 < d < 870".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 775.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "Silicona pura".into() => Material {
    name: "Silicona pura".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.35, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(5000.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 300".into() => Material {
    name: "Hormigón celular curado en autoclave d 300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.09, density: 300.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "mCp".into() => Material {
    name: "mCp".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 565.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Cámara de aire sin ventilar horizontal 1 cm".into() => Material {
    name: "Cámara de aire sin ventilar horizontal 1 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.15)
    },
     "Hoal1500".into() => Material {
    name: "Hoal1500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.52, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "HAuto400".into() => Material {
    name: "HAuto400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 400.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "PMMA".into() => Material {
    name: "PMMA".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 1180.0, specificheat: 1500.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "tabFib1000".into() => Material {
    name: "tabFib1000".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 875.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "butadieno".into() => Material {
    name: "butadieno".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 980.0, specificheat: 1000.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "BC290".into() => Material {
    name: "BC290".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.42647, density: 1080.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHad190".into() => Material {
    name: "BHad190".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.19), conductivity: 0.86364, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUho300".into() => Material {
    name: "FUho300".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.42857, density: 1240.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "FRsin250".into() => Material {
    name: "FRsin250".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 4.16667, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Hoal900".into() => Material {
    name: "Hoal900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.27, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Silicona masilla".into() => Material {
    name: "Silicona masilla".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.5, density: 1450.0, specificheat: 1000.0, vapourdiffusivity: Some(5000.0) }), resistance: None
    },
     "tierraVegetal".into() => Material {
    name: "tierraVegetal".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.52, density: 2050.0, specificheat: 1840.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Piedra pómez natural [d < 400]".into() => Material {
    name: "Piedra pómez natural [d < 400]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 390.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "PUR_inyec".into() => Material {
    name: "PUR_inyec".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.04, density: 17.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "XPS_CO2_038".into() => Material {
    name: "XPS_CO2_038".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.038, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "tabFib550".into() => Material {
    name: "tabFib550".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.14, density: 450.0, specificheat: 1700.0, vapourdiffusivity: Some(12.0) }), resistance: None
    },
     "PP".into() => Material {
    name: "PP".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.22, density: 910.0, specificheat: 1800.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "calizaD".into() => Material {
    name: "calizaD".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.7, density: 2095.0, specificheat: 1000.0, vapourdiffusivity: Some(150.0) }), resistance: None
    },
     "cchapdo750".into() => Material {
    name: "cchapdo750".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.21, density: 675.0, specificheat: 1600.0, vapourdiffusivity: Some(110.0) }), resistance: None
    },
     "FR Entrevigado de hormigón aligerado -Canto 400 mm ".into() => Material {
    name: "FR Entrevigado de hormigón aligerado -Canto 400 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.936, density: 1480.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "silicona_mas".into() => Material {
    name: "silicona_mas".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.5, density: 1450.0, specificheat: 1000.0, vapourdiffusivity: Some(5000.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado descolgado -Canto 400 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado descolgado -Canto 400 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 0.43956044, density: 1290.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into() => Material {
    name: "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.115), conductivity: 0.667, density: 1140.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Bloque de hormigon AD 200 mm".into() => Material {
    name: "Bloque de hormigon AD 200 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.22)
    },
     "Paneles de fibras con conglomerante hidráulico 350 < d < 450".into() => Material {
    name: "Paneles de fibras con conglomerante hidráulico 350 < d < 450".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 400.0, specificheat: 1700.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Politetrafluoretileno [PTFE]".into() => Material {
    name: "Politetrafluoretileno [PTFE]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 2200.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "BH aligerado macizo espesor 250 mm".into() => Material {
    name: "BH aligerado macizo espesor 250 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.3, density: 850.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hierro".into() => Material {
    name: "Hierro".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 72.0, density: 7870.0, specificheat: 450.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Hormigón convencional d 2200".into() => Material {
    name: "Hormigón convencional d 2200".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.57, density: 2200.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Tablero de partículas 180 < d < 270".into() => Material {
    name: "Tablero de partículas 180 < d < 270".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 225.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Tableros de fibras incluyendo MDF 750 < d < 1000".into() => Material {
    name: "Tableros de fibras incluyendo MDF 750 < d < 1000".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 875.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "losaMa400".into() => Material {
    name: "losaMa400".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 2.5, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "tejaHor".into() => Material {
    name: "tejaHor".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.5, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "enlYlq1000".into() => Material {
    name: "enlYlq1000".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.4, density: 850.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "LDPE".into() => Material {
    name: "LDPE".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.33, density: 920.0, specificheat: 2200.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "enlYAlq900".into() => Material {
    name: "enlYAlq900".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "poliisobutileno".into() => Material {
    name: "poliisobutileno".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 930.0, specificheat: 1100.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "BHad290".into() => Material {
    name: "BHad290".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 1.11538, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1200".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1200".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.37, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FRsin350".into() => Material {
    name: "FRsin350".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 4.375, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "PUR Proyección con CO2 celda cerrada [ 0.035 W/[mK]]".into() => Material {
    name: "PUR Proyección con CO2 celda cerrada [ 0.035 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.035, density: 50.0, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "Enlucido de yeso d < 1000".into() => Material {
    name: "Enlucido de yeso d < 1000".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.4, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Frondosa de peso medio 565 < d < 750".into() => Material {
    name: "Frondosa de peso medio 565 < d < 750".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 660.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "mB".into() => Material {
    name: "mB".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.057, density: 200.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Tabicón de LH triple Gran Formato 100 mm < E < 110 mm".into() => Material {
    name: "Tabicón de LH triple Gran Formato 100 mm < E < 110 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1), conductivity: 0.206, density: 620.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Tablero de partículas con cemento d < 1200".into() => Material {
    name: "Tablero de partículas con cemento d < 1200".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 1200.0, specificheat: 1500.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "BHalm290".into() => Material {
    name: "BHalm290".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.30526, density: 1050.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Frondosa muy ligera 200 < d < 435".into() => Material {
    name: "Frondosa muy ligera 200 < d < 435".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 320.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "FU Entrevigado de EPS mecanizado enrasado -Canto 350 mm".into() => Material {
    name: "FU Entrevigado de EPS mecanizado enrasado -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.255, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 0.4787234, density: 1280.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into() => Material {
    name: "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.04), conductivity: 0.445, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "HAuto500".into() => Material {
    name: "HAuto500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.14, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "butilo".into() => Material {
    name: "butilo".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.24, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(200000.0) }), resistance: None
    },
     "PUR Inyección en tabiquería con dióxido de carbono CO2".into() => Material {
    name: "PUR Inyección en tabiquería con dióxido de carbono CO2".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.04, density: 17.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "LHd".into() => Material {
    name: "LHd".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.075), conductivity: 0.469, density: 930.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 1000 < d < 1250".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 1000 < d < 1250".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.55, density: 1125.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hae500".into() => Material {
    name: "Hae500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.16, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "EPS Poliestireno Expandido [ 0.037 W/[mK]]".into() => Material {
    name: "EPS Poliestireno Expandido [ 0.037 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.0375, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "cchapdo600".into() => Material {
    name: "cchapdo600".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 550.0, specificheat: 1600.0, vapourdiffusivity: Some(90.0) }), resistance: None
    },
     "PU".into() => Material {
    name: "PU".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1200.0, specificheat: 1800.0, vapourdiffusivity: Some(6000.0) }), resistance: None
    },
     "Hae700".into() => Material {
    name: "Hae700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.22, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "LHorADmac".into() => Material {
    name: "LHorADmac".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.12), conductivity: 1.71429, density: 1800.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Espuma de poliuretano [PU]".into() => Material {
    name: "Espuma de poliuretano [PU]".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 70.0, specificheat: 1500.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Hormigón con áridos ligeros 1600 < d < 1800".into() => Material {
    name: "Hormigón con áridos ligeros 1600 < d < 1800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.15, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "hierro".into() => Material {
    name: "hierro".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 72.0, density: 7870.0, specificheat: 450.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BHalm140".into() => Material {
    name: "BHalm140".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.175, density: 1134.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "PUR Proyección con CO2 celda cerrada [ 0.032 W/[mK]]".into() => Material {
    name: "PUR Proyección con CO2 celda cerrada [ 0.032 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.032, density: 50.0, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "FUeps300_mold".into() => Material {
    name: "FUeps300_mold".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.2, density: 670.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FReps450_mec".into() => Material {
    name: "FReps450_mec".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 1.323, density: 1046.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FReps350_mold".into() => Material {
    name: "FReps350_mold".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.4023, density: 1008.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "losaAlv200".into() => Material {
    name: "losaAlv200".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 1.42857, density: 1410.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Panel de vidrio celular [CG]".into() => Material {
    name: "Panel de vidrio celular [CG]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 125.0, specificheat: 1000.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "clv_ver_5".into() => Material {
    name: "clv_ver_5".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.09)
    },
     "Hormigón con arcilla expandida como árido principal d 1100".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1100".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.39, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "LHt".into() => Material {
    name: "LHt".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.105), conductivity: 0.456, density: 920.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUc350".into() => Material {
    name: "FUc350".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.0, density: 1030.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "XPS Expandido con hidrofluorcarbonos HFC [ 0.032 W/[mK]]".into() => Material {
    name: "XPS Expandido con hidrofluorcarbonos HFC [ 0.032 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.032, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.8, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Polietileno baja densidad [LDPE]".into() => Material {
    name: "Polietileno baja densidad [LDPE]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.33, density: 920.0, specificheat: 2200.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada vertical 2 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada vertical 2 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.085)
    },
     "acrilicos".into() => Material {
    name: "acrilicos".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1050.0, specificheat: 1500.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FUeps300_mole".into() => Material {
    name: "FUeps300_mole".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.34091, density: 740.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "clv_ver_2".into() => Material {
    name: "clv_ver_2".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.085)
    },
     "HC1600".into() => Material {
    name: "HC1600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.97, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "poliacetato".into() => Material {
    name: "poliacetato".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1410.0, specificheat: 1400.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "Hormigón convencional d 2100".into() => Material {
    name: "Hormigón convencional d 2100".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.44, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Butadieno".into() => Material {
    name: "Butadieno".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 980.0, specificheat: 1000.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "BH convencional espesor 200 mm".into() => Material {
    name: "BH convencional espesor 200 mm".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 0.923, density: 860.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "PUR Proyección con Hidrofluorcarbono HFC [ 0.028 W/[mK]]".into() => Material {
    name: "PUR Proyección con Hidrofluorcarbono HFC [ 0.028 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.028, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "tabFib750".into() => Material {
    name: "tabFib750".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 650.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Tablero contrachapado 500 < d < 600".into() => Material {
    name: "Tablero contrachapado 500 < d < 600".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 550.0, specificheat: 1600.0, vapourdiffusivity: Some(90.0) }), resistance: None
    },
     "losaAl400".into() => Material {
    name: "losaAl400".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.66667, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "FRho450".into() => Material {
    name: "FRho450".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 2.045, density: 1185.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Vidrio prensado".into() => Material {
    name: "Vidrio prensado".into(), group: "Vidrios".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.2, density: 2000.0, specificheat: 750.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "mCpm".into() => Material {
    name: "mCpm".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 477.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FReps250_mec".into() => Material {
    name: "FReps250_mec".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.19048, density: 1280.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "gres_cal".into() => Material {
    name: "gres_cal".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.9, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "LPmp_70".into() => Material {
    name: "LPmp_70".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1225), conductivity: 0.583, density: 1020.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "cnv_hor_10".into() => Material {
    name: "cnv_hor_10".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.18)
    },
     "tc1200".into() => Material {
    name: "tc1200".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.5, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "FUeps350_mold".into() => Material {
    name: "FUeps350_mold".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.22293, density: 640.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "HC2300".into() => Material {
    name: "HC2300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.72, density: 2300.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Estaño".into() => Material {
    name: "Estaño".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 66.6, density: 7310.0, specificheat: 227.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Corcho Expandido puro 100 < d < 150".into() => Material {
    name: "Corcho Expandido puro 100 < d < 150".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.049, density: 125.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 800".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.27, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FU Entrevigado de hormigón -Canto 300 mm".into() => Material {
    name: "FU Entrevigado de hormigón -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.422, density: 1240.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "gres_cuar".into() => Material {
    name: "gres_cuar".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.6, density: 2700.0, specificheat: 1000.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "BHalp140".into() => Material {
    name: "BHalp140".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.20588, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Caliza dureza media [1800 < d < 1990]".into() => Material {
    name: "Caliza dureza media [1800 < d < 1990]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.4, density: 1895.0, specificheat: 1000.0, vapourdiffusivity: Some(40.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1300".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.5, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Resina poliéster no saturado [UP]".into() => Material {
    name: "Resina poliéster no saturado [UP]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.19, density: 1400.0, specificheat: 1200.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FRc300".into() => Material {
    name: "FRc300".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.667, density: 1215.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Frondosa ligera 435 < d < 565".into() => Material {
    name: "Frondosa ligera 435 < d < 565".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 500.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "Hoal700".into() => Material {
    name: "Hoal700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.74, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Sin capa de compresión -Canto 300 mm".into() => Material {
    name: "Sin capa de compresión -Canto 300 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.618, density: 1290.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "cauchoCel".into() => Material {
    name: "cauchoCel".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.06, density: 70.0, specificheat: 1500.0, vapourdiffusivity: Some(7000.0) }), resistance: None
    },
     "calizaMD".into() => Material {
    name: "calizaMD".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2395.0, specificheat: 1000.0, vapourdiffusivity: Some(200.0) }), resistance: None
    },
     "tc1800".into() => Material {
    name: "tc1800".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 1.0, density: 1800.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "cchapdo500".into() => Material {
    name: "cchapdo500".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 475.0, specificheat: 1600.0, vapourdiffusivity: Some(70.0) }), resistance: None
    },
     "Frondosa muy pesada [d > 870]".into() => Material {
    name: "Frondosa muy pesada [d > 870]".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.29, density: 900.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "corchoRes125".into() => Material {
    name: "corchoRes125".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.049, density: 125.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "LPp_90".into() => Material {
    name: "LPp_90".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.26), conductivity: 0.553, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "esquisto".into() => Material {
    name: "esquisto".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.2, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(800.0) }), resistance: None
    },
     "LMmp".into() => Material {
    name: "LMmp".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1225), conductivity: 1.02, density: 2170.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BC con mortero convencional espesor 140 mm".into() => Material {
    name: "BC con mortero convencional espesor 140 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.443, density: 1170.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "LPmp_50".into() => Material {
    name: "LPmp_50".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1225), conductivity: 0.68, density: 1140.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BCais240".into() => Material {
    name: "BCais240".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.2963, density: 920.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 500".into() => Material {
    name: "Hormigón celular curado en autoclave d 500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.14, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Aluminio aleaciones de".into() => Material {
    name: "Aluminio aleaciones de".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 160.0, density: 2800.0, specificheat: 880.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Poliacetato".into() => Material {
    name: "Poliacetato".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1410.0, specificheat: 1400.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "cauchoRig".into() => Material {
    name: "cauchoRig".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FUc250".into() => Material {
    name: "FUc250".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.893, density: 1220.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "rocaNatural".into() => Material {
    name: "rocaNatural".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.55, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "EPS029".into() => Material {
    name: "EPS029".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.029, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "LPp_70".into() => Material {
    name: "LPp_70".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.26), conductivity: 0.634, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Resina epoxi".into() => Material {
    name: "Resina epoxi".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "mCmp".into() => Material {
    name: "mCmp".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 650.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "BHalp190".into() => Material {
    name: "BHalp190".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.19), conductivity: 0.25333, density: 950.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Sin capa de compresión -Canto 200 mm".into() => Material {
    name: "Sin capa de compresión -Canto 200 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 1.404, density: 1410.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Arcilla o limo [1200 < d < 1800]".into() => Material {
    name: "Arcilla o limo [1200 < d < 1800]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.5, density: 1500.0, specificheat: 2100.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "Basalto [2700 < d < 3000]".into() => Material {
    name: "Basalto [2700 < d < 3000]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.5, density: 2850.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Corcho Expandido con resinas sintéticas 100 < d < 150".into() => Material {
    name: "Corcho Expandido con resinas sintéticas 100 < d < 150".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.049, density: 125.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "BPd250".into() => Material {
    name: "BPd250".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.55556, density: 900.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "cchapdo250".into() => Material {
    name: "cchapdo250".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.09, density: 200.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "pomez".into() => Material {
    name: "pomez".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 400.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Moquetas revestimientos textiles".into() => Material {
    name: "Moquetas revestimientos textiles".into(), group: "Textiles".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.06, density: 200.0, specificheat: 1300.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "FUeps250_mold".into() => Material {
    name: "FUeps250_mold".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.17606, density: 710.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FUeps350_mec".into() => Material {
    name: "FUeps350_mec".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.25547, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Placa de yeso laminado [PYL] 750 < d < 900".into() => Material {
    name: "Placa de yeso laminado [PYL] 750 < d < 900".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 825.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Haep1500".into() => Material {
    name: "Haep1500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.61, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Bloque de hormigon AL-M 170 mm".into() => Material {
    name: "Bloque de hormigon AL-M 170 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.55)
    },
     "Espuma de silicona".into() => Material {
    name: "Espuma de silicona".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Hoal1600".into() => Material {
    name: "Hoal1600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.59, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Traquita andesita [2000 < d < 2700]".into() => Material {
    name: "Traquita andesita [2000 < d < 2700]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.1, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "Titanio".into() => Material {
    name: "Titanio".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 21.9, density: 4500.0, specificheat: 522.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "PUR Plancha con HFC o Pentano y rev. permeable a gases [ 0.03 W/[mK]]".into() => Material {
    name: "PUR Plancha con HFC o Pentano y rev. permeable a gases [ 0.03 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.03, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "BH aligerado macizo -muro de carga- espesor 300 mm".into() => Material {
    name: "BH aligerado macizo -muro de carga- espesor 300 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.338, density: 940.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "mFl".into() => Material {
    name: "mFl".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 500.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "EPS046".into() => Material {
    name: "EPS046".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.046, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Teja cerámica-porcelana".into() => Material {
    name: "Teja cerámica-porcelana".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 2300.0, specificheat: 840.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "Espuma de polietileno".into() => Material {
    name: "Espuma de polietileno".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 70.0, specificheat: 2300.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "corchoCom".into() => Material {
    name: "corchoCom".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 500.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Tableros de fibras incluyendo MDF 550 < d < 750".into() => Material {
    name: "Tableros de fibras incluyendo MDF 550 < d < 750".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 650.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "PUR Plancha con HFC o Pentano y rev. permeable gases [ 0.027 W/[mK]]".into() => Material {
    name: "PUR Plancha con HFC o Pentano y rev. permeable gases [ 0.027 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.027, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FU Entrevigado de EPS moldeado enrasado -Canto 350 mm".into() => Material {
    name: "FU Entrevigado de EPS moldeado enrasado -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.368, density: 690.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada horizontal 5 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada horizontal 5 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.08)
    },
     "traquita".into() => Material {
    name: "traquita".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.1, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "panFib350".into() => Material {
    name: "panFib350".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1150.0, specificheat: 1600.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "FReps350_mec".into() => Material {
    name: "FReps350_mec".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.2963, density: 1092.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FUhal350".into() => Material {
    name: "FUhal350".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.2069, density: 1080.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hormigón convencional d 2300".into() => Material {
    name: "Hormigón convencional d 2300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.72, density: 2300.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1000".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.35, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 900".into() => Material {
    name: "Hormigón celular curado en autoclave d 900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.27, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "asperon2200".into() => Material {
    name: "asperon2200".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.8, density: 2200.0, specificheat: 1000.0, vapourdiffusivity: Some(40.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 800".into() => Material {
    name: "Hormigón celular curado en autoclave d 800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "MORgt2000".into() => Material {
    name: "MORgt2000".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.8, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1700".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.76, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "MORgt1250".into() => Material {
    name: "MORgt1250".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.7, density: 1350.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con áridos ligeros 1800 < d < 2000".into() => Material {
    name: "Hormigón con áridos ligeros 1800 < d < 2000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.35, density: 1900.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Haep1100".into() => Material {
    name: "Haep1100".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.39, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "calizaB".into() => Material {
    name: "calizaB".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.1, density: 1695.0, specificheat: 1000.0, vapourdiffusivity: Some(25.0) }), resistance: None
    },
     "PYL".into() => Material {
    name: "PYL".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 825.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Mortero de áridos ligeros [vermiculita perlita]".into() => Material {
    name: "Mortero de áridos ligeros [vermiculita perlita]".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.41, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHad240".into() => Material {
    name: "BHad240".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.96, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Con capa de compresión -Canto 250 mm".into() => Material {
    name: "Con capa de compresión -Canto 250 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.56, density: 1580.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Polisulfuro".into() => Material {
    name: "Polisulfuro".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.4, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "hierroFund".into() => Material {
    name: "hierroFund".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 50.0, density: 7500.0, specificheat: 450.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FUpic300".into() => Material {
    name: "FUpic300".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.88235, density: 1273.0, specificheat: 800.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Tablero contrachapado 350 < d < -450".into() => Material {
    name: "Tablero contrachapado 350 < d < -450".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 400.0, specificheat: 1600.0, vapourdiffusivity: Some(70.0) }), resistance: None
    },
     "HDPE".into() => Material {
    name: "HDPE".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.5, density: 980.0, specificheat: 1800.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "1 pie LP métrico o catalán 80 mm< G < 100 mm".into() => Material {
    name: "1 pie LP métrico o catalán 80 mm< G < 100 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.512, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado -Canto 300 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.128, density: 1090.0, specificheat: 1000.0, vapourdiffusivity: Some(7.0) }), resistance: None
    },
     "HAuto900".into() => Material {
    name: "HAuto900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.27, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Acrílicos".into() => Material {
    name: "Acrílicos".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1050.0, specificheat: 1500.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Poliisobutileno".into() => Material {
    name: "Poliisobutileno".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 930.0, specificheat: 1100.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "1/2 pie LP métrico o catalán 60 mm< G < 80 mm".into() => Material {
    name: "1/2 pie LP métrico o catalán 60 mm< G < 80 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.115), conductivity: 0.567, density: 1020.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado d< 1200 -Canto 250 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado d< 1200 -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.121, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FR Entrevigado de EPS mecanizado enrasado -Canto 300 mm ".into() => Material {
    name: "FR Entrevigado de EPS mecanizado enrasado -Canto 300 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.667, density: 1470.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FU Entrevigado cerámico -Canto 350 mm".into() => Material {
    name: "FU Entrevigado cerámico -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.995, density: 1030.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "XPS_CO2_034".into() => Material {
    name: "XPS_CO2_034".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.034, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FR Sin Entrevigado -Canto 350 mm".into() => Material {
    name: "FR Sin Entrevigado -Canto 350 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 4.651, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "losaAlv250cc".into() => Material {
    name: "losaAlv250cc".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.5625, density: 1580.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "losaAl300".into() => Material {
    name: "losaAl300".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.66667, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Haep1400".into() => Material {
    name: "Haep1400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.55, density: 1400.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "plaqGres".into() => Material {
    name: "plaqGres".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm ".into() => Material {
    name: "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.056, density: 1460.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "BH convencional espesor 250 mm".into() => Material {
    name: "BH convencional espesor 250 mm".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.007, density: 685.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Cámara de aire sin ventilar vertical 5 cm".into() => Material {
    name: "Cámara de aire sin ventilar vertical 5 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.18)
    },
     "tc1400".into() => Material {
    name: "tc1400".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.6, density: 1400.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "cv_hor_asc".into() => Material {
    name: "cv_hor_asc".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.06)
    },
     "Conífera pesada 520 < d < 610".into() => Material {
    name: "Conífera pesada 520 < d < 610".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 570.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FR Entrevigado de hormigón aligerado -Canto 350 mm ".into() => Material {
    name: "FR Entrevigado de hormigón aligerado -Canto 350 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.89, density: 1515.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "BH convencional espesor 300 mm".into() => Material {
    name: "BH convencional espesor 300 mm".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.16, density: 585.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 600".into() => Material {
    name: "Hormigón con otros áridos ligeros d 600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.83, density: 600.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 700".into() => Material {
    name: "Hormigón celular curado en autoclave d 700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FR Entrevigado cerámico -Canto 300 mm ".into() => Material {
    name: "FR Entrevigado cerámico -Canto 300 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.678, density: 1580.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "AT".into() => Material {
    name: "AT".into(), group: "Añadidos a la BDC".into(), properties: Some(MaterialProperties { thickness: Some(0.05), conductivity: 0.03, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "cobre".into() => Material {
    name: "cobre".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 380.0, density: 8900.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "HC1800".into() => Material {
    name: "HC1800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.12, density: 1800.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "HALlt2000".into() => Material {
    name: "HALlt2000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.35, density: 1900.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Tablero contrachapado 450 < d < 500".into() => Material {
    name: "Tablero contrachapado 450 < d < 500".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 475.0, specificheat: 1600.0, vapourdiffusivity: Some(70.0) }), resistance: None
    },
     "FRhal400".into() => Material {
    name: "FRhal400".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.90476, density: 1162.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Paneles de fibras con conglomerante hidráulico 450 < d < 550".into() => Material {
    name: "Paneles de fibras con conglomerante hidráulico 450 < d < 550".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 500.0, specificheat: 1700.0, vapourdiffusivity: Some(12.0) }), resistance: None
    },
     "Butilo [isobuteno] compacto/colado en caliente".into() => Material {
    name: "Butilo [isobuteno] compacto/colado en caliente".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.24, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(200000.0) }), resistance: None
    },
     "Aislamiento a determinar por el usuario".into() => Material {
    name: "Aislamiento a determinar por el usuario".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.0001)
    },
     "FRc250".into() => Material {
    name: "FRc250".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.667, density: 1277.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUho350".into() => Material {
    name: "FUho350".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.52174, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Uretano o poliuretano [rotura de puente térmico]".into() => Material {
    name: "Uretano o poliuretano [rotura de puente térmico]".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.21, density: 1300.0, specificheat: 1800.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Linóleo".into() => Material {
    name: "Linóleo".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(800.0) }), resistance: None
    },
     "tabPart270".into() => Material {
    name: "tabPart270".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.1, density: 225.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Corcho Expandido con resinas sintéticas 150 < d < 250".into() => Material {
    name: "Corcho Expandido con resinas sintéticas 150 < d < 250".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.055, density: 200.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "AislanteTermicoCalculado".into() => Material {
    name: "AislanteTermicoCalculado".into(), group: "Añadidos a la BDC".into(), properties: Some(MaterialProperties { thickness: Some(0.05), conductivity: 0.03, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BH convencional espesor 150 mm".into() => Material {
    name: "BH convencional espesor 150 mm".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.15), conductivity: 0.789, density: 1040.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "PURhfc_imper_025".into() => Material {
    name: "PURhfc_imper_025".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.025, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FRhal250_d12".into() => Material {
    name: "FRhal250_d12".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.78571, density: 1238.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "HAuto300".into() => Material {
    name: "HAuto300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.09, density: 300.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "XPS Expandido con dióxido de carbono CO4 [ 0.042 W/[mK]]".into() => Material {
    name: "XPS Expandido con dióxido de carbono CO4 [ 0.042 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.042, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 600".into() => Material {
    name: "Hormigón celular curado en autoclave d 600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 600.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FR Entrevigado de hormigón aligerado -Canto 250 mm ".into() => Material {
    name: "FR Entrevigado de hormigón aligerado -Canto 250 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.788, density: 1645.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado enrasado -Canto 400 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado enrasado -Canto 400 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.073, density: 1380.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "OSB".into() => Material {
    name: "OSB".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 650.0, specificheat: 1700.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "MORgt500".into() => Material {
    name: "MORgt500".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 625.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hierro fundición".into() => Material {
    name: "Hierro fundición".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 50.0, density: 7500.0, specificheat: 450.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BC con mortero aislante espesor 140 mm".into() => Material {
    name: "BC con mortero aislante espesor 140 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.324, density: 1020.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado de EPS mecanizado enrasado -Canto 250 mm".into() => Material {
    name: "FU Entrevigado de EPS mecanizado enrasado -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.266, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "HAgt2500".into() => Material {
    name: "HAgt2500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.5, density: 2600.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "HAlq2300".into() => Material {
    name: "HAlq2300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "HALlt1800".into() => Material {
    name: "HALlt1800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.15, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Haep900".into() => Material {
    name: "Haep900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "arena".into() => Material {
    name: "arena".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.0, density: 1950.0, specificheat: 1045.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "HC1900".into() => Material {
    name: "HC1900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.2, density: 1900.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "FRho250".into() => Material {
    name: "FRho250".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.923, density: 1338.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado de EPS moldeado descolgado -Canto 350 mm".into() => Material {
    name: "FU Entrevigado de EPS moldeado descolgado -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.22292994, density: 640.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "granito".into() => Material {
    name: "granito".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.8, density: 2600.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "BPals90".into() => Material {
    name: "BPals90".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.09), conductivity: 0.33333, density: 1000.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "losaAl500".into() => Material {
    name: "losaAl500".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.5), conductivity: 1.66667, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "MW_04".into() => Material {
    name: "MW_04".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.04, density: 40.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "HAuto700".into() => Material {
    name: "HAuto700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 700.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "espSilicona".into() => Material {
    name: "espSilicona".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Cámara de aire sin ventilar horizontal 10 cm".into() => Material {
    name: "Cámara de aire sin ventilar horizontal 10 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.18)
    },
     "Tablero contrachapado 250 < d < 350".into() => Material {
    name: "Tablero contrachapado 250 < d < 350".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.11, density: 300.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "espPU".into() => Material {
    name: "espPU".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 70.0, specificheat: 1500.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "betun_lamina".into() => Material {
    name: "betun_lamina".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "BPs90".into() => Material {
    name: "BPs90".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.09), conductivity: 0.40909, density: 1200.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Haep1600".into() => Material {
    name: "Haep1600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.68, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FRsin300".into() => Material {
    name: "FRsin300".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 4.28571, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Betún puro".into() => Material {
    name: "Betún puro".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1050.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "Con capa de compresión -Canto 350 mm".into() => Material {
    name: "Con capa de compresión -Canto 350 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.698, density: 1440.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "panFib450".into() => Material {
    name: "panFib450".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 400.0, specificheat: 1700.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "PA6_6".into() => Material {
    name: "PA6_6".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1450.0, specificheat: 1600.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "tabParCem".into() => Material {
    name: "tabParCem".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 1200.0, specificheat: 1500.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "MORgt1450".into() => Material {
    name: "MORgt1450".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.8, density: 1525.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "PPfv".into() => Material {
    name: "PPfv".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1200.0, specificheat: 1800.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FRhal300".into() => Material {
    name: "FRhal300".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.875, density: 1231.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FReps300_mole".into() => Material {
    name: "FReps300_mole".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.36364, density: 1123.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "vidrioFlot".into() => Material {
    name: "vidrioFlot".into(), group: "Vidrios".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 2500.0, specificheat: 750.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "moqueta".into() => Material {
    name: "moqueta".into(), group: "Textiles".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.06, density: 200.0, specificheat: 1300.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Panel de perlita expandida [EPB] [>80%]".into() => Material {
    name: "Panel de perlita expandida [EPB] [>80%]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.062, density: 190.0, specificheat: 1000.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Haep800".into() => Material {
    name: "Haep800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.27, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Caliza muy dura [2200 < d < 2590]".into() => Material {
    name: "Caliza muy dura [2200 < d < 2590]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2395.0, specificheat: 1000.0, vapourdiffusivity: Some(200.0) }), resistance: None
    },
     "Hormigón convencional d 1800".into() => Material {
    name: "Hormigón convencional d 1800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.12, density: 1800.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado -Canto 250 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.02, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado d< 1200 - Canto 400 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado d< 1200 - Canto 400 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.408, density: 985.0, specificheat: 1000.0, vapourdiffusivity: Some(9.0) }), resistance: None
    },
     "mFpm".into() => Material {
    name: "mFpm".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 657.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "EPDM".into() => Material {
    name: "EPDM".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(6000.0) }), resistance: None
    },
     "Cámara de aire sin ventilar vertical 2 cm".into() => Material {
    name: "Cámara de aire sin ventilar vertical 2 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.17)
    },
     "BHalm290_mc".into() => Material {
    name: "BHalm290_mc".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.46032, density: 1160.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "BC con mortero aislante espesor 190 mm".into() => Material {
    name: "BC con mortero aislante espesor 190 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.19), conductivity: 0.306, density: 910.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BPald200".into() => Material {
    name: "BPald200".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 0.41667, density: 900.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHalp110".into() => Material {
    name: "BHalp110".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.11), conductivity: 0.18644, density: 1095.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "BC con mortero aislante espesor 290 mm".into() => Material {
    name: "BC con mortero aislante espesor 290 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.295, density: 910.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado cerámico -Canto 250 mm".into() => Material {
    name: "FU Entrevigado cerámico -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.908, density: 1220.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón convencional d 1900".into() => Material {
    name: "Hormigón convencional d 1900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.2, density: 1900.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "FR Entrevigado de hormigón -Canto 350 mm".into() => Material {
    name: "FR Entrevigado de hormigón -Canto 350 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.995, density: 1610.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado descolgado -Canto 350 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado descolgado -Canto 350 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.40229884, density: 1310.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada horizontal 2 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.08)
    },
     "YDM".into() => Material {
    name: "YDM".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "FUeps350_mole".into() => Material {
    name: "FUeps350_mole".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.36842, density: 690.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FRho400".into() => Material {
    name: "FRho400".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 2.0, density: 1208.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "betun".into() => Material {
    name: "betun".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1050.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "EPB".into() => Material {
    name: "EPB".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.062, density: 190.0, specificheat: 900.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Plaqueta o baldosa cerámica".into() => Material {
    name: "Plaqueta o baldosa cerámica".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 2000.0, specificheat: 800.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "BC240".into() => Material {
    name: "BC240".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.42105, density: 1090.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Sílica gel [desecante]".into() => Material {
    name: "Sílica gel [desecante]".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 720.0, specificheat: 1000.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1400".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.55, density: 1400.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "EPS Poliestireno Expandido [ 0.029 W/[mK]]".into() => Material {
    name: "EPS Poliestireno Expandido [ 0.029 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.029, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Gneis Pórfido [2300 < d < 2900]".into() => Material {
    name: "Gneis Pórfido [2300 < d < 2900]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.5, density: 2600.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Sin capa de compresión -Canto 350 mm".into() => Material {
    name: "Sin capa de compresión -Canto 350 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.698, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Cámara de aire sin ventilar vertical 10 cm".into() => Material {
    name: "Cámara de aire sin ventilar vertical 10 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.19)
    },
     "estano".into() => Material {
    name: "estano".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 66.6, density: 7310.0, specificheat: 227.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BHad90".into() => Material {
    name: "BHad90".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.09), conductivity: 0.5625, density: 1400.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUho250".into() => Material {
    name: "FUho250".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.31579, density: 1330.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "tc800".into() => Material {
    name: "tc800".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.3, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "LHorAL".into() => Material {
    name: "LHorAL".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1225), conductivity: 0.39516, density: 1183.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "clv_hor_1".into() => Material {
    name: "clv_hor_1".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.075)
    },
     "FUeps250_mole".into() => Material {
    name: "FUeps250_mole".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.3125, density: 790.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "clv_hor_10".into() => Material {
    name: "clv_hor_10".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.09)
    },
     "Tablero contrachapado 700 < d < 900 ".into() => Material {
    name: "Tablero contrachapado 700 < d < 900 ".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.24, density: 800.0, specificheat: 1600.0, vapourdiffusivity: Some(110.0) }), resistance: None
    },
     "Bloque de hormigon AL-M 250 mm".into() => Material {
    name: "Bloque de hormigon AL-M 250 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.91)
    },
     "FUhal300_d12".into() => Material {
    name: "FUhal300_d12".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.2, density: 1040.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hoal1800".into() => Material {
    name: "Hoal1800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.22, density: 1800.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHalp90".into() => Material {
    name: "BHalp90".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.09), conductivity: 0.17308, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "cnv_hor_2".into() => Material {
    name: "cnv_hor_2".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.16)
    },
     "UP".into() => Material {
    name: "UP".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.19, density: 1400.0, specificheat: 1200.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Conífera de peso medio 435 < d < 520".into() => Material {
    name: "Conífera de peso medio 435 < d < 520".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 480.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FReps400_mec".into() => Material {
    name: "FReps400_mec".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.33333, density: 1069.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FRhal450_d12".into() => Material {
    name: "FRhal450_d12".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 1.95652, density: 1092.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "bronce".into() => Material {
    name: "bronce".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 65.0, density: 8700.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FUeps250_mec".into() => Material {
    name: "FUeps250_mec".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.26596, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "panFib550".into() => Material {
    name: "panFib550".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 500.0, specificheat: 1700.0, vapourdiffusivity: Some(12.0) }), resistance: None
    },
     "cascote de ladrillo".into() => Material {
    name: "cascote de ladrillo".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.115), conductivity: 0.41, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BPd200".into() => Material {
    name: "BPd200".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 0.5, density: 1000.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "silica".into() => Material {
    name: "silica".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 720.0, specificheat: 1000.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FUeps300_mec".into() => Material {
    name: "FUeps300_mec".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.25641, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FUc210".into() => Material {
    name: "FUc210".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.21), conductivity: 0.84, density: 1338.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón en masa 2300 < d < 2600".into() => Material {
    name: "Hormigón en masa 2300 < d < 2600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.0, density: 2450.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado -Canto 350 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.211, density: 1035.0, specificheat: 1000.0, vapourdiffusivity: Some(8.0) }), resistance: None
    },
     "Cloruro de polivinilo [PVC]".into() => Material {
    name: "Cloruro de polivinilo [PVC]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1390.0, specificheat: 900.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado - Canto 400 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado - Canto 400 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.311, density: 985.0, specificheat: 1000.0, vapourdiffusivity: Some(9.0) }), resistance: None
    },
     "gres_sil".into() => Material {
    name: "gres_sil".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2395.0, specificheat: 1000.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "FU Entrevigado de EPS moldeado enrasado -Canto 300 mm".into() => Material {
    name: "FU Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.341, density: 740.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1000".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUhal400_d12".into() => Material {
    name: "FUhal400_d12".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.429, density: 940.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Roca natural porosa [por ejem. Lava] d < 1600".into() => Material {
    name: "Roca natural porosa [por ejem. Lava] d < 1600".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.55, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "cnv_ver_1".into() => Material {
    name: "cnv_ver_1".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.15)
    },
     "XPS Expandido con dióxido de carbono CO2 [ 0.034 W/[mK]]".into() => Material {
    name: "XPS Expandido con dióxido de carbono CO2 [ 0.034 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.034, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "XPS Expandido con hidrofluorcarbonos HFC [ 0.039 W/[mK]]".into() => Material {
    name: "XPS Expandido con hidrofluorcarbonos HFC [ 0.039 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.039, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "cchapdo900".into() => Material {
    name: "cchapdo900".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.24, density: 800.0, specificheat: 1600.0, vapourdiffusivity: Some(110.0) }), resistance: None
    },
     "EPS037".into() => Material {
    name: "EPS037".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.037, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "tejaPlast".into() => Material {
    name: "tejaPlast".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FU Entrevigado de EPS mecanizado enrasado -Canto 300 mm".into() => Material {
    name: "FU Entrevigado de EPS mecanizado enrasado -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.256, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "BHad140".into() => Material {
    name: "BHad140".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.73684, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Conífera ligera d < 435".into() => Material {
    name: "Conífera ligera d < 435".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 430.0, specificheat: 1600.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada horizontal 10 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada horizontal 10 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.09)
    },
     "losaAl350".into() => Material {
    name: "losaAl350".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.66667, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Níquel".into() => Material {
    name: "Níquel".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 90.7, density: 8900.0, specificheat: 444.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "cuarzo".into() => Material {
    name: "cuarzo".into(), group: "Vidrios".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.4, density: 2200.0, specificheat: 750.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "cv_ver".into() => Material {
    name: "cv_ver".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.09)
    },
     "Hormigón con otros áridos ligeros d 1300".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.42, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Polietileno alta densidad [HDPE]".into() => Material {
    name: "Polietileno alta densidad [HDPE]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.5, density: 980.0, specificheat: 1800.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 1450 < d < 1600".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 1450 < d < 1600".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.8, density: 1525.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "LPp_50".into() => Material {
    name: "LPp_50".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.26), conductivity: 0.743, density: 1220.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada vertical 5 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada vertical 5 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.09)
    },
     "Bloque de hormigon AL-M 300 mm".into() => Material {
    name: "Bloque de hormigon AL-M 300 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.95)
    },
     "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into() => Material {
    name: "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.06), conductivity: 0.212, density: 630.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "MORgt1600".into() => Material {
    name: "MORgt1600".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHalp290_mc".into() => Material {
    name: "BHalp290_mc".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.32584, density: 970.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Yeso de alta dureza 1200 < d < 1500".into() => Material {
    name: "Yeso de alta dureza 1200 < d < 1500".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.56, density: 1350.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "LMp".into() => Material {
    name: "LMp".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.26), conductivity: 1.529, density: 2140.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FR FR Entrevigado cerámico -Canto 250 mm ".into() => Material {
    name: "FR FR Entrevigado cerámico -Canto 250 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.64, density: 1660.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUr310_12".into() => Material {
    name: "FUr310_12".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.31), conductivity: 1.33506, density: 840.0, specificheat: 800.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "tabFib200".into() => Material {
    name: "tabFib200".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.07, density: 125.0, specificheat: 1700.0, vapourdiffusivity: Some(2.0) }), resistance: None
    },
     "Caucho natural".into() => Material {
    name: "Caucho natural".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 910.0, specificheat: 1100.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Hoal1000".into() => Material {
    name: "Hoal1000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUc300".into() => Material {
    name: "FUc300".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.937, density: 1110.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado de EPS moldeado descolgado -Canto 300 mm".into() => Material {
    name: "FU Entrevigado de EPS moldeado descolgado -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.2, density: 670.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "marmol".into() => Material {
    name: "marmol".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.5, density: 2700.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FRhal250".into() => Material {
    name: "FRhal250".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.78571, density: 1292.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FU Entrevigado de hormigón -Canto 250 mm".into() => Material {
    name: "FU Entrevigado de hormigón -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.323, density: 1330.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "PYA".into() => Material {
    name: "PYA".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "enlYlq1300".into() => Material {
    name: "enlYlq1300".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.57, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "calizaMB".into() => Material {
    name: "calizaMB".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.85, density: 1495.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FRhal300_d12".into() => Material {
    name: "FRhal300_d12".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.875, density: 1185.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "cnv_ver_5".into() => Material {
    name: "cnv_ver_5".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.18)
    },
     "FR Entrevigado cerámico -Canto 350 mm ".into() => Material {
    name: "FR Entrevigado cerámico -Canto 350 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.717, density: 1520.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "CG".into() => Material {
    name: "CG".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 125.0, specificheat: 1000.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1100".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1100".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.34, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Caucho celular".into() => Material {
    name: "Caucho celular".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.06, density: 70.0, specificheat: 1500.0, vapourdiffusivity: Some(7000.0) }), resistance: None
    },
     "Hormigón convencional d 1600".into() => Material {
    name: "Hormigón convencional d 1600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.97, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 800".into() => Material {
    name: "Hormigón con otros áridos ligeros d 800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.65, density: 800.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "HC2000".into() => Material {
    name: "HC2000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.32, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "plaqCer".into() => Material {
    name: "plaqCer".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 2000.0, specificheat: 800.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "FRhal350_d12".into() => Material {
    name: "FRhal350_d12".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.84211, density: 1138.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Betún fieltro o lámina".into() => Material {
    name: "Betún fieltro o lámina".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "BPalt250".into() => Material {
    name: "BPalt250".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.40323, density: 900.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "tabPart450".into() => Material {
    name: "tabPart450".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 360.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "losaAlv250".into() => Material {
    name: "losaAlv250".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.5625, density: 1380.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Tabicón de LH doble [60 mm < E < 90 mm]".into() => Material {
    name: "Tabicón de LH doble [60 mm < E < 90 mm]".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.06), conductivity: 0.432, density: 930.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "cnv_hor_1".into() => Material {
    name: "cnv_hor_1".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.15)
    },
     "clv_hor_2".into() => Material {
    name: "clv_hor_2".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.08)
    },
     "Cámara de aire sin ventilar vertical 1 cm".into() => Material {
    name: "Cámara de aire sin ventilar vertical 1 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.15)
    },
     "1/2 pie LM métrico o catalán 40 mm< G < 50 mm".into() => Material {
    name: "1/2 pie LM métrico o catalán 40 mm< G < 50 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.115), conductivity: 0.991, density: 2170.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Tableros de fibras incluyendo MDF d < 200".into() => Material {
    name: "Tableros de fibras incluyendo MDF d < 200".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.07, density: 180.0, specificheat: 1700.0, vapourdiffusivity: Some(2.0) }), resistance: None
    },
     "Enlucido de yeso aislante 500 < d < 600".into() => Material {
    name: "Enlucido de yeso aislante 500 < d < 600".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 550.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Placa de yeso o escayola 750 < d < 900".into() => Material {
    name: "Placa de yeso o escayola 750 < d < 900".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 825.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Yeso dureza media 600 < d < 900".into() => Material {
    name: "Yeso dureza media 600 < d < 900".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Policarbonatos [PC]".into() => Material {
    name: "Policarbonatos [PC]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1200.0, specificheat: 1200.0, vapourdiffusivity: Some(5000.0) }), resistance: None
    },
     "1/2 pie LP métrico o catalán 80 mm< G < 100 mm".into() => Material {
    name: "1/2 pie LP métrico o catalán 80 mm< G < 100 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.115), conductivity: 0.512, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Zinc".into() => Material {
    name: "Zinc".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 110.0, density: 7200.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "FR Entrevigado de hormigón aligerado -Canto 300 mm ".into() => Material {
    name: "FR Entrevigado de hormigón aligerado -Canto 300 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.838, density: 1570.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Sin capa de compresión -Canto 500 mm".into() => Material {
    name: "Sin capa de compresión -Canto 500 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.5), conductivity: 2.02, density: 1120.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "mFml".into() => Material {
    name: "mFml".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 317.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "mFmp".into() => Material {
    name: "mFmp".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.29, density: 900.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "BPald150".into() => Material {
    name: "BPald150".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.15), conductivity: 0.34884, density: 950.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BC con mortero convencional espesor 190 mm".into() => Material {
    name: "BC con mortero convencional espesor 190 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.19), conductivity: 0.433, density: 1080.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FRc350".into() => Material {
    name: "FRc350".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.75, density: 1169.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Enlucido de yeso 1000 < d < 1300".into() => Material {
    name: "Enlucido de yeso 1000 < d < 1300".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.57, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hormigón convencional d 2000".into() => Material {
    name: "Hormigón convencional d 2000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.32, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "Bloque de hormigon AD 290 mm".into() => Material {
    name: "Bloque de hormigon AD 290 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.26)
    },
     "Bloque de hormigon AL-P 150 mm".into() => Material {
    name: "Bloque de hormigon AL-P 150 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.68)
    },
     "BPald120".into() => Material {
    name: "BPald120".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.12), conductivity: 0.3871, density: 900.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FU Entrevigado de hormigón -Canto 350 mm".into() => Material {
    name: "FU Entrevigado de hormigón -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.528, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "YADlt1200".into() => Material {
    name: "YADlt1200".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.43, density: 1050.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Tablero de partículas 450 < d < 640".into() => Material {
    name: "Tablero de partículas 450 < d < 640".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 545.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Asfalto".into() => Material {
    name: "Asfalto".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.7, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "FR Entrevigado de hormigón -Canto 450 mm".into() => Material {
    name: "FR Entrevigado de hormigón -Canto 450 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 2.091, density: 1540.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Sin capa de compresión -Canto 250 mm".into() => Material {
    name: "Sin capa de compresión -Canto 250 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.56, density: 1380.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "XPS_HFC_039".into() => Material {
    name: "XPS_HFC_039".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.039, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "losaAlv300cc".into() => Material {
    name: "losaAlv300cc".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.579, density: 1530.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "mFp".into() => Material {
    name: "mFp".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 810.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "Hormigón con arcilla expandida sin otros áridos d 600".into() => Material {
    name: "Hormigón con arcilla expandida sin otros áridos d 600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.19, density: 600.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "HMlt2600".into() => Material {
    name: "HMlt2600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.0, density: 2450.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Polimetilmetacrilato [PMMA]".into() => Material {
    name: "Polimetilmetacrilato [PMMA]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 1180.0, specificheat: 1500.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "genis".into() => Material {
    name: "genis".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.5, density: 2600.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Arena y grava [1700 < d < 2200]".into() => Material {
    name: "Arena y grava [1700 < d < 2200]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.0, density: 1450.0, specificheat: 1050.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "cchapdo400".into() => Material {
    name: "cchapdo400".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 400.0, specificheat: 1600.0, vapourdiffusivity: Some(70.0) }), resistance: None
    },
     "BH aligerado hueco -muro de carga- espesor 300 mm motero aligerado".into() => Material {
    name: "BH aligerado hueco -muro de carga- espesor 300 mm motero aligerado".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.421, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Cloruro de polivinilo [PVC] + 40% plastificante".into() => Material {
    name: "Cloruro de polivinilo [PVC] + 40% plastificante".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.14, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "tabPart640".into() => Material {
    name: "tabPart640".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 545.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Bronce".into() => Material {
    name: "Bronce".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 65.0, density: 8700.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Con capa de compresión -Canto 500 mm".into() => Material {
    name: "Con capa de compresión -Canto 500 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.5), conductivity: 2.02, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Hae400".into() => Material {
    name: "Hae400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 400.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Hoal1100".into() => Material {
    name: "Hoal1100".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.34, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "fenolica".into() => Material {
    name: "fenolica".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1300.0, specificheat: 1700.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "silicona_pura".into() => Material {
    name: "silicona_pura".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.35, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(5000.0) }), resistance: None
    },
     "Haep1000".into() => Material {
    name: "Haep1000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.35, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FReps400_mole".into() => Material {
    name: "FReps400_mole".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.379, density: 1062.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "MW Lana mineral [0.04 W/[mK]]".into() => Material {
    name: "MW Lana mineral [0.04 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.0405, density: 40.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 900".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "losaMa500".into() => Material {
    name: "losaMa500".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.5), conductivity: 2.5, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Haep1300".into() => Material {
    name: "Haep1300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.5, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado d< 1200 -Canto 350 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado d< 1200 -Canto 350 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.311, density: 1035.0, specificheat: 1000.0, vapourdiffusivity: Some(8.0) }), resistance: None
    },
     "basalto".into() => Material {
    name: "basalto".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.5, density: 2850.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FR Entrevigado de hormigón -Canto 300 mm".into() => Material {
    name: "FR Entrevigado de hormigón -Canto 300 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.947, density: 1670.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Etileno propileno dieno monómero [EPDM]".into() => Material {
    name: "Etileno propileno dieno monómero [EPDM]".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(6000.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1600".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.59, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hae600".into() => Material {
    name: "Hae600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.19, density: 600.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "PURhfc_per_027".into() => Material {
    name: "PURhfc_per_027".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.027, density: 45.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "losaAl250".into() => Material {
    name: "losaAl250".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.66667, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Plaqueta o baldosa de gres".into() => Material {
    name: "Plaqueta o baldosa de gres".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "vidrioPrens".into() => Material {
    name: "vidrioPrens".into(), group: "Vidrios".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.2, density: 2000.0, specificheat: 750.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "YADlt1500".into() => Material {
    name: "YADlt1500".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.56, density: 1350.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Placas de yeso armado con fibras minerales 800 < d < 1000".into() => Material {
    name: "Placas de yeso armado con fibras minerales 800 < d < 1000".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 1600 < d < 1800".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 1600 < d < 1800".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 1525.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "corchoPlac".into() => Material {
    name: "corchoPlac".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.065, density: 400.0, specificheat: 1500.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "BC140".into() => Material {
    name: "BC140".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.4375, density: 1170.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Bloque de picón de 200 mm".into() => Material {
    name: "Bloque de picón de 200 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.39)
    },
     "Arenisca [2200 < d < 2600]".into() => Material {
    name: "Arenisca [2200 < d < 2600]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.0, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "FReps250_mole".into() => Material {
    name: "FReps250_mole".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.25, density: 1280.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FU Entrevigado cerámico -Canto 300 mm".into() => Material {
    name: "FU Entrevigado cerámico -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.846, density: 1110.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Asperón [1300 < d < 1900]".into() => Material {
    name: "Asperón [1300 < d < 1900]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.9, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FR Entrevigado de hormigón -Canto 250 mm".into() => Material {
    name: "FR Entrevigado de hormigón -Canto 250 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.901, density: 1740.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Cámara de aire sin ventilar horizontal 5 cm".into() => Material {
    name: "Cámara de aire sin ventilar horizontal 5 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.16)
    },
     "Mármol [2600 < d < 2800]".into() => Material {
    name: "Mármol [2600 < d < 2800]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 3.5, density: 2700.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FR Entrevigado de EPS mecanizado enrasado -Canto 350 mm ".into() => Material {
    name: "FR Entrevigado de EPS mecanizado enrasado -Canto 350 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.707, density: 1420.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Hoal2000".into() => Material {
    name: "Hoal2000".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.5, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "losaMa300".into() => Material {
    name: "losaMa300".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 2.5, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Tierra apisonada adobe bloques de tierra comprimida [1770 < d < 2000]".into() => Material {
    name: "Tierra apisonada adobe bloques de tierra comprimida [1770 < d < 2000]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.1, density: 1885.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Hormigón armado 2300 < d < 2500".into() => Material {
    name: "Hormigón armado 2300 < d < 2500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 500 < d < 750".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 500 < d < 750".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 625.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Teja de arcilla cocida".into() => Material {
    name: "Teja de arcilla cocida".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 2000.0, specificheat: 800.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "Silex [2600 < d < 2800]".into() => Material {
    name: "Silex [2600 < d < 2800]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.6, density: 2700.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FR Sin Entrevigado -Canto 300 mm".into() => Material {
    name: "FR Sin Entrevigado -Canto 300 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 4.286, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "1 pie LP métrico o catalán 60 mm< G < 80 mm".into() => Material {
    name: "1 pie LP métrico o catalán 60 mm< G < 80 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.567, density: 1150.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "MORgt1000".into() => Material {
    name: "MORgt1000".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.55, density: 1125.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BCais290".into() => Material {
    name: "BCais290".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.29592, density: 910.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Subcapa lana".into() => Material {
    name: "Subcapa lana".into(), group: "Textiles".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.06, density: 200.0, specificheat: 1300.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "XPS_CO2_042".into() => Material {
    name: "XPS_CO2_042".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.042, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Bloque de hormigon AL-P 200 mm".into() => Material {
    name: "Bloque de hormigon AL-P 200 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.75)
    },
     "Neopreno [policloropreno]".into() => Material {
    name: "Neopreno [policloropreno]".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.23, density: 1240.0, specificheat: 2140.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1200".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1200".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.44, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Gres calcáreo 2000 < d < 2700".into() => Material {
    name: "Gres calcáreo 2000 < d < 2700".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.9, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "tabPart820".into() => Material {
    name: "tabPart820".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 730.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "MW Lana mineral [0.031 W/[mK]]".into() => Material {
    name: "MW Lana mineral [0.031 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.031, density: 40.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "PVC40".into() => Material {
    name: "PVC40".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.14, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "1 pie LP métrico o catalán 40 mm< G < 60 mm".into() => Material {
    name: "1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.667, density: 1220.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "calizaDM".into() => Material {
    name: "calizaDM".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.4, density: 1895.0, specificheat: 1000.0, vapourdiffusivity: Some(40.0) }), resistance: None
    },
     "niquel".into() => Material {
    name: "niquel".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 90.7, density: 8900.0, specificheat: 444.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "zinc".into() => Material {
    name: "zinc".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 110.0, density: 7200.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Hoal1200".into() => Material {
    name: "Hoal1200".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.37, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado enrasado -Canto 350 mm ".into() => Material {
    name: "FR Entrevigado de EPS moldeado enrasado -Canto 350 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.065, density: 1420.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "azulejo".into() => Material {
    name: "azulejo".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 2300.0, specificheat: 840.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BH aligerado hueco -muro de carga- espesor 300 mm".into() => Material {
    name: "BH aligerado hueco -muro de carga- espesor 300 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.475, density: 1160.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FReps450_mole".into() => Material {
    name: "FReps450_mole".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 1.406, density: 1046.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "cnv_hor_5".into() => Material {
    name: "cnv_hor_5".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.16)
    },
     "BH aligerado macizo -muro de carga- espesor 300 mm mortero aligerado".into() => Material {
    name: "BH aligerado macizo -muro de carga- espesor 300 mm mortero aligerado".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.287, density: 910.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "losaMa200".into() => Material {
    name: "losaMa200".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 2.5, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.1905, density: 1280.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Aquí va el aislante".into() => Material {
    name: "Aquí va el aislante".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.0001)
    },
     "Acero Inoxidable".into() => Material {
    name: "Acero Inoxidable".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 17.0, density: 7900.0, specificheat: 460.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Polipropileno 25%fibra vidrio".into() => Material {
    name: "Polipropileno 25%fibra vidrio".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1200.0, specificheat: 1800.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "FRhal350".into() => Material {
    name: "FRhal350".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.84211, density: 1192.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Caliza muy blanda [d < 1590]".into() => Material {
    name: "Caliza muy blanda [d < 1590]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.85, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Tabicón de LH triple [100 mm < E < 110 mm]".into() => Material {
    name: "Tabicón de LH triple [100 mm < E < 110 mm]".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.1), conductivity: 0.427, density: 920.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con arcilla expandida sin otros áridos d 400".into() => Material {
    name: "Hormigón con arcilla expandida sin otros áridos d 400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 400.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Cámara de aire ligeramente ventilada horizontal 1 cm".into() => Material {
    name: "Cámara de aire ligeramente ventilada horizontal 1 cm".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.075)
    },
     "epoxi".into() => Material {
    name: "epoxi".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "arcilla".into() => Material {
    name: "arcilla".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.5, density: 1500.0, specificheat: 2085.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "espPolietileno".into() => Material {
    name: "espPolietileno".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 70.0, specificheat: 2300.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "asfal_ar".into() => Material {
    name: "asfal_ar".into(), group: "Bituminosos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.15, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "tejaCer".into() => Material {
    name: "tejaCer".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.0, density: 2000.0, specificheat: 800.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "MW Lana mineral [0.05 W/[mK]]".into() => Material {
    name: "MW Lana mineral [0.05 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 40.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Tablero de partículas 640 < d < 820".into() => Material {
    name: "Tablero de partículas 640 < d < 820".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 730.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into() => Material {
    name: "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.313, density: 790.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FRho350".into() => Material {
    name: "FRho350".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.944, density: 1238.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "losaAlv500".into() => Material {
    name: "losaAlv500".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.5), conductivity: 2.0, density: 1120.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "lana".into() => Material {
    name: "lana".into(), group: "Textiles".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.06, density: 200.0, specificheat: 1300.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "asperon1600".into() => Material {
    name: "asperon1600".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.9, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "BHalp290".into() => Material {
    name: "BHalp290".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.30526, density: 860.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hoal500".into() => Material {
    name: "Hoal500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.94, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Ladrillo perforado de hormigón".into() => Material {
    name: "Ladrillo perforado de hormigón".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.12)
    },
     "Poliamida 6.6 [PA6.6] 25%fibra vidrio".into() => Material {
    name: "Poliamida 6.6 [PA6.6] 25%fibra vidrio".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1450.0, specificheat: 1600.0, vapourdiffusivity: Some(50000.0) }), resistance: None
    },
     "BHad50".into() => Material {
    name: "BHad50".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.05), conductivity: 1.0, density: 2090.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Bloque de hormigon AL-P 80 mm".into() => Material {
    name: "Bloque de hormigon AL-P 80 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.45)
    },
     "Hormigón con otros áridos ligeros d 500".into() => Material {
    name: "Hormigón con otros áridos ligeros d 500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.94, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "losaAlv200cc".into() => Material {
    name: "losaAlv200cc".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 1.429, density: 1810.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Hormigón con arcilla expandida sin otros áridos d 500".into() => Material {
    name: "Hormigón con arcilla expandida sin otros áridos d 500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.16, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "Con capa de compresión -Canto 300 mm".into() => Material {
    name: "Con capa de compresión -Canto 300 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.618, density: 1530.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "FR Entrevigado de EPS mecanizado enrasado - Canto 400 mm ".into() => Material {
    name: "FR Entrevigado de EPS mecanizado enrasado - Canto 400 mm ".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 0.741, density: 1390.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "uretano".into() => Material {
    name: "uretano".into(), group: "Sellantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.21, density: 1300.0, specificheat: 1800.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FU Entrevigado de hormigón aligerado d< 1200 -Canto 300 mm".into() => Material {
    name: "FU Entrevigado de hormigón aligerado d< 1200 -Canto 300 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.22, density: 1090.0, specificheat: 1000.0, vapourdiffusivity: Some(7.0) }), resistance: None
    },
     "FUpic350".into() => Material {
    name: "FUpic350".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 0.97222, density: 1306.0, specificheat: 800.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 900".into() => Material {
    name: "Hormigón con otros áridos ligeros d 900".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.27, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHad110".into() => Material {
    name: "BHad110".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.11), conductivity: 0.64706, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "cv_hor_des".into() => Material {
    name: "cv_hor_des".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.13)
    },
     "Bloque de hormigon AL-M 150 mm".into() => Material {
    name: "Bloque de hormigon AL-M 150 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.8)
    },
     "tc1600".into() => Material {
    name: "tc1600".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.8, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(3.0) }), resistance: None
    },
     "linoleo".into() => Material {
    name: "linoleo".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(800.0) }), resistance: None
    },
     "MYS".into() => Material {
    name: "MYS".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.8, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "cnv_ver_2".into() => Material {
    name: "cnv_ver_2".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.17)
    },
     "Sin capa de compresión -Canto 400 mm".into() => Material {
    name: "Sin capa de compresión -Canto 400 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.4), conductivity: 1.8, density: 1180.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Azulejo cerámico".into() => Material {
    name: "Azulejo cerámico".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 2300.0, specificheat: 840.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Teja de hormigón".into() => Material {
    name: "Teja de hormigón".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.5, density: 2100.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "XPS Expandido con dióxido de carbono CO3 [ 0.038 W/[mK]]".into() => Material {
    name: "XPS Expandido con dióxido de carbono CO3 [ 0.038 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.038, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(100.0) }), resistance: None
    },
     "Enlucido de yeso aislante 600 < d < 900".into() => Material {
    name: "Enlucido de yeso aislante 600 < d < 900".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 750.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "BHalm240".into() => Material {
    name: "BHalm240".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.26374, density: 900.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "losaAlv500cc".into() => Material {
    name: "losaAlv500cc".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.5), conductivity: 2.0, density: 1300.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "BCais140".into() => Material {
    name: "BCais140".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.14), conductivity: 0.31818, density: 1020.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1400".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.46, density: 1400.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BHalp290_mcAis".into() => Material {
    name: "BHalp290_mcAis".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.27619, density: 910.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "HAuto600".into() => Material {
    name: "HAuto600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 600.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hormigón celular curado en autoclave d 400".into() => Material {
    name: "Hormigón celular curado en autoclave d 400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.12, density: 400.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "LHGFs".into() => Material {
    name: "LHGFs".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.05), conductivity: 0.278, density: 670.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Bloque de picón de 290 mm".into() => Material {
    name: "Bloque de picón de 290 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.45)
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 1250 < d < 1450".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 1250 < d < 1450".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.7, density: 1350.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BH convencional espesor 100 mm".into() => Material {
    name: "BH convencional espesor 100 mm".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.1), conductivity: 0.632, density: 1210.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Esquisto Pizarra [2000 < d < 2800]".into() => Material {
    name: "Esquisto Pizarra [2000 < d < 2800]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.2, density: 2400.0, specificheat: 1000.0, vapourdiffusivity: Some(800.0) }), resistance: None
    },
     "HC1700".into() => Material {
    name: "HC1700".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.03, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(120.0) }), resistance: None
    },
     "BH aligerado hueco espesor 250 mm".into() => Material {
    name: "BH aligerado hueco espesor 250 mm".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.472, density: 760.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "LHGFd".into() => Material {
    name: "LHGFd".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.075), conductivity: 0.227, density: 630.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "losaAl200".into() => Material {
    name: "losaAl200".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 1.66667, density: 2000.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Bloque de picón de 250 mm".into() => Material {
    name: "Bloque de picón de 250 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.45)
    },
     "BHad60".into() => Material {
    name: "BHad60".into(), group: "Fábricas de bloque de hormigón convencional".into(), properties: Some(MaterialProperties { thickness: Some(0.06), conductivity: 0.85714, density: 1552.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "corchoExp".into() => Material {
    name: "corchoExp".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.049, density: 125.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "Bloque de hormigon AD 150 mm".into() => Material {
    name: "Bloque de hormigon AD 150 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.19)
    },
     "Cobre".into() => Material {
    name: "Cobre".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 380.0, density: 8900.0, specificheat: 380.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Hoal1400".into() => Material {
    name: "Hoal1400".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.46, density: 1400.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "EPS Poliestireno Expandido [ 0.046 W/[mK]]".into() => Material {
    name: "EPS Poliestireno Expandido [ 0.046 W/[mK]]".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.046, density: 30.0, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Cromo".into() => Material {
    name: "Cromo".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 93.7, density: 7160.0, specificheat: 449.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "MAL".into() => Material {
    name: "MAL".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.41, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 1900.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Mortero de cemento o cal para albañilería y para revoco/enlucido 750 < d < 1000".into() => Material {
    name: "Mortero de cemento o cal para albañilería y para revoco/enlucido 750 < d < 1000".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.4, density: 875.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Tablero contrachapado d < 250".into() => Material {
    name: "Tablero contrachapado d < 250".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.09, density: 200.0, specificheat: 1600.0, vapourdiffusivity: Some(50.0) }), resistance: None
    },
     "ARCexp".into() => Material {
    name: "ARCexp".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.148, density: 537.5, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Hormigón con otros áridos ligeros d 1800".into() => Material {
    name: "Hormigón con otros áridos ligeros d 1800".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.22, density: 1800.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1500".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.61, density: 1500.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hormigón armado d > 2500".into() => Material {
    name: "Hormigón armado d > 2500".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.5, density: 2600.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "Hormigón en masa 2000 < d < 2300".into() => Material {
    name: "Hormigón en masa 2000 < d < 2300".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.65, density: 2150.0, specificheat: 1000.0, vapourdiffusivity: Some(70.0) }), resistance: None
    },
     "Tableros de fibras incluyendo MDF 350 < d < 550".into() => Material {
    name: "Tableros de fibras incluyendo MDF 350 < d < 550".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.14, density: 450.0, specificheat: 1700.0, vapourdiffusivity: Some(12.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado descolgado -Canto 300 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado descolgado -Canto 300 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.35714287, density: 1330.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Cámara de aire ventilada".into() => Material {
    name: "Cámara de aire ventilada".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.09)
    },
     "Piedra artificial".into() => Material {
    name: "Piedra artificial".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 1700.0, specificheat: 1000.0, vapourdiffusivity: Some(40.0) }), resistance: None
    },
     "Tablero de virutas orientadas [OSB] d < 650".into() => Material {
    name: "Tablero de virutas orientadas [OSB] d < 650".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 600.0, specificheat: 1700.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "FR Sin Entrevigado -Canto 250 mm".into() => Material {
    name: "FR Sin Entrevigado -Canto 250 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 4.167, density: 2350.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "piedraArtificial".into() => Material {
    name: "piedraArtificial".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 1.3, density: 1750.0, specificheat: 1000.0, vapourdiffusivity: Some(40.0) }), resistance: None
    },
     "cromo".into() => Material {
    name: "cromo".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 93.7, density: 7160.0, specificheat: 449.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "XPS_HFC_032".into() => Material {
    name: "XPS_HFC_032".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.032, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "cauchoNat".into() => Material {
    name: "cauchoNat".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 910.0, specificheat: 1100.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "PS".into() => Material {
    name: "PS".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.16, density: 1050.0, specificheat: 1300.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "BPd120".into() => Material {
    name: "BPd120".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.12), conductivity: 0.46154, density: 1100.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Resina fenolica".into() => Material {
    name: "Resina fenolica".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.3, density: 1300.0, specificheat: 1700.0, vapourdiffusivity: Some(100000.0) }), resistance: None
    },
     "Con capa de compresión -Canto 200 mm".into() => Material {
    name: "Con capa de compresión -Canto 200 mm".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.2), conductivity: 1.404, density: 1810.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "FR Entrevigado de EPS moldeado enrasado -Canto 450 mm".into() => Material {
    name: "FR Entrevigado de EPS moldeado enrasado -Canto 450 mm".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 1.079, density: 1360.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "XPS_HFC_029".into() => Material {
    name: "XPS_HFC_029".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.029, density: 37.5, specificheat: 1000.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "MW_031".into() => Material {
    name: "MW_031".into(), group: "Aislantes".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.031, density: 40.0, specificheat: 1000.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "cascote".into() => Material {
    name: "cascote".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: None, conductivity: 0.41, density: 1300.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Al_alea".into() => Material {
    name: "Al_alea".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 160.0, density: 2800.0, specificheat: 880.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Bloque de picón de 120 mm".into() => Material {
    name: "Bloque de picón de 120 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.23)
    },
     "Granito [2500 < d < 2700]".into() => Material {
    name: "Granito [2500 < d < 2700]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.8, density: 2600.0, specificheat: 1000.0, vapourdiffusivity: Some(10000.0) }), resistance: None
    },
     "Caucho rigido [ebonita] sólido".into() => Material {
    name: "Caucho rigido [ebonita] sólido".into(), group: "Cauchos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.17, density: 1200.0, specificheat: 1400.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "BC con mortero convencional espesor 240 mm".into() => Material {
    name: "BC con mortero convencional espesor 240 mm".into(), group: "Fábricas de bloque cerámico de arcilla aligerada".into(), properties: Some(MaterialProperties { thickness: Some(0.24), conductivity: 0.424, density: 1090.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FUhal250_d12".into() => Material {
    name: "FUhal250_d12".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 1.136, density: 1130.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Bloque de hormigon AD 250 mm".into() => Material {
    name: "Bloque de hormigon AD 250 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.25)
    },
     "BPd150".into() => Material {
    name: "BPd150".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.15), conductivity: 0.42857, density: 1150.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "BPald250".into() => Material {
    name: "BPald250".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.46296, density: 800.0, specificheat: 800.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "plomo".into() => Material {
    name: "plomo".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 35.0, density: 11300.0, specificheat: 130.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "aluminio".into() => Material {
    name: "aluminio".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 230.0, density: 2700.0, specificheat: 880.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Tierra vegetal [d < 2050]".into() => Material {
    name: "Tierra vegetal [d < 2050]".into(), group: "Pétreos y suelos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.52, density: 2000.0, specificheat: 1840.0, vapourdiffusivity: Some(1.0) }), resistance: None
    },
     "Gres(sílice) 2200 < d < 2590".into() => Material {
    name: "Gres(sílice) 2200 < d < 2590".into(), group: "Cerámicos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 2.3, density: 2395.0, specificheat: 1000.0, vapourdiffusivity: Some(30.0) }), resistance: None
    },
     "Tablero contrachapado 600 < d < 750".into() => Material {
    name: "Tablero contrachapado 600 < d < 750".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.21, density: 675.0, specificheat: 1600.0, vapourdiffusivity: Some(110.0) }), resistance: None
    },
     "Haep1200".into() => Material {
    name: "Haep1200".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.44, density: 1200.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "Hormigón con arcilla expandida como árido principal d 1600".into() => Material {
    name: "Hormigón con arcilla expandida como árido principal d 1600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.68, density: 1600.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "corchoRes200".into() => Material {
    name: "corchoRes200".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.055, density: 200.0, specificheat: 1560.0, vapourdiffusivity: Some(5.0) }), resistance: None
    },
     "MORgt750".into() => Material {
    name: "MORgt750".into(), group: "Morteros".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.4, density: 875.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "Yeso baja dureza d < 600".into() => Material {
    name: "Yeso baja dureza d < 600".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 500.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "BHalm290_mcAis".into() => Material {
    name: "BHalm290_mcAis".into(), group: "Fábricas de bloque de hormigón aligerado".into(), properties: Some(MaterialProperties { thickness: Some(0.29), conductivity: 0.40845, density: 1100.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FRhal450".into() => Material {
    name: "FRhal450".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.45), conductivity: 1.956, density: 1146.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FReps350_mole".into() => Material {
    name: "FReps350_mole".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.35), conductivity: 1.4, density: 1092.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "Acero".into() => Material {
    name: "Acero".into(), group: "Metales".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 50.0, density: 7800.0, specificheat: 450.0, vapourdiffusivity: Some(1000000000000000000000000000000.0) }), resistance: None
    },
     "Hoal600".into() => Material {
    name: "Hoal600".into(), group: "Hormigones".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.83, density: 600.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "PC".into() => Material {
    name: "PC".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.2, density: 1200.0, specificheat: 1200.0, vapourdiffusivity: Some(5000.0) }), resistance: None
    },
     "YBD".into() => Material {
    name: "YBD".into(), group: "Yesos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 450.0, specificheat: 1000.0, vapourdiffusivity: Some(4.0) }), resistance: None
    },
     "LHs".into() => Material {
    name: "LHs".into(), group: "Fábricas de ladrillo".into(), properties: Some(MaterialProperties { thickness: Some(0.05), conductivity: 0.556, density: 1000.0, specificheat: 1000.0, vapourdiffusivity: Some(10.0) }), resistance: None
    },
     "FReps300_mec".into() => Material {
    name: "FReps300_mec".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 1.30435, density: 1131.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "enlYAlq500".into() => Material {
    name: "enlYAlq500".into(), group: "Enlucidos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.18, density: 550.0, specificheat: 1000.0, vapourdiffusivity: Some(6.0) }), resistance: None
    },
     "FU Entrevigado de EPS moldeado descolgado -Canto 250 mm".into() => Material {
    name: "FU Entrevigado de EPS moldeado descolgado -Canto 250 mm".into(), group: "Forjados unidireccionales".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 0.17605634, density: 710.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "FReps300_mold".into() => Material {
    name: "FReps300_mold".into(), group: "Forjados reticulares".into(), properties: Some(MaterialProperties { thickness: Some(0.3), conductivity: 0.35714, density: 1023.0, specificheat: 1000.0, vapourdiffusivity: Some(60.0) }), resistance: None
    },
     "losaMa250".into() => Material {
    name: "losaMa250".into(), group: "Losas alveolares".into(), properties: Some(MaterialProperties { thickness: Some(0.25), conductivity: 2.5, density: 2500.0, specificheat: 1000.0, vapourdiffusivity: Some(80.0) }), resistance: None
    },
     "clv_ver_1".into() => Material {
    name: "clv_ver_1".into(), group: "Cámaras de aire".into(), properties: None, resistance: Some(0.075)
    },
     "Tablero de partículas 270 < d < 450".into() => Material {
    name: "Tablero de partículas 270 < d < 450".into(), group: "Maderas".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.13, density: 360.0, specificheat: 1700.0, vapourdiffusivity: Some(20.0) }), resistance: None
    },
     "Bloque de hormigon AD 80 mm".into() => Material {
    name: "Bloque de hormigon AD 80 mm".into(), group: "Añadidos a la BDC".into(), properties: None, resistance: Some(0.1)
    },
     "fieltro".into() => Material {
    name: "fieltro".into(), group: "Textiles".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.05, density: 120.0, specificheat: 1300.0, vapourdiffusivity: Some(15.0) }), resistance: None
    },
     "Poliuretano [PU]".into() => Material {
    name: "Poliuretano [PU]".into(), group: "Plásticos".into(), properties: Some(MaterialProperties { thickness: Some(0.02), conductivity: 0.25, density: 1200.0, specificheat: 1800.0, vapourdiffusivity: Some(6000.0) }), resistance: None }

    },

    wallcons: hashmap! {
        "Ss_ue".into() => WallCons { name: "Ss_ue".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.25], absorptance: 0.0
    },
        "F 3.20".into() => WallCons { name: "F 3.20".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 250 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.25, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 3.34".into() => WallCons { name: "F 3.34".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 120 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.12, 0.0, 0.0, 0.08, 0.02], absorptance: 0.0
    },
        "F 5.02".into() => WallCons { name: "F 5.02".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 08mv.2".into() => WallCons { name: "C 08mv.2".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 5.06".into() => WallCons { name: "F 5.06".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 7.01".into() => WallCons { name: "F 7.01".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 3.14".into() => WallCons { name: "F 3.14".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.15, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "PIHa_l".into() => WallCons { name: "PIHa_l".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.15, 0.02], absorptance: 0.0
    },
        "F 6.09".into() => WallCons { name: "F 6.09".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 7.07".into() => WallCons { name: "F 7.07".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 8.03".into() => WallCons { name: "F 8.03".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BC con mortero convencional espesor 140 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.14, 0.02], absorptance: 0.0
    },
        "Qp_l".into() => WallCons { name: "Qp_l".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.15, 0.02], absorptance: 0.0
    },
        "C 05con.2".into() => WallCons { name: "C 05con.2".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.22".into() => WallCons { name: "F 6.22".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 140 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.14, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.24".into() => WallCons { name: "F 6.24".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 140 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.14, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 11lv.1".into() => WallCons { name: "C 11lv.1".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero contrachapado 700 < d < 900".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.03, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 9.02".into() => WallCons { name: "F 9.02".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 300 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.3, 0.02], absorptance: 0.0
    },
        "Sea_l".into() => WallCons { name: "Sea_l".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Hormigón armado d > 2500".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.15, 0.0000001, 0.02], absorptance: 0.0
    },
        "C 07inv.6".into() => WallCons { name: "C 07inv.6".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Cubierta por defecto C, D".into() => WallCons { name: "Cubierta por defecto C, D".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.15, 0.02], absorptance: 0.0
    },
        "Fm".into() => WallCons { name: "Fm".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "cascote de ladrillo".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.27, 0.02], absorptance: 0.0
    },
        "Sea_rh".into() => WallCons { name: "Sea_rh".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "C 05con.3".into() => WallCons { name: "C 05con.3".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 08lv.6".into() => WallCons { name: "C 08lv.6".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Qpca_rh".into() => WallCons { name: "Qpca_rh".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 01inv.3".into() => WallCons { name: "C 01inv.3".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Se_uc".into() => WallCons { name: "Se_uc".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "Fb1_14".into() => WallCons { name: "Fb1_14".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "BH convencional espesor 150 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.02], absorptance: 0.0
    },
        "Sea_rn".into() => WallCons { name: "Sea_rn".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "C 03lv.1".into() => WallCons { name: "C 03lv.1".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Fl1_m".into() => WallCons { name: "Fl1_m".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.02], absorptance: 0.0
    },
        "C 07con.5".into() => WallCons { name: "C 07con.5".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 2.05".into() => WallCons { name: "F 2.05".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 3.15".into() => WallCons { name: "F 3.15".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 250 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.25, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 3.18".into() => WallCons { name: "F 3.18".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 250 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.25, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 3.37".into() => WallCons { name: "F 3.37".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 250 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.25, 0.0, 0.02], absorptance: 0.0
    },
        "C 09inv.2".into() => WallCons { name: "C 09inv.2".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.0, 0.002, 0.25, 0.015], absorptance: 0.0
    },
        "Qifa_ue".into() => WallCons { name: "Qifa_ue".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "F 4.07".into() => WallCons { name: "F 4.07".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Ladrillo perforado de hormigón".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.12, 0.02], absorptance: 0.0
    },
        "F 7.04".into() => WallCons { name: "F 7.04".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.115, 0.02], absorptance: 0.0
    },
        "Qpa_rh".into() => WallCons { name: "Qpa_rh".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 05con.7".into() => WallCons { name: "C 05con.7".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 05con.4".into() => WallCons { name: "C 05con.4".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "F 7.17".into() => WallCons { name: "F 7.17".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.08, 0.02], absorptance: 0.0
    },
        "PIHa_uh".into() => WallCons { name: "PIHa_uh".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 09con.5".into() => WallCons { name: "C 09con.5".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.0, 0.04, 0.015], absorptance: 0.0
    },
        "F 1.14".into() => WallCons { name: "F 1.14".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.02, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "PIH_ue".into() => WallCons { name: "PIH_ue".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 02.2".into() => WallCons { name: "C 02.2".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 06.1".into() => WallCons { name: "C 06.1".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 06.3".into() => WallCons { name: "C 06.3".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 03mv.1".into() => WallCons { name: "C 03mv.1".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 09inv.3".into() => WallCons { name: "C 09inv.3".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.0, 0.002, 0.25, 0.015], absorptance: 0.0
    },
        "C 04inv.6".into() => WallCons { name: "C 04inv.6".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 5.03".into() => WallCons { name: "F 5.03".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.27".into() => WallCons { name: "F 6.27".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 240 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.24, 0.0, 0.02], absorptance: 0.0
    },
        "C 03lv.6".into() => WallCons { name: "C 03lv.6".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Qpc_rc".into() => WallCons { name: "Qpc_rc".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "C 03mv.8".into() => WallCons { name: "C 03mv.8".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.2, 0.015], absorptance: 0.0
    },
        "F 1.09".into() => WallCons { name: "F 1.09".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.02, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.32".into() => WallCons { name: "F 6.32".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.12, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 7.06".into() => WallCons { name: "F 7.06".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 8.06".into() => WallCons { name: "F 8.06".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.25, 0.02], absorptance: 0.0
    },
        "C 08mv.1".into() => WallCons { name: "C 08mv.1".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.01".into() => WallCons { name: "F 3.01".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.115, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Qifa_uc".into() => WallCons { name: "Qifa_uc".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "F 3.16".into() => WallCons { name: "F 3.16".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 250 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.25, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "PIH_uc".into() => WallCons { name: "PIH_uc".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "Fl1i_m".into() => WallCons { name: "Fl1i_m".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "AT".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.12, 0.0000001, 0.02], absorptance: 0.0
    },
        "PIV_f".into() => WallCons { name: "PIV_f".into(), group: "Partición interior vertical".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.02], absorptance: 0.0
    },
        "C 09con.4".into() => WallCons { name: "C 09con.4".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.0, 0.2, 0.015], absorptance: 0.0
    },
        "Sea_rc".into() => WallCons { name: "Sea_rc".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "F 2.07".into() => WallCons { name: "F 2.07".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 6.13".into() => WallCons { name: "F 6.13".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.15, 0.0, 0.02], absorptance: 0.0
    },
        "C 01con.7".into() => WallCons { name: "C 01con.7".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Fl2a_p".into() => WallCons { name: "Fl2a_p".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "AT".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.26, 0.0000001, 0.05, 0.02], absorptance: 0.0
    },
        "Fl1i_p".into() => WallCons { name: "Fl1i_p".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "AT".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.26, 0.0000001, 0.02], absorptance: 0.0
    },
        "Qpc_re".into() => WallCons { name: "Qpc_re".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "Qpc_rh".into() => WallCons { name: "Qpc_rh".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "Se_ue".into() => WallCons { name: "Se_ue".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "C 10.3".into() => WallCons { name: "C 10.3".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.31".into() => WallCons { name: "F 6.31".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.12, 0.0, 0.02], absorptance: 0.0
    },
        "Fachada por defecto G".into() => WallCons { name: "Fachada por defecto G".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "AT".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0000001, 0.05, 0.02], absorptance: 0.0
    },
        "C 12mv.3".into() => WallCons { name: "C 12mv.3".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 2.04".into() => WallCons { name: "F 2.04".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "C 13.4".into() => WallCons { name: "C 13.4".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into(), "Cámara de aire sin ventilar horizontal 2 cm".into()], thickness: vec![0.02, 0.02, 0.0], absorptance: 0.0
    },
        "F 1.13".into() => WallCons { name: "F 1.13".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.15, 0.02, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 07con.3".into() => WallCons { name: "C 07con.3".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 07con.8".into() => WallCons { name: "C 07con.8".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "C 04inv.4".into() => WallCons { name: "C 04inv.4".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "Fp2".into() => WallCons { name: "Fp2".into(), group: "Fachadas".into(), material: vec!["Caliza dura [2000 < d < 2190]".into(), "Tabicón de LH triple [100 mm < E < 110 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.36, 0.11, 0.02], absorptance: 0.0
    },
        "Fachada por defecto E, F".into() => WallCons { name: "Fachada por defecto E, F".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "AT".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0000001, 0.05, 0.02], absorptance: 0.0
    },
        "C 01inv.1".into() => WallCons { name: "C 01inv.1".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Sea_uc".into() => WallCons { name: "Sea_uc".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "F 3.29".into() => WallCons { name: "F 3.29".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Ladrillo perforado de hormigón".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.12, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "C 04inv.8".into() => WallCons { name: "C 04inv.8".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "F 6.26".into() => WallCons { name: "F 6.26".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 240 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Qifa_uh".into() => WallCons { name: "Qifa_uh".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "Ssa_uc".into() => WallCons { name: "Ssa_uc".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FU Entrevigado de hormigón -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.0000001, 0.25], absorptance: 0.0
    },
        "Fl1e_m".into() => WallCons { name: "Fl1e_m".into(), group: "Fachadas".into(), material: vec!["AT".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.12, 0.02], absorptance: 0.0
    },
        "Cubierta por defecto A, B".into() => WallCons { name: "Cubierta por defecto A, B".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.15, 0.02], absorptance: 0.0
    },
        "Qpa_uh".into() => WallCons { name: "Qpa_uh".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 04con.4".into() => WallCons { name: "C 04con.4".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "C 06.8".into() => WallCons { name: "C 06.8".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "C 08lv.7".into() => WallCons { name: "C 08lv.7".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Qif_uc".into() => WallCons { name: "Qif_uc".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "MED por defecto A, B".into() => WallCons { name: "MED por defecto A, B".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "cascote de ladrillo".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.17, 0.02], absorptance: 0.0
    },
        "C 11mv.1".into() => WallCons { name: "C 11mv.1".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.30".into() => WallCons { name: "F 6.30".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "C 12lv.1".into() => WallCons { name: "C 12lv.1".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero cerámico".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Fc1_14".into() => WallCons { name: "Fc1_14".into(), group: "Fachadas".into(), material: vec!["BC con mortero aislante espesor 140 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.14, 0.02], absorptance: 0.0
    },
        "Sea_re".into() => WallCons { name: "Sea_re".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "C 02.5".into() => WallCons { name: "C 02.5".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.27".into() => WallCons { name: "F 3.27".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 240 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.24, 0.0, 0.02], absorptance: 0.0
    },
        "Fc2c".into() => WallCons { name: "Fc2c".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "BC con mortero convencional espesor 140 mm".into(), "Cámara de aire ligeramente ventilada vertical 2 cm".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.14, 0.03, 0.05, 0.02], absorptance: 0.0
    },
        "C 10.4".into() => WallCons { name: "C 10.4".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.2, 0.015], absorptance: 0.0
    },
        "C 04inv.1".into() => WallCons { name: "C 04inv.1".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Fc2a".into() => WallCons { name: "Fc2a".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "BC con mortero convencional espesor 140 mm".into(), "AT".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.14, 0.0000001, 0.05, 0.02], absorptance: 0.0
    },
        "F 6.20".into() => WallCons { name: "F 6.20".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 250 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.25, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 01inv.6".into() => WallCons { name: "C 01inv.6".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.25".into() => WallCons { name: "F 3.25".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 240 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.24, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 8.01".into() => WallCons { name: "F 8.01".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.115, 0.02], absorptance: 0.0
    },
        "Separacion por defecto".into() => WallCons { name: "Separacion por defecto".into(), group: "Separación de espacios no habitables".into(), material: vec!["Hormigón armado d > 2500".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.3, 0.02], absorptance: 0.0
    },
        "F 7.13".into() => WallCons { name: "F 7.13".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "Fl2c_m".into() => WallCons { name: "Fl2c_m".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire ligeramente ventilada vertical 2 cm".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.03, 0.05, 0.02], absorptance: 0.0
    },
        "PIHa_rn".into() => WallCons { name: "PIHa_rn".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "F 5.10".into() => WallCons { name: "F 5.10".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 3.36".into() => WallCons { name: "F 3.36".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 200 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.2, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "Ss_uc".into() => WallCons { name: "Ss_uc".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.25], absorptance: 0.0
    },
        "C 07inv.1".into() => WallCons { name: "C 07inv.1".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 05con.5".into() => WallCons { name: "C 05con.5".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.10".into() => WallCons { name: "F 6.10".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "PIV_fa".into() => WallCons { name: "PIV_fa".into(), group: "Partición interior vertical".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "AT".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.0000001, 0.08, 0.02], absorptance: 0.0
    },
        "F 1.05".into() => WallCons { name: "F 1.05".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Qpc_ue".into() => WallCons { name: "Qpc_ue".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "Qpc_uh".into() => WallCons { name: "Qpc_uh".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "F 3.21".into() => WallCons { name: "F 3.21".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 140 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.14, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.18".into() => WallCons { name: "F 6.18".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 250 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.25, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 3.05".into() => WallCons { name: "F 3.05".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.24, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Se_l".into() => WallCons { name: "Se_l".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.15, 0.02], absorptance: 0.0
    },
        "F 3.07".into() => WallCons { name: "F 3.07".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.24, 0.0, 0.02], absorptance: 0.0
    },
        "Fl2a_m".into() => WallCons { name: "Fl2a_m".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "AT".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0000001, 0.05, 0.02], absorptance: 0.0
    },
        "Fb1_19".into() => WallCons { name: "Fb1_19".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "BH convencional espesor 200 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.2, 0.02], absorptance: 0.0
    },
        "Qpc_uc".into() => WallCons { name: "Qpc_uc".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "PIHa_ue".into() => WallCons { name: "PIHa_ue".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "Ssa_uh".into() => WallCons { name: "Ssa_uh".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FU Entrevigado de hormigón -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.0000001, 0.25], absorptance: 0.0
    },
        "C 06.2".into() => WallCons { name: "C 06.2".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Sea_uh".into() => WallCons { name: "Sea_uh".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "C 09inv.6".into() => WallCons { name: "C 09inv.6".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero cerámico".into(), "Cámara de aire sin ventilar horizontal 2 cm".into(), "Aquí va el aislamiento acústico".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.0, 0.002, 0.0, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 10.1".into() => WallCons { name: "C 10.1".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 10.6".into() => WallCons { name: "C 10.6".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Tablero cerámico".into(), "Cámara de aire sin ventilar horizontal 2 cm".into(), "Aquí va el aislamiento acústico".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.002, 0.0, 0.0, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 1.02".into() => WallCons { name: "F 1.02".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.115, 0.02, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 2.02".into() => WallCons { name: "F 2.02".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "Qpa_uc".into() => WallCons { name: "Qpa_uc".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "Se_rh".into() => WallCons { name: "Se_rh".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "PIH_rc".into() => WallCons { name: "PIH_rc".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "MED_t".into() => WallCons { name: "MED_t".into(), group: "Medianeras".into(), material: vec!["Tierra apisonada adobe bloques de tierra comprimida [1770 < d < 2000]".into()], thickness: vec![0.4], absorptance: 0.0
    },
        "C 04con.5".into() => WallCons { name: "C 04con.5".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 02.8".into() => WallCons { name: "C 02.8".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "F 3.22".into() => WallCons { name: "F 3.22".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 140 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into()], thickness: vec![0.02, 0.14, 0.0, 0.0, 0.07], absorptance: 0.0
    },
        "F 3.31".into() => WallCons { name: "F 3.31".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Ladrillo perforado de hormigón".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.12, 0.0, 0.02], absorptance: 0.0
    },
        "PIV_e".into() => WallCons { name: "PIV_e".into(), group: "Partición interior vertical".into(), material: vec!["Placa de yeso laminado [PYL] 750 < d < 900".into(), "AT".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.0000001, 0.02], absorptance: 0.0
    },
        "F 3.06".into() => WallCons { name: "F 3.06".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.24, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Qpa_rc".into() => WallCons { name: "Qpa_rc".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "F 3.02".into() => WallCons { name: "F 3.02".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.115, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "St".into() => WallCons { name: "St".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Hormigón armado d > 2500".into(), "Arena y grava [1700 < d < 2200]".into()], thickness: vec![0.03, 0.04, 0.2, 0.2], absorptance: 0.0
    },
        "F 5.05".into() => WallCons { name: "F 5.05".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "MED_ha".into() => WallCons { name: "MED_ha".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "BH convencional espesor 100 mm".into(), "AT".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.1, 0.0000001, 0.1, 0.02], absorptance: 0.0
    },
        "F 6.08".into() => WallCons { name: "F 6.08".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.24, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 01con.4".into() => WallCons { name: "C 01con.4".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "F 5.09".into() => WallCons { name: "F 5.09".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 11mv.3".into() => WallCons { name: "C 11mv.3".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 11mv.4".into() => WallCons { name: "C 11mv.4".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.2, 0.015], absorptance: 0.0
    },
        "F 6.38".into() => WallCons { name: "F 6.38".into(), group: "Fachadas".into(), material: vec!["Bloque de picón de 250 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.25, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 7.03".into() => WallCons { name: "F 7.03".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.115, 0.07, 0.02], absorptance: 0.0
    },
        "Ft".into() => WallCons { name: "Ft".into(), group: "Fachadas".into(), material: vec!["Tierra apisonada adobe bloques de tierra comprimida [1770 < d < 2000]".into()], thickness: vec![0.4], absorptance: 0.0
    },
        "F 1.07".into() => WallCons { name: "F 1.07".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.24, 0.0, 0.02], absorptance: 0.0
    },
        "F 3.30".into() => WallCons { name: "F 3.30".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Ladrillo perforado de hormigón".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.12, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "PIH_rn".into() => WallCons { name: "PIH_rn".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 07inv.5".into() => WallCons { name: "C 07inv.5".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.35".into() => WallCons { name: "F 6.35".into(), group: "Fachadas".into(), material: vec!["Bloque de picón de 200 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.2, 0.0, 0.02], absorptance: 0.0
    },
        "F 7.15".into() => WallCons { name: "F 7.15".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Ladrillo perforado de hormigón".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.12, 0.1, 0.02], absorptance: 0.0
    },
        "Sta".into() => WallCons { name: "Sta".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "Hormigón armado d > 2500".into(), "Arena y grava [1700 < d < 2200]".into()], thickness: vec![0.03, 0.04, 0.0000001, 0.2, 0.2], absorptance: 0.0
    },
        "Qif_uh".into() => WallCons { name: "Qif_uh".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "PIV_ea".into() => WallCons { name: "PIV_ea".into(), group: "Partición interior vertical".into(), material: vec!["Placa de yeso laminado [PYL] 750 < d < 900".into(), "AT".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.0000001, 0.02], absorptance: 0.0
    },
        "C 12lv.3".into() => WallCons { name: "C 12lv.3".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero cerámico".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Qpa_l".into() => WallCons { name: "Qpa_l".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.15, 0.02], absorptance: 0.0
    },
        "F 3.24".into() => WallCons { name: "F 3.24".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 140 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.14, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 8.02".into() => WallCons { name: "F 8.02".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 150 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.15, 0.02], absorptance: 0.0
    },
        "F 7.19".into() => WallCons { name: "F 7.19".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "Fachada por defecto D".into() => WallCons { name: "Fachada por defecto D".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire ligeramente ventilada vertical 2 cm".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.03, 0.05, 0.02], absorptance: 0.0
    },
        "F 6.06".into() => WallCons { name: "F 6.06".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "PIHa_re".into() => WallCons { name: "PIHa_re".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "F 7.09".into() => WallCons { name: "F 7.09".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 150 mm".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.15, 0.1, 0.02], absorptance: 0.0
    },
        "F 7.12".into() => WallCons { name: "F 7.12".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BC con mortero convencional espesor 140 mm".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.14, 0.02], absorptance: 0.0
    },
        "C 03mv.7".into() => WallCons { name: "C 03mv.7".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 7.14".into() => WallCons { name: "F 7.14".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "Se_uh".into() => WallCons { name: "Se_uh".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "C 09con.1".into() => WallCons { name: "C 09con.1".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.35".into() => WallCons { name: "F 3.35".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 200 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.2, 0.0, 0.02], absorptance: 0.0
    },
        "Qp_uh".into() => WallCons { name: "Qp_uh".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 07inv.8".into() => WallCons { name: "C 07inv.8".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "Qpca_rn".into() => WallCons { name: "Qpca_rn".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "F 2.01".into() => WallCons { name: "F 2.01".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "PIHa_uc".into() => WallCons { name: "PIHa_uc".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "Qpca_l".into() => WallCons { name: "Qpca_l".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.15, 0.02], absorptance: 0.0
    },
        "Se_re".into() => WallCons { name: "Se_re".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "C 04con.7".into() => WallCons { name: "C 04con.7".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 7.11".into() => WallCons { name: "F 7.11".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BC con mortero convencional espesor 140 mm".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.14, 0.07, 0.02], absorptance: 0.0
    },
        "Qp_re".into() => WallCons { name: "Qp_re".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "F 1.11".into() => WallCons { name: "F 1.11".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 150 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.0, 0.0, 0.15, 0.02], absorptance: 0.0
    },
        "F 4.09".into() => WallCons { name: "F 4.09".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Bloque de picón de 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.25, 0.02], absorptance: 0.0
    },
        "C 03mv.2".into() => WallCons { name: "C 03mv.2".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Fb2a".into() => WallCons { name: "Fb2a".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "BH convencional espesor 150 mm".into(), "AT".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.0000001, 0.1, 0.02], absorptance: 0.0
    },
        "C 02.4".into() => WallCons { name: "C 02.4".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "Ss_uh".into() => WallCons { name: "Ss_uh".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.25], absorptance: 0.0
    },
        "C 05con.9".into() => WallCons { name: "C 05con.9".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Acero".into()], thickness: vec![0.05, 0.002, 0.0, 0.01], absorptance: 0.0
    },
        "Qpca_rc".into() => WallCons { name: "Qpca_rc".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 09con.2".into() => WallCons { name: "C 09con.2".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 11mv.2".into() => WallCons { name: "C 11mv.2".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Qpc_l".into() => WallCons { name: "Qpc_l".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.15, 0.02], absorptance: 0.0
    },
        "C 01inv.7".into() => WallCons { name: "C 01inv.7".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 09con.6".into() => WallCons { name: "C 09con.6".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Tablero cerámico".into(), "Cámara de aire sin ventilar horizontal 2 cm".into(), "Aquí va el aislamiento acústico".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.002, 0.0, 0.0, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "Qifa_l".into() => WallCons { name: "Qifa_l".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.15, 0.02], absorptance: 0.0
    },
        "C 03lv.7".into() => WallCons { name: "C 03lv.7".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Ssa_ue".into() => WallCons { name: "Ssa_ue".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.0000001, 0.25], absorptance: 0.0
    },
        "Fc1_24".into() => WallCons { name: "Fc1_24".into(), group: "Fachadas".into(), material: vec!["BC con mortero aislante espesor 240 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.02], absorptance: 0.0
    },
        "C 04con.6".into() => WallCons { name: "C 04con.6".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 08mv.4".into() => WallCons { name: "C 08mv.4".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.3, 0.015], absorptance: 0.0
    },
        "C 09inv.5".into() => WallCons { name: "C 09inv.5".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.0, 0.002, 0.04, 0.015], absorptance: 0.0
    },
        "F 1.10".into() => WallCons { name: "F 1.10".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "C 07inv.4".into() => WallCons { name: "C 07inv.4".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "Fp1".into() => WallCons { name: "Fp1".into(), group: "Fachadas".into(), material: vec!["Caliza dura [2000 < d < 2190]".into()], thickness: vec![0.5], absorptance: 0.0
    },
        "C 12lv.2".into() => WallCons { name: "C 12lv.2".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero cerámico".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 12mv.4".into() => WallCons { name: "C 12mv.4".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.2, 0.015], absorptance: 0.0
    },
        "F 3.13".into() => WallCons { name: "F 3.13".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 150 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.15, 0.0, 0.02], absorptance: 0.0
    },
        "F 3.28".into() => WallCons { name: "F 3.28".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 240 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.24, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "Fb2c".into() => WallCons { name: "Fb2c".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "BH convencional espesor 150 mm".into(), "Cámara de aire ligeramente ventilada vertical 2 cm".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.03, 0.1, 0.02], absorptance: 0.0
    },
        "F 3.08".into() => WallCons { name: "F 3.08".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.24, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 04inv.3".into() => WallCons { name: "C 04inv.3".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 5.07".into() => WallCons { name: "F 5.07".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 6.11".into() => WallCons { name: "F 6.11".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "MED por defecto C, D, E".into() => WallCons { name: "MED por defecto C, D, E".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.02], absorptance: 0.0
    },
        "Qp_rh".into() => WallCons { name: "Qp_rh".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "Fachada por defecto B, C".into() => WallCons { name: "Fachada por defecto B, C".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.26, 0.02], absorptance: 0.0
    },
        "C 07con.4".into() => WallCons { name: "C 07con.4".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "C 09inv.1".into() => WallCons { name: "C 09inv.1".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.0, 0.002, 0.25, 0.015], absorptance: 0.0
    },
        "F 4.03".into() => WallCons { name: "F 4.03".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "BH convencional espesor 150 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.15, 0.02], absorptance: 0.0
    },
        "PIH_l".into() => WallCons { name: "PIH_l".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.15, 0.02], absorptance: 0.0
    },
        "F 4.05".into() => WallCons { name: "F 4.05".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "BC con mortero convencional espesor 140 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.14, 0.02], absorptance: 0.0
    },
        "F 6.07".into() => WallCons { name: "F 6.07".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.24, 0.0, 0.02], absorptance: 0.0
    },
        "F 6.16".into() => WallCons { name: "F 6.16".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 250 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.25, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "C 08mv.8".into() => WallCons { name: "C 08mv.8".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.2, 0.015], absorptance: 0.0
    },
        "F 6.19".into() => WallCons { name: "F 6.19".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 250 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.25, 0.0, 0.02], absorptance: 0.0
    },
        "Qpa_rn".into() => WallCons { name: "Qpa_rn".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "PIH_uh".into() => WallCons { name: "PIH_uh".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "F 7.22".into() => WallCons { name: "F 7.22".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 250 mm".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.25, 0.02], absorptance: 0.0
    },
        "F 3.09".into() => WallCons { name: "F 3.09".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 150 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "C 12lv.4".into() => WallCons { name: "C 12lv.4".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero cerámico".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.04, 0.0, 0.0, 0.2, 0.015], absorptance: 0.0
    },
        "Qif_ue".into() => WallCons { name: "Qif_ue".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "F 6.04".into() => WallCons { name: "F 6.04".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.115, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "PIHa_rc".into() => WallCons { name: "PIHa_rc".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "Qpca_uh".into() => WallCons { name: "Qpca_uh".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 09con.3".into() => WallCons { name: "C 09con.3".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Sea_ue".into() => WallCons { name: "Sea_ue".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "AT".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.03, 0.04, 0.25, 0.0000001, 0.02], absorptance: 0.0
    },
        "F 6.17".into() => WallCons { name: "F 6.17".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 250 mm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.25, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "Fl1_p".into() => WallCons { name: "Fl1_p".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.26, 0.02], absorptance: 0.0
    },
        "Qp_rc".into() => WallCons { name: "Qp_rc".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "Qia".into() => WallCons { name: "Qia".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "Conífera pesada 520 < d < 610".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.01], absorptance: 0.0
    },
        "F 7.21".into() => WallCons { name: "F 7.21".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 200 mm".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.2, 0.02], absorptance: 0.0
    },
        "Qpca_re".into() => WallCons { name: "Qpca_re".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 05con.1".into() => WallCons { name: "C 05con.1".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 06.9".into() => WallCons { name: "C 06.9".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Acero".into()], thickness: vec![0.002, 0.0, 0.01], absorptance: 0.0
    },
        "F 5.08".into() => WallCons { name: "F 5.08".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.08, 0.02], absorptance: 0.0
    },
        "PIH por defecto".into() => WallCons { name: "PIH por defecto".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "Qp_rn".into() => WallCons { name: "Qp_rn".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 03lv.2".into() => WallCons { name: "C 03lv.2".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "PIH_rh".into() => WallCons { name: "PIH_rh".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 03mv.5".into() => WallCons { name: "C 03mv.5".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 7.18".into() => WallCons { name: "F 7.18".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 02.3".into() => WallCons { name: "C 02.3".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 08lv.8".into() => WallCons { name: "C 08lv.8".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.2, 0.015], absorptance: 0.0
    },
        "C 08mv.6".into() => WallCons { name: "C 08mv.6".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 1.12".into() => WallCons { name: "F 1.12".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.15, 0.02, 0.0, 0.02], absorptance: 0.0
    },
        "C 03lv.5".into() => WallCons { name: "C 03lv.5".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 1.17".into() => WallCons { name: "F 1.17".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.12, 0.02, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 01inv.4".into() => WallCons { name: "C 01inv.4".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "C 02.7".into() => WallCons { name: "C 02.7".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.38".into() => WallCons { name: "F 3.38".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 250 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.25, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 07inv.3".into() => WallCons { name: "C 07inv.3".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 07con.7".into() => WallCons { name: "C 07con.7".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 07inv.2".into() => WallCons { name: "C 07inv.2".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 11lv.3".into() => WallCons { name: "C 11lv.3".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero contrachapado 700 < d < 900".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.03, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.32".into() => WallCons { name: "F 3.32".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Ladrillo perforado de hormigón".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.12, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 4.02".into() => WallCons { name: "F 4.02".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.24, 0.02], absorptance: 0.0
    },
        "F 6.21".into() => WallCons { name: "F 6.21".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 140 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.14, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.05".into() => WallCons { name: "F 6.05".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 3.17".into() => WallCons { name: "F 3.17".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 250 mm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.25, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 6.25".into() => WallCons { name: "F 6.25".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 240 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 7.02".into() => WallCons { name: "F 7.02".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 7.08".into() => WallCons { name: "F 7.08".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 150 mm".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.15, 0.07, 0.02], absorptance: 0.0
    },
        "F 3.33".into() => WallCons { name: "F 3.33".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 120 mm".into(), "Aquí va el aislante".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.12, 0.0, 0.08, 0.02], absorptance: 0.0
    },
        "F 6.23".into() => WallCons { name: "F 6.23".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 140 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.14, 0.0, 0.02], absorptance: 0.0
    },
        "F 1.16".into() => WallCons { name: "F 1.16".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.12, 0.02, 0.0, 0.02], absorptance: 0.0
    },
        "C 04inv.7".into() => WallCons { name: "C 04inv.7".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.12".into() => WallCons { name: "F 6.12".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.15, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "PIHa_rh".into() => WallCons { name: "PIHa_rh".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "AT".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "MED_c".into() => WallCons { name: "MED_c".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "BC con mortero convencional espesor 140 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.14, 0.02], absorptance: 0.0
    },
        "C 01con.3".into() => WallCons { name: "C 01con.3".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 01con.1".into() => WallCons { name: "C 01con.1".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "PIV por defecto".into() => WallCons { name: "PIV por defecto".into(), group: "Partición interior vertical".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.02], absorptance: 0.0
    },
        "C 06.6".into() => WallCons { name: "C 06.6".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.15".into() => WallCons { name: "F 6.15".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 250 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.25, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Qi".into() => WallCons { name: "Qi".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Conífera pesada 520 < d < 610".into()], thickness: vec![0.02, 0.02, 0.01], absorptance: 0.0
    },
        "Qpc_rn".into() => WallCons { name: "Qpc_rn".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.25, 0.02], absorptance: 0.0
    },
        "MTh".into() => WallCons { name: "MTh".into(), group: "Cerramiento en contacto con el terreno".into(), material: vec!["Hormigón armado d > 2500".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.3, 0.02], absorptance: 0.0
    },
        "C 13.3".into() => WallCons { name: "C 13.3".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into()], thickness: vec![0.02, 0.002], absorptance: 0.0
    },
        "C 03lv.8".into() => WallCons { name: "C 03lv.8".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.2, 0.015], absorptance: 0.0
    },
        "C 04con.3".into() => WallCons { name: "C 04con.3".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Fachada por defecto A".into() => WallCons { name: "Fachada por defecto A".into(), group: "Fachadas".into(), material: vec!["Caliza dura [2000 < d < 2190]".into(), "Tabicón de LH triple [100 mm < E < 110 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.36, 0.11, 0.02], absorptance: 0.0
    },
        "C 02.1".into() => WallCons { name: "C 02.1".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 05con.8".into() => WallCons { name: "C 05con.8".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "MED_la".into() => WallCons { name: "MED_la".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "AT".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.0000001, 0.08, 0.02], absorptance: 0.0
    },
        "MED_h".into() => WallCons { name: "MED_h".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "BH convencional espesor 150 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.02], absorptance: 0.0
    },
        "F 6.03".into() => WallCons { name: "F 6.03".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.115, 0.0, 0.02], absorptance: 0.0
    },
        "F 7.20".into() => WallCons { name: "F 7.20".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 120 mm".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.12, 0.08, 0.02], absorptance: 0.0
    },
        "C 02.6".into() => WallCons { name: "C 02.6".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.03, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 1.04".into() => WallCons { name: "F 1.04".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.115, 0.02, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 5.01".into() => WallCons { name: "F 5.01".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "C 04inv.5".into() => WallCons { name: "C 04inv.5".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "MED_m".into() => WallCons { name: "MED_m".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "cascote de ladrillo".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.17, 0.02], absorptance: 0.0
    },
        "C 03lv.3".into() => WallCons { name: "C 03lv.3".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.02".into() => WallCons { name: "F 6.02".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.115, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.14".into() => WallCons { name: "F 6.14".into(), group: "Fachadas".into(), material: vec!["BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.15, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 8.04".into() => WallCons { name: "F 8.04".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Ladrillo perforado de hormigón".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.12, 0.02], absorptance: 0.0
    },
        "C 07con.6".into() => WallCons { name: "C 07con.6".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 10.5".into() => WallCons { name: "C 10.5".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.04, 0.015], absorptance: 0.0
    },
        "C 06.4".into() => WallCons { name: "C 06.4".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.3, 0.015], absorptance: 0.0
    },
        "PIH_re".into() => WallCons { name: "PIH_re".into(), group: "Partición interior horizontal".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Arena y grava [1700 < d < 2200]".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 12mv.1".into() => WallCons { name: "C 12mv.1".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 01inv.8".into() => WallCons { name: "C 01inv.8".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "C 04con.8".into() => WallCons { name: "C 04con.8".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "C 11lv.4".into() => WallCons { name: "C 11lv.4".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero contrachapado 700 < d < 900".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.03, 0.0, 0.0, 0.2, 0.015], absorptance: 0.0
    },
        "F 1.01".into() => WallCons { name: "F 1.01".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.115, 0.02, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 3.23".into() => WallCons { name: "F 3.23".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 140 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.14, 0.0, 0.02], absorptance: 0.0
    },
        "F 6.34".into() => WallCons { name: "F 6.34".into(), group: "Fachadas".into(), material: vec!["Bloque de picón de 120 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0, 0.0, 0.08, 0.02], absorptance: 0.0
    },
        "F 7.05".into() => WallCons { name: "F 7.05".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 6.37".into() => WallCons { name: "F 6.37".into(), group: "Fachadas".into(), material: vec!["Bloque de picón de 250 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.25, 0.0, 0.02], absorptance: 0.0
    },
        "F 9.01".into() => WallCons { name: "F 9.01".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 290 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.29, 0.02], absorptance: 0.0
    },
        "F 1.15".into() => WallCons { name: "F 1.15".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.02, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 3.11".into() => WallCons { name: "F 3.11".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 150 mm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 4.01".into() => WallCons { name: "F 4.01".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.115, 0.02], absorptance: 0.0
    },
        "Fl2c_p".into() => WallCons { name: "Fl2c_p".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire ligeramente ventilada vertical 2 cm".into(), "Tabique de LH sencillo [40 mm < Espesor < 60 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.26, 0.03, 0.05, 0.02], absorptance: 0.0
    },
        "C 03lv.4".into() => WallCons { name: "C 03lv.4".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.035, 0.0, 0.0, 0.3, 0.015], absorptance: 0.0
    },
        "C 04con.2".into() => WallCons { name: "C 04con.2".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 08lv.3".into() => WallCons { name: "C 08lv.3".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 09inv.4".into() => WallCons { name: "C 09inv.4".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.0, 0.002, 0.2, 0.015], absorptance: 0.0
    },
        "C 04inv.2".into() => WallCons { name: "C 04inv.2".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "MED_l".into() => WallCons { name: "MED_l".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.02], absorptance: 0.0
    },
        "Qpca_uc".into() => WallCons { name: "Qpca_uc".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 10.2".into() => WallCons { name: "C 10.2".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Qpca_ue".into() => WallCons { name: "Qpca_ue".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Tabique de LH sencillo Gran Formato [40 mm < E < 60 mm]".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "AT".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.05, 0.02, 0.0000001, 0.25, 0.02], absorptance: 0.0
    },
        "C 08lv.4".into() => WallCons { name: "C 08lv.4".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.3, 0.015], absorptance: 0.0
    },
        "C 11lv.2".into() => WallCons { name: "C 11lv.2".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cloruro de polivinilo [PVC]".into(), "Tablero contrachapado 700 < d < 900".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.02, 0.002, 0.03, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.01".into() => WallCons { name: "F 6.01".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.115, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Qpa_ue".into() => WallCons { name: "Qpa_ue".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "F 1.03".into() => WallCons { name: "F 1.03".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.115, 0.02, 0.0, 0.02], absorptance: 0.0
    },
        "F 6.28".into() => WallCons { name: "F 6.28".into(), group: "Fachadas".into(), material: vec!["BC con mortero convencional espesor 240 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.24, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 6.29".into() => WallCons { name: "F 6.29".into(), group: "Fachadas".into(), material: vec!["Ladrillo perforado de hormigón".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 6.33".into() => WallCons { name: "F 6.33".into(), group: "Fachadas".into(), material: vec!["Bloque de picón de 120 mm".into(), "Aquí va el aislante".into(), "Bloque de picón de 80 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.12, 0.0, 0.08, 0.02], absorptance: 0.0
    },
        "F 7.16".into() => WallCons { name: "F 7.16".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Ladrillo perforado de hormigón".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.12, 0.02], absorptance: 0.0
    },
        "F 9.03".into() => WallCons { name: "F 9.03".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Bloque de picón de 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.25, 0.02], absorptance: 0.0
    },
        "C 06.5".into() => WallCons { name: "C 06.5".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 05inv.9".into() => WallCons { name: "C 05inv.9".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Acero".into()], thickness: vec![0.05, 0.0, 0.002, 0.01], absorptance: 0.0
    },
        "F 8.05".into() => WallCons { name: "F 8.05".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Bloque de picón de 200 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.2, 0.02], absorptance: 0.0
    },
        "Qif_l".into() => WallCons { name: "Qif_l".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.02, 0.15, 0.02], absorptance: 0.0
    },
        "F 5.04".into() => WallCons { name: "F 5.04".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "Se_rc".into() => WallCons { name: "Se_rc".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "C 01inv.5".into() => WallCons { name: "C 01inv.5".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Se_rn".into() => WallCons { name: "Se_rn".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.03, 0.04, 0.25, 0.02], absorptance: 0.0
    },
        "F 4.04".into() => WallCons { name: "F 4.04".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "BH convencional espesor 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.25, 0.02], absorptance: 0.0
    },
        "C 07con.2".into() => WallCons { name: "C 07con.2".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 01con.6".into() => WallCons { name: "C 01con.6".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "MED_p".into() => WallCons { name: "MED_p".into(), group: "Medianeras".into(), material: vec!["Caliza dura [2000 < d < 2190]".into()], thickness: vec![0.5], absorptance: 0.0
    },
        "C 08mv.5".into() => WallCons { name: "C 08mv.5".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.03".into() => WallCons { name: "F 3.03".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.115, 0.0, 0.02], absorptance: 0.0
    },
        "C 01inv.2".into() => WallCons { name: "C 01inv.2".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "MED_ca".into() => WallCons { name: "MED_ca".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "BC con mortero convencional espesor 140 mm".into(), "AT".into(), "BC con mortero convencional espesor 140 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.14, 0.0000001, 0.14, 0.02], absorptance: 0.0
    },
        "F 7.10".into() => WallCons { name: "F 7.10".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 150 mm".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.0, 0.0, 0.15, 0.02], absorptance: 0.0
    },
        "F 3.19".into() => WallCons { name: "F 3.19".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 250 mm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.25, 0.0, 0.02], absorptance: 0.0
    },
        "C 01con.2".into() => WallCons { name: "C 01con.2".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 08lv.5".into() => WallCons { name: "C 08lv.5".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "MED por defecto F".into() => WallCons { name: "MED por defecto F".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "AT".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.0000001, 0.08, 0.02], absorptance: 0.0
    },
        "C 08lv.1".into() => WallCons { name: "C 08lv.1".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "Fachada por defecto A-".into() => WallCons { name: "Fachada por defecto A-".into(), group: "Fachadas".into(), material: vec!["1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.26, 0.02], absorptance: 0.0
    },
        "F 4.08".into() => WallCons { name: "F 4.08".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "Bloque de picón de 200 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.2, 0.02], absorptance: 0.0
    },
        "Qp_ue".into() => WallCons { name: "Qp_ue".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 12mv.2".into() => WallCons { name: "C 12mv.2".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 6.36".into() => WallCons { name: "F 6.36".into(), group: "Fachadas".into(), material: vec!["Bloque de picón de 200 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.2, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 01con.8".into() => WallCons { name: "C 01con.8".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "Hormigón armado d > 2500".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.2, 0.015], absorptance: 0.0
    },
        "C 07inv.7".into() => WallCons { name: "C 07inv.7".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 03mv.3".into() => WallCons { name: "C 03mv.3".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 06.7".into() => WallCons { name: "C 06.7".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 1.08".into() => WallCons { name: "F 1.08".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.24, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "F 2.06".into() => WallCons { name: "F 2.06".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.1, 0.02], absorptance: 0.0
    },
        "F 3.26".into() => WallCons { name: "F 3.26".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BC con mortero convencional espesor 240 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.24, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "Contacto por defecto".into() => WallCons { name: "Contacto por defecto".into(), group: "Cerramientos en contacto con el terreno".into(), material: vec!["Hormigón armado d > 2500".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into()], thickness: vec![0.3, 0.02], absorptance: 0.0
    },
        "C 03mv.6".into() => WallCons { name: "C 03mv.6".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 04con.1".into() => WallCons { name: "C 04con.1".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 4.06".into() => WallCons { name: "F 4.06".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "Aquí va el aislante".into(), "BC con mortero convencional espesor 240 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.0, 0.24, 0.02], absorptance: 0.0
    },
        "Qpa_re".into() => WallCons { name: "Qpa_re".into(), group: "Cubiertas".into(), material: vec!["AT".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0000001, 0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 05con.6".into() => WallCons { name: "C 05con.6".into(), group: "Cubiertas".into(), material: vec!["Arena y grava [1700 < d < 2200]".into(), "Aquí va el aislante".into(), "Cloruro de polivinilo [PVC]".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.05, 0.0, 0.002, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "F 3.10".into() => WallCons { name: "F 3.10".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "C 08lv.2".into() => WallCons { name: "C 08lv.2".into(), group: "Cubiertas".into(), material: vec!["Cloruro de polivinilo [PVC]".into(), "Tabicón de LH doble Gran Formato 60 mm < E < 90 mm".into(), "Cámara de aire ligeramente ventilada horizontal 2 cm".into(), "Aquí va el aislante".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.002, 0.04, 0.0, 0.0, 0.25, 0.015], absorptance: 0.0
    },
        "F 2.03".into() => WallCons { name: "F 2.03".into(), group: "Fachadas".into(), material: vec!["Cámara de aire ventilada".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "F 3.04".into() => WallCons { name: "F 3.04".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "1/2 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar horizontal 1 cm".into(), "Aquí va el aislante".into(), "Placa de yeso laminado [PYL] 750 < d < 900".into()], thickness: vec![0.02, 0.115, 0.0, 0.0, 0.02], absorptance: 0.0
    },
        "C 07con.1".into() => WallCons { name: "C 07con.1".into(), group: "Cubiertas".into(), material: vec!["Tierra vegetal [d < 2050]".into(), "Subcapa fieltro".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado de EPS moldeado enrasado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.3, 0.001, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "C 01con.5".into() => WallCons { name: "C 01con.5".into(), group: "Cubiertas".into(), material: vec!["Plaqueta o baldosa cerámica".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "Cloruro de polivinilo [PVC]".into(), "Aquí va el aislante".into(), "Hormigón con áridos ligeros 1800 < d < 2000".into(), "FR FR Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.015, 0.04, 0.002, 0.0, 0.07, 0.25, 0.015], absorptance: 0.0
    },
        "Qp_uc".into() => WallCons { name: "Qp_uc".into(), group: "Cubiertas".into(), material: vec!["Hormigón con áridos ligeros 1800 < d < 2000".into(), "FU Entrevigado cerámico -Canto 250 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.05, 0.25, 0.02], absorptance: 0.0
    },
        "C 08mv.3".into() => WallCons { name: "C 08mv.3".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FU Entrevigado de hormigón -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "C 13.2".into() => WallCons { name: "C 13.2".into(), group: "Cubiertas".into(), material: vec!["Teja de arcilla cocida".into()], thickness: vec![0.02], absorptance: 0.0
    },
        "Suelo por defecto A, B, C, D, E".into() => WallCons { name: "Suelo por defecto A, B, C, D, E".into(), group: "Suelos".into(), material: vec!["Piedra artificial".into(), "Mortero de cemento o cal para albañilería y para revoco/enlucido d >2000".into(), "FU Entrevigado de hormigón -Canto 250 mm".into()], thickness: vec![0.03, 0.04, 0.25], absorptance: 0.0
    },
        "F 1.06".into() => WallCons { name: "F 1.06".into(), group: "Fachadas".into(), material: vec!["1 pie LP métrico o catalán 40 mm< G < 60 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.24, 0.0, 0.0, 0.07, 0.02], absorptance: 0.0
    },
        "C 08mv.7".into() => WallCons { name: "C 08mv.7".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR Sin Entrevigado -Canto 250 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.25, 0.015], absorptance: 0.0
    },
        "MED por defecto G".into() => WallCons { name: "MED por defecto G".into(), group: "Medianeras".into(), material: vec!["Enlucido de yeso 1000 < d < 1300".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "AT".into(), "Tabicón de LH doble [60 mm < E < 90 mm]".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.08, 0.0000001, 0.08, 0.02], absorptance: 0.0
    },
        "C 03mv.4".into() => WallCons { name: "C 03mv.4".into(), group: "Cubiertas".into(), material: vec!["Aquí va el aislante".into(), "FR Entrevigado de EPS moldeado enrasado -Canto 300 mm".into(), "Enlucido de yeso d < 1000".into()], thickness: vec![0.0, 0.3, 0.015], absorptance: 0.0
    },
        "F 3.12".into() => WallCons { name: "F 3.12".into(), group: "Fachadas".into(), material: vec!["Mortero de cemento o cal para albañilería y para revoco/enlucido 1800 < d < 2000".into(), "BH convencional espesor 150 mm".into(), "Cámara de aire sin ventilar vertical 2 cm".into(), "Aquí va el aislante".into(), "BH convencional espesor 100 mm".into(), "Enlucido de yeso 1000 < d < 1300".into()], thickness: vec![0.02, 0.15, 0.0, 0.0, 0.1, 0.02], absorptance: 0.0 }

    },

    windowcons: hashmap! {
        "Doble bajo emisivo -- Mpvc o mad - Gris claro".into() => WindowCons { name: "Doble bajo emisivo -- Mpvc o mad - Gris claro".into(), group: "Huecos".into(), glass: "Doble baja emisividad".into(), glassgroup: "Vidrios".into(), frame: "Mpvc o mad - Gris claro".into(), framegroup: "Marcos".into(), framefrac: 0.1, infcoeff: 25.0, deltau: 0.0, gglshwi: None
    },
        "Doble bajo emisivo argon -- Mpvc o mad - Gris claro".into() => WindowCons { name: "Doble bajo emisivo argon -- Mpvc o mad - Gris claro".into(), group: "Huecos".into(), glass: "Doble baja emisividad argon".into(), glassgroup: "Vidrios".into(), frame: "Mpvc o mad - Gris claro".into(), framegroup: "Marcos".into(), framefrac: 0.1, infcoeff: 25.0, deltau: 0.0, gglshwi: None
    },
        "Sencillo -- Mpvc o mad - Gris claro".into() => WindowCons { name: "Sencillo -- Mpvc o mad - Gris claro".into(), group: "Huecos".into(), glass: "Sencillo".into(), glassgroup: "Vidrios".into(), frame: "Mpvc o mad - Gris claro".into(), framegroup: "Marcos".into(), framefrac: 0.1, infcoeff: 25.0, deltau: 0.0, gglshwi: None
    },
        "Doble -- Mrpt - Gris claro".into() => WindowCons { name: "Doble -- Mrpt - Gris claro".into(), group: "Huecos".into(), glass: "Doble".into(), glassgroup: "Vidrios".into(), frame: "Mrpt - Gris claro".into(), framegroup: "Marcos".into(), framefrac: 0.1, infcoeff: 25.0, deltau: 0.0, gglshwi: None
    },
        "Sencillo -- Met - Gris claro".into() => WindowCons { name: "Sencillo -- Met - Gris claro".into(), group: "Huecos".into(), glass: "Sencillo".into(), glassgroup: "Vidrios".into(), frame: "Met - Gris claro".into(), framegroup: "Marcos".into(), framefrac: 0.1, infcoeff: 25.0, deltau: 0.0, gglshwi: None }

    },

    glasses: hashmap! {
        "VER_DC_4-15-331".into() => Glass { name: "VER_DC_4-15-331".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "VER_DB1_4-9-551a".into() => Glass { name: "VER_DB1_4-9-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DC_4-9-441a".into() => Glass { name: "HOR_DC_4-9-441a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "HOR_DB1_4-12-661a".into() => Glass { name: "HOR_DB1_4-12-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB1_4-9-441a".into() => Glass { name: "HOR_DB1_4-9-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB3_4-12-6".into() => Glass { name: "HOR_DB3_4-12-6".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DB1_4-6-331".into() => Glass { name: "HOR_DB1_4-6-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.9, g_gln: 0.7082353
    },
        "VER_DC_4-6-6".into() => Glass { name: "VER_DC_4-6-6".into(), group: "Dobles en posición vertical".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DB1_4-15-331".into() => Glass { name: "VER_DB1_4-15-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DB2_4-20-551a".into() => Glass { name: "VER_DB2_4-20-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-6-331".into() => Glass { name: "HOR_DB2_4-6-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.8, g_gln: 0.7082353
    },
        "HOR_DC_4-12-6".into() => Glass { name: "HOR_DC_4-12-6".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "HOR_DB3_4-15-6".into() => Glass { name: "HOR_DB3_4-15-6".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DC_4-9-551a".into() => Glass { name: "HOR_DC_4-9-551a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "VER_DB3_4-6-551a".into() => Glass { name: "VER_DB3_4-6-551a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_DB1_4-6-551a".into() => Glass { name: "HOR_DB1_4-6-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.9, g_gln: 0.7082353
    },
        "VER_DB1_4-15-551a".into() => Glass { name: "VER_DB1_4-15-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DB1_4-20-4".into() => Glass { name: "VER_DB1_4-20-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DB2_4-9-331".into() => Glass { name: "HOR_DB2_4-9-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_DB3_4-20-6".into() => Glass { name: "HOR_DB3_4-20-6".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "VER_DB3_4-9-441a".into() => Glass { name: "VER_DB3_4-9-441a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.9, g_gln: 0.7082353
    },
        "VER_DB2_4-20-4".into() => Glass { name: "VER_DB2_4-20-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB2_4-9-4".into() => Glass { name: "VER_DB2_4-9-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB1_4-6-441a".into() => Glass { name: "HOR_DB1_4-6-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.9, g_gln: 0.7082353
    },
        "VER_DB1_4-12-551a".into() => Glass { name: "VER_DB1_4-12-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.0, g_gln: 0.7082353
    },
        "HOR_DB3_4-12-441a".into() => Glass { name: "HOR_DB3_4-12-441a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DC_4-9-331".into() => Glass { name: "HOR_DC_4-9-331".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "VER_DB3_4-15-551a".into() => Glass { name: "VER_DB3_4-15-551a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DB1_4-20-441a".into() => Glass { name: "HOR_DB1_4-20-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB1_4-12-4".into() => Glass { name: "HOR_DB1_4-12-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DB1_4-12-4".into() => Glass { name: "VER_DB1_4-12-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.0, g_gln: 0.7082353
    },
        "HOR_DB2_4-20-6".into() => Glass { name: "HOR_DB2_4-20-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-20-4".into() => Glass { name: "VER_DB3_4-20-4".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DB1_4-15-661a".into() => Glass { name: "HOR_DB1_4-15-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB3_4-20-661a".into() => Glass { name: "HOR_DB3_4-20-661a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB1_4-20-6".into() => Glass { name: "HOR_DB1_4-20-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DB2_4-12-6".into() => Glass { name: "VER_DB2_4-12-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DC_4-20-661a".into() => Glass { name: "HOR_DC_4-20-661a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.2, g_gln: 0.7588236
    },
        "HOR_DB2_4-15-4".into() => Glass { name: "HOR_DB2_4-15-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_DB2_4-9-6".into() => Glass { name: "HOR_DB2_4-9-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB1_4-6-661a".into() => Glass { name: "HOR_DB1_4-6-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.9, g_gln: 0.7082353
    },
        "HOR_DB2_4-12-661a".into() => Glass { name: "HOR_DB2_4-12-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB2_4-20-6".into() => Glass { name: "VER_DB2_4-20-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-15-661a".into() => Glass { name: "HOR_DB2_4-15-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-15-661a".into() => Glass { name: "VER_DB3_4-15-661a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DB2_4-9-441a".into() => Glass { name: "HOR_DB2_4-9-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB1_4-9-661a".into() => Glass { name: "VER_DB1_4-9-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "VER_DC_4-9-661a".into() => Glass { name: "VER_DC_4-9-661a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.9, g_gln: 0.7588236
    },
        "VER_DC_4-9-551a".into() => Glass { name: "VER_DC_4-9-551a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.9, g_gln: 0.7588236
    },
        "VER_DB2_4-20-441a".into() => Glass { name: "VER_DB2_4-20-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB3_4-20-661a".into() => Glass { name: "VER_DB3_4-20-661a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DC_4-15-6".into() => Glass { name: "HOR_DC_4-15-6".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "VER_DB3_4-12-441a".into() => Glass { name: "VER_DB3_4-12-441a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_ML_441a".into() => Glass { name: "HOR_ML_441a".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.7, g_gln: 0.86
    },
        "HOR_DB2_4-12-331".into() => Glass { name: "HOR_DB2_4-12-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB3_4-9-6".into() => Glass { name: "VER_DB3_4-9-6".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.9, g_gln: 0.7082353
    },
        "HOR_DB2_4-15-6".into() => Glass { name: "HOR_DB2_4-15-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB1_4-6-331".into() => Glass { name: "VER_DB1_4-6-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "Sencillo".into() => Glass { name: "Sencillo".into(), group: "Vidrios".into(), conductivity: 5.7, g_gln: 0.86
    },
        "HOR_DC_4-20-4".into() => Glass { name: "HOR_DC_4-20-4".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DB1_4-20-331".into() => Glass { name: "HOR_DB1_4-20-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB2_4-20-551a".into() => Glass { name: "HOR_DB2_4-20-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB1_4-9-331".into() => Glass { name: "VER_DB1_4-9-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "HOR_DB1_4-12-331".into() => Glass { name: "HOR_DB1_4-12-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB3_4-9-4".into() => Glass { name: "HOR_DB3_4-9-4".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "HOR_M_6".into() => Glass { name: "HOR_M_6".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.8, g_gln: 0.86
    },
        "VER_ML_441a".into() => Glass { name: "VER_ML_441a".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.6, g_gln: 0.86
    },
        "HOR_DB1_4-15-331".into() => Glass { name: "HOR_DB1_4-15-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB2_4-6-661a".into() => Glass { name: "HOR_DB2_4-6-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DB2_4-20-331".into() => Glass { name: "HOR_DB2_4-20-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "HOR_DB2_4-20-661a".into() => Glass { name: "HOR_DB2_4-20-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-20-551a".into() => Glass { name: "VER_DB3_4-20-551a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "VER_DB3_4-9-661a".into() => Glass { name: "VER_DB3_4-9-661a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.9, g_gln: 0.7082353
    },
        "VER_DC_4-12-661a".into() => Glass { name: "VER_DC_4-12-661a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.8, g_gln: 0.7588236
    },
        "VER_DB2_4-12-441a".into() => Glass { name: "VER_DB2_4-12-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_ML_661a".into() => Glass { name: "VER_ML_661a".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.4, g_gln: 0.86
    },
        "HOR_DB3_4-12-331".into() => Glass { name: "HOR_DB3_4-12-331".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "VER_DC_4-15-661a".into() => Glass { name: "VER_DC_4-15-661a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "HOR_DB1_4-12-551a".into() => Glass { name: "HOR_DB1_4-12-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-15-441a".into() => Glass { name: "HOR_DB2_4-15-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DC_4-9-6".into() => Glass { name: "VER_DC_4-9-6".into(), group: "Dobles en posición vertical".into(), conductivity: 3.0, g_gln: 0.7588236
    },
        "Doble baja emisividad".into() => Glass { name: "Doble baja emisividad".into(), group: "Vidrios".into(), conductivity: 1.9, g_gln: 0.8
    },
        "VER_DB2_4-20-661a".into() => Glass { name: "VER_DB2_4-20-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB3_4-20-441a".into() => Glass { name: "VER_DB3_4-20-441a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "VER_DB1_4-20-441a".into() => Glass { name: "VER_DB1_4-20-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DC_4-20-441a".into() => Glass { name: "HOR_DC_4-20-441a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DB3_4-20-331".into() => Glass { name: "HOR_DB3_4-20-331".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "VER_ML_331".into() => Glass { name: "VER_ML_331".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.6, g_gln: 0.86
    },
        "HOR_DB3_4-6-6".into() => Glass { name: "HOR_DB3_4-6-6".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-9-661a".into() => Glass { name: "HOR_DB2_4-9-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_DB1_4-12-441a".into() => Glass { name: "HOR_DB1_4-12-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB3_4-9-551a".into() => Glass { name: "HOR_DB3_4-9-551a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DC_4-6-551a".into() => Glass { name: "VER_DC_4-6-551a".into(), group: "Dobles en posición vertical".into(), conductivity: 3.2, g_gln: 0.7588236
    },
        "Doble baja emisividad argon".into() => Glass { name: "Doble baja emisividad argon".into(), group: "Vidrios".into(), conductivity: 1.2, g_gln: 0.8
    },
        "HOR_DB2_4-9-4".into() => Glass { name: "HOR_DB2_4-9-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DC_4-9-4".into() => Glass { name: "HOR_DC_4-9-4".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "HOR_ML_331a".into() => Glass { name: "HOR_ML_331a".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.8, g_gln: 0.86
    },
        "VER_DB3_4-12-4".into() => Glass { name: "VER_DB3_4-12-4".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB2_4-15-441a".into() => Glass { name: "VER_DB2_4-15-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-20-441a".into() => Glass { name: "HOR_DB2_4-20-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DC_4-6-441a".into() => Glass { name: "VER_DC_4-6-441a".into(), group: "Dobles en posición vertical".into(), conductivity: 3.2, g_gln: 0.7588236
    },
        "VER_DC_4-20-441a".into() => Glass { name: "VER_DC_4-20-441a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "HOR_DB3_4-12-551a".into() => Glass { name: "HOR_DB3_4-12-551a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DB3_4-6-331".into() => Glass { name: "HOR_DB3_4-6-331".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DB2_4-6-661a".into() => Glass { name: "VER_DB2_4-6-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DC_4-15-551a".into() => Glass { name: "HOR_DC_4-15-551a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DB1_4-20-331".into() => Glass { name: "VER_DB1_4-20-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DC_4-12-331".into() => Glass { name: "HOR_DC_4-12-331".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "HOR_DC_4-6-6".into() => Glass { name: "HOR_DC_4-6-6".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.6, g_gln: 0.7588236
    },
        "VER_DC_4-9-331".into() => Glass { name: "VER_DC_4-9-331".into(), group: "Dobles en posición vertical".into(), conductivity: 3.0, g_gln: 0.7588236
    },
        "VER_DB2_4-12-331".into() => Glass { name: "VER_DB2_4-12-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DC_4-20-331".into() => Glass { name: "VER_DC_4-20-331".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "VER_DB3_4-15-4".into() => Glass { name: "VER_DB3_4-15-4".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "VER_DB3_4-20-331".into() => Glass { name: "VER_DB3_4-20-331".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "VER_DC_4-6-331".into() => Glass { name: "VER_DC_4-6-331".into(), group: "Dobles en posición vertical".into(), conductivity: 3.2, g_gln: 0.7588236
    },
        "HOR_DB2_4-20-4".into() => Glass { name: "HOR_DB2_4-20-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-12-551a".into() => Glass { name: "VER_DB3_4-12-551a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB1_4-9-4".into() => Glass { name: "VER_DB1_4-9-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_M_6".into() => Glass { name: "VER_M_6".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.7, g_gln: 0.86
    },
        "VER_DB1_4-6-4".into() => Glass { name: "VER_DB1_4-6-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DB1_4-6-4".into() => Glass { name: "HOR_DB1_4-6-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 3.0, g_gln: 0.7082353
    },
        "VER_M_4".into() => Glass { name: "VER_M_4".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.7, g_gln: 0.86
    },
        "VER_DB1_4-12-6".into() => Glass { name: "VER_DB1_4-12-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.0, g_gln: 0.7082353
    },
        "VER_DB2_4-6-551a".into() => Glass { name: "VER_DB2_4-6-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB3_4-15-331".into() => Glass { name: "HOR_DB3_4-15-331".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "VER_DB3_4-9-331".into() => Glass { name: "VER_DB3_4-9-331".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.9, g_gln: 0.7082353
    },
        "HOR_DB1_4-9-6".into() => Glass { name: "HOR_DB1_4-9-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "VER_DB2_4-9-6".into() => Glass { name: "VER_DB2_4-9-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "VER_DC_4-12-6".into() => Glass { name: "VER_DC_4-12-6".into(), group: "Dobles en posición vertical".into(), conductivity: 2.8, g_gln: 0.7588236
    },
        "HOR_DC_4-20-6".into() => Glass { name: "HOR_DC_4-20-6".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DB3_4-15-441a".into() => Glass { name: "VER_DB3_4-15-441a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DC_4-6-4".into() => Glass { name: "HOR_DC_4-6-4".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.6, g_gln: 0.7588236
    },
        "HOR_DC_4-20-551a".into() => Glass { name: "HOR_DC_4-20-551a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DB2_4-12-661a".into() => Glass { name: "VER_DB2_4-12-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DB1_4-15-4".into() => Glass { name: "VER_DB1_4-15-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DB2_4-12-4".into() => Glass { name: "HOR_DB2_4-12-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_ML_331".into() => Glass { name: "HOR_ML_331".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.8, g_gln: 0.86
    },
        "HOR_DB3_4-6-551a".into() => Glass { name: "HOR_DB3_4-6-551a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB3_4-9-6".into() => Glass { name: "HOR_DB3_4-9-6".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DC_4-15-4".into() => Glass { name: "VER_DC_4-15-4".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "HOR_DB3_4-20-4".into() => Glass { name: "HOR_DB3_4-20-4".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB1_4-9-661a".into() => Glass { name: "HOR_DB1_4-9-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB3_4-6-441a".into() => Glass { name: "HOR_DB3_4-6-441a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DC_4-20-6".into() => Glass { name: "VER_DC_4-20-6".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "VER_DB3_4-12-331".into() => Glass { name: "VER_DB3_4-12-331".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-6-441a".into() => Glass { name: "HOR_DB2_4-6-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DC_4-9-661a".into() => Glass { name: "HOR_DC_4-9-661a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DB2_4-15-551a".into() => Glass { name: "VER_DB2_4-15-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DC_4-15-551a".into() => Glass { name: "VER_DC_4-15-551a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "VER_DB3_4-6-4".into() => Glass { name: "VER_DB3_4-6-4".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DB3_4-20-6".into() => Glass { name: "VER_DB3_4-20-6".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DC_4-6-441a".into() => Glass { name: "HOR_DC_4-6-441a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.6, g_gln: 0.7588236
    },
        "VER_DB3_4-9-551a".into() => Glass { name: "VER_DB3_4-9-551a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.9, g_gln: 0.7082353
    },
        "VER_DB1_4-20-551a".into() => Glass { name: "VER_DB1_4-20-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_M_4".into() => Glass { name: "HOR_M_4".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.9, g_gln: 0.86
    },
        "VER_DB1_4-6-441a".into() => Glass { name: "VER_DB1_4-6-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DB2_4-6-4".into() => Glass { name: "HOR_DB2_4-6-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.8, g_gln: 0.7082353
    },
        "HOR_DB3_4-15-661a".into() => Glass { name: "HOR_DB3_4-15-661a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB1_4-12-6".into() => Glass { name: "HOR_DB1_4-12-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DB2_4-15-6".into() => Glass { name: "VER_DB2_4-15-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB1_4-12-661a".into() => Glass { name: "VER_DB1_4-12-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.0, g_gln: 0.7082353
    },
        "VER_DC_4-20-551a".into() => Glass { name: "VER_DC_4-20-551a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "VER_DB2_4-9-551a".into() => Glass { name: "VER_DB2_4-9-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB3_4-12-661a".into() => Glass { name: "HOR_DB3_4-12-661a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "VER_DB1_4-15-441a".into() => Glass { name: "VER_DB1_4-15-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DB1_4-20-4".into() => Glass { name: "HOR_DB1_4-20-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DC_4-12-4".into() => Glass { name: "VER_DC_4-12-4".into(), group: "Dobles en posición vertical".into(), conductivity: 2.8, g_gln: 0.7588236
    },
        "HOR_DC_4-12-661a".into() => Glass { name: "HOR_DC_4-12-661a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DC_4-15-441a".into() => Glass { name: "HOR_DC_4-15-441a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DC_4-6-661a".into() => Glass { name: "HOR_DC_4-6-661a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.5, g_gln: 0.7588236
    },
        "HOR_DC_4-15-331".into() => Glass { name: "HOR_DC_4-15-331".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DB1_4-6-6".into() => Glass { name: "HOR_DB1_4-6-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.9, g_gln: 0.7082353
    },
        "VER_DB2_4-20-331".into() => Glass { name: "VER_DB2_4-20-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-6-551a".into() => Glass { name: "HOR_DB2_4-6-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DB2_4-6-6".into() => Glass { name: "HOR_DB2_4-6-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.8, g_gln: 0.7082353
    },
        "HOR_DB3_4-15-4".into() => Glass { name: "HOR_DB3_4-15-4".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "VER_DB2_4-15-4".into() => Glass { name: "VER_DB2_4-15-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DB2_4-12-551a".into() => Glass { name: "HOR_DB2_4-12-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_DB1_4-15-6".into() => Glass { name: "HOR_DB1_4-15-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DB3_4-15-6".into() => Glass { name: "VER_DB3_4-15-6".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "HOR_DB3_4-6-661a".into() => Glass { name: "HOR_DB3_4-6-661a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB1_4-9-551a".into() => Glass { name: "HOR_DB1_4-9-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB3_4-6-4".into() => Glass { name: "HOR_DB3_4-6-4".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DC_4-12-551a".into() => Glass { name: "HOR_DC_4-12-551a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DB3_4-9-331".into() => Glass { name: "HOR_DB3_4-9-331".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB1_4-15-661a".into() => Glass { name: "VER_DB1_4-15-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DB1_4-20-551a".into() => Glass { name: "HOR_DB1_4-20-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DB2_4-9-441a".into() => Glass { name: "VER_DB2_4-9-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB3_4-20-441a".into() => Glass { name: "HOR_DB3_4-20-441a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DB3_4-9-661a".into() => Glass { name: "HOR_DB3_4-9-661a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB1_4-12-331".into() => Glass { name: "VER_DB1_4-12-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.0, g_gln: 0.7082353
    },
        "HOR_DC_4-6-331".into() => Glass { name: "HOR_DC_4-6-331".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.6, g_gln: 0.7588236
    },
        "VER_DC_4-15-441a".into() => Glass { name: "VER_DC_4-15-441a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "HOR_DC_4-12-4".into() => Glass { name: "HOR_DC_4-12-4".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "VER_DC_4-9-4".into() => Glass { name: "VER_DC_4-9-4".into(), group: "Dobles en posición vertical".into(), conductivity: 3.0, g_gln: 0.7588236
    },
        "VER_DB2_4-15-331".into() => Glass { name: "VER_DB2_4-15-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DC_4-20-661a".into() => Glass { name: "VER_DC_4-20-661a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "VER_DB1_4-6-6".into() => Glass { name: "VER_DB1_4-6-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DB2_4-15-331".into() => Glass { name: "HOR_DB2_4-15-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-9-4".into() => Glass { name: "VER_DB3_4-9-4".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.9, g_gln: 0.7082353
    },
        "VER_DB1_4-6-551a".into() => Glass { name: "VER_DB1_4-6-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "HOR_DC_4-9-6".into() => Glass { name: "HOR_DC_4-9-6".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "HOR_DB3_4-20-551a".into() => Glass { name: "HOR_DB3_4-20-551a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "VER_DB1_4-9-6".into() => Glass { name: "VER_DB1_4-9-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-6-441a".into() => Glass { name: "VER_DB3_4-6-441a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB2_4-6-441a".into() => Glass { name: "VER_DB2_4-6-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DC_4-9-441a".into() => Glass { name: "VER_DC_4-9-441a".into(), group: "Dobles en posición vertical".into(), conductivity: 3.0, g_gln: 0.7588236
    },
        "VER_DB2_4-9-661a".into() => Glass { name: "VER_DB2_4-9-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "HOR_DC_4-15-661a".into() => Glass { name: "HOR_DC_4-15-661a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_DB3_4-15-441a".into() => Glass { name: "HOR_DB3_4-15-441a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DB1_4-15-4".into() => Glass { name: "HOR_DB1_4-15-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB1_4-15-441a".into() => Glass { name: "HOR_DB1_4-15-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DC_4-12-551a".into() => Glass { name: "VER_DC_4-12-551a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.8, g_gln: 0.7588236
    },
        "VER_DB2_4-6-6".into() => Glass { name: "VER_DB2_4-6-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DB3_4-12-661a".into() => Glass { name: "VER_DB3_4-12-661a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "HOR_DC_4-6-551a".into() => Glass { name: "HOR_DC_4-6-551a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.5, g_gln: 0.7588236
    },
        "HOR_DB3_4-15-551a".into() => Glass { name: "HOR_DB3_4-15-551a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "VER_DC_4-6-661a".into() => Glass { name: "VER_DC_4-6-661a".into(), group: "Dobles en posición vertical".into(), conductivity: 3.2, g_gln: 0.7588236
    },
        "HOR_DC_4-12-441a".into() => Glass { name: "HOR_DC_4-12-441a".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DC_4-12-441a".into() => Glass { name: "VER_DC_4-12-441a".into(), group: "Dobles en posición vertical".into(), conductivity: 2.8, g_gln: 0.7588236
    },
        "VER_DB2_4-12-551a".into() => Glass { name: "VER_DB2_4-12-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DC_4-15-6".into() => Glass { name: "VER_DC_4-15-6".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "HOR_ML_661a".into() => Glass { name: "HOR_ML_661a".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.5, g_gln: 0.86
    },
        "VER_DB3_4-6-6".into() => Glass { name: "VER_DB3_4-6-6".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DC_4-20-4".into() => Glass { name: "VER_DC_4-20-4".into(), group: "Dobles en posición vertical".into(), conductivity: 2.7, g_gln: 0.7588236
    },
        "HOR_DB3_4-12-4".into() => Glass { name: "HOR_DB3_4-12-4".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.2, g_gln: 0.7082353
    },
        "HOR_DB1_4-9-331".into() => Glass { name: "HOR_DB1_4-9-331".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_DB1_4-12-441a".into() => Glass { name: "VER_DB1_4-12-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.0, g_gln: 0.7082353
    },
        "VER_DB2_4-6-4".into() => Glass { name: "VER_DB2_4-6-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "VER_ML_331a".into() => Glass { name: "VER_ML_331a".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.6, g_gln: 0.86
    },
        "VER_DB2_4-6-331".into() => Glass { name: "VER_DB2_4-6-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "VER_DB2_4-15-661a".into() => Glass { name: "VER_DB2_4-15-661a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB3_4-6-331".into() => Glass { name: "VER_DB3_4-6-331".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB1_4-6-661a".into() => Glass { name: "VER_DB1_4-6-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.6, g_gln: 0.7082353
    },
        "HOR_DB1_4-15-551a".into() => Glass { name: "HOR_DB1_4-15-551a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB2_4-12-6".into() => Glass { name: "HOR_DB2_4-12-6".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "HOR_DB2_4-15-551a".into() => Glass { name: "HOR_DB2_4-15-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "HOR_DC_4-15-4".into() => Glass { name: "HOR_DC_4-15-4".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.4, g_gln: 0.7588236
    },
        "HOR_DB3_4-9-441a".into() => Glass { name: "HOR_DB3_4-9-441a".into(), group: "Dobles bajo emisivos <0.03 en posición horizontal".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_DB3_4-12-6".into() => Glass { name: "VER_DB3_4-12-6".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.6, g_gln: 0.7082353
    },
        "VER_DB1_4-20-661a".into() => Glass { name: "VER_DB1_4-20-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DB1_4-15-6".into() => Glass { name: "VER_DB1_4-15-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "HOR_DB2_4-9-551a".into() => Glass { name: "HOR_DB2_4-9-551a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "Doble".into() => Glass { name: "Doble".into(), group: "Vidrios".into(), conductivity: 3.3, g_gln: 0.8
    },
        "HOR_DC_4-20-331".into() => Glass { name: "HOR_DC_4-20-331".into(), group: "Dobles en posición horizontal".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "VER_DB3_4-15-331".into() => Glass { name: "VER_DB3_4-15-331".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 1.4, g_gln: 0.7082353
    },
        "VER_DB1_4-20-6".into() => Glass { name: "VER_DB1_4-20-6".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353
    },
        "VER_DC_4-12-331".into() => Glass { name: "VER_DC_4-12-331".into(), group: "Dobles en posición vertical".into(), conductivity: 2.8, g_gln: 0.7588236
    },
        "HOR_DB1_4-20-661a".into() => Glass { name: "HOR_DB1_4-20-661a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.5, g_gln: 0.7082353
    },
        "HOR_DB2_4-12-441a".into() => Glass { name: "HOR_DB2_4-12-441a".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición horizontal".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DB1_4-9-441a".into() => Glass { name: "VER_DB1_4-9-441a".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición vertical".into(), conductivity: 2.3, g_gln: 0.7082353
    },
        "VER_ML_551a".into() => Glass { name: "VER_ML_551a".into(), group: "Monolíticos en posición vertical".into(), conductivity: 5.5, g_gln: 0.86
    },
        "HOR_DB1_4-9-4".into() => Glass { name: "HOR_DB1_4-9-4".into(), group: "Dobles bajo emisivos 0.1-0.2 en posición horizontal".into(), conductivity: 2.7, g_gln: 0.7082353
    },
        "VER_DB2_4-9-331".into() => Glass { name: "VER_DB2_4-9-331".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 2.1, g_gln: 0.7082353
    },
        "VER_DB3_4-6-661a".into() => Glass { name: "VER_DB3_4-6-661a".into(), group: "Dobles bajo emisivos <0.03 en posición vertical".into(), conductivity: 2.4, g_gln: 0.7082353
    },
        "VER_DC_4-6-4".into() => Glass { name: "VER_DC_4-6-4".into(), group: "Dobles en posición vertical".into(), conductivity: 3.3, g_gln: 0.7588236
    },
        "HOR_ML_551a".into() => Glass { name: "HOR_ML_551a".into(), group: "Monolíticos en posición horizontal".into(), conductivity: 6.6, g_gln: 0.86
    },
        "VER_DB2_4-12-4".into() => Glass { name: "VER_DB2_4-12-4".into(), group: "Dobles bajo emisivos 0.03-0.1 en posición vertical".into(), conductivity: 1.8, g_gln: 0.7082353 }

    },

    frames: hashmap! {
        "HOR_Con rotura de puente térmico mayor de 12 mm".into() => Frame { name: "HOR_Con rotura de puente térmico mayor de 12 mm".into(), group: "Metálicos en posición horizontal".into(), conductivity: 3.5, absorptivity: 0.7, width: 0.1
    },
        "HOR_Normal sin rotura de puente térmico".into() => Frame { name: "HOR_Normal sin rotura de puente térmico".into(), group: "Metálicos en posición horizontal".into(), conductivity: 7.2, absorptivity: 0.7, width: 0.1
    },
        "HOR_PVC dos cámaras".into() => Frame { name: "HOR_PVC dos cámaras".into(), group: "De PVC en posición horizontal".into(), conductivity: 2.4, absorptivity: 0.7, width: 0.1
    },
        "VER_Madera de densidad media alta".into() => Frame { name: "VER_Madera de densidad media alta".into(), group: "De Madera en posición vertical".into(), conductivity: 2.2, absorptivity: 0.7, width: 0.1
    },
        "VER_Con rotura de puente térmico entre 4 y 12 mm".into() => Frame { name: "VER_Con rotura de puente térmico entre 4 y 12 mm".into(), group: "Metálicos en posición vertical".into(), conductivity: 4.0, absorptivity: 0.7, width: 0.1
    },
        "VER_Madera de densidad media baja".into() => Frame { name: "VER_Madera de densidad media baja".into(), group: "De Madera en posición vertical".into(), conductivity: 2.0, absorptivity: 0.7, width: 0.1
    },
        "HOR_PVC tres cámaras".into() => Frame { name: "HOR_PVC tres cámaras".into(), group: "De PVC en posición horizontal".into(), conductivity: 1.9, absorptivity: 0.7, width: 0.1
    },
        "HOR_Madera de densidad media alta".into() => Frame { name: "HOR_Madera de densidad media alta".into(), group: "De Madera en posición horizontal".into(), conductivity: 2.4, absorptivity: 0.7, width: 0.1
    },
        "VER_PVC dos cámaras".into() => Frame { name: "VER_PVC dos cámaras".into(), group: "De PVC en posición vertical".into(), conductivity: 2.2, absorptivity: 0.7, width: 0.1
    },
        "VER_Normal sin rotura de puente térmico".into() => Frame { name: "VER_Normal sin rotura de puente térmico".into(), group: "Metálicos en posición vertical".into(), conductivity: 5.7, absorptivity: 0.7, width: 0.1
    },
        "VER_Con rotura de puente térmico mayor de 12 mm".into() => Frame { name: "VER_Con rotura de puente térmico mayor de 12 mm".into(), group: "Metálicos en posición vertical".into(), conductivity: 3.2, absorptivity: 0.7, width: 0.1
    },
        "HOR_Con rotura de puente térmico entre 4 y 12 mm".into() => Frame { name: "HOR_Con rotura de puente térmico entre 4 y 12 mm".into(), group: "Metálicos en posición horizontal".into(), conductivity: 4.5, absorptivity: 0.7, width: 0.1
    },
        "VER_PVC tres cámaras".into() => Frame { name: "VER_PVC tres cámaras".into(), group: "De PVC en posición vertical".into(), conductivity: 1.8, absorptivity: 0.7, width: 0.1
    },
        "HOR_Madera de densidad media baja".into() => Frame { name: "HOR_Madera de densidad media baja".into(), group: "De Madera en posición horizontal".into(), conductivity: 2.1, absorptivity: 0.7, width: 0.1
    },
        "Met - Gris claro".into() => Frame { name: "Met - Gris claro".into(), group: "Marcos".into(), conductivity: 5.7, absorptivity: 0.4, width: 0.1
    },
        "Mrpt - Gris claro".into() => Frame { name: "Mrpt - Gris claro".into(), group: "Marcos".into(), conductivity: 3.2, absorptivity: 0.4, width: 0.1
    },
        "Mpvc o mad - Gris claro".into() => Frame { name: "Mpvc o mad - Gris claro".into(), group: "Marcos".into(), conductivity: 2.2, absorptivity: 0.4, width: 0.1 }

    },
});
