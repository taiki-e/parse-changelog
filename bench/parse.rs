// SPDX-License-Identifier: Apache-2.0 OR MIT

/*

Just for reference purposes, leave results of local benchmarks.


## Versions

- parse-changelog 0.6.11 (9057e4056cec6dc32300d4b62ee6ce66a2d1c683)
  - indexmap 2.7.1
  - memchr 2.7.4
  - regex 1.11.1


## Apple M1 Pro (MacBook Pro 2021, macOS 15.2)

```console
$ sysctl machdep.cpu.brand_string
machdep.cpu.brand_string: Apple M1 Pro

$ rustc -vV
rustc 1.86.0-nightly (854f22563 2025-01-31)
binary: rustc
commit-hash: 854f22563c8daf92709fae18ee6aed52953835cd
commit-date: 2025-01-31
host: aarch64-apple-darwin
release: 1.86.0-nightly
LLVM version: 19.1.7

$ cargo bench -p bench
<cargo log omitted>

Benchmarking parse_changelog_atx/parse: Collecting 100 samples in estimated 5.8152 s (10k ite
parse_changelog_atx/parse
                        time:   [574.60 µs 574.87 µs 575.14 µs]
                        thrpt:  [1.1653 GiB/s 1.1659 GiB/s 1.1664 GiB/s]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
Benchmarking parse_changelog_atx/parse_custom: Collecting 100 samples in estimated 5.8195 s (
parse_changelog_atx/parse_custom
                        time:   [574.55 µs 574.87 µs 575.23 µs]
                        thrpt:  [1.1652 GiB/s 1.1659 GiB/s 1.1665 GiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
Benchmarking parse_changelog_atx/parse_iter: Collecting 100 samples in estimated 5.7442 s (10
parse_changelog_atx/parse_iter
                        time:   [568.36 µs 568.65 µs 568.95 µs]
                        thrpt:  [1.1780 GiB/s 1.1786 GiB/s 1.1792 GiB/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
Benchmarking parse_changelog_atx/parse_iter_first: Collecting 100 samples in estimated 5.0087
parse_changelog_atx/parse_iter_first
                        time:   [5.6040 µs 5.6091 µs 5.6144 µs]
                        thrpt:  [119.38 GiB/s 119.49 GiB/s 119.60 GiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

Benchmarking parse_changelog_setext/parse: Collecting 100 samples in estimated 6.1965 s (10k
parse_changelog_setext/parse
                        time:   [612.24 µs 612.54 µs 612.85 µs]
                        thrpt:  [1.1062 GiB/s 1.1068 GiB/s 1.1074 GiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
Benchmarking parse_changelog_setext/parse_custom: Collecting 100 samples in estimated 6.1793
parse_changelog_setext/parse_custom
                        time:   [611.75 µs 612.03 µs 612.33 µs]
                        thrpt:  [1.1072 GiB/s 1.1077 GiB/s 1.1083 GiB/s]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Benchmarking parse_changelog_setext/parse_iter: Collecting 100 samples in estimated 6.1266 s
parse_changelog_setext/parse_iter
                        time:   [605.87 µs 606.55 µs 607.53 µs]
                        thrpt:  [1.1159 GiB/s 1.1177 GiB/s 1.1190 GiB/s]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
Benchmarking parse_changelog_setext/parse_iter_first: Collecting 100 samples in estimated 5.0
parse_changelog_setext/parse_iter_first
                        time:   [6.0356 µs 6.0408 µs 6.0461 µs]
                        thrpt:  [112.13 GiB/s 112.23 GiB/s 112.33 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

```


## Intel Core i7-9750H (MacBook Pro 2019, macOS 15.3)

```console
$ sysctl machdep.cpu.brand_string
machdep.cpu.brand_string: Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz

$ rustc -vV
rustc 1.86.0-nightly (854f22563 2025-01-31)
binary: rustc
commit-hash: 854f22563c8daf92709fae18ee6aed52953835cd
commit-date: 2025-01-31
host: x86_64-apple-darwin
release: 1.86.0-nightly
LLVM version: 19.1.7

$ cargo bench -p bench
<cargo log omitted>

Benchmarking parse_changelog_atx/parse: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.9s, enable flat sampling, or reduce sample count to 60.
parse_changelog_atx/parse
                        time:   [1.1639 ms 1.1642 ms 1.1645 ms]
                        thrpt:  [589.38 MiB/s 589.54 MiB/s 589.69 MiB/s]
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe
Benchmarking parse_changelog_atx/parse_custom: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.9s, enable flat sampling, or reduce sample count to 60.
parse_changelog_atx/parse_custom
                        time:   [1.1612 ms 1.1615 ms 1.1619 ms]
                        thrpt:  [590.70 MiB/s 590.88 MiB/s 591.05 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high severe
Benchmarking parse_changelog_atx/parse_iter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 60.
parse_changelog_atx/parse_iter
                        time:   [1.1495 ms 1.1499 ms 1.1502 ms]
                        thrpt:  [596.70 MiB/s 596.87 MiB/s 597.04 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
parse_changelog_atx/parse_iter_first
                        time:   [12.842 µs 12.846 µs 12.849 µs]
                        thrpt:  [52.161 GiB/s 52.175 GiB/s 52.190 GiB/s]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

Benchmarking parse_changelog_setext/parse: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.3s, enable flat sampling, or reduce sample count to 60.
parse_changelog_setext/parse
                        time:   [1.2456 ms 1.2460 ms 1.2464 ms]
                        thrpt:  [556.99 MiB/s 557.17 MiB/s 557.36 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
Benchmarking parse_changelog_setext/parse_custom: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.3s, enable flat sampling, or reduce sample count to 60.
parse_changelog_setext/parse_custom
                        time:   [1.2430 ms 1.2435 ms 1.2441 ms]
                        thrpt:  [558.03 MiB/s 558.29 MiB/s 558.53 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe
Benchmarking parse_changelog_setext/parse_iter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
parse_changelog_setext/parse_iter
                        time:   [1.2290 ms 1.2298 ms 1.2305 ms]
                        thrpt:  [564.17 MiB/s 564.53 MiB/s 564.87 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
parse_changelog_setext/parse_iter_first
                        time:   [13.713 µs 13.718 µs 13.723 µs]
                        thrpt:  [49.405 GiB/s 49.423 GiB/s 49.439 GiB/s]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

```

*/

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

    parse_changelog(c, "atx", "rust-atx.md");
    parse_changelog(c, "setext", "rust.md");
}

criterion_group!(benches, parse_changelog_benches);
criterion_main!(benches);
