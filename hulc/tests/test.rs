// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::convert::TryFrom;

use hulc::bdl;
use na::Point2;

// Utilidades para tests ------------------

/// Redondea valor a 2 decimales
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}

macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $c:expr) => {
        if ($a - $b).abs() > $c {
            panic!(format!(
                "assertion failed: `abs(left - right) < {}`, (left: `{}`, right: `{}`)",
                $c, $a, $b
            ));
        }
    };
}

// --------------

#[test]
fn bdl_polygon() {
    use bdl::{BdlBlock, Polygon};
    let polblk: BdlBlock = r#""P01_E01_Pol2" = POLYGON                                             
    V1   =( 14.97, 11.39 )
    V2   =( 10.84, 11.39 )
    V3   =( 10.86, 0 )
    V4   =( 18.22, 0 )
    V5   =( 18.22, 9.04 )
    V6   =( 14.97, 9.04 )
    .."#
    .parse()
    .unwrap();
    let pol: Polygon = Polygon::try_from(polblk).unwrap();
    assert_almost_eq!(pol.area(), 76.307, 0.001);
    assert_eq!(pol.edge_vertices("V1").unwrap(), [&Point2::new(14.97, 11.39), &Point2::new(10.84, 11.39)]);
    assert_eq!(pol.edge_vertices("V6").unwrap(), [&Point2::new(14.97, 9.04), &Point2::new(14.97, 11.39)]);
    assert_almost_eq!(pol.edge_length("V3"), 18.22 - 10.86, 0.001);
}

#[test]
fn bdl_polygon2() {
    use bdl::{BdlBlock, Polygon};
    let polblk: BdlBlock = r#""TEST_POLYGON" = POLYGON                                             
    V1   =( 1, 1 )
    V2   =( 2, 1 )
    V3   =( 3, 2 )
    V4   =( 3, 3 )
    V5   =( 1, 3 )
    V6   =( 0, 2 )
    .."#
    .parse()
    .unwrap();
    let pol: Polygon = Polygon::try_from(polblk).unwrap();
    assert_almost_eq!(pol.area(), 4.5, 0.001);
    assert_almost_eq!(pol.perimeter(), 8.2426405, 0.001);
    assert_eq!(pol.edge_vertices("V1").unwrap(), [&Point2::new(1.0, 1.0), &Point2::new(2.0, 1.0)]);
    assert_eq!(pol.edge_vertices("V6").unwrap(), [&Point2::new(0.0, 2.0), &Point2::new(1.0, 1.0)]);
    assert_almost_eq!(pol.edge_length("V3"), 1.0, 0.001);
    // lado horizontal hacia la derecha
    assert_almost_eq!(pol.edge_normal_to_y("V1"), 180.0, 0.001);
    // lado inclinado 45º hacia la derecha-arriba
    assert_almost_eq!(pol.edge_normal_to_y("V2"), 135.0, 0.001);
    // lado vertical hacia arriba
    assert_almost_eq!(pol.edge_normal_to_y("V3"), 90.0, 0.001);
    // lado horizontal hacia la izquierda
    assert_almost_eq!(pol.edge_normal_to_y("V4"), 0.0, 0.001);
    // lado inclinado 45º hacia la izquierda-abajo
    assert_almost_eq!(pol.edge_normal_to_y("V5"), 315.0, 0.001);
    // lado inclinado 45º hacia la derecha-abajo
    assert_almost_eq!(pol.edge_normal_to_y("V6"), 225.0, 0.001);
}

#[test]
fn bdl_construction() {
    use bdl::{BdlBlock, Construction};
    let mut ccblk: BdlBlock = r#""muro_opaco0.40" =  CONSTRUCTION
    TYPE   = LAYERS  
    LAYERS = "muro_opaco"
    ABSORPTANCE = 0.400000
    ..
"#
    .parse()
    .unwrap();
    ccblk.parent = Some("Muro_ficticio".to_string());
    let cc = Construction::try_from(ccblk).unwrap();
    assert_eq!(cc.name, "muro_opaco0.40");
    assert_eq!(cc.wallcons, "muro_opaco");
    assert_almost_eq!(cc.absorptance.unwrap(), 0.40, 0.01);
}

