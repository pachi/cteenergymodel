/* -*- coding: utf-8 -*-

Copyright (c) 2018-2019 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use hulc2envolventecte::{bdl, collect_hulc_data, ctehexml, find_hulc_files, tbl, utils};
use std::convert::TryFrom;

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
    use bdl::Polygon;
    use hulc2envolventecte::bdl::BdlBlock;
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
    assert_eq!(pol.area(), 76.306793);
    assert_eq!(pol.edge_indices("V1").unwrap(), [0, 1]);
    assert_eq!(pol.edge_indices("V6").unwrap(), [5, 0]);
    assert_eq!(pol.edge_length("V3"), 18.22 - 10.86);
}

#[test]
fn test_polygon2() {
    use bdl::Polygon;
    use hulc2envolventecte::bdl::BdlBlock;
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
    assert_eq!(pol.area(), 4.5);
    assert_eq!(pol.perimeter(), 8.2426405);
    assert_eq!(pol.edge_indices("V1").unwrap(), [0, 1]);
    assert_eq!(pol.edge_indices("V6").unwrap(), [5, 0]);
    assert_eq!(pol.edge_length("V3"), 1.0);
    // lado horizontal hacia la derecha
    assert_eq!(pol.edge_orient("V1", 0.0), 0.0);
    // lado inclinado 45º hacia la derecha-arriba
    assert_eq!(pol.edge_orient("V2", 0.0), 45.0);
    // lado vertical hacia arriba
    assert_eq!(pol.edge_orient("V3", 0.0), 90.0);
    // lado horizontal hacia la izquierda
    assert_eq!(pol.edge_orient("V4", 0.0), 180.0);
    // lado inclinado 45º hacia la izquierda-abajo
    assert_eq!(pol.edge_orient("V5", 0.0), 225.0);
    // lado inclinado 45º hacia la derecha-abajo
    assert_eq!(pol.edge_orient("V6", 0.0), 315.0);
    // V1 con norte desviado 45º
    assert_eq!(pol.edge_orient("V1", 45.0), 315.0);
    // V5 con norte desviado 45º
    assert_eq!(pol.edge_orient("V5", 45.0), 180.0);
    // V2 con norte desviado 45º
    assert_eq!(pol.edge_orient("V2", 45.0), 0.0);
}

#[test]
fn test_test_spaces_caso_a() {
    let hulcfiles = find_hulc_files("tests/casoA").unwrap();
    let tbl = tbl::parse(&hulcfiles.tbl).unwrap();
    let xmldata = ctehexml::parse(&hulcfiles.ctehexml).unwrap();
    let bdl = xmldata.bdldata;

    for s in tbl.spaces {
        let spc = bdl.get_space(&s.name).unwrap();
        let poly = bdl.polygons.get(&spc.polygon).unwrap();
        assert_eq!(s.area, poly.area())
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
        floors,
        spaces,
        walls,
        windows,
        shadings,
        polygons,
        constructions,
        spaceconds,
        systemconds,
        schedules,
    } = bdldb;
    // println!("{:#?}", db);
    // println!("{:#?}", constructions);
    // println!("{:#?}", floors);
    // println!("{:#?}", spaces);
    println!("{:#?}", walls);
    // println!("{:#?}", shadings);
    // println!("{:#?}", polygons);

    // Cálculos básicos sobre elementos de la envolvente
    // - TODO: perímetro
    // TODO: Hacer más casos de orientación respecto al sur, con muros definidos con AZIMUTH
    // ya que ahora mismo solo se prueban los definidos por vértices y no está claro
    // si los valores que se obtienen en ese parámetro son respecto al norte (los de espacios sí)

    // Forjado interior
    let w = bdldb.get_wall("P02_E01_FI001").unwrap();
    assert_eq!(w.gross_area(bdldb).unwrap(), 49.985004);
    assert_eq!(w.net_area(bdldb).unwrap(), 49.985004);
    assert_eq!(w.space, "P02_E01");
    assert_eq!(w.tilt(), 180.0);
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0); // Horizontal

    // Solera
    let w = bdldb.get_wall("P01_E01_FTER001").unwrap();
    assert_eq!(w.gross_area(bdldb).unwrap(), 50.0);
    assert_eq!(w.net_area(bdldb).unwrap(), 50.0);
    assert_eq!(w.space, "P01_E01");
    assert_eq!(w.tilt(), 180.0);
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0); // Horizontal

    // Pared exterior
    let w = bdldb.get_wall("P01_E01_PE003").unwrap();
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0); // Norte
    let w = bdldb.get_wall("P04_E01_ME001").unwrap();
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0); // Norte
    assert_eq!(w.gross_area(bdldb).unwrap(), 17.5);

    // Muro exterior
    let w = bdldb.get_wall("P01_E01_PE001").unwrap();
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 180.0); // Sur

    // Muro exterior
    let w = bdldb.get_wall("P02_E01_PE003").unwrap();
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 90.0); // Este

    // Muro interior
    let w = bdldb.get_wall("P02_E01_PE001").unwrap();
    assert_eq!(w.gross_area(bdldb).unwrap(), 30.0);
    assert_eq!(w.net_area(bdldb).unwrap(), 28.0);
    assert_eq!(w.space, "P02_E01");
    assert_eq!(w.tilt(), 90.0);
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 270.0); // Oeste

    let v = bdldb.get_window("P02_E01_PE001_V").unwrap();
    assert_eq!(v.area(), 2.0);
    assert_eq!(v.wall, "P02_E01_PE001");
    assert_eq!(v.tilt(bdldb).unwrap(), 90.0);
    assert_eq!(v.azimuth(0.0, bdldb).unwrap(), 270.0); // Oeste

    // Cubiertas
    let w = bdldb.get_wall("P03_E01_CUB001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.0, 0.005);
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 0.0); // Horizontal
    assert_eq!(w.tilt(), 0.0); // Horizontal
    let w = bdldb.get_wall("P04_E01_CUB001").unwrap();
    assert_almost_eq!(w.gross_area(bdldb).unwrap(), 50.99020, 0.005);
    assert_eq!(w.azimuth(0.0, bdldb).unwrap(), 90.0); // Este
    assert_eq!(w.tilt(), 11.30993);

}

#[test]
fn test_test_caso_a() {
    let hulcfiles = find_hulc_files("tests/casoA").unwrap();
    let data = collect_hulc_data(&hulcfiles).unwrap();
    assert_eq!(data.a_util_ref(), 400.0);
    assert_eq!(data.clima, "D3");
    assert_eq!(data.envolvente.huecos.len(), 10);
    eprintln!("XXXX: {:?}", data.envolvente.opacos);
    assert_eq!(data.envolvente.opacos.len(), 19);
    assert_eq!(data.envolvente.pts.len(), 7);
}

#[test]
fn test_test_caso_c() {
    let hulcfiles = find_hulc_files("tests/casoC").unwrap();
    let data = collect_hulc_data(&hulcfiles).unwrap();
    assert_eq!(data.a_util_ref(), 400.0);
    assert_eq!(data.clima, "D3");
    assert_eq!(data.envolvente.huecos.len(), 9);
    assert_eq!(data.envolvente.opacos.len(), 27);
    assert_eq!(data.envolvente.pts.len(), 7);
}

// Caso más antiguo con archivo generado con el HULC2018 que salió a información pública
#[test]
fn parse_test_data() {
    let hulcfiles = find_hulc_files("tests/data").unwrap();
    let data = collect_hulc_data(&hulcfiles).unwrap();
    assert_eq!(data.a_util_ref(), 1673.92);
    assert_eq!(data.clima, "D3");
    assert_eq!(data.envolvente.huecos.len(), 92);
    assert_eq!(data.envolvente.opacos.len(), 68);
    assert_eq!(data.envolvente.pts.len(), 6);
}

#[test]
fn parse_test_data2() {
    let hulcfiles = find_hulc_files("tests/ejemplopmt_HuecosOK").unwrap();
    // Las versiones más nuevas usan la coma en KyGananciasSolares.txt como separador decimal
    let data = collect_hulc_data(&hulcfiles).unwrap();
    assert_eq!(data.a_util_ref(), 1073.76);
    assert_eq!(data.clima, "B3");
    assert_eq!(data.envolvente.huecos.len(), 29);
    assert_eq!(data.envolvente.opacos.len(), 60);
    assert_eq!(data.envolvente.pts.len(), 7);
}

#[ignore]
#[test]
fn parse_lider_bdl() {
    let mut count: u32 = 0;
    for ff in std::fs::read_dir("tests/liderdata/").unwrap() {
        let file = ff.unwrap().path().to_str().unwrap().to_string();
        if !file.ends_with(".CTE") && !file.ends_with(".cte") {
            continue;
        };
        println!("Examinando archivo {:#?}", file);
        let strdata = utils::read_latin1_file(&file).unwrap();
        let _data = bdl::Data::new(&strdata).unwrap();
        count += 1;
    }
    println!("Comprobados {} archivos antiguos", count);
}
