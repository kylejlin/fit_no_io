#[macro_use]
extern crate criterion;

use criterion::Criterion;
use fit_no_io as fit;
use std::path::PathBuf;

fn read_file() {
    let mmap = mmap::from_path("data/garmin_1000.fit");
    fit::run(&mmap);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read_file", |b| b.iter(|| read_file()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
