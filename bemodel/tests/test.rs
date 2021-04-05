// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use bemodel::{climatedata, Model};

// Utilidades para tests ------------------

/// Redondea valor a 2 decimales
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}

macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $c:expr) => {
        if ($a - $b).abs() > $c {
            panic!(
                "assertion failed: `abs(left - right) < {}`, (left: `{}`, right: `{}`)",
                $c, $a, $b
            );
        }
    };
}

// --------------

#[test]
fn model_json_conversion() {
    let strdata = include_str!("./data/e4h_medianeras.json");
    let model = Model::from_json(&strdata).unwrap();
    let climatezone = model.meta.climate;
    let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
    assert_almost_eq!(model.a_ref(), 1673.92, 0.01);
    assert_almost_eq!(model.compacity(), 3.17, 0.01);
    assert_almost_eq!(model.K_he2019().K, 0.37, 0.01);
    assert_almost_eq!(model.q_soljul(&totradjul).q_soljul, 0.43, 0.01);
    assert_almost_eq!(model.n50(), 2.96, 0.01);
    assert_almost_eq!(model.n50_he2019().n50, 2.96, 0.01);
    assert_almost_eq!(model.C_o(), 16.0, 0.01);
    assert_almost_eq!(model.C_o_he2019(), 16.0, 0.01);
    assert_almost_eq!(model.vol_env_net(), 4666.05, 0.01);
    assert_almost_eq!(model.vol_env_gross(), 5231.0, 0.1);

    let json = model.as_json().unwrap();
    let model = Model::from_json(&json).unwrap();
    let json2 = model.as_json().unwrap();
    assert_eq!(&json, &json2);
}
