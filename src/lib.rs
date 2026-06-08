// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
Simple changelog parser, written in Rust.

### Usage

<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

To use this crate as a library, add this to your `Cargo.toml`:

```toml
[dependencies]
parse-changelog = { version = "0.6", default-features = false }
```

<div class="rustdoc-alert rustdoc-alert-note">

> **ⓘ Note**
>
> We recommend disabling default features because they enable CLI-related
> dependencies which the library part does not use.

</div>

<!-- omit in toc -->
### Examples

```
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

<!-- omit in toc -->
### Optional features

- **`serde`** — Implements [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait for parse-changelog types.

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

<div class="rustdoc-alert rustdoc-alert-note">

> **ⓘ Note**
>
> To get the 'Unreleased' section in the CLI, you need to explicitly specify 'Unreleased' as the version.

</div>

To customize the version format, use the [`Parser::version_format`] method (library) or `--version-format` option (CLI).

<!-- omit in toc -->
#### Suffixes

You can freely include characters after the version.

```text
# 0.1.0 - 2020-01-01
       ^^^^^^^^^^^^^
```

## Related Projects

- [create-gh-release-action]: GitHub Action for creating GitHub Releases based on changelog. This action uses this crate for changelog parsing.

[`Parser::prefix_format`]: https://docs.rs/parse-changelog/latest/parse_changelog/struct.Parser.html#method.prefix_format
[`Parser::version_format`]: https://docs.rs/parse-changelog/latest/parse_changelog/struct.Parser.html#method.version_format
[create-gh-release-action]: https://github.com/taiki-e/create-gh-release-action
[keepachangelog]: https://keepachangelog.com
[semver]: https://semver.org

<!-- tidy:sync-markdown-to-rustdoc:end -->
*/

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(allow(
        dead_code,
        unused_variables,
        clippy::undocumented_unsafe_blocks,
        clippy::unused_trait_names,
    ))
))]
#![forbid(unsafe_code)]
#![warn(
    // Lints that may help when writing public library.
    missing_debug_implementations,
    missing_docs,
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    // clippy::missing_inline_in_public_items,
)]
// docs.rs only (cfg is enabled by docs.rs, not build script)
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;
extern crate std;

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "gen/tests/assert_impl.rs"]
mod assert_impl;
#[cfg(feature = "serde")]
#[path = "gen/serde.rs"]
mod serde_impl;
#[cfg(test)]
#[path = "gen/tests/track_size.rs"]
mod track_size;

mod error;

use alloc::{borrow::Cow, format, string::String};
use core::mem;
use std::sync::OnceLock;

use indexmap::IndexMap;
use regex::Regex;

pub use self::error::Error;
use self::error::Result;

/// A changelog.
///
/// The key is a version, and the value is the release note for that version.
///
/// The order is the same as the order written in the original text. (e.g., if
/// [the latest version comes first][keepachangelog], `changelog[0]` is the
/// release note for the latest version)
///
/// This type is returned by [`parse`] function or [`Parser::parse`] method.
///
/// [keepachangelog]: https://keepachangelog.com
pub type Changelog<'a> = IndexMap<&'a str, Release<'a>>;

/// Parses release notes from the given `text`.
///
/// This function uses the default version and prefix format. If you want to use
/// another format, consider using the [`Parser`] type instead.
///
/// See the [crate-level documentation](crate) for changelog and version
/// format supported by default.
///
/// # Errors
///
/// Returns an error if any of the following:
///
/// - There are multiple release notes for one version.
/// - No release note was found. This usually means that the changelog isn't
///   written in the supported format.
///
/// If you want to handle these cases manually without making errors,
/// consider using [`parse_iter`].
pub fn parse(text: &str) -> Result<Changelog<'_>> {
    Parser::new().parse(text)
}

/// Returns an iterator over all release notes in the given `text`.
///
/// Unlike [`parse`] function, the returned iterator doesn't error on
/// duplicate release notes or empty changelog.
///
/// This function uses the default version and prefix format. If you want to use
/// another format, consider using the [`Parser`] type instead.
///
/// See the [crate-level documentation](crate) for changelog and version
/// format supported by default.
pub fn parse_iter(text: &str) -> ParseIter<'_, 'static> {
    ParseIter::new(text, None, None)
}

/// A release note for a version.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Release<'a> {
    /// The version of this release.
    ///
    /// ```text
    /// ## Version 0.1.0 -- 2020-01-01
    ///            ^^^^^
    /// ```
    ///
    /// This is the same value as the key of the [`Changelog`] type.
    pub version: &'a str,
    /// The title of this release.
    ///
    /// ```text
    /// ## Version 0.1.0 -- 2020-01-01
    ///    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// Note:
    /// - Leading and trailing [whitespaces](char::is_whitespace) have been removed.
    /// - This retains links in the title. Use [`title_no_link`](Self::title_no_link)
    ///   if you want to use the title with links removed.
    pub title: &'a str,
    /// The descriptions of this release.
    ///
    /// Note that leading and trailing newlines have been removed.
    pub notes: &'a str,
}

