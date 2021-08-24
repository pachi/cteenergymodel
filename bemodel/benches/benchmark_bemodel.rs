// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use bemodel::Model;
use criterion::{criterion_group, criterion_main, Criterion};

fn fshobst_update_benchmark(c: &mut Criterion) {
    let strdata = include_str!("../tests/data/e4h_medianeras.json");
    let mut model = Model::from_json(strdata).unwrap();

    c.bench_function("Actualiza F_sh;obst", |b| b.iter(|| model.update_fshobst()));
}

// Configuración del benchmarking
criterion_group! {
    name = benches;
    // https://docs.rs/criterion/0.3.4/criterion/struct.Criterion.html
    config = Criterion::default().sample_size(10);
    targets = fshobst_update_benchmark
}

// Genera función main que ejecuta benchmarks en el grupo indicado
criterion_main!(benches);