#[test]
fn bdl_walcons() {
    use bdl::{BdlBlock, WallCons};
    let wcblk: BdlBlock =
r#""Forjado interior" = LAYERS
    GROUP        = "Forjados Interiores"
    NAME_CALENER = ""
    NAME         = "Forjado interior"
    TYPE-DEFINITION = 1
    MATERIAL     = ("Frondosa ligera 435 < d < 565","Cámara de aire sin ventilar horizontal 2 cm","FUr310_12","Mortero de yeso")
    THICKNESS = (           0.01,           0.05,           0.31,           0.01)
    LIBRARY       =  NO
    UTIL          =  YES
    IMAGE = ""
    DEFAULT = NO
..
"#        .parse()
.unwrap();
    let wc = WallCons::try_from(wcblk).unwrap();
    assert_eq!(wc.name, "Forjado interior");
    assert_eq!(wc.group, "Forjados Interiores");
    assert_eq!(
        wc.material,
        vec![
            "Frondosa ligera 435 < d < 565",
            "Cámara de aire sin ventilar horizontal 2 cm",
            "FUr310_12",
            "Mortero de yeso"
        ]
    );
    assert_eq!(wc.thickness, vec![0.01, 0.02, 0.31, 0.01]);
    assert_almost_eq!(wc.absorptance, 0.0, 0.01);
}

#[test]
fn bdl_wincons() {
    use bdl::{BdlBlock, WindowCons};
    let wcblk: BdlBlock = r#""ventana estandar" = GAP
    NAME           = "ventana estandar"
    TYPE           = 1
    GROUP          = "muro_cortina"
    GROUP-GLASS         = "Vidrios"
    GLASS-TYPE          = "Doble baja emisividad argon"
    GROUP-FRAME       = "Metálicos en posición vertical"
    NAME-FRAME        = "VER_Con rotura de puente térmico mayor de 12 mm"
    PORCENTAGE        = 20.000000
    INF-COEF          = 9.000000
    porcentajeIncrementoU = 10.000000
    NAME_CALENER      = ""
    TransmisividadJulio = 1.000000
    VIGENCIA = ( "A", "B", "C", "D", "E", "F")
    IMAGE = ""
    TRANSMITANCIA       =            5.7
    SHADING-COEF        =           0.86
    SHADE-COEF-SUMMER   =              1
    SHADE-COEF-WINTER   =              1
    MARKER-SUMMER       =              1
    MARKER-WINTER       =              1
    LIBRARY           =  NO
    UTIL              =  YES
    ISDOOR            = NO
    DEFAULT           = NO
   ..
"#
    .parse()
    .unwrap();
    let wc = WindowCons::try_from(wcblk).unwrap();
    assert_eq!(wc.name, "ventana estandar");
    assert_eq!(wc.group, "muro_cortina");
    assert_eq!(wc.glass, "Doble baja emisividad argon");
    assert_eq!(wc.glassgroup, "Vidrios");
    assert_eq!(wc.frame, "VER_Con rotura de puente térmico mayor de 12 mm");
    assert_eq!(wc.framegroup, "Metálicos en posición vertical");
    assert_almost_eq!(wc.framefrac, 0.20, 0.01);
    assert_almost_eq!(wc.infcoeff, 9.0, 0.1);
    assert_almost_eq!(wc.deltau, 10.0, 0.1);
    assert_almost_eq!(wc.gglshwi.unwrap(), 1.0, 0.1);
}