impl<'a> Release<'a> {
    /// Returns the title of this release with link removed.
    #[must_use]
    pub fn title_no_link(&self) -> Cow<'a, str> {
        full_unlink(self.title)
    }
}

/// A changelog parser.
#[derive(Debug, Default)]
pub struct Parser {
    /// Version format. e.g., "0.1.0" in "# v0.1.0 (2020-01-01)".
    ///
    /// If `None`, `DEFAULT_VERSION_FORMAT` is used.
    version_format: Option<Regex>,
    /// Prefix format. e.g., "v" in "# v0.1.0 (2020-01-01)", "Version " in
    /// "# Version 0.1.0 (2020-01-01)".
    ///
    /// If `None`, `DEFAULT_PREFIX_FORMAT` is used.
    prefix_format: Option<Regex>,
}

impl Parser {
    /// Creates a new changelog parser.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the version format.
    ///
    /// ```text
    /// ## v0.1.0 -- 2020-01-01
    ///     ^^^^^
    /// ```
    ///
    /// *Tip*: To customize the text before the version number (e.g., "v" in "# v0.1.0",
    /// "Version " in "# Version 0.1.0", etc.), use the [`prefix_format`] method
    /// instead of this method.
    ///
    /// # Default
    ///
    /// The default version format is based on [Semantic Versioning][semver].
    ///
    /// This is parsed by using the following regular expression:
    ///
    /// ```text
    /// ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z\.-]+)?(\+[0-9A-Za-z\.-]+)?$|^Unreleased$
    /// ```
    ///
    /// **Note:** To get the 'Unreleased' section in the CLI, you need to explicitly specify 'Unreleased' as the version.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following:
    ///
    /// - The specified format is not a valid regular expression or supported by
    ///   [regex] crate.
    /// - The specified format is empty or contains only
    ///   [whitespace](char::is_whitespace).
    ///
    /// [`prefix_format`]: Self::prefix_format
    /// [regex]: https://docs.rs/regex
    /// [semver]: https://semver.org
    pub fn version_format(&mut self, format: &str) -> Result<&mut Self> {
        if format.trim_start().is_empty() {
            return Err(Error::format("empty or whitespace version format"));
        }
        self.version_format = Some(Regex::new(format).map_err(Error::new)?);
        Ok(self)
    }

    /// Sets the prefix format.
    ///
    /// "Prefix" means the range from the first non-whitespace character after
    /// heading to the character before the version (including whitespace
    /// characters). For example:
    ///
    /// ```text
    /// ## Version 0.1.0 -- 2020-01-01
    ///    ^^^^^^^^
    /// ```
    ///
    /// ```text
    /// ## v0.1.0 -- 2020-01-01
    ///    ^
    /// ```
    ///
    /// # Default
    ///
    /// By default only "v", "Version ", "Release ", and "" (no prefix) are
    /// allowed as prefixes.
    ///
    /// This is parsed by using the following regular expression:
    ///
    /// ```text
    /// ^(v|Version |Release )?
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following:
    ///
    /// - The specified format is not a valid regular expression or supported by
    ///   [regex] crate.
    ///
    /// [regex]: https://docs.rs/regex
    pub fn prefix_format(&mut self, format: &str) -> Result<&mut Self> {
        self.prefix_format = Some(Regex::new(format).map_err(Error::new)?);
        Ok(self)
    }

