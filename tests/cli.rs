// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "default")]
#![cfg(not(miri))] // Miri doesn't support pipe2 (inside std::process::Command::output)

mod auxiliary;

use indexmap::IndexMap;
use serde_derive::Deserialize;

use self::auxiliary::cli::*;

#[test]
fn failures() {
    parse_changelog([] as [&str; 0])
        .assert_failure()
        .stderr_contains("no changelog path specified");

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0", "0.0.1"])
        .assert_failure()
        .stderr_contains(r#"unexpected argument "0.0.1""#);

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0", "--title", "--title-no-link"])
        .assert_failure()
        .stderr_contains("--title may not be used together with --title-no-link");

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0", "--title", "--title"])
        .assert_failure()
        .stderr_contains(
            "the argument '--title' was provided more than once, but cannot be used multiple times",
        );

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0"])
        .assert_failure()
        .stderr_contains("not found release note for '0.0.0' in tests/fixtures/pin-project.md");

    parse_changelog(["tests/fixtures/cargo.md", "1.50", "--prefix", "Cargo "])
        .assert_failure()
        .stderr_contains("not found release note for '1.50' in tests/fixtures/cargo.md");
}

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
