use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parse_changelog::parse;

fn parse_changelog(c: &mut Criterion) {
    let text = include_str!("../tests/fixtures/rust.md");
    c.bench_function("parse_changelog", |b| {
        b.iter(|| parse(black_box(text)).unwrap());
    });
}

criterion_group!(benches, parse_changelog);
criterion_main!(benches);
