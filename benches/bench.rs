#![expect(missing_docs)] // criterion_group! generates a public bench entrypoint

use criterion::{criterion_group, criterion_main};

mod insert;

criterion_group!(
    benches,
    insert::register_benches,
);
criterion_main!(benches);
