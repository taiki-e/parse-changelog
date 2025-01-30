// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "default")]
#![cfg(not(miri))] // Miri doesn't support pipe2 (inside std::process::Command::output)

mod auxiliary;

use std::{env, path::Path};

use fs_err as fs;
use indexmap::IndexMap;
use serde_derive::Deserialize;

use self::auxiliary::{cli::*, *};

#[test]
fn success() {
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/pin-project-1.0.0.md"));
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0", "--title"])
        .assert_success()
        .stdout_eq("[1.0.0] - 2020-10-13");
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0", "--title-no-link"])
        .assert_success()
        .stdout_eq("1.0.0 - 2020-10-13");

    parse_changelog(["tests/fixtures/rust.md", "1.46.0"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/rust-1.46.0.md"));

    parse_changelog(["tests/fixtures/rust-atx.md", "1.46.0"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/rust-1.46.0-atx.md"));

    parse_changelog([
        "tests/fixtures/cargo.md",
        "1.50",
        "--prefix",
        "Cargo ",
        "--version-format",
        r"^[0-9]+\.[0-9]+(\.[0-9])?$",
    ])
    .assert_success()
    .stdout_eq(include_str!("fixtures/cargo-1.50.md"));
}

#[test]
fn failure() {
    parse_changelog([] as [&str; 0])
        .assert_failure()
        .stderr_contains("no changelog path specified");

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0", "0.0.1"])
        .assert_failure()
        .stderr_contains(r#"unexpected argument "0.0.1""#);

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0", "--title", "--title-no-link"])
        .assert_failure()
        .stderr_contains("--title may not be used together with --title-no-link");

    // multiple arguments
    for flag in &[
        "-t",
        "--title",
        "--title-no-link",
        "--json",
        "--version-format=version",
        "--prefix-format=v",
    ] {
        parse_changelog(["tests/fixtures/pin-project.md", "0.0.0", flag, flag])
            .assert_failure()
            .stderr_contains(&format!(
                "the argument '{}' was provided more than once, but cannot be used multiple times",
                flag.split('=').next().unwrap()
            ));
    }

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0"])
        .assert_failure()
        .stderr_contains("not found release note for '0.0.0' in tests/fixtures/pin-project.md");

    parse_changelog(["tests/fixtures/cargo.md", "1.50", "--prefix", "Cargo "])
        .assert_failure()
        .stderr_contains("not found release note for '1.50' in tests/fixtures/cargo.md");
}

type ChangelogOwned = IndexMap<String, ReleaseOwned>;

#[derive(Debug, PartialEq, Deserialize)]
struct ReleaseOwned {
    version: String,
    title: String,
    notes: String,
}

#[test]
fn json() {
    let text = parse_changelog(["tests/fixtures/pin-project.md", "--json"]).assert_success().stdout;
    let changelog: ChangelogOwned = serde_json::from_str(&text).unwrap();
    assert_eq!(changelog.len(), 82);

    let text = parse_changelog(["tests/fixtures/rust.md", "--json"]).assert_success().stdout;
    let changelog: ChangelogOwned = serde_json::from_str(&text).unwrap();
    assert_eq!(changelog.len(), 116);

    let text = parse_changelog(["tests/fixtures/rust-atx.md", "--json"]).assert_success().stdout;
    let changelog: ChangelogOwned = serde_json::from_str(&text).unwrap();
    assert_eq!(changelog.len(), 116);

    let text = parse_changelog([
        "tests/fixtures/cargo.md",
        "--json",
        "--prefix",
        "Cargo ",
        "--version-format",
        r"^[0-9]+\.[0-9]+(\.[0-9])?$",
    ])
    .assert_success()
    .stdout;
    let changelog: ChangelogOwned = serde_json::from_str(&text).unwrap();
    assert_eq!(changelog.len(), 54);
}

#[test]
fn version() {
    parse_changelog(["--version"]).assert_success().stdout_contains(env!("CARGO_PKG_VERSION"));
}

#[test]
fn update_readme() {
    let new = &*parse_changelog(["--help"]).assert_success().stdout;
    let path = &Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
    let base = fs::read_to_string(path).unwrap();
    let mut out = String::with_capacity(base.capacity());
    let mut lines = base.lines();
    let mut start = false;
    let mut end = false;
    while let Some(line) = lines.next() {
        out.push_str(line);
        out.push('\n');
        if line == "<!-- readme-long-help:start -->" {
            start = true;
            out.push_str("```console\n");
            out.push_str("$ parse-changelog --help\n");
            out.push_str(new);
            for line in &mut lines {
                if line == "<!-- readme-long-help:end -->" {
                    out.push_str("```\n");
                    out.push_str(line);
                    out.push('\n');
                    end = true;
                    break;
                }
            }
        }
    }
    if start && end {
        assert_diff(path, out);
    } else if start {
        panic!("missing `<!-- readme-long-help:end -->` comment in README.md");
    } else {
        panic!("missing `<!-- readme-long-help:start -->` comment in README.md");
    }
}
