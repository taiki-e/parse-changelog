#![cfg(feature = "default")]
#![warn(rust_2018_idioms, single_use_lifetimes)]

mod auxiliary;

use auxiliary::{parse_changelog, CommandExt};
use indexmap::IndexMap;

#[test]
fn failures() {
    parse_changelog([] as [&str; 0])
        .assert_failure()
        .stderr_contains("no changelog path specified");

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0"])
        .assert_failure()
        .stderr_contains("not found release note for '0.0.0'");

    parse_changelog(["tests/fixtures/cargo.md", "1.50", "--prefix", "Cargo "])
        .assert_failure()
        .stderr_contains("no release was found");
}

#[test]
fn success() {
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/pin-project-1.0.0.md"));

    parse_changelog(["tests/fixtures/rust.md", "1.46.0"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/rust-1.46.0.md"));

    parse_changelog([
        "tests/fixtures/cargo.md",
        "1.50",
        "--prefix",
        "Cargo ",
        "--version-format",
        r"^[0-9]+\.[0-9]+$",
    ])
    .assert_success()
    .stdout_eq(include_str!("fixtures/cargo-1.50.md"));
}

type ChangelogOwned = IndexMap<String, ReleaseOwned>;

#[derive(Debug, PartialEq, Eq, serde_crate::Deserialize)]
#[serde(crate = "serde_crate")]
struct ReleaseOwned {
    version: String,
    title: String,
    notes: String,
}

#[test]
fn json() {
    let text = parse_changelog(["tests/fixtures/rust.md", "--json"]).assert_success().stdout;
    let changelog: ChangelogOwned = serde_json::from_str(&text).unwrap();
    assert_eq!(changelog.len(), 72);
}

#[test]
fn version() {
    parse_changelog(["--version"]).assert_success().stdout_contains(env!("CARGO_PKG_VERSION"));
}
