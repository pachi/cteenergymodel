// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use bemodel::{
    climatedata,
    energy::{ray_dir_to_sun, Intersectable, Ray, AABB},
    Geometry, Model, Window,
};
use nalgebra::{point, vector};

extern crate env_logger;
use log::info;

// Utilidades para tests ------------------

macro_rules! assert_almost_eq {
    ($a:expr, $b:expr) => {
        assert_almost_eq!($a, $b, 0.001)
    };
    ($a:expr, $b:expr, $c:expr) => {
        if ($a - $b).abs() > $c {
            panic!(
                "assertion failed: `abs(left - right) < {}`, (left: `{}`, right: `{}`)",
                $c, $a, $b
            );
        }
    };
}

fn get_window_by_name<'a>(model: &'a Model, win_name: &str) -> &'a Window {
    model.windows.iter().find(|w| w.name == win_name).unwrap()
}

// --------------

// Inicialización para los tests
// El logger solo se activa si es un test y emite diagnósticos si el test falla
// Basado en https://docs.rs/env_logger/0.7.1/env_logger/#capturing-logs-in-tests
// Se debe llamar a esta función al principio de cada test
fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn model_json_conversion() {
    init();

    let strdata = include_str!("./data/e4h_medianeras.json");
    let model = Model::from_json(strdata).unwrap();
    let climatezone = model.meta.climate;
    let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
    let n50data = model.n50();
    assert_almost_eq!(model.a_ref(), 1673.92, 0.01);
    assert_almost_eq!(model.compacity(), 3.17, 0.01);
    assert_almost_eq!(model.K().K, 0.37, 0.01);
    // En HULC es q_solul = 0.43
    // con model.update_fshobst = 0.47
    // con el modelo simple (solo retranqueos) = 0.53
    assert_almost_eq!(model.q_soljul(&totradjul).q_soljul, 0.47, 0.01);

    assert_almost_eq!(n50data.n50, 2.96, 0.01);
    assert_almost_eq!(n50data.n50_ref, 2.96, 0.01);
    assert_almost_eq!(n50data.walls_c_ref, 16.0, 0.01);
    assert_almost_eq!(n50data.walls_c, 16.0, 0.01);
    assert_almost_eq!(model.vol_env_net(), 4666.05, 0.01);
    assert_almost_eq!(model.vol_env_gross(), 5231.0, 0.1);

    let json = model.as_json().unwrap();
    let model = Model::from_json(&json).unwrap();
    let json2 = model.as_json().unwrap();
    assert_eq!(&json, &json2);

    // Sombras
    let sun_azimuth = 0.0;
    let sun_altitude = 45.0;
    let ray_dir = ray_dir_to_sun(sun_azimuth, sun_altitude);

    let occluders = model.collect_occluders();
    // Ventana P04_E03_PE009_V sunlit Fshobst_HULC = 0.58 - Bloquea Sombra011 + retranqueo 20cm
    let window = get_window_by_name(&model, "P04_E03_PE009_V");
    let ray_origins = model.ray_origins_for_window(window);
    assert_almost_eq!(
        model.sunlit_fraction(window, &ray_origins, &ray_dir, &occluders),
        0.6
    );

    // Ventana P01_E04_PE001_V Fshobst_HULC = 0.65 - Bloquea Sombra003 + retranqueo 20cm
    let window = get_window_by_name(&model, "P01_E04_PE001_V");
    let ray_origins = model.ray_origins_for_window(window);
    assert_almost_eq!(
        model.sunlit_fraction(window, &ray_origins, &ray_dir, &occluders),
        0.8
    );

    // P04_E03_PE009_V_8 Fshobst_HULC = 0.69 (retranqueo 20 cm, sin alero)
    let window = get_window_by_name(&model, "P04_E03_PE009_V_8");
    let ray_origins = model.ray_origins_for_window(window);
    assert_almost_eq!(
        model.sunlit_fraction(window, &ray_origins, &ray_dir, &occluders),
        0.8
    );
}

#[test]
fn model_json_unif() {
    init();

    let strdata = include_str!("./data/ejemploviv_unif.json");
    let mut model = Model::from_json(strdata).unwrap();
    let climatezone = model.meta.climate;
    let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
    let n50data = model.n50();
    assert_almost_eq!(model.a_ref(), 102.37, 0.01);
    assert_almost_eq!(model.compacity(), 1.36, 0.01);
    assert_almost_eq!(model.K().K, 0.62, 0.01);
    assert_almost_eq!(model.q_soljul(&totradjul).q_soljul, 0.59, 0.01);

    assert_almost_eq!(n50data.n50, 6.89, 0.01);
    assert_almost_eq!(n50data.n50_ref, 6.89, 0.01);
    assert_almost_eq!(model.vol_env_net(), 258.25, 0.01);
    assert_almost_eq!(model.vol_env_gross(), 292.79, 0.1);

    model.update_fshobst();
    info!(
        "sunlit map:\n{}",
        model
            .windows
            .iter()
            .map(|w| format!("{}: {}", w.name, w.fshobst))
            .collect::<Vec<_>>()
            .join("\n")
    );
    // HULC: 0.54
    assert_almost_eq!(model.q_soljul(&totradjul).q_soljul, 0.55, 0.01);
}

