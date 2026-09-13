//! Benchmark `push` into
use criterion::Criterion;
use rand::RngExt;
use std::hint::black_box;

use timescape_viewer::core::{IForestIndex, Sum};

const ITEM_COUNT: usize = 1_000_000;

pub(crate) fn register_benches(c: &mut Criterion) {
    let mut rng = rand::rng();
    let values: Vec<f32> = (0..ITEM_COUNT)
        .map(|_| rng.random())
        .collect();

    c.bench_function("push", |b| {
        let mut forest: IForestIndex<Sum> = IForestIndex::with_capacity(ITEM_COUNT);
        b.iter(|| {
            forest.clear();
            for v in &values {
                forest.push(*v);
            }
            black_box(forest.query(0..ITEM_COUNT))
        });
    });

}