// TODO: material definido por resistencia
#[test]
fn bdl_material_properties() {
    use bdl::{BdlBlock, Material};
    let blk: BdlBlock = r#""FR Entrevigado de EPS moldeado descolgado -Canto 450 mm" = MATERIAL
    TYPE              = PROPERTIES
    THICKNESS         =           0.45
    THICKNESS_CHANGE         = YES
    THICKNESS_MAX         =              2
    THICKNESS_MIN         =          0.001
    CONDUCTIVITY      =      0.4787234
    DENSITY           =           1280
    SPECIFIC-HEAT     =           1000
    VAPOUR-DIFFUSIVITY-FACTOR =             60
    NAME          = "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm"
    GROUP         = "Forjados reticulares"
    IMAGE          = "ladrillo.bmp"
    NAME_CALENER   = "oldeado descolgado -Canto 450 "
    LIBRARY       = NO
    UTIL          =  NO
    OBSOLETE      = NO
    ..
"#
    .parse()
    .unwrap();
    let elem = Material::try_from(blk).unwrap();
    assert_eq!(
        elem.name,
        "FR Entrevigado de EPS moldeado descolgado -Canto 450 mm"
    );
    assert_eq!(elem.group, "Forjados reticulares");
    assert!(elem.resistance.is_none());
    let props = elem.properties.unwrap();
    assert_almost_eq!(props.thickness.unwrap(), 0.45, 0.01);
    assert_almost_eq!(props.conductivity, 0.4787, 0.0001);
    assert_almost_eq!(props.density, 1280.0, 0.1);
    assert_almost_eq!(props.specificheat, 1000.0, 0.1);
    assert_almost_eq!(props.vapourdiffusivity.unwrap(), 60.0, 0.1);
}

#[test]
fn bdl_glass() {
    use bdl::{BdlBlock, Glass};
    let blk: BdlBlock = r#""Vidrio Triple Bajo Emisivo" = GLASS-TYPE
    GROUP             = "Vidrios HULC2020"
    TYPE              = SHADING-COEF
    SHADING-COEF      =      0.5882353
    GLASS-CONDUCTANCE =           1.25
    NAME_CALENER      = ""
    LIBRARY       =  NO
    UTIL          =  NO
    ..
"#
    .parse()
    .unwrap();
    let elem = Glass::try_from(blk).unwrap();
    assert_eq!(elem.name, "Vidrio Triple Bajo Emisivo");
    assert_eq!(elem.group, "Vidrios HULC2020");
    assert_almost_eq!(elem.conductivity, 1.25, 0.01);
    assert_almost_eq!(elem.g_gln, 0.5882353 * 0.86, 0.001);
}

#[test]
fn bdl_frame() {
    use bdl::{BdlBlock, Frame};
    let blk: BdlBlock = r#""Marco PVC_1" = NAME-FRAME
    GROUP         = "Marcos HULC2020"
    FRAME-WIDTH   =            0.1
    FRAME-CONDUCT =            1.3
    FRAME-ABS     =            0.7
    NAME_CALENER  = ""
    LIBRARY       = NO
    UTIL          =  NO
    ..
"#
    .parse()
    .unwrap();
    let elem = Frame::try_from(blk).unwrap();
    assert_eq!(elem.name, "Marco PVC_1");
    assert_eq!(elem.group, "Marcos HULC2020");
    assert_almost_eq!(elem.conductivity, 1.3, 0.01);
    assert_almost_eq!(elem.absorptivity, 0.7, 0.01);
    assert_almost_eq!(elem.width, 0.1, 0.01);
}

#[test]
fn bdl_floor() {
    use bdl::{BdlBlock, Floor};
    let blk: BdlBlock = r#""P02" = FLOOR
    Z             =               3
    POLYGON       =  "P02_Poligono1"
    FLOOR-HEIGHT  =              3
    SPACE-HEIGHT  =              3
    MULTIPLIER    = 12
    SHAPE         =  POLYGON
    PREVIOUS      =  "P01"
    ..
"#
    .parse()
    .unwrap();
    let elem = Floor::try_from(blk).unwrap();
    assert_eq!(elem.name, "P02");
    assert_almost_eq!(elem.z, 3.0, 0.01);
    assert_almost_eq!(elem.height, 3.0, 0.01);
    assert_almost_eq!(elem.multiplier, 12.0, 0.1);
    assert_eq!(elem.previous, "P01");
}

