# parse-changelog

[![crates.io](https://img.shields.io/crates/v/parse-changelog?style=flat-square&logo=rust)](https://crates.io/crates/parse-changelog)
[![docs.rs](https://img.shields.io/badge/docs.rs-parse--changelog-blue?style=flat-square&logo=docs.rs)](https://docs.rs/parse-changelog)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.70-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/parse-changelog/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/parse-changelog/actions)

Simple changelog parser, written in Rust.

- [Installation](#installation)
  - [CLI](#cli)
  - [Library](#library)
- [Usage (CLI)](#usage-cli)
- [Usage (Library)](#usage-library)
- [Supported Format](#supported-format)
- [Related Projects](#related-projects)
- [License](#license)

## Installation

### CLI

<!-- omit in toc -->
#### From source

```sh
cargo +stable install parse-changelog --locked
```

<!-- omit in toc -->
#### From prebuilt binaries

You can download prebuilt binaries from the [Release page](https://github.com/taiki-e/parse-changelog/releases).
Prebuilt binaries are available for macOS, Linux (gnu and musl), Windows (static executable), FreeBSD, and illumos.

<details>
<summary>Example of script to download parse-changelog</summary>

```sh
# Get host target
host=$(rustc -vV | grep '^host:' | cut -d' ' -f2)
# Download binary and install to $HOME/.cargo/bin
curl --proto '=https' --tlsv1.2 -fsSL https://github.com/taiki-e/parse-changelog/releases/latest/download/parse-changelog-$host.tar.gz | tar xzf - -C "$HOME/.cargo/bin"
```

</details>

<!-- omit in toc -->
#### Via Homebrew

You can install parse-changelog from the [Homebrew tap maintained by us](https://github.com/taiki-e/homebrew-tap/blob/HEAD/Formula/parse-changelog.rb) (x86_64/aarch64 macOS, x86_64/aarch64 Linux):

```sh
brew install taiki-e/tap/parse-changelog
```

<!-- omit in toc -->
#### Via Scoop (Windows)

You can install parse-changelog from the [Scoop bucket maintained by us](https://github.com/taiki-e/scoop-bucket/blob/HEAD/bucket/parse-changelog.json):

```sh
scoop bucket add taiki-e https://github.com/taiki-e/scoop-bucket
scoop install parse-changelog
```

<!-- omit in toc -->
#### Via cargo-binstall

You can install parse-changelog using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```sh
cargo binstall parse-changelog
```

<!-- omit in toc -->
#### Via MPR (Debian/Ubuntu)

You can install [parse-changelog from the MPR](https://mpr.makedeb.org/packages/parse-changelog):

```sh
git clone 'https://mpr.makedeb.org/parse-changelog'
cd parse-changelog/
makedeb -si
```

<!-- omit in toc -->
##### Via Prebuilt-MPR

You can also install parse-changelog from the Prebuilt-MPR, which provides automatic upgrades directly via APT. First [set up the Prebuilt-MPR on your system](https://docs.makedeb.org/prebuilt-mpr/getting-started), and then run the following:

```sh
sudo apt install parse-changelog
```

Note: The MPR/Prebuilt-MPR packages are maintained by the community, not the maintainer of parse-changelog.

<!-- omit in toc -->
#### On GitHub Actions

You can use [taiki-e/install-action](https://github.com/taiki-e/install-action) to install prebuilt binaries on Linux, macOS, and Windows.
This makes the installation faster and may avoid the impact of [problems caused by upstream changes](https://github.com/tokio-rs/bytes/issues/506).

```yaml
- uses: taiki-e/install-action@parse-changelog
```

### Library

To use this crate as a library, add this to your `Cargo.toml`:

```toml
[dependencies]
parse-changelog = { version = "0.6", default-features = false }
```

Note: When using this crate as a library, we recommend disabling the default
features because the default features enable CLI-related dependencies and the
library part of this crate does not use them.

## Usage (CLI)

`parse-changelog` command parses changelog and returns a release note for the
specified version.

<details>
<summary>Click to show a complete list of options</summary>

<!-- readme-long-help:start -->
```console
$ parse-changelog --help
parse-changelog

Simple changelog parser, written in Rust.

Parses changelog and returns a release note for the specified version.

USAGE:
    parse-changelog [OPTIONS] <PATH> [VERSION]

ARGS:
    <PATH>       Path to the changelog file (use '-' for standard input)
    [VERSION]    Specify version (by default, select the latest release)

OPTIONS:
    -t, --title                       Returns title instead of notes
        --title-no-link               Similar to --title, but remove links from title
        --json                        Returns JSON representation of all releases in changelog
        --version-format <PATTERN>    Specify version format
        --prefix-format <PATTERN>     Specify prefix format [aliases: prefix]
    -h, --help                        Print help information
    -V, --version                     Print version information
```
<!-- readme-long-help:end -->

</details>

<!-- omit in toc -->
### Example: Get Rust's release notes

Get the release note for version 1.46.0 from [Rust's release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md):

```sh
curl -fsSL https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md \
  | parse-changelog - 1.46.0
```

[*output of the above command.*](tests/fixtures/rust-1.46.0.md)

<!-- omit in toc -->
### Example: Get Cargo's changelog

In [Cargo's changelog](https://github.com/rust-lang/cargo/blob/master/CHANGELOG.md),
the title starts with "Cargo ", and the patch version is omitted if zero. This is a
format `parse-changelog` don't support by default, so use `--prefix` and
`--version-format` to specify a custom format. For example:

```sh
curl -fsSL https://raw.githubusercontent.com/rust-lang/cargo/master/CHANGELOG.md \
  | parse-changelog --prefix 'Cargo ' --version-format '^[0-9]+\.[0-9]+(\.[0-9])?$' - 1.50
```

[*output of the above command.*](tests/fixtures/cargo-1.50.md)

`--prefix` is the same as [`Parser::prefix_format`] and `--version-format` is
the same as [`Parser::version_format`]. See documentation of those methods for
more information.

<!-- omit in toc -->
### Example: Create a new GitHub release from changelog

With [GitHub CLI](https://cli.github.com/manual/gh_release_create):

```sh
tag=...
version=...

# Get notes for $version from CHANGELOG.md.
notes=$(parse-changelog CHANGELOG.md "$version")
# Create a new GitHub release with GitHub CLI.
gh release create "$tag" --title "$version" --notes "$notes"
```

See also [create-gh-release-action].

## Usage (Library)

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
let changelog = parse_changelog::parse(changelog).unwrap();

// Get the latest release.
assert_eq!(changelog[0].version, "0.1.2");
assert_eq!(changelog[0].title, "0.1.2 - 2020-03-01");
assert_eq!(changelog[0].notes, "- Bug fixes.");

// Get the specified release.
assert_eq!(changelog["0.1.0"].title, "0.1.0 - 2020-01-01");
assert_eq!(changelog["0.1.0"].notes, "Initial release");
assert_eq!(changelog["0.1.1"].title, "0.1.1 - 2020-02-01");
assert_eq!(
    changelog["0.1.1"].notes,
    "- Added `Foo`.\n\
     - Added `Bar`."
);
```

See [documentation](https://docs.rs/parse-changelog) for more information on
`parse-changelog` as a library.

## Supported Format

By default, this crate is intended to support markdown-based changelogs
that have the title of each release starts with the version format based on
[Semantic Versioning][semver]. (e.g., [Keep a Changelog][keepachangelog]'s
changelog format.)

<!-- omit in toc -->
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

<!-- omit in toc -->
### Titles

The title of each release must start with a text or a link text (text with
`[` and `]`) that starts with a valid [version format](#versions) or
[prefix format](#prefixes). For example:

```markdown
# [0.2.0]

description...

# 0.1.0

description...
```

<!-- omit in toc -->
#### Prefixes

You can include characters before the version as prefix.

```text
## Version 0.1.0
   ^^^^^^^^
```

By default only "v", "Version ", "Release ", and "" (no prefix) are
allowed as prefixes.

To customize the prefix format, use the [`Parser::prefix_format`] method (library) or `--prefix-format` option (CLI).

<!-- omit in toc -->
#### Versions

```text
## v0.1.0 -- 2020-01-01
    ^^^^^
```

The default version format is based on [Semantic Versioning][semver].

This is parsed by using the following regular expression:

```text
^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z\.-]+)?(\+[0-9A-Za-z\.-]+)?$|^Unreleased$
```

**Note:** To get the 'Unreleased' section in the CLI, you need to explicitly specify 'Unreleased' as the version.

To customize the version format, use the [`Parser::version_format`] method (library) or `--version-format` option (CLI).

<!-- omit in toc -->
#### Suffixes

You can freely include characters after the version.

```text
# 0.1.0 - 2020-01-01
       ^^^^^^^^^^^^^
```

## Related Projects

- [create-gh-release-action]: GitHub Action for creating GitHub Releases based on changelog. (Using this crate for changelog parsing.)

[`Parser::prefix_format`]: https://docs.rs/parse-changelog/latest/parse_changelog/struct.Parser.html#method.prefix_format
[`Parser::version_format`]: https://docs.rs/parse-changelog/latest/parse_changelog/struct.Parser.html#method.version_format
[create-gh-release-action]: https://github.com/taiki-e/create-gh-release-action
[keepachangelog]: https://keepachangelog.com
[semver]: https://semver.org

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
