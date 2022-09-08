# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Fix bug in handling of link in title.

## [0.5.0] - 2022-07-24

- Change the default version format to accept the 'Unreleased' section as a valid changelog entry. ([#25](https://github.com/taiki-e/parse-changelog/pull/25), thanks @hwittenborn)

  Note that this does not change the behavior of CLI when the version is not specified. To get the 'Unreleased' section in the CLI, you need to explicitly specify 'Unreleased' as the version.

- Change the default version format to more strictly adhered to semver. Previous default version format accepted versions that are invalid as semver, such as leading zero in major, minor, or patch version.

## [0.4.9] - 2022-07-08

- Add metadata for cargo binstall.

## [0.4.8] - 2022-06-02

- Distribute prebuilt binaries for aarch64 macOS. ([#21](https://github.com/taiki-e/parse-changelog/pull/21))

## [0.4.7] - 2022-01-21

- Distribute prebuilt binaries for aarch64 Linux (gnu and musl).

## [0.4.6] - 2022-01-03

### CLI

- Fix bugs in argument parsing introduced in 0.4.5. ([#20](https://github.com/taiki-e/parse-changelog/pull/20))

## [0.4.5] - 2021-10-15

- Support Rust 1.51 again. ([#19](https://github.com/taiki-e/parse-changelog/pull/19))

## [0.4.4] - 2021-10-13

- Increase the minimum supported Rust version from Rust 1.51 to Rust 1.54.

- Allow specifying empty prefix format in `Parser::prefix_format` method (library) and `--prefix-format` option (CLI).

- Distribute statically linked binary on Windows MSVC. ([#18](https://github.com/taiki-e/parse-changelog/pull/18))

## [0.4.3] - 2021-07-24

- [Fix bug in parsing of heading.](https://github.com/taiki-e/parse-changelog/pull/13)

## [0.4.2] - 2021-07-22

- [Performance improvements.](https://github.com/taiki-e/parse-changelog/pull/11)

### CLI

- [Add `--json` flag to return JSON representation of changelog.](https://github.com/taiki-e/parse-changelog/pull/12)

### Library

- [Add `serde` optional feature.](https://github.com/taiki-e/parse-changelog/pull/12)

## [0.4.1] - 2021-07-22

- Documentation improvements.

## [0.4.0] - 2021-07-22

- [Fix bug in parsing of atx style heading.](https://github.com/taiki-e/parse-changelog/pull/9)

- [Performance improvements.](https://github.com/taiki-e/parse-changelog/pull/8)

### Library

- [Change `Release::notes` field from `String` to `&str`.](https://github.com/taiki-e/parse-changelog/pull/8)

- [Add `parse_iter` for partial parsing changelog.](https://github.com/taiki-e/parse-changelog/pull/9)

- Implement `PartialEq` and `Eq` for `Release`.

## [0.3.0] - 2021-04-12

- Increase the minimum supported Rust version from Rust 1.45 to Rust 1.51.

### Library

- Change `Error` from enum to struct.

- Remove `Error::is_regex` method. Use `Error::is_format` method instead.

## [0.2.2] - 2021-01-05

### Library

- Add `Error::{is_regex, is_format, is_parse}` methods.

## [0.2.1] - 2020-12-03

### CLI

No public API changes from 0.2.1.

- Distribute `*.tar.gz` file for Windows via GitHub Releases. See [#4](https://github.com/taiki-e/parse-changelog/pull/4) for more.

- Distribute x86_64-unknown-linux-musl binary via GitHub Releases.

## [0.2.0] - 2020-11-30

### CLI

- [Add `--title` option.](https://github.com/taiki-e/parse-changelog/pull/1)

- [Add support for standard input.](https://github.com/taiki-e/parse-changelog/pull/1)

### Library

- Add [`Changelog`](https://docs.rs/parse-changelog/0.2/parse_changelog/type.Changelog.html) type alias.

- Add [`Error`](https://docs.rs/parse-changelog/0.2/parse_changelog/enum.Error.html) type.

## [0.1.0] - 2020-11-29

Initial release

[Unreleased]: https://github.com/taiki-e/parse-changelog/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/taiki-e/parse-changelog/compare/v0.4.9...v0.5.0
[0.4.9]: https://github.com/taiki-e/parse-changelog/compare/v0.4.8...v0.4.9
[0.4.8]: https://github.com/taiki-e/parse-changelog/compare/v0.4.7...v0.4.8
[0.4.7]: https://github.com/taiki-e/parse-changelog/compare/v0.4.6...v0.4.7
[0.4.6]: https://github.com/taiki-e/parse-changelog/compare/v0.4.5...v0.4.6
[0.4.5]: https://github.com/taiki-e/parse-changelog/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/taiki-e/parse-changelog/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/taiki-e/parse-changelog/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/taiki-e/parse-changelog/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/taiki-e/parse-changelog/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/taiki-e/parse-changelog/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/taiki-e/parse-changelog/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/taiki-e/parse-changelog/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/parse-changelog/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/parse-changelog/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/parse-changelog/releases/tag/v0.1.0
