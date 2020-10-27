// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::convert::TryFrom;

use hulc2envolventecte::{
    collect_hulc_data,
    cte::{climatedata, ClimateZone, Model},
    parsers::{bdl, ctehexml, kyg, tbl},
    utils::{fround2, read_file, read_latin1_file},
};

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
    assert_almost_eq!(pol.area(), 76.306793, 0.001);
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

#[test]
fn test_test_spaces_caso_a() {
    let tbl = tbl::parse("tests/casoA/NewBDL_O.tbl").unwrap();
    let ctehexmlpath = ctehexml::find_ctehexml("tests/casoA").unwrap();
    let xmldata = ctehexml::parse(&ctehexmlpath.unwrap()).unwrap();
    let bdl = xmldata.bdldata;

    for (name, space) in tbl.spaces {
        let spc = bdl.get_space(&name).unwrap();
        let poly = &spc.polygon;
        assert_almost_eq!(space.area, poly.area(), 0.001)
    }
}

#[test]
fn test_bdl_parse() {
    let _data = ctehexml::parse("tests/00_plurif_s3_v0_d3/00_plurif_s3_v0_d3.ctehexml").unwrap();
    let _data = ctehexml::parse("tests/casoA/casoa.ctehexml").unwrap();
    let data = ctehexml::parse("tests/casoC/casoc.ctehexml").unwrap();
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

    // TODO: Hacer más casos de orientación respecto al sur, con muros definidos con AZIMUTH
    // ya que ahora mismo solo se prueban los definidos por vértices y no está claro
    // si los valores que se obtienen en ese parámetro son respecto al norte (los de espacios sí)

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
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0, 0.001); // Horizontal

    // Solera
    let w = bdldb.get_wall("P01_E01_FTER001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.0, 0.001);
    assert_almost_eq!(w.net_area(bdldb).unwrap(), 50.0, 0.001);
    assert_eq!(w.space, "P01_E01");
    assert_almost_eq!(w.tilt, 180.0, 0.001);
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0, 0.001); // Horizontal

    // Pared exterior
    let w = bdldb.get_wall("P01_E01_PE003").unwrap();
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0, 0.001); // Norte
    let w = bdldb.get_wall("P04_E01_ME001").unwrap();
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0, 0.001); // Norte
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 17.5, 0.001);

    // Muro exterior
    let w = bdldb.get_wall("P01_E01_PE001").unwrap();
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 180.0, 0.001); // Sur

    // Muro exterior
    let w = bdldb.get_wall("P02_E01_PE003").unwrap();
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 90.0, 0.001); // Este

    // Muro interior
    let w = bdldb.get_wall("P02_E01_PE001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 30.0, 0.001);
    assert_almost_eq!(w.net_area(bdldb).unwrap(), 28.0, 0.001);
    assert_eq!(w.space, "P02_E01");
    assert_almost_eq!(w.tilt, 90.0, 0.001);
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 270.0, 0.001); // Oeste

    let v = bdldb.get_window("P02_E01_PE001_V").unwrap();
    assert_almost_eq!(v.area(), 2.0, 0.001);
    assert_eq!(v.wall, "P02_E01_PE001");
    assert_almost_eq!(v.tilt(bdldb).unwrap(), 90.0, 0.001);
    assert_almost_eq!(v.azimuth(0.0, bdldb).unwrap(), 270.0, 0.001); // Oeste

    // Cubiertas
    let w = bdldb.get_wall("P03_E01_CUB001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.0, 0.005);
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0, 0.001); // Horizontal
    assert_almost_eq!(w.tilt, 0.0, 0.001); // Horizontal
    let w = bdldb.get_wall("P04_E01_CUB001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.99020, 0.005);
    assert_almost_eq!(w.azimuth(0.0, bdldb).unwrap(), 90.0, 0.001); // Este
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

#[test]
fn test_caso_a() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/casoA").unwrap();
    let kygpath = kyg::find_kyg("tests/casoA").unwrap();
    let tblpath = tbl::find_tbl("tests/casoA").unwrap();
    let data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
    assert_almost_eq!(data.a_ref(), 400.0, 0.001);
    assert_eq!(data.meta.climate, ClimateZone::D3);
    assert_eq!(data.windows.len(), 10);
    assert_eq!(data.walls.len(), 35); // 19 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 7 en kyg
    let results: Vec<&str> = vec![
        "P02_E01_FI001",
        "P02_E01_FI002",
        "P02_E01_ME001",
        "P02_E01_MED001",
        "P02_E01_PE001",
        "P02_E01_PE002",
        "P02_E01_PE003",
        "P03_E01_FI003",
    ];
    let space = data.get_space_by_name("P02_E01").unwrap();
    let mut wallsofspace = data
        .walls_of_space(&space.id)
        .map(|w| w.name.as_str())
        .collect::<Vec<_>>();
    wallsofspace.sort();
    assert_eq!(wallsofspace, results);
    // Suelo al exterior (aire), HULC=0.34
    let wall = data.get_wall_by_name("P02_E01_ME001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.33, 0.001);
    // Fachada exterior, HULC=0.30
    let wall = data.get_wall_by_name("P01_E01_ME001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.30, 0.001);
    // Cubierta exterior, HULC=0.34
    let wall = data.get_wall_by_name("P03_E01_FE004").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.34, 0.001);
    // Muro de sótano (z=0), HULC=0.0 (por no habitable)
    let wall = data.get_wall_by_name("P01_E02_TER001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.59, 0.001);
    // Solera (z=0), HULC=0.47
    let wall = data.get_wall_by_name("P01_E01_FTER001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.34, 0.001);
    // Forjado interior, HULC=1.37
    let wall = data.get_wall_by_name("P03_E01_FI003").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 1.37, 0.001);
    // Partición interior vertical con espacio no habitable, HULC=0.81
    let wall = data.get_wall_by_name("P01_E01_Med001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.81, 0.001);
    // Partición interior horizontal (suelo) con espacio no habitable y enterrado, HULC=0.65
    let wall = data.get_wall_by_name("P02_E01_FI002").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.54, 0.001);
    // Partición interior horizontal (techo) con espacio no habitable/acondicionado, HULC=0.77
    let wall = data.get_wall_by_name("P03_E01_FI001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.46, 0.001);
    // Partición interior vertical con espacio no habitable/acondicionado, HULC=0.68
    let wall = data.get_wall_by_name("P04_E01_Med001").unwrap();
    assert_almost_eq!(fround2(data.u_for_wall(&wall).unwrap()), 0.66, 0.001);

    // Cálculo de K, n50, C_o
    assert_almost_eq!(fround2(data.K_he2019().K), 0.51, 0.001);
    assert_almost_eq!(fround2(data.n50_he2019().n50), 4.58, 0.001);
    assert_almost_eq!(fround2(data.n50()), 5.32, 0.001);
    assert_almost_eq!(fround2(data.n50_he2019().n50), 4.58, 0.001);
    assert_almost_eq!(fround2(data.C_o_he2019()), 16.00, 0.001);
    assert_almost_eq!(fround2(data.C_o()), 18.97, 0.001);
}

#[test]
fn test_caso_c() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/casoC").unwrap();
    let kygpath = kyg::find_kyg("tests/casoC").unwrap();
    let tblpath = tbl::find_tbl("tests/casoC").unwrap();
    let data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
    assert_almost_eq!(data.a_ref(), 400.0, 0.001);
    assert_eq!(data.meta.climate, ClimateZone::D3);
    assert_eq!(data.windows.len(), 9);
    assert_eq!(data.walls.len(), 33); // 27 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 7 en kyg
}

// Caso más antiguo con archivo generado con el HULC2018 que salió a información pública
#[test]
fn parse_test_data() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/data").unwrap();
    let kygpath = kyg::find_kyg("tests/data").unwrap();
    let tblpath = tbl::find_tbl("tests/data").unwrap();
    let data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
    assert_almost_eq!(data.a_ref(), 1673.92, 0.001);
    assert_eq!(data.meta.climate, ClimateZone::D3);
    assert_eq!(data.windows.len(), 92);
    assert_eq!(data.walls.len(), 127); // 68 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 6 en kyg

    let json = data.as_json().unwrap();
    println!("{}", json);
    let model = Model::from_json(&json).unwrap();
    let json2 = model.as_json().unwrap();
    assert_eq!(&json, &json2);
}

#[test]
fn parse_test_data2() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/ejemplopmt_HuecosOK").unwrap();
    let kygpath = kyg::find_kyg("tests/ejemplopmt_HuecosOK").unwrap();
    let tblpath = tbl::find_tbl("tests/ejemplopmt_HuecosOK").unwrap();
    // Las versiones más nuevas usan la coma en KyGananciasSolares.txt como separador decimal
    let data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
    assert_almost_eq!(data.a_ref(), 1063.03, 0.001);
    assert_eq!(data.meta.climate, ClimateZone::B3);
    assert_eq!(data.windows.len(), 29);
    assert_eq!(data.walls.len(), 95); // 60 en ET
    assert_eq!(data.thermal_bridges.len(), 10); // 7 en kyg
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
        let strdata = read_latin1_file(&file).unwrap();
        let _data = bdl::Data::new(&strdata).unwrap();
        count += 1;
    }
    println!("Comprobados {} archivos antiguos", count);
}

#[test]
fn parse_json_to_model() {
    let strdata = read_file("tests/data/e4h_medianeras.json").unwrap();
    let model = Model::from_json(&strdata).unwrap();
    let climatezone = model.meta.climate;
    let totradjul = climatedata::total_radiation_in_july_by_orientation(&climatezone);
    assert_eq!(model.a_ref(), 1673.92);
    assert_almost_eq!(model.compacity(), 3.17, 0.01);
    assert_almost_eq!(model.K_he2019().K, 0.37, 0.01);
    assert_almost_eq!(model.q_soljul(&totradjul), 0.43, 0.01);
    assert_almost_eq!(model.n50(), 2.96, 0.01);
    assert_almost_eq!(model.n50_he2019().n50, 2.96, 0.01);
    assert_eq!(model.C_o(), 16.0);
    assert_eq!(model.C_o_he2019(), 16.0);
    assert_eq!(model.vol_env_net(), 4666.05);
    assert_eq!(model.vol_env_gross(), 5231.0);
}