#[test]
fn intersections() {
    init();

    // Horizontal
    let geom1 = Geometry {
        tilt: 0.0,
        azimuth: 0.0,
        position: Some(point![0.0, 0.0, 0.0]),
        polygon: vec![
            point![0.0, 0.0],
            point![2.0, 0.0],
            point![2.0, 2.0],
            point![0.0, 2.0],
        ],
    };

    // Vertical a Sur
    let geom2 = Geometry {
        tilt: 90.0,
        azimuth: 0.0,
        position: Some(point![0.0, 0.0, 0.0]),
        polygon: vec![
            point![0.0, 0.0],
            point![2.0, 0.0],
            point![2.0, 2.0],
            point![0.0, 2.0],
        ],
    };

    // Vertical a Este
    let geom3 = Geometry {
        tilt: 90.0,
        azimuth: 90.0,
        position: Some(point![0.0, 0.0, 0.0]),
        polygon: vec![
            point![0.0, 0.0],
            point![2.0, 0.0],
            point![2.0, 2.0],
            point![0.0, 2.0],
        ],
    };

    // Vertical sur
    let geom4 = Geometry {
        tilt: 90.0,
        azimuth: 0.0,
        position: Some(point![9.11, 0.0, 9.5]),
        polygon: vec![
            point![0.0, 0.0],
            point![9.11, 0.0],
            point![9.11, 3.0],
            point![0.0, 3.0],
        ],
    };

    #[allow(clippy::approx_constant)]
    let testcases = vec![
        (
            &geom1,
            Ray::new(point![1.0, 1.0, 2.0], vector![0.0, 0.0, -1.0]),
            true,
        ),
        (
            &geom2,
            Ray::new(point![1.0, 1.0, 2.0], vector![0.0, 0.0, -1.0]),
            false,
        ),
        (
            &geom2,
            Ray::new(point![1.0, 2.0, 1.0], vector![0.0, -1.0, 0.0]),
            true,
        ),
        (
            &geom3,
            Ray::new(point![1.0, 1.0, 2.0], vector![0.0, 0.0, -1.0]),
            false,
        ),
        (
            &geom3,
            Ray::new(point![2.0, 1.0, 1.0], vector![-1.0, 0.0, 0.0]),
            true,
        ),
        (
            &geom3,
            Ray::new(point![-2.0, 1.0, 1.0], vector![1.0, 0.0, 0.0]),
            true,
        ),
        (
            &geom3,
            Ray::new(point![-2.0, 1.0, 1.0], vector![-1.0, 0.0, 0.0]),
            false,
        ),
        (
            &geom4,
            Ray::new(
                point![-0.709, 2.1975, 0.0],
                vector![0.0, -0.707106, 0.707106],
            ),
            false,
        ),
    ];

    for (geo, r, res) in testcases {
        info!("Inclinación: {}, azimuth: {}", geo.tilt, geo.azimuth);
        info!("Polígono: {:?}", geo.polygon);
        info!("Posición: {:?}", geo.position);
        info!("Rayo: {}, {}", r.origin, r.dir);
        let result = &geo.intersects(&r);
        info!("Intersección con rayo: {:?}", result);
        assert!(res == result.is_some());
    }
}

#[test]
fn aabb_intersections() {
    let aabb1 = AABB {
        min: point![1.0, 1.0, 1.0],
        max: point![5.0, 5.0, 5.0],
    };

    assert!(aabb1
        .intersects(&Ray::new(point![0.0, 0.0, 0.0], vector![1.0, 1.0, 1.0]))
        .is_some());

    assert!(aabb1
        .intersects(&Ray::new(point![6.0, 6.0, 6.0], vector![1.0, 1.0, 1.0]))
        .is_none());

    assert!(aabb1
        .intersects(&Ray::new(point![6.0, 6.0, 6.0], vector![-1.0, -1.0, -1.0]))
        .is_some());

    assert!(aabb1
        .intersects(&Ray::new(point![0.0, 2.0, 2.0], vector![1.0, 0.0, 0.0]))
        .is_some());

    assert!(aabb1
        .intersects(&Ray::new(point![0.0, 2.0, 2.0], vector![0.0, 0.0, 1.0]))
        .is_none());
}
