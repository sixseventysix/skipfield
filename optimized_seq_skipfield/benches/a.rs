use criterion::{criterion_group, criterion_main, Criterion};

fn bench_bitmask_count_skipped(c: &mut Criterion) {
    c.bench_function("count_skipped 100K bitmask", |b| {
        let sf = optimized_seq_skipfield::Skipfield::new(100_000);
        b.iter(|| {
            let _ = sf.count_skipped();
        });
    });
}

criterion_group!(benches, bench_bitmask_count_skipped);
criterion_main!(benches);
