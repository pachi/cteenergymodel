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

use hulc2envolventecte::{bdl, collect_hulc_data, ctehexml, find_hulc_files, tbl};
use std::convert::TryFrom;

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
    .."#.parse().unwrap();
    let pol: Polygon = Polygon::try_from(polblk).unwrap();
    assert_eq!(pol.area(), 76.306793);
}

#[test]
fn test_test_spaces_caso_a() {
    let hulcfiles = find_hulc_files("tests/casoA").unwrap();
    let tbl = tbl::parse(&hulcfiles.tbl).unwrap();
    let xmldata = ctehexml::parse(&hulcfiles.ctehexml).unwrap();
    let bdl = xmldata.bdldata;

    for s in tbl.spaces {
        let spc = bdl.spaces.iter().find(|ss| &ss.name == &s.name).unwrap();
        let poly = bdl.polygons.get(&spc.polygon).unwrap();
        assert_eq!(s.area, poly.area())
    }
}

#[test]
fn test_bdl_parse() {
    let _data = ctehexml::parse("tests/00_plurif_s3_v0_d3/00_plurif_s3_v0_d3.ctehexml").unwrap();
    let _data = ctehexml::parse("tests/casoA/casoa.ctehexml").unwrap();
    let data = ctehexml::parse("tests/casoC/casoc.ctehexml").unwrap();
    let bdl::BdlData {
        meta: _,
        db,
        floors,
        spaces,
        env,
        shadings,
        polygons,
        constructions,
        spaceconds: _,
        systemconds: _,
        schedules: _,
    } = &data.bdldata;
    println!("{:#?}", db);
    println!("{:#?}", constructions);
    println!("{:#?}", floors);
    println!("{:#?}", spaces);
    println!("{:#?}", env);
    println!("{:#?}", shadings);
    println!("{:#?}", polygons);
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
    assert_eq!(data.envolvente.huecos.len(), 10);
    assert_eq!(data.envolvente.opacos.len(), 29);
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
