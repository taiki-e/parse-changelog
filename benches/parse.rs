use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parse_changelog::{parse, parse_iter};

fn parse_changelog_atx(c: &mut Criterion) {
    let text = black_box(include_str!("../tests/fixtures/rust-atx.md"));
    c.bench_function("parse_atx", |b| {
        b.iter(|| black_box(parse(text).unwrap()));
    });
    c.bench_function("parse_iter_atx", |b| {
        b.iter(|| parse_iter(text).for_each(|r| drop(black_box(r))));
    });
    c.bench_function("parse_iter_first_atx", |b| {
        b.iter(|| parse_iter(text).next().unwrap());
    });
}

fn parse_changelog_setext(c: &mut Criterion) {
    let text = black_box(include_str!("../tests/fixtures/rust.md"));
    c.bench_function("parse_setext", |b| {
        b.iter(|| black_box(parse(text).unwrap()));
    });
    c.bench_function("parse_iter_setext", |b| {
        b.iter(|| parse_iter(text).for_each(|r| drop(black_box(r))));
    });
    c.bench_function("parse_iter_first_setext", |b| {
        b.iter(|| parse_iter(text).next().unwrap());
    });
}

criterion_group!(benches, parse_changelog_atx, parse_changelog_setext);
criterion_main!(benches);
