use criterion::{criterion_group, criterion_main, Criterion};
use boolean_skipfield::BoolSkipfield;

fn bench_count_skipped(c: &mut Criterion) {
    c.bench_function("count_skipped 100K bools", |b| {
        let sf = boolean_skipfield::BoolSkipfield::new(100_000);
        b.iter(|| {
            let _ = sf.count_skipped();
        });
    });
}

criterion_group!(benches, bench_count_skipped);
criterion_main!(benches);