    /// Parses release notes from the given `text`.
    ///
    /// See the [crate-level documentation](crate) for changelog and version
    /// format supported by default.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following:
    ///
    /// - There are multiple release notes for one version.
    /// - No release note was found. This usually means that the changelog isn't
    ///   written in the supported format, or that the specified format is wrong
    ///   if you specify your own format.
    ///
    /// If you want to handle these cases manually without making errors,
    /// consider using [`parse_iter`].
    ///
    /// [`parse_iter`]: Self::parse_iter
    pub fn parse<'a>(&self, text: &'a str) -> Result<Changelog<'a>> {
        let mut map = IndexMap::new();
        for release in self.parse_iter(text) {
            if let Some(release) = map.insert(release.version, release) {
                return Err(Error::parse(format!(
                    "multiple release notes for '{}'",
                    release.version
                )));
            }
        }
        if map.is_empty() {
            return Err(Error::parse("no release note was found"));
        }
        Ok(map)
    }

    /// Returns an iterator over all release notes in the given `text`.
    ///
    /// Unlike [`parse`] method, the returned iterator doesn't error on
    /// duplicate release notes or empty changelog.
    ///
    /// See the [crate-level documentation](crate) for changelog and version
    /// format supported by default.
    ///
    /// [`parse`]: Self::parse
    pub fn parse_iter<'a, 'r>(&'r self, text: &'a str) -> ParseIter<'a, 'r> {
        ParseIter::new(text, self.version_format.as_ref(), self.prefix_format.as_ref())
    }
}

/// An iterator over release notes.
///
/// This type is returned by [`parse_iter`] function or [`Parser::parse_iter`] method.
#[allow(missing_debug_implementations)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ParseIter<'a, 'r> {
    version_format: &'r Regex,
    prefix_format: &'r Regex,
    lines: Lines<'a>,
    /// The heading level of release sections. 1-6
    level: Option<u8>,
}

fn default_prefix_format() -> &'static Regex {
    static DEFAULT_PREFIX_FORMAT: OnceLock<Regex> = OnceLock::new();
    fn init() -> Regex {
        Regex::new(r"^(v|Version |Release )?").unwrap()
    }
    DEFAULT_PREFIX_FORMAT.get_or_init(init)
}
fn default_version_format() -> &'static Regex {
    static DEFAULT_VERSION_FORMAT: OnceLock<Regex> = OnceLock::new();
    fn init() -> Regex {
        Regex::new(r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z\.-]+)?(\+[0-9A-Za-z\.-]+)?$|^Unreleased$")
            .unwrap()
    }
    DEFAULT_VERSION_FORMAT.get_or_init(init)
}

impl<'a, 'r> ParseIter<'a, 'r> {
    fn new(
        text: &'a str,
        version_format: Option<&'r Regex>,
        prefix_format: Option<&'r Regex>,
    ) -> Self {
        Self {
            version_format: version_format.unwrap_or_else(|| default_version_format()),
            prefix_format: prefix_format.unwrap_or_else(|| default_prefix_format()),
            lines: Lines::new(text),
            level: None,
        }
    }

