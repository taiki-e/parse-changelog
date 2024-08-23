// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::drop_non_drop)]

use std::{hint::black_box, path::PathBuf};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fs_err as fs;
use parse_changelog::{parse, parse_iter, Parser};

fn fixtures_dir() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.pop(); // bench
    dir.push("tests");
    dir.push("fixtures");
    dir
}

fn parse_changelog_benches(c: &mut Criterion) {
    parse_changelog(c, "atx", "rust-atx.md");
    parse_changelog(c, "setext", "rust.md");
}
fn parse_changelog(c: &mut Criterion, name: &str, path: &str) {
    let path = &fixtures_dir().join(path);
    let text = &fs::read_to_string(path).unwrap();

    let mut g = c.benchmark_group(format!("parse_changelog_{name}"));
    g.throughput(Throughput::Bytes(text.len() as u64));
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
    g.finish();
}

criterion_group!(benches, parse_changelog_benches);
criterion_main!(benches);
