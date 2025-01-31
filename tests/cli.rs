// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "default")]
#![cfg(not(miri))] // Miri doesn't support pipe2 (inside std::process::Command::output)

use std::{ffi::OsStr, path::Path, process::Command};

use indexmap::IndexMap;
use serde_derive::Deserialize;
use test_helper::cli::{ChildExt as _, CommandExt as _};

fn parse_changelog<O: AsRef<OsStr>>(args: impl AsRef<[O]>) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_parse-changelog"));
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.args(args.as_ref());
    cmd
}

#[test]
fn success() {
    parse_changelog(["tests/fixtures/pin-project.md"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/pin-project-latest.md"));
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/pin-project-1.0.0.md"));
    parse_changelog(["tests/fixtures/pin-project.md", "Unreleased"]).assert_success().stdout_eq("");
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0", "--title"])
        .assert_success()
        .stdout_eq("[1.0.0] - 2020-10-13");
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0", "--title-no-link"])
        .assert_success()
        .stdout_eq("1.0.0 - 2020-10-13");
    parse_changelog(["-", "1.0.0"])
        .spawn_with_stdin(include_bytes!("fixtures/pin-project.md"))
        .assert_success()
        .stdout_eq(include_str!("fixtures/pin-project-1.0.0.md"));

    parse_changelog(["tests/fixtures/rust.md"])
        .assert_success()
        .stdout_eq(include_str!("fixtures/rust-latest.md"));
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
            .stderr_contains(format!(
                "the argument '{}' was provided more than once, but cannot be used multiple times",
                flag.split('=').next().unwrap()
            ));
    }

    parse_changelog(["tests/fixtures/pin-project.md", "0.0.0"])
        .assert_failure()
        .stderr_contains("not found release note for '0.0.0' in tests/fixtures/pin-project.md");
    parse_changelog(["-", "0.0.0"])
        .spawn_with_stdin(include_bytes!("fixtures/pin-project.md"))
        .assert_failure()
        .stderr_contains("not found release note for '0.0.0' in changelog (standard input)");

    parse_changelog(["tests/fixtures/cargo.md", "1.50", "--prefix", "Cargo "])
        .assert_failure()
        .stderr_contains("not found release note for '1.50' in tests/fixtures/cargo.md");

    parse_changelog(["-"])
        .spawn_with_stdin("\n")
        .assert_failure()
        .stderr_contains("error: no release note was found in changelog (standard input)");

    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0", "--version-format=\\"])
        .assert_failure()
        .stderr_contains("error: regex parse error");
    parse_changelog(["tests/fixtures/pin-project.md", "1.0.0", "--prefix-format=\\"])
        .assert_failure()
        .stderr_contains("error: regex parse error");

    parse_changelog(["-"])
        .spawn_with_stdin("# Unreleased\n")
        .assert_failure()
        .stderr_contains("error: not found release; to get 'Unreleased' section specify release explicitly: `parse-changelog - Unreleased`");

    parse_changelog(["-"])
        .spawn_with_stdin([b'f', b'o', 0x80, b'o'])
        .assert_failure()
        .stderr_contains(
            "error: failed to read from standard input: stream did not contain valid UTF-8",
        );
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
    assert_eq!(changelog.len(), 117);

    let text = parse_changelog(["tests/fixtures/rust-atx.md", "--json"]).assert_success().stdout;
    let changelog: ChangelogOwned = serde_json::from_str(&text).unwrap();
    assert_eq!(changelog.len(), 117);

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
fn help() {
    let short = parse_changelog(["-h"]).assert_success();
    let long = parse_changelog(["--help"]).assert_success();
    assert_eq!(short.stdout, long.stdout);
}

#[test]
fn version() {
    let expected = &format!("parse-changelog {}", env!("CARGO_PKG_VERSION"));
    parse_changelog(["-V"]).assert_success().stdout_eq(expected);
    parse_changelog(["--version"]).assert_success().stdout_eq(expected);
}

#[test]
fn update_readme() {
    let new = parse_changelog(["--help"]).assert_success().stdout;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
    let command = "parse-changelog --help";
    test_helper::doc::sync_command_output_to_markdown(path, "readme-long-help", command, new);
}