    fn end_release(
        &self,
        mut cur_release: Release<'a>,
        release_note_start: usize,
        line_start: usize,
    ) -> Release<'a> {
        assert!(!cur_release.version.is_empty());
        if release_note_start < line_start {
            // Remove trailing newlines.
            cur_release.notes = self.lines.text[release_note_start..line_start - 1].trim_end();
        }
        cur_release
    }

    fn handle_comment(&self, on_comment: &mut bool, is_inline_comment: &mut bool, line: &'a [u8]) {
        let mut first = true;
        let mut line = Some(line);
        while let Some(l) = line {
            let Some(pos) = memchr::memchr(b'-', l) else { break };
            match l.get(pos + 1) {
                // --
                // ^   pos
                //  ^  pos + 1
                Some(&b'-') => {}
                Some(_) => {
                    line = l.get(pos + 2..);
                    first = false;
                    continue;
                }
                None => break,
            }
            if let Some(open) = pos.checked_sub(2) {
                if l.get(open) == Some(&b'<') && l.get(open + 1) == Some(&b'!') {
                    // <!--
                    // ^    open
                    //   ^  pos
                    if !*on_comment {
                        *on_comment = true;
                        *is_inline_comment = !first || open != 0;
                    }
                    line = l.get(pos + 2..);
                    first = false;
                    continue;
                }
            }
            if let Some(close) = pos.checked_add(2) {
                if l.get(close) == Some(&b'>') {
                    // -->
                    //   ^ close
                    // ^   pos
                    *on_comment = false;
                    line = l.get(close + 1..);
                    first = false;
                    continue;
                }
            }
            line = l.get(pos + 2..);
        }
    }
}

impl<'a> Iterator for ParseIter<'a, '_> {
    type Item = Release<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // If `true`, we are in a code block (``` or ~~~).
        let mut on_code_block: Option<&[u8]> = None;
        // If `true`, we are in a comment (`<!--` and `-->`).
        let mut on_comment = false;
        // If `true`, we are in an inline comment (`... <!--`).
        let mut is_inline_comment = false;
        let mut release_note_start = None;
        let mut cur_release = Release { version: "", title: "", notes: "" };

        while let Some((line, line_start, line_end)) = self.lines.peek() {
            let line = trim_start(line);
            let heading = if on_code_block.is_some() || on_comment && !is_inline_comment {
                None
            } else {
                heading(line, &mut self.lines)
            };
            let Some(heading) = heading else {
                let line = line.as_bytes();
                self.lines.next();
                if let Some(fence) = on_code_block {
                    if let Some(rest) = line.strip_prefix(fence) {
                        let b = fence[0];
                        if all_allow_end_spaces(rest, b) {
                            on_code_block = None;
                        }
                    }
                } else {
                    if !on_comment {
                        if let Some(&b @ (b'`' | b'~')) = line.first() {
                            let mut len = 1;
                            while line.get(len) == Some(&b) {
                                len += 1;
                            }
                            if len >= 3 && (b != b'`' || !line[len..].contains(&b'`')) {
                                on_code_block = Some(&line[..len]);
                            }
                        }
                    }
                    if on_code_block.is_none() {
                        self.handle_comment(&mut on_comment, &mut is_inline_comment, line);
                    }
                }

                // Non-heading lines are always considered part of the current
                // section.

                if line_end == self.lines.text.len() {
                    break;
                }
                continue;
            };
            on_comment = false;
            if let Some(release_level) = self.level {
                if heading.level > release_level {
                    // Consider sections that have lower heading levels than
                    // release sections are part of the current section.
                    self.lines.next();
                    if line_end == self.lines.text.len() {
                        break;
                    }
                    continue;
                }
                if heading.level < release_level {
                    // Ignore sections that have higher heading levels than
                    // release sections.
                    self.lines.next();
                    if let Some(release_note_start) = release_note_start {
                        return Some(self.end_release(cur_release, release_note_start, line_start));
                    }
                    if line_end == self.lines.text.len() {
                        break;
                    }
                    continue;
                }
                if let Some(release_note_start) = release_note_start {
                    return Some(self.end_release(cur_release, release_note_start, line_start));
                }
            }

            debug_assert!(release_note_start.is_none());
            let version = extract_version_from_title(heading.text, self.prefix_format).0;
            if !self.version_format.is_match(version) {
                // Ignore non-release sections that have the same heading
                // levels as release sections.
                self.lines.next();
                if line_end == self.lines.text.len() {
                    break;
                }
                continue;
            }

            cur_release.version = version;
            cur_release.title = heading.text;
            self.level.get_or_insert(heading.level);

            self.lines.next();
            if heading.style == HeadingStyle::Setext {
                // Skip an underline after a Setext-style heading.
                self.lines.next();
            }
            while let Some((next, ..)) = self.lines.peek() {
                if next.trim_start().is_empty() {
                    // Skip newlines after a heading.
                    self.lines.next();
                } else {
                    break;
                }
            }
            if let Some((_, line_start, _)) = self.lines.peek() {
                release_note_start = Some(line_start);
            } else {
                break;
            }
        }

        if !cur_release.version.is_empty() {
            if let Some(release_note_start) = release_note_start {
                // Remove trailing newlines.
                cur_release.notes = self.lines.text[release_note_start..].trim_end();
            }
            return Some(cur_release);
        }

        None
    }
}

