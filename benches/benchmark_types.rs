use criterion::{criterion_group, criterion_main, Criterion};
use hulc2envolventecte::{
    collect_hulc_data,
    parsers::{ctehexml, kyg, tbl},
};

fn load_caso_a() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/casoA").unwrap();
    let kygpath = kyg::find_kyg("tests/casoA").unwrap();
    let tblpath = tbl::find_tbl("tests/casoA").unwrap();
    let _data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
}

fn load_caso_c() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/casoC").unwrap();
    let kygpath = kyg::find_kyg("tests/casoC").unwrap();
    let tblpath = tbl::find_tbl("tests/casoC").unwrap();
    let _data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
}

// Caso más antiguo con archivo generado con el HULC2018 que salió a información pública
fn load_test_data() {
    let ctehexmlpath = ctehexml::find_ctehexml("tests/data").unwrap();
    let kygpath = kyg::find_kyg("tests/data").unwrap();
    let tblpath = tbl::find_tbl("tests/data").unwrap();
    let _data = collect_hulc_data(ctehexmlpath, kygpath, tblpath).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("caso A", |b| b.iter(|| load_caso_a()));
    c.bench_function("caso C", |b| b.iter(|| load_caso_c()));
    c.bench_function("test_data", |b| b.iter(|| load_test_data()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
