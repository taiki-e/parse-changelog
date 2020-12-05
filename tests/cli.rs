#![cfg(feature = "default")]

mod auxiliary;

use auxiliary::{parse_changelog, CommandExt};

#[test]
fn failures() {
    parse_changelog([] as [&str; 0])
        .assert_failure()
        .stderr_contains("The following required arguments were not provided");

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
        r"^\d+\.\d+",
    ])
    .assert_success()
    .stdout_eq(include_str!("fixtures/cargo-1.50.md"));
}
