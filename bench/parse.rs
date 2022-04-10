#![allow(clippy::drop_non_drop)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parse_changelog::{parse, parse_iter, Parser};

fn parse_changelog_atx(c: &mut Criterion) {
    let mut g = c.benchmark_group("atx");
    let text = include_str!("../tests/fixtures/rust-atx.md");
    g.bench_function("parse", |b| {
        b.iter(|| parse(black_box(text)).unwrap());
    });
    g.bench_function("parse_custom", |b| {
        let mut p = Parser::new();
        p.prefix_format("^Version ").unwrap();
        b.iter(|| p.parse(black_box(text)).unwrap());
    });
    g.bench_function("parse_iter", |b| {
        b.iter(|| parse_iter(black_box(text)).for_each(|r| drop(black_box(r))));
    });
    g.bench_function("parse_iter_first", |b| {
        b.iter(|| parse_iter(black_box(text)).next().unwrap());
    });
}

fn parse_changelog_setext(c: &mut Criterion) {
    let mut g = c.benchmark_group("setext");
    let text = include_str!("../tests/fixtures/rust.md");
    g.bench_function("parse", |b| {
        b.iter(|| parse(black_box(text)).unwrap());
    });
    g.bench_function("parse_custom", |b| {
        let mut p = Parser::new();
        p.prefix_format("^Version ").unwrap();
        b.iter(|| p.parse(black_box(text)).unwrap());
    });
    g.bench_function("parse_iter", |b| {
        b.iter(|| parse_iter(black_box(text)).for_each(|r| drop(black_box(r))));
    });
    g.bench_function("parse_iter_first", |b| {
        b.iter(|| parse_iter(black_box(text)).next().unwrap());
    });
}

criterion_group!(benches, parse_changelog_atx, parse_changelog_setext);
criterion_main!(benches);
