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

use hulc2envolventecte::{bdl, collect_hulc_data, ctehexml, find_hulc_files};

#[test]
fn test_bdl() {
    let data = ctehexml::parse("tests/00_plurif_s3_v0_d3/00_plurif_s3_v0_d3.ctehexml").unwrap();
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
    println!("{:#?}", floors);
    println!("{:#?}", spaces);
    println!("{:#?}", env);
    println!("{:#?}", shadings);
    println!("{:#?}", polygons);
}

#[test]
fn parse_test_data() {
    let hulcfiles = find_hulc_files("tests/data").unwrap();
    let data = collect_hulc_data(&hulcfiles).unwrap();
    assert_eq!(data.autil, 1673.92);
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
    assert_eq!(data.autil, 1073.76);
    assert_eq!(data.clima, "B3");
    assert_eq!(data.envolvente.huecos.len(), 29);
    assert_eq!(data.envolvente.opacos.len(), 60);
    assert_eq!(data.envolvente.pts.len(), 7);
}
