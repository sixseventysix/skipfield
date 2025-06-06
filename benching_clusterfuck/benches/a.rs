use criterion::{criterion_group, criterion_main, Criterion, black_box};
use benching_clusterfuck::{
    lcjc_skipfield::LCJCSkipfield,
    bitmask_skipfield::BitmaskSkipfield,
};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const N: usize = 1_000_000;
const SKIP_RATIO: f64 = 0.3;

fn setup_lcjc() -> LCJCSkipfield {
    let mut sf = LCJCSkipfield::new(N);
    let mut rng = StdRng::seed_from_u64(42);
    for i in 0..N {
        if rng.gen::<f64>() < SKIP_RATIO {
            sf.skip(i);
        }
    }
    sf
}

fn setup_bitmask() -> BitmaskSkipfield {
    let mut sf = BitmaskSkipfield::new(N);
    let mut rng = StdRng::seed_from_u64(42);
    for i in 0..N {
        if rng.gen::<f64>() < SKIP_RATIO {
            sf.skip(i);
        }
    }
    sf
}

fn bench_lcjc_iter_next(c: &mut Criterion) {
    let sf = setup_lcjc();

    c.bench_function("LCJCSkipfield::iter().next()", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for i in sf.iter() {
                sum += i;
            }
            black_box(sum);
        });
    });
}

fn bench_bitmask_iter_next(c: &mut Criterion) {
    let sf = setup_bitmask();

    c.bench_function("BitmaskSkipfield::iter().next()", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for i in sf.iter() {
                sum += i;
            }
            black_box(sum);
        });
    });
}

criterion_group!(
    iter_benches,
    bench_lcjc_iter_next,
    bench_bitmask_iter_next
);
criterion_main!(iter_benches);

// fn setup_bool_skipfield() -> BoolSkipfield {
//     let mut sf = BoolSkipfield::new(N);
//     let mut rng = StdRng::seed_from_u64(42);

//     for i in 0..N {
//         if rng.gen::<f64>() < SKIP_RATIO {
//             sf.skip(i);
//         }
//     }

//     sf
// }

// fn setup_bitmask_skipfield() -> BitmaskSkipfield {
//     let mut sf = BitmaskSkipfield::new(N);
//     let mut rng = StdRng::seed_from_u64(42);

//     for i in 0..N {
//         if rng.gen::<f64>() < SKIP_RATIO {
//             sf.skip(i);
//         }
//     }

//     sf
// }

// fn setup_lcjc_skipfield() -> LCJCSkipfield {
//     let mut sf = LCJCSkipfield::new(N);
//     let mut rng = StdRng::seed_from_u64(42);

//     for i in 0..N {
//         if rng.gen::<f64>() < SKIP_RATIO {
//             sf.skip(i);
//         }
//     }

//     sf
// }

// fn bench_bool_skipfield(c: &mut Criterion) {
//     let sf = setup_bool_skipfield();

//     c.bench_function("BoolSkipfield::count_skipped", |b| {
//         b.iter(|| black_box(sf.count_skipped()));
//     });

//     c.bench_function("BoolSkipfield::count_active", |b| {
//         b.iter(|| black_box(sf.count_active()));
//     });

//     c.bench_function("BoolSkipfield::first_active", |b| {
//         b.iter(|| black_box(sf.first_active()));
//     });

//     c.bench_function("BoolSkipfield::active_indices", |b| {
//         b.iter(|| {
//             for idx in sf.active_indices() {
//                 black_box(idx);
//             }
//         });
//     });
// }

// fn bench_bitmask_skipfield(c: &mut Criterion) {
//     let sf = setup_bitmask_skipfield();

//     c.bench_function("BitmaskSkipfield::count_skipped", |b| {
//         b.iter(|| black_box(sf.count_skipped()));
//     });

//     c.bench_function("BitmaskSkipfield::count_active", |b| {
//         b.iter(|| black_box(sf.count_active()));
//     });

//     c.bench_function("BitmaskSkipfield::first_active", |b| {
//         b.iter(|| black_box(sf.first_active()));
//     });

//     c.bench_function("BitmaskSkipfield::active_indices_1", |b| {
//         b.iter(|| {
//             for idx in sf.active_indices_1() {
//                 black_box(idx);
//             }
//         });
//     });

//     c.bench_function("BitmaskSkipfield::active_indices_2", |b| {
//         b.iter(|| {
//             for idx in sf.active_indices_2() {
//                 black_box(idx);
//             }
//         });
//     });
// }

// fn bench_lcjc_skipfield(c: &mut Criterion) {
//     let sf = setup_lcjc_skipfield();

//     c.bench_function("LCJCSkipfield::count_skipped", |b| {
//         b.iter(|| black_box(sf.count_skipped()));
//     });

//     c.bench_function("LCJCSkipfield::count_active", |b| {
//         b.iter(|| black_box(sf.count_active()));
//     });

//     c.bench_function("LCJCSkipfield::first_active", |b| {
//         b.iter(|| black_box(sf.first_active()));
//     });

//     c.bench_function("LCJCSkipfield::active_indices", |b| {
//         b.iter(|| {
//             for idx in sf.active_indices() {
//                 black_box(idx);
//             }
//         });
//     });
// }

// criterion_group!(
//     benches,
//     bench_bool_skipfield,
//     bench_bitmask_skipfield,
//     bench_lcjc_skipfield
// );
// criterion_main!(benches);