#[test]
fn bdl_shading_geometry() {
    use bdl::{BdlBlock, Shading};
    let blk: BdlBlock = r#""patio1_lateral2" = BUILDING-SHADE
    BULB-TRA = "Default.bulb"
    BULB-REF = "Default.bulb"
    TRAN     =              0
    REFL     =            0.7
    X        = 18.200001
    Y        = 9.030000
    Z        = 0.000000
    HEIGHT   = 12.500000
    WIDTH    = 3.500000
    TILT     = 90.000000
    AZIMUTH  = 180.000000
    ..
"#
    .parse()
    .unwrap();
    let elem = Shading::try_from(blk).unwrap();
    assert_eq!(elem.name, "patio1_lateral2");
    assert_almost_eq!(elem.tran, 0.0, 0.01);
    assert_almost_eq!(elem.refl, 0.7, 0.01);
    let geom = elem.geometry.unwrap();
    assert_almost_eq!(geom.x, 18.20, 0.01);
    assert_almost_eq!(geom.y, 9.03, 0.01);
    assert_almost_eq!(geom.z, 0.0, 0.01);
    assert_almost_eq!(geom.height, 12.5, 0.1);
    assert_almost_eq!(geom.width, 3.5, 0.1);
    assert_almost_eq!(geom.tilt, 90.0, 0.1);
    assert_almost_eq!(geom.azimuth, 180.0, 0.1);
}

#[test]
fn bdl_shading_vertices() {
    use bdl::{BdlBlock, Shading};
    let blk: BdlBlock = r#""Sombra016" = BUILDING-SHADE
    BULB-TRA = "Default.bulb"
    BULB-REF = "Default.bulb"
    TRAN     =              0
    REFL     =            0.7
    V1       =( 9.11, 25.7901, 12.5 )
    V2       =( 9.11, 27.04, 12.5 )
    V3       =( 6, 27.04, 12.5 )
    V4       =( 6, 25.7901, 12.5 )
    ..
"#
    .parse()
    .unwrap();
    let elem = Shading::try_from(blk).unwrap();
    assert_eq!(elem.name, "Sombra016");
    assert_almost_eq!(elem.tran, 0.0, 0.01);
    assert_almost_eq!(elem.refl, 0.7, 0.01);
    let vertices = elem.vertices.unwrap();
    assert_eq!(vertices.len(), 4);
    assert_almost_eq!(vertices[0].x, 9.11, 0.01);
    assert_almost_eq!(vertices[0].y, 25.7901, 0.0001);
    assert_almost_eq!(vertices[0].z, 12.5, 0.01);
}

#[test]
fn bdl_space() {
    use bdl::{BdlBlock, Space};
    let mut blk: BdlBlock = r#""P01_E01" = SPACE
    nCompleto = "P01_E01"
    HEIGHT        =            3.5
    SHAPE             = POLYGON
    POLYGON           = "P01_E01_Pol2"
    TYPE              = CONDITIONED
    SPACE-TYPE        = "Residencial"
    SYSTEM-CONDITIONS = "Residencial"
    SPACE-CONDITIONS  = "Residencial"
    FLOOR-WEIGHT      =              0
    MULTIPLIER        = 1
    MULTIPLIED        = 0
    PILLARS-NUMBERS   = 0
    FactorSuperficieUtil   = 1.0
    perteneceALaEnvolventeTermica   = SI
    INTERIOR-RADIATION  = FIXED
    POWER     = 4.4
    VEEI-OBJ  = 7.000000
    VEEI-REF  = 10.000000
    ..
"#
    .parse()
    .unwrap();
    blk.parent = Some("P01".to_string());
    let elem = Space::try_from(blk).unwrap();
    assert_eq!(elem.name, "P01_E01");
    assert_eq!(elem.stype, "CONDITIONED");
    // El polígono se inserta en el postproceso de bloques
    assert_almost_eq!(elem.height, 3.5, 0.1);
    // La cota se recibe del objeto floor en el postproceso. Inicialmente se pone a cero
    assert_almost_eq!(elem.z, 0.0, 0.1);
    assert_eq!(elem.insidete, true);
    assert_eq!(elem.floor, "P01");
    assert_almost_eq!(elem.power, 4.4, 0.1);
    assert_almost_eq!(elem.veeiobj, 7.0, 0.1);
    assert_almost_eq!(elem.veeiref, 10.0, 0.1);
    assert_eq!(elem.spacetype, "Residencial");
    assert_eq!(elem.spaceconds, "Residencial");
    assert_eq!(elem.systemconds, "Residencial");
    assert_almost_eq!(elem.floor_multiplier, 1.0, 0.1);
    assert_almost_eq!(elem.multiplier, 1.0, 0.1);
    assert_eq!(elem.ismultiplied, false);
    assert_eq!(elem.airchanges_h, None);
}

