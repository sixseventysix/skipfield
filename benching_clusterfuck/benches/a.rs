use criterion::{criterion_group, criterion_main, Criterion};
use benching_clusterfuck::{Skipfield, BoolSkipfield};

const N: usize = 100_000;

fn bench_bool_skipfield(c: &mut Criterion) {
    c.bench_function("BoolSkipfield::count_skipped", |b| {
        let sf = BoolSkipfield::new(N);
        b.iter(|| {
            let _ = sf.count_skipped();
        });
    });
}

fn bench_bitmask_skipfield(c: &mut Criterion) {
    c.bench_function("BitmaskSkipfield::count_skipped", |b| {
        let sf = Skipfield::new(N);
        b.iter(|| {
            let _ = sf.count_skipped();
        });
    });
}

criterion_group!(
    benches,
    bench_bool_skipfield,
    bench_bitmask_skipfield
);
criterion_main!(benches);
