// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use criterion::{criterion_group, criterion_main, Criterion};

use hulc2model::collect_hulc_data;

fn load_caso_a() {
    let _data = collect_hulc_data("tests/casoA", true, true).unwrap();
}

fn load_caso_c() {
    let _data = collect_hulc_data("tests/casoC", true, true).unwrap();
}

// Caso más antiguo con archivo generado con el HULC2018 que salió a información pública
fn load_test_data() {
    let _data = collect_hulc_data("tests/data", true, true).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("caso A", |b| b.iter(load_caso_a));
    c.bench_function("caso C", |b| b.iter(load_caso_c));
    c.bench_function("test_data", |b| b.iter(load_test_data));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