#[test]
fn bdl_thermalbridge() {
    use bdl::{BdlBlock, ThermalBridge};
    let blk: BdlBlock = r#""UNION_CUBIERTA" = THERMAL-BRIDGE
    LONG-TOTAL = 148.341034
    DEFINICION = 3
    TTL    = 0.226667
    LISTA-N   = ( "Cubiertas planas - Forjado no interrumpe el aislamiento en fachada")
    LISTA-L   = ( 100)
    LISTA-MURO   = ( 0.230000)
    LISTA-MARCO   = ( 0.200000)
    FRSI        = 0.28
    ANGLE-MIN   = 0
    ANGLE-MAX   = 135
    TYPE        = SLAB
    PARTITION   = BOTH
    ..
"#
    .parse()
    .unwrap();
    let elem = ThermalBridge::try_from(blk).unwrap();
    assert_eq!(elem.name, "UNION_CUBIERTA");
    assert_almost_eq!(elem.length.unwrap(), 148.34, 0.01);
    assert_almost_eq!(elem.psi, 0.227, 0.001);
    assert_almost_eq!(elem.frsi, 0.28, 0.01);
    assert_eq!(elem.tbtype, "SLAB");
    let geom = elem.geometry.unwrap();
    assert_almost_eq!(geom.anglemin, 0.0, 0.1);
    assert_almost_eq!(geom.anglemax, 135.0, 0.1);
    assert_eq!(geom.partition, "BOTH");
    let catalog = elem.catalog.unwrap();
    assert_eq!(catalog.classes.len(), 1);
    assert_eq!(
        catalog.classes[0],
        "Cubiertas planas - Forjado no interrumpe el aislamiento en fachada"
    );
    assert_almost_eq!(catalog.pcts[0], 100.0, 0.1);
    assert_almost_eq!(catalog.firstelems[0], 0.23, 0.01);
    assert_almost_eq!(catalog.secondelems.as_ref().unwrap()[0], 0.20, 0.01);
}

#[test]
fn bdl_wall_location_space() {
    use bdl::{BdlBlock, BoundaryType, Wall};
    let mut blk: BdlBlock = r#""P01_E02_PE006" = EXTERIOR-WALL
    ABSORPTANCE   =            0.6
    COMPROBAR-REQUISITOS-MINIMOS = YES
    TYPE_ABSORPTANCE    = 1
    COLOR_ABSORPTANCE   = 0
    DEGREE_ABSORPTANCE   = 2
    CONSTRUCCION_MURO  = "muro_opaco"
    CONSTRUCTION  = "muro_opaco0.60"
    LOCATION      = SPACE-V11
    ..
"#
    .parse()
    .unwrap();
    // Espacio madre
    blk.parent = Some("P01_E02".to_string());
    let elem = Wall::try_from(blk).unwrap();
    assert_eq!(elem.name, "P01_E02_PE006");
    assert_eq!(elem.space, "P01_E02");
    assert_eq!(elem.cons, "muro_opaco0.60");
    assert_eq!(elem.location, Some("V11".to_string()));
    assert_eq!(elem.bounds, BoundaryType::EXTERIOR);
    assert_almost_eq!(elem.tilt, 90.0, 0.1);
    assert_eq!(elem.nextto, None);
    assert!(elem.polygon.is_none());
}

#[test]
fn bdl_wall_location_top() {
    use bdl::{BdlBlock, BoundaryType, Wall};
    let mut blk: BdlBlock = r#""P03_E01_CUB001" = ROOF
    ABSORPTANCE   =            0.6
    COMPROBAR-REQUISITOS-MINIMOS = YES
    TYPE_ABSORPTANCE    = 0
    COLOR_ABSORPTANCE   = 0
    DEGREE_ABSORPTANCE   = 2
    CONSTRUCTION  = "cubierta"
    LOCATION      = TOP
    ..
"#
    .parse()
    .unwrap();
    // Espacio madre
    blk.parent = Some("P03_E01".to_string());
    let elem = Wall::try_from(blk).unwrap();
    assert_eq!(elem.name, "P03_E01_CUB001");
    assert_eq!(elem.space, "P03_E01");
    assert_eq!(elem.cons, "cubierta");
    assert_eq!(elem.location, Some("TOP".to_string()));
    assert_eq!(elem.bounds, BoundaryType::EXTERIOR);
    assert_almost_eq!(elem.tilt, 0.0, 0.1);
    assert_eq!(elem.nextto, None);
    assert!(elem.polygon.is_none());
}

