// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use bemodel::{
    energy::{ray_dir_to_sun, Intersectable, Ray, AABB},
    Model, WallGeom, Window,
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
fn parse() {
    init();

    let space = bemodel::Space {
        id: uuid::Uuid::parse_str("df9422f0-9693-6c17-d5ea-d3783d9c0b74").unwrap(),
        name: "P01_E01".to_string(),
        multiplier: 1.0,
        kind: bemodel::SpaceType::CONDITIONED,
        inside_tenv: true,
        height: 2.7,
        n_v: None,
        z: 0.0,
        loads: Some(uuid::Uuid::parse_str("be9422f0-9693-6c17-d5ea-d3783d9c0b74").unwrap()),
        sys_settings: Some(uuid::Uuid::parse_str("af9422f0-9693-6c17-d5ea-d3783d9c0b74").unwrap()),
    };
    let space_str = r#"{
        "id": "df9422f0-9693-6c17-d5ea-d3783d9c0b74",
        "name": "P01_E01",
        "kind": "CONDITIONED",
        "inside_tenv": true,
        "height": 2.7,
        "space_conds": "be9422f0-9693-6c17-d5ea-d3783d9c0b74",
        "system_conds": "af9422f0-9693-6c17-d5ea-d3783d9c0b74"
      }"#;
    let sp: bemodel::Space = serde_json::from_str(space_str).unwrap();

    assert_eq!(sp.multiplier, space.multiplier);
}

#[test]
fn model_json_e4h_medianeras() {
    init();

    let strdata = include_str!("./data/e4h_medianeras.json");
    let model = Model::from_json(strdata).unwrap();

    // // Cubiertas
    let wall = model.get_wall_by_name("P04_E01_CUB001").unwrap();
    assert_almost_eq!(wall.area(), 96.14, 0.01);
    let wall = model.get_wall_by_name("P04_E02_CUB001").unwrap();
    assert_almost_eq!(wall.area(), 95.45, 0.01);

    let ind = model.energy_indicators();
    assert_almost_eq!(ind.area_ref, 1673.56, 0.1);
    assert_almost_eq!(ind.compacity, 3.17, 0.01);
    assert_almost_eq!(ind.K_data.K, 0.37, 0.01);
    // En HULC es q_solul = 0.43
    // con compute_fshobst = 0.47
    // con el modelo simple (solo retranqueos) = 0.53
    assert_almost_eq!(ind.q_soljul_data.q_soljul, 0.47, 0.01);

    assert_almost_eq!(ind.n50_data.n50, 2.96, 0.01);
    assert_almost_eq!(ind.n50_data.n50_ref, 2.96, 0.01);
    assert_almost_eq!(ind.n50_data.walls_c_ref, 16.0, 0.01);
    assert_almost_eq!(ind.n50_data.walls_c, 16.0, 0.01);
    assert_almost_eq!(ind.vol_env_net, 4671.36, 0.1);
    assert_almost_eq!(ind.vol_env_gross, 5229.93, 0.1);

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
fn model_json_ejemploviv_unif() {
    init();

    let strdata = include_str!("./data/ejemploviv_unif.json");
    let model = Model::from_json(strdata).unwrap();

    // Datos geométricos

    // Espacio
    let spc = model.get_space_by_name("P01_E01").unwrap();
    assert_almost_eq!(spc.area(&model.walls), 25.04, 0.01);
    assert_almost_eq!(spc.height_net(&model.walls, &model.cons), 2.48, 0.01);
    assert_almost_eq!(
        spc.slab_char_dim(&model.walls, &model.spaces)
            .unwrap_or_default(),
        6.95,
        0.01
    );

    // Forjado interior
    let wall = model.get_wall_by_name("P02_E01_FI001").unwrap();
    assert_almost_eq!(wall.area(), 11.12, 0.01);
    assert_almost_eq!(wall.area_net(&model.windows), 11.12, 0.01);
    assert_almost_eq!(wall.perimeter(), 13.37, 0.01);

    // Solera
    let wall = model.get_wall_by_name("P01_E01_FTER001").unwrap();
    assert_almost_eq!(wall.area(), 25.04, 0.01);
    assert_almost_eq!(wall.area_net(&model.windows), 25.04, 0.01);
    assert_eq!(wall.space, model.get_space_by_name("P01_E01").unwrap().id);

    // Pared exterior
    let wall = model.get_wall_by_name("P01_E01_PE003").unwrap();
    assert_almost_eq!(wall.area(), 9.76, 0.01);

    let wall = model.get_wall_by_name("P01_E01_PE001").unwrap();
    assert_almost_eq!(wall.area(), 9.71, 0.01);

    // Medianera
    let wall = model.get_wall_by_name("P02_E02_ME001").unwrap();
    assert_almost_eq!(wall.area(), 5.27, 0.01);

    // Muro interior
    let wall = model.get_wall_by_name("P02_E01_PE001").unwrap();
    assert_almost_eq!(wall.area(), 10.79, 0.01);
    assert_almost_eq!(wall.area_net(&model.windows), 9.35, 0.01);
    assert_eq!(wall.space, model.get_space_by_name("P02_E01").unwrap().id);

    let win = model.get_window_by_name("P02_E01_PE001_V").unwrap();
    assert_almost_eq!(win.area(), 1.44, 0.01);
    assert_eq!(
        win.wall,
        model.get_wall_by_name("P02_E01_PE001").unwrap().id
    );

    // Indicators

    let ind = model.energy_indicators();
    assert_almost_eq!(ind.area_ref, 102.33, 0.1);
    assert_almost_eq!(ind.compacity, 1.36, 0.01);
    assert_almost_eq!(ind.K_data.K, 0.62, 0.01);
    assert_almost_eq!(ind.q_soljul_data.q_soljul, 0.55, 0.01); // HULC 0.54

    assert_almost_eq!(ind.n50_data.n50, 6.89, 0.01);
    assert_almost_eq!(ind.n50_data.n50_ref, 6.89, 0.01);
    assert_almost_eq!(ind.vol_env_net, 257.98, 0.1);
    assert_almost_eq!(ind.vol_env_gross, 292.67, 0.1);

    // Ventanas
    let winid = model.get_window_by_name("P02_E01_PE001_V").unwrap().id;
    assert_almost_eq!(
        ind.props.windows.get(&winid).unwrap().f_shobst.unwrap(),
        0.76,
        0.01
    );
}

#[test]
fn intersections() {
    init();

    // Horizontal
    let geom1 = WallGeom {
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
    let geom2 = WallGeom {
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
    let geom3 = WallGeom {
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
    let geom4 = WallGeom {
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
