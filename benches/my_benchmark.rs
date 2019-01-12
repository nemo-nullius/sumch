#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate sumch;

fn criterion_benchmark(c: &mut Criterion) {
    let s = sumch::get_from_file("./benches/benchtext").unwrap();
    c.bench_function("mixtext", move |b| b.iter(|| sumch::run(&s)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
