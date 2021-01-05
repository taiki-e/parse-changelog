# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

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

[Unreleased]: https://github.com/taiki-e/parse-changelog/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/taiki-e/parse-changelog/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/parse-changelog/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/parse-changelog/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/parse-changelog/releases/tag/v0.1.0
