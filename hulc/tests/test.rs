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
fn test_polygon() {
    use bdl::{BdlBlock, Polygon};
    let polblk: BdlBlock =
        r#"\"P01_E01_Pol2\" = POLYGON                                             
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
fn test_polygon2() {
    use bdl::{BdlBlock, Polygon};
    let polblk: BdlBlock =
        r#"\"TEST_POLYGON\" = POLYGON                                             
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
fn test_walcons() {
    use bdl::{BdlBlock, WallCons};
    let wcblk: BdlBlock =
r#"\"Forjado interior\" = LAYERS
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
    assert_eq!(wc.thickness, vec![0.01, 0.02, 0.31, 0.01]);
}
