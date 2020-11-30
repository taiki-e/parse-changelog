# parse-changelog

[![crates.io](https://img.shields.io/crates/v/parse-changelog.svg?style=flat-square&logo=rust)](https://crates.io/crates/parse-changelog)
[![docs.rs](https://img.shields.io/badge/docs.rs-parse--changelog-blue?style=flat-square)](https://docs.rs/parse-changelog)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.45+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/parse-changelog/CI/master?style=flat-square)](https://github.com/taiki-e/parse-changelog/actions?query=workflow%3ACI+branch%3Amaster)

A changelog parser, written in Rust.

## Installation

To use this crate as a command line tool, run the following command:

```sh
cargo install parse-changelog
```

To use this crate as a library, add this to your `Cargo.toml`:

```toml
[dependencies]
# When using this crate as a library, we recommend disabling the default feature
# due to the default feature enables CLI-related dependencies.
parse-changelog = { version = "0.1", default-features = false }
```

*Compiler support: requires rustc 1.45+*

## Examples (as a command line tool)

`parse-changelog` command parses changelog and returns a release note for the specified version.

```text
USAGE:
    parse-changelog <PATH> [VERSION]

OPTIONS:
        --version-format <PATTERN>    Specify version format
        --prefix <PATTERN>            Alias for --prefix-format
        --prefix-format <PATTERN>     Specify prefix format
    -h, --help                        Prints help information
    -V, --version                     Prints version information

ARGS:
    <PATH>       Path to the changelog file
    <VERSION>    Specify version (by default, select the latest release)
```

An example of getting [Rust's release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md):

```sh
curl -sSf https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md > rust-releases.md
parse-changelog rust-releases.md 1.46.0
```

[*output of the above command.*](tests/fixtures/rust-1.46.0.md)

In [Cargo's changelog](https://github.com/rust-lang/cargo/blob/master/CHANGELOG.md), the title starts with "Cargo ", and the patch version is omitted. This is a format `parse-changelog` don't support by default, so use `--prefix` and `--version-format` to specify a custom format. For example:

```sh
curl -sSf https://raw.githubusercontent.com/rust-lang/cargo/master/CHANGELOG.md > cargo-changelog.md
parse-changelog --prefix 'Cargo ' --version-format '^\d+\.\d+' cargo-changelog.md 1.50
```

[*output of the above command.*](tests/fixtures/cargo-1.50.md)

`--prefix` is the same as [`Parser::prefix_format`] and `--version-format` is the same as [`Parser::version_format`]. See documentation of those methods for more information.

## Examples (as a library)

```rust
let changelog = "\
## 0.1.2 - 2020-03-01

- Bug fixes.

## 0.1.1 - 2020-02-01

- Added `Foo`.
- Added `Bar`.

## 0.1.0 - 2020-01-01

Initial release
";

// Parse changelog.
let releases = parse_changelog::parse(changelog).unwrap();

// Get the latest release.
assert_eq!(releases[0].version, "0.1.2");
assert_eq!(releases[0].title, "0.1.2 - 2020-03-01");
assert_eq!(releases[0].notes, "- Bug fixes.");

// Get the specified release.
assert_eq!(releases["0.1.0"].title, "0.1.0 - 2020-01-01");
assert_eq!(releases["0.1.0"].notes, "Initial release");
assert_eq!(releases["0.1.1"].title, "0.1.1 - 2020-02-01");
assert_eq!(
    releases["0.1.1"].notes,
    "- Added `Foo`.\n\
     - Added `Bar`."
);
```

See [documentation](https://docs.rs/parse-changelog) for more information on `parse-changelog` as a library.

## Format

By default, this crate is intended to support markdown-based changelogs
that have the title of each release starts with the version format based on
[Semantic Versioning][semver]. (e.g., [Keep a Changelog][keepachangelog]'s
changelog format.)

### Headings

The heading for each release must be Atx-style (1-6 `#`) or
Setext-style (`=` or `-` in a line under text), and the heading levels
must match with other releases.

Atx-style headings:

```markdown
# 0.1.0
```

```markdown
## 0.1.0
```

Setext-style headings:

```markdown
0.1.0
=====
```

```markdown
0.1.0
-----
```

### Titles

The title of each release must start with a text or a link text (text with `[` and `]`)
that starts with a valid version format. For example:

```markdown
# [0.2.0]

description...

# 0.1.0

description...
```

You can also include characters before the version as prefix. For example:

```markdown
## Version 0.1.0
```

By default only "v", "Version " and "Release " are allowed as prefixes and
can be customized using the [`Parser::prefix_format`] method
(`--prefix-format` option if command line).

You can freely include characters after the version (this crate
does not parse it). For example:

```markdown
# v0.1.0 - 2020-01-01
```

### Versions

The default version format is
`MAJOR.MINOR.PATCH(-PRE_RELEASE)?(+BUILD_METADATA)?`, and is
based on [Semantic Versioning][semver]. (Pre-release version and build
metadata are optional.)

This is parsed using the following regular expression:

```text
^\d+\.\d+\.\d+(-[\w\.-]+)?(\+[\w\.-]+)?
```

To customize the version format, use the [`Parser::version_format`] method
(`--version-format` option if command line).

[`Parser::prefix_format`]: https://docs.rs/parse-changelog/0.1/parse_changelog/struct.Parser.html#method.prefix_format
[`Parser::version_format`]: https://docs.rs/parse-changelog/0.1/parse_changelog/struct.Parser.html#method.version_format
[keepachangelog]: https://keepachangelog.com/en/1.0.0
[semver]: https://semver.org/spec/v2.0.0.html

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