struct Lines<'a> {
    text: &'a str,
    iter: memchr::Memchr<'a>,
    line_start: usize,
    peeked: Option<(&'a str, usize, usize)>,
    peeked2: Option<(&'a str, usize, usize)>,
}

impl<'a> Lines<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            iter: memchr::memchr_iter(b'\n', text.as_bytes()),
            line_start: 0,
            peeked: None,
            peeked2: None,
        }
    }

    fn peek(&mut self) -> Option<(&'a str, usize, usize)> {
        self.peeked = self.next();
        self.peeked
    }

    fn peek2(&mut self) -> Option<(&'a str, usize, usize)> {
        let peeked = self.next();
        let peeked2 = self.next();
        self.peeked = peeked;
        self.peeked2 = peeked2;
        self.peeked2
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = (&'a str, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(triple) = self.peeked.take() {
            return Some(triple);
        }
        if let Some(triple) = self.peeked2.take() {
            return Some(triple);
        }
        let (line, line_end) = match self.iter.next() {
            Some(line_end) => (&self.text[self.line_start..line_end], line_end),
            None => (self.text.get(self.line_start..)?, self.text.len()),
        };
        let line_start = mem::replace(&mut self.line_start, line_end + 1);
        Some((line, line_start, line_end))
    }
}

struct Heading<'a> {
    text: &'a str,
    level: u8,
    style: HeadingStyle,
}

#[derive(PartialEq)]
enum HeadingStyle {
    /// Atx-style headings use 1-6 `#` characters at the start of the line,
    /// corresponding to header levels 1-6.
    Atx,
    /// Setext-style headings are "underlined" using equal signs `=` (for
    /// first-level headings) and dashes `-` (for second-level headings).
    Setext,
}

fn heading<'a>(line: &'a str, lines: &mut Lines<'a>) -> Option<Heading<'a>> {
    if line.as_bytes().first() == Some(&b'#') {
        let mut level = 1;
        while level <= 7 && line.as_bytes().get(level) == Some(&b'#') {
            level += 1;
        }
        // https://pandoc.org/try/?params=%7B%22text%22%3A%22%23%23%23%23%23%23%5Cn%3D%3D%3D%5Cn%5Cn%23%23%23%23%23%23%23%5Cn%3D%3D%3D%5Cn%5Cn%23%23%23%23%23%23+%5Cn%3D%3D%3D%5Cn%5Cn%23%23%23%23%23%23%5Ct%5Cn%3D%3D%3D%5Cn%5Cn%23%23%23%23%23%23+a%5Cn%3D%3D%3D%5Cn%5Cn%23%23%23%23%23%23%5Cta%5Cn%3D%3D%3D%5Cn%5Cn%23%23%23%23%23%23+b%5Cn%5Cn%22%2C%22to%22%3A%22html5%22%2C%22from%22%3A%22commonmark%22%2C%22standalone%22%3Afalse%2C%22embed-resources%22%3Afalse%2C%22table-of-contents%22%3Afalse%2C%22number-sections%22%3Afalse%2C%22citeproc%22%3Afalse%2C%22html-math-method%22%3A%22plain%22%2C%22wrap%22%3A%22auto%22%2C%22highlight-style%22%3Anull%2C%22files%22%3A%7B%7D%2C%22template%22%3Anull%7D
        if level < 7 && line.as_bytes().get(level).is_none_or(|&b| matches!(b, b' ' | b'\t')) {
            return Some(Heading {
                text: line.get(level + 1..).map(str::trim).unwrap_or_default(),
                #[allow(clippy::cast_possible_truncation)] // false positive: level is < 7: https://github.com/rust-lang/rust-clippy/issues/7486
                level: level as u8,
                style: HeadingStyle::Atx,
            });
        }
    }
    if let Some((next, ..)) = lines.peek2() {
        let next = trim_start_bytes(next.as_bytes());
        if let Some((&b @ (b'=' | b'-'), next)) = next.split_first() {
            if all_allow_end_spaces(next, b) {
                return Some(Heading {
                    text: line.trim_end(),
                    level: if b == b'=' { 1 } else { 2 },
                    style: HeadingStyle::Setext,
                });
            }
        }
    }
    None
}

