// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::convert::TryFrom;

use hulc::bdl;

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
    assert_eq!(pol.edge_indices("V1").unwrap(), [0, 1]);
    assert_eq!(pol.edge_indices("V6").unwrap(), [5, 0]);
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
    assert_eq!(pol.edge_indices("V1").unwrap(), [0, 1]);
    assert_eq!(pol.edge_indices("V6").unwrap(), [5, 0]);
    assert_almost_eq!(pol.edge_length("V3"), 1.0, 0.001);
    // lado horizontal hacia la derecha
    assert_almost_eq!(pol.edge_orient("V1", 0.0), 0.0, 0.001);
    // lado inclinado 45º hacia la derecha-arriba
    assert_almost_eq!(pol.edge_orient("V2", 0.0), 45.0, 0.001);
    // lado vertical hacia arriba
    assert_almost_eq!(pol.edge_orient("V3", 0.0), 90.0, 0.001);
    // lado horizontal hacia la izquierda
    assert_almost_eq!(pol.edge_orient("V4", 0.0), 180.0, 0.001);
    // lado inclinado 45º hacia la izquierda-abajo
    assert_almost_eq!(pol.edge_orient("V5", 0.0), 225.0, 0.001);
    // lado inclinado 45º hacia la derecha-abajo
    assert_almost_eq!(pol.edge_orient("V6", 0.0), 315.0, 0.001);
    // V1 con norte desviado 45º
    assert_almost_eq!(pol.edge_orient("V1", 45.0), 315.0, 0.001);
    // V5 con norte desviado 45º
    assert_almost_eq!(pol.edge_orient("V5", 45.0), 180.0, 0.001);
    // V2 con norte desviado 45º
    assert_almost_eq!(pol.edge_orient("V2", 45.0), 0.0, 0.001);
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
