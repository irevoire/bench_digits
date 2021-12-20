use bench_digits::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_digits(c: &mut Criterion) {
    let mut group = c.benchmark_group("Digits");
    for i in [
        0usize,
        100,
        1000,
        4000,
        10_000,
        50_000,
        100_000,
        500_000,
        usize::MAX,
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("log10", i), i, |b, i| b.iter(|| log10(*i)));
        group.bench_with_input(BenchmarkId::new("divide", i), i, |b, i| {
            b.iter(|| divide_loop(*i))
        });
        group.bench_with_input(BenchmarkId::new("kendal willets", i), i, |b, i| {
            b.iter(|| kendal_willets(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_digits);
criterion_main!(benches);