#[inline]
fn trim_start(s: &str) -> &str {
    let mut count = 0;
    while s.as_bytes().get(count) == Some(&b' ') {
        count += 1;
        if count == 4 {
            return s;
        }
    }
    // Indents less than 4 are ignored.
    &s[count..]
}
#[inline]
fn trim_start_bytes(s: &[u8]) -> &[u8] {
    let mut count = 0;
    while s.get(count) == Some(&b' ') {
        count += 1;
        if count == 4 {
            return s;
        }
    }
    // Indents less than 4 are ignored.
    &s[count..]
}

#[inline]
fn all_allow_end_spaces(mut s: &[u8], b: u8) -> bool {
    while let Some((&b_, s_next)) = s.split_first() {
        if b_ != b {
            break;
        }
        s = s_next;
    }
    while let Some((b' ' | b'\t' | b'\r', s_next)) = s.split_first() {
        s = s_next;
    }
    s.is_empty()
}

fn extract_version_from_title<'a>(mut text: &'a str, prefix_format: &Regex) -> (&'a str, &'a str) {
    // Remove link from prefix
    // [Version 1.0.0 2022-01-01]
    // ^
    text = text.strip_prefix('[').unwrap_or(text);
    // Remove prefix
    // Version 1.0.0 2022-01-01]
    // ^^^^^^^^
    if let Some(m) = prefix_format.find(text) {
        text = &text[m.end()..];
    }
    // Remove whitespace after the version and the strings following it
    // 1.0.0 2022-01-01]
    //      ^^^^^^^^^^^^
    text = text.split(char::is_whitespace).next().unwrap();
    // Remove link from version
    // Version [1.0.0 2022-01-01]
    //         ^
    // [Version 1.0.0] 2022-01-01
    //               ^
    // Version [1.0.0] 2022-01-01
    //         ^     ^
    unlink(text)
}

/// Remove a link from the given markdown text.
///
/// # Note
///
/// This is not a full "unlink" on markdown. See `full_unlink` for "full" version.
fn unlink(mut s: &str) -> (&str, &str) {
    // [1.0.0]
    // ^
    s = s.strip_prefix('[').unwrap_or(s);
    if let Some(pos) = memchr::memchr(b']', s.as_bytes()) {
        // 1.0.0]
        //      ^
        if pos + 1 == s.len() {
            return (&s[..pos], "");
        }
        let remaining = &s[pos + 1..];
        // 1.0.0](link)
        //      ^^^^^^^
        // 1.0.0][link]
        //      ^^^^^^^
        for (open, close) in [(b'(', b')'), (b'[', b']')] {
            if remaining.as_bytes().first() == Some(&open) {
                if let Some(r_pos) = memchr::memchr(close, &remaining.as_bytes()[1..]) {
                    return (&s[..pos], &remaining[r_pos + 2..]);
                }
            }
        }
        return (&s[..pos], remaining);
    }
    (s, "")
}

/// Remove links from the given markdown text.
fn full_unlink(s: &str) -> Cow<'_, str> {
    let mut remaining = s;
    if let Some(mut pos) = memchr::memchr(b'[', remaining.as_bytes()) {
        let mut buf = String::with_capacity(remaining.len());
        loop {
            buf.push_str(&remaining[..pos]);
            let (t, r) = unlink(&remaining[pos..]);
            buf.push_str(t);
            remaining = r;
            match memchr::memchr(b'[', remaining.as_bytes()) {
                Some(p) => pos = p,
                None => break,
            }
        }
        buf.push_str(remaining);
        buf.into()
    } else {
        remaining.into()
    }
}
