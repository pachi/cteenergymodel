// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use hulc::{bdl, ctehexml, kyg, tbl};
use hulc2model::collect_hulc_data;

// Utilidades para tests ------------------

/// Redondea valor a 2 decimales
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}

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

// --------------

#[test]
fn test_caso_a() {
    let data = collect_hulc_data("tests/casoA", true, true).unwrap();
    assert_almost_eq!(data.a_ref(), 400.0, 0.001);
    assert_eq!(&data.meta.climate.to_string(), "D3");
    assert_eq!(data.windows.len(), 10);
    assert_eq!(data.walls.len(), 35); // 19 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 7 en kyg
    let mut wallsofspace = data
        .walls_of_space_iter(&data.space_by_name("P02_E01").unwrap().id)
        .map(|w| w.name.as_str())
        .collect::<Vec<_>>();
    wallsofspace.sort_unstable();
    assert_eq!(
        wallsofspace,
        vec![
            "P02_E01_FI001",
            "P02_E01_FI002",
            "P02_E01_ME001",
            "P02_E01_MED001",
            "P02_E01_PE001",
            "P02_E01_PE002",
            "P02_E01_PE003",
            "P03_E01_FI003",
        ]
    );
    // Suelo al exterior (aire), HULC=0.34
    let wall = data.wall_by_name("P02_E01_ME001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.33, 0.001);
    // Fachada exterior, HULC=0.30
    let wall = data.wall_by_name("P01_E01_ME001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.30, 0.001);
    // Cubierta exterior, HULC=0.34
    let wall = data.wall_by_name("P03_E01_FE004").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.34, 0.001);
    // Muro de sótano (z=0), HULC=0.0 (por no habitable)
    let wall = data.wall_by_name("P01_E02_TER001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.59, 0.001);
    // Solera (z=0), HULC=0.47
    let wall = data.wall_by_name("P01_E01_FTER001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.34, 0.001);
    // Forjado interior, HULC=1.37
    let wall = data.wall_by_name("P03_E01_FI003").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 1.37, 0.001);
    // Partición interior vertical con espacio no habitable, HULC=0.81
    let wall = data.wall_by_name("P01_E01_Med001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.81, 0.001);
    // Partición interior horizontal (suelo) con espacio no habitable y enterrado, HULC=0.65
    let wall = data.wall_by_name("P02_E01_FI002").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.54, 0.001);
    // Partición interior horizontal (techo) con espacio no habitable/acondicionado, HULC=0.77
    let wall = data.wall_by_name("P03_E01_FI001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.46, 0.001);
    // Partición interior vertical con espacio no habitable/acondicionado, HULC=0.68
    let wall = data.wall_by_name("P04_E01_Med001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(wall).unwrap()), 0.66, 0.001);

    // Cálculo de K, n50, C_o
    let n50data = data.n50();
    assert_almost_eq!(fround2(data.K().K), 0.51, 0.001);
    assert_almost_eq!(fround2(n50data.n50_ref), 4.58, 0.001); // HULC 4.33
    assert_almost_eq!(fround2(n50data.n50), 5.32, 0.001);
    assert_almost_eq!(fround2(n50data.walls_c_ref), 16.00, 0.001);
    assert_almost_eq!(fround2(n50data.walls_c), 18.97, 0.001);
}

#[test]
fn test_caso_c() {
    let data = collect_hulc_data("tests/casoC", true, true).unwrap();
    assert_almost_eq!(data.a_ref(), 400.0, 0.001);
    assert_eq!(&data.meta.climate.to_string(), "D3");
    assert_eq!(data.windows.len(), 9);
    assert_eq!(data.walls.len(), 33); // 27 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 7 en kyg
}

// Caso más antiguo con archivo generado con el HULC2018 que salió a información pública
#[test]
fn parse_test_data() {
    let data = collect_hulc_data("tests/data", true, true).unwrap();
    assert_almost_eq!(data.a_ref(), 1673.92, 0.001);
    assert_eq!(&data.meta.climate.to_string(), "D3");
    assert_eq!(data.windows.len(), 92);
    assert_eq!(data.walls.len(), 127); // 68 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 6 en kyg
}

#[test]
fn parse_test_data2() {
    // Las versiones más nuevas usan la coma en KyGananciasSolares.txt como separador decimal
    let data = collect_hulc_data("tests/ejemplopmt_HuecosOK", true, true).unwrap();
    assert_almost_eq!(data.a_ref(), 1063.03, 0.001);
    assert_eq!(&data.meta.climate.to_string(), "B3");
    assert_eq!(data.windows.len(), 29);
    assert_eq!(data.walls.len(), 95); // 60 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 7 en kyg
}

#[test]
fn test_kyg() {
    let kygpath = kyg::find_kyg("tests/casoA").unwrap().unwrap();
    let kyg = kyg::parse_from_path(kygpath).unwrap();
    assert_eq!(kyg.walls.len(), 19);
    assert_eq!(kyg.windows.len(), 10);
    assert_eq!(kyg.thermal_bridges.len(), 7);
    assert_eq!(kyg.hfactors.len(), 9);
    assert_almost_eq!(kyg.k, 0.51, 0.01);
}

#[test]
fn test_test_spaces_caso_a() {
    let tbl = tbl::parse("tests/casoA/NewBDL_O.tbl").unwrap();
    let ctehexmlpath = ctehexml::find_ctehexml("tests/casoA").unwrap();
    let xmldata = ctehexml::parse_from_path(&ctehexmlpath.unwrap()).unwrap();
    let bdl = xmldata.bdldata;

    for (name, space) in tbl.spaces {
        let spc = bdl.get_space(&name).unwrap();
        let poly = &spc.polygon;
        assert_almost_eq!(space.area, poly.area(), 0.001)
    }
}

#[test]
fn test_bdl_parse() {
    let _data =
        ctehexml::parse_from_path("tests/00_plurif_s3_v0_d3/00_plurif_s3_v0_d3.ctehexml").unwrap();
    let _data = ctehexml::parse_from_path("tests/casoA/casoa.ctehexml").unwrap();
    let data = ctehexml::parse_from_path("tests/casoC/casoc.ctehexml").unwrap();
    let bdldb = &data.bdldata;

    #[allow(unused_variables)]
    let bdl::Data {
        meta,
        db,
        spaces,
        walls,
        windows,
        tbridges,
        shadings,
        spaceconds,
        systemconds,
        schedules,
    } = bdldb;
    // println!("{:#?}", db);
    // println!("{:#?}", spaces);
    // println!("{:#?}", walls);
    // println!("{:#?}", shadings);

    // Cálculos básicos sobre elementos de la envolvente

    // Espacio
    let s = bdldb.get_space("P02_E01").unwrap();
    assert_almost_eq!(s.height, 3.0, 0.001); // Altura
    assert_almost_eq!(s.space_height(bdldb).unwrap(), 2.65, 0.001); // Altura libre
    assert_almost_eq!(s.area(), 150.0, 0.001); // Área 10m x 15m
    assert_almost_eq!(s.perimeter(), 50.0, 0.001); // Perímetro (10 + 15) x 2

    // Forjado interior
    let w = bdldb.get_wall("P02_E01_FI001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 49.985004, 0.001);
    assert_almost_eq!(w.net_area(bdldb).unwrap(), 49.985004, 0.001);
    assert_eq!(w.space, "P02_E01");
    assert_almost_eq!(w.tilt, 180.0, 0.001);
    assert_almost_eq!(w.angle_with_space_north, 90.0, 0.001); // Horizontal

    // Solera
    let w = bdldb.get_wall("P01_E01_FTER001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.0, 0.001);
    assert_almost_eq!(w.net_area(bdldb).unwrap(), 50.0, 0.001);
    assert_eq!(w.space, "P01_E01");
    assert_almost_eq!(w.tilt, 180.0, 0.001);
    assert_almost_eq!(w.angle_with_space_north, 180.0, 0.001); // Horizontal

    // Pared exterior
    let w = bdldb.get_wall("P01_E01_PE003").unwrap();
    assert_almost_eq!(w.angle_with_space_north, 0.0, 0.001); // Norte
    let w = bdldb.get_wall("P04_E01_ME001").unwrap();
    assert_almost_eq!(w.angle_with_space_north, 0.0, 0.001); // Norte
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 17.5, 0.001);

    // Muro exterior
    let w = bdldb.get_wall("P01_E01_PE001").unwrap();
    assert_almost_eq!(w.angle_with_space_north, 180.0, 0.001); // Sur

    // Muro exterior
    let w = bdldb.get_wall("P02_E01_PE003").unwrap();
    assert_almost_eq!(w.angle_with_space_north, 90.0, 0.001); // Este

    // Muro interior
    let w = bdldb.get_wall("P02_E01_PE001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 30.0, 0.001);
    assert_almost_eq!(w.net_area(bdldb).unwrap(), 28.0, 0.001);
    assert_eq!(w.space, "P02_E01");
    assert_almost_eq!(w.tilt, 90.0, 0.001);
    assert_almost_eq!(w.angle_with_space_north, 270.0, 0.001); // Oeste

    let v = bdldb.get_window("P02_E01_PE001_V").unwrap();
    assert_almost_eq!(v.area(), 2.0, 0.001);
    assert_eq!(v.wall, "P02_E01_PE001");

    // Cubiertas
    let w = bdldb.get_wall("P03_E01_CUB001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.0, 0.005);
    assert_almost_eq!(w.angle_with_space_north, 180.0, 0.001); // Horizontal
    assert_almost_eq!(w.tilt, 0.0, 0.001); // Horizontal
    let w = bdldb.get_wall("P04_E01_CUB001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.9902, 0.005);
    assert_almost_eq!(w.angle_with_space_north, 90.0, 0.001); // Este
    assert_almost_eq!(w.tilt, 11.30993, 0.001);

    // // Volumen acondicionado de la envolvente:
    // // - volumen de los espacios acondicionados
    // // - restamos volumen de los forjados interiores y de las cubiertas
    // let mut v = 0.0;
    // for spc in &bdldb.spaces {
    //     if spc.stype == "CONDITIONED" {
    //         v += spc.space_height(bdldb).unwrap() * spc.area(bdldb).unwrap();
    //     }
    // }
    // assert_almost_eq!(v, 1055.949951, 0.005);
}

// #[ignore]
#[test]
fn parse_lider_bdl() {
    let mut count: u32 = 0;
    for ff in std::fs::read_dir("tests/liderdata/").unwrap() {
        let file = ff.unwrap().path().to_str().unwrap().to_string();
        if !file.ends_with(".CTE") && !file.ends_with(".cte") {
            continue;
        };
        println!("Examinando archivo {:#?}", file);
        let _data = bdl::Data::new_from_path(&file).unwrap();
        count += 1;
    }
    println!("Comprobados {} archivos antiguos", count);
}

#[test]
fn convert_shading_vertices() {
    use bdl::{BdlBlock, Data};
    use std::convert::TryInto;

    // Sombra orientada al norte (Azimuth 180º) inclinación (tilt) de 135º sobre la horizontal (hacia abajo)
    // Normal (0, 0.707, -0.707)
    let blk1: BdlBlock = r#""Sombra006" = BUILDING-SHADE
    BULB-TRA = "Default.bulb"
    BULB-REF = "Default.bulb"
    TRAN     =              0
    REFL     =            0.7
    V1       =( 0, 10, 0 )
    V2       =( 0, 20, 10 )
    V3       =( 10, 20, 10 )
    V4       =( 10, 10, 0 )
         ..
"#
    .parse()
    .unwrap();

    // Sombra orientada al oeste (Azimuth 270 (o -90)) con inclinación (tilt) de 45º sobre la horizontal (hacia arriba)
    // Normal (-0.707, 0, 0.707)
    let blk2: BdlBlock = r#""Sombra007" = BUILDING-SHADE
    BULB-TRA = "Default.bulb"
    BULB-REF = "Default.bulb"
    TRAN     =              0
    REFL     =            0.7
    V1       =( 10, 0, 0 )
    V2       =( 20, 0, 10 )
    V3       =( 20, 10, 10 )
    V4       =( 10, 10, 0 )
         ..
"#
    .parse()
    .unwrap();

    let bdldata = Data {
        shadings: vec![blk1.try_into().unwrap(), blk2.try_into().unwrap()],
        ..Default::default()
    };
    let hdata = ctehexml::CtehexmlData {
        bdldata,
        ..Default::default()
    };
    let model: bemodel::Model = (&hdata).try_into().unwrap();
    assert_almost_eq!(model.shades[0].geometry.tilt, 135.0, 0.1);
    assert_almost_eq!(model.shades[0].geometry.azimuth, -180.0, 0.1);
    assert_almost_eq!(model.shades[1].geometry.tilt, 45.0, 0.1);
    assert_almost_eq!(model.shades[1].geometry.azimuth, -90.0, 0.1);
}