#[test]
fn bdl_wall_polygon() {
    use bdl::{BdlBlock, BoundaryType, Wall};
    let mut blk: BdlBlock = r#""P03_E01_CUB001" = ROOF
    ABSORPTANCE   =            0.6
    COMPROBAR-REQUISITOS-MINIMOS = YES
    TYPE_ABSORPTANCE    = 0
    COLOR_ABSORPTANCE   = 0
    DEGREE_ABSORPTANCE   = 2
    CONSTRUCTION  = "SATE"
    X             =          2.496
    Y             =         -4.888
    Z             =              3
    AZIMUTH       =            180
    LOCATION      = TOP
    POLYGON       = "P03_E01_FE004_Poligono3"
    ..
"#
    .parse()
    .unwrap();
    // Espacio madre
    blk.parent = Some("P03_E01".to_string());
    let elem = Wall::try_from(blk).unwrap();
    assert_eq!(elem.name, "P03_E01_CUB001");
    assert_eq!(elem.space, "P03_E01");
    assert_eq!(elem.cons, "SATE");
    assert_eq!(elem.location, Some("TOP".to_string()));
    assert_eq!(elem.bounds, BoundaryType::EXTERIOR);
    assert_almost_eq!(elem.tilt, 0.0, 0.1);
    assert_eq!(elem.nextto, None);
    // El nombre del polígono se fija en el postproceso, no probamos que hay un polígono
    assert_almost_eq!(elem.x, 2.496, 0.001);
    assert_almost_eq!(elem.y, -4.888, 0.001);
    assert_almost_eq!(elem.z, 3.0, 0.001);
    assert_almost_eq!(elem.angle_with_space_north, 180.0, 0.1);
    assert_almost_eq!(elem.tilt, 0.0, 0.1);
}

#[test]
fn bdl_window() {
    use bdl::{BdlBlock, Window};
    let mut blk: BdlBlock = r#""P01_E02_PE005_V" = WINDOW
    X              =            0.2
    Y              =            0.1
    SETBACK        =              0
    HEIGHT         =            2.6
    WIDTH          =              5
    GAP            = "muro_cortina_controlsolar"
    COEFF = ( 1.000000, 1.000000, 1.000000, 1.000000)
    transmisividadJulio        = 0.220000
    GLASS-TYPE     = "Doble baja emisividad argon"
    FRAME-WIDTH   =      0.1329403
    FRAME-CONDUCT =       5.299999
    FRAME-ABS     =            0.7
    INF-COEF       =              9
    OVERHANG-A     =              0
    OVERHANG-B     =              0
    OVERHANG-W     =              0
    OVERHANG-D     =              0
    OVERHANG-ANGLE =              0
    LEFT-FIN-A     =              0
    LEFT-FIN-B     =              0
    LEFT-FIN-H     =              0
    LEFT-FIN-D     =              0
    RIGHT-FIN-A    =              0
    RIGHT-FIN-B    =              0
    RIGHT-FIN-H    =              0
    RIGHT-FIN-D    =              0
    ..
"#
    .parse()
    .unwrap();
    // Espacio madre
    blk.parent = Some("P03_E01_M01".to_string());
    let elem = Window::try_from(blk).unwrap();
    assert_eq!(elem.name, "P01_E02_PE005_V");
    assert_eq!(elem.wall, "P03_E01_M01");
    assert_eq!(elem.cons, "muro_cortina_controlsolar");
    assert_almost_eq!(elem.x, 0.2, 0.001);
    assert_almost_eq!(elem.y, 0.1, 0.001);
    assert_almost_eq!(elem.height, 2.6, 0.1);
    assert_almost_eq!(elem.width, 5.0, 0.1);
    assert_almost_eq!(elem.setback, 0.0, 0.1);
    let coefs = elem.coefs.unwrap();
    assert_eq!(coefs.len(), 4);
    assert_almost_eq!(coefs[0], 1.0, 0.1);
}
