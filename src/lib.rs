//! A simple changelog parser, written in Rust.
//!
//! # Examples
//!
//! ```rust
//! let changelog = "\
//! ## 0.1.2 - 2020-03-01
//!
//! - Bug fixes.
//!
//! ## 0.1.1 - 2020-02-01
//!
//! - Added `Foo`.
//! - Added `Bar`.
//!
//! ## 0.1.0 - 2020-01-01
//!
//! Initial release
//! ";
//!
//! // Parse changelog.
//! let changelog = parse_changelog::parse(changelog).unwrap();
//!
//! // Get the latest release.
//! assert_eq!(changelog[0].version, "0.1.2");
//! assert_eq!(changelog[0].title, "0.1.2 - 2020-03-01");
//! assert_eq!(changelog[0].notes, "- Bug fixes.");
//!
//! // Get the specified release.
//! assert_eq!(changelog["0.1.0"].title, "0.1.0 - 2020-01-01");
//! assert_eq!(changelog["0.1.0"].notes, "Initial release");
//! assert_eq!(changelog["0.1.1"].title, "0.1.1 - 2020-02-01");
//! assert_eq!(
//!     changelog["0.1.1"].notes,
//!     "- Added `Foo`.\n\
//!      - Added `Bar`."
//! );
//! ```
//!
//! The key of the map returned does not include prefixes such as
//! "v", "Version ", etc.
//!
//! ```rust
//! let changelog_a = "\
//! ## Version 0.1.0 - 2020-01-01
//! Initial release
//! ";
//! let changelog_b = "\
//! ## v0.1.0 - 2020-02-01
//! Initial release
//! ";
//!
//! let changelog_a = parse_changelog::parse(changelog_a).unwrap();
//! let changelog_b = parse_changelog::parse(changelog_b).unwrap();
//! // Not `changelog_a["Version 0.1.0"]`
//! assert_eq!(changelog_a["0.1.0"].version, "0.1.0");
//! assert_eq!(changelog_a["0.1.0"].title, "Version 0.1.0 - 2020-01-01");
//! assert_eq!(changelog_a["0.1.0"].notes, "Initial release");
//! // Not `changelog_b["v0.1.0"]`
//! assert_eq!(changelog_b["0.1.0"].version, "0.1.0");
//! assert_eq!(changelog_b["0.1.0"].title, "v0.1.0 - 2020-02-01");
//! assert_eq!(changelog_b["0.1.0"].notes, "Initial release");
//! ```
//!
//! # Supported Format
//!
//! By default, this crate is intended to support markdown-based changelogs
//! that have the title of each release starts with the version format based on
//! [Semantic Versioning][semver]. (e.g., [Keep a Changelog][keepachangelog]'s
//! changelog format.)
//!
//! ## Headings
//!
//! The heading for each release must be Atx-style (1-6 `#`) or
//! Setext-style (`=` or `-` in a line under text), and the heading levels
//! must match with other releases.
//!
//! Atx-style headings:
//!
//! ```markdown
//! ## 0.1.0
//! ```
//!
//! ```markdown
//! ### 0.1.0
//! ```
//!
//! Setext-style headings:
//!
//! ```markdown
//! 0.1.0
//! =====
//! ```
//!
//! ```markdown
//! 0.1.0
//! -----
//! ```
//!
//! ## Titles
//!
//! The title of each release must start with a text or a link text (text with
//! `[` and `]`) that starts with a valid version format. For example:
//!
//! ```markdown
//! ## [0.2.0]
//!
//! description...
//!
//! ## 0.1.0
//!
//! description...
//! ```
//!
//! You can also include characters before the version as prefix.
//!
//! ```text
//! ### Version 0.1.0
//!    ^^^^^^^^
//! ```
//!
//! By default only "v", "Version " and "Release " are allowed as prefixes and
//! can be customized using the [`Parser::prefix_format`] method.
//!
//! You can freely include characters after the version (this crate does not
//! parse it).
//!
//! ```text
//! ## 0.1.0 - 2020-01-01
//!        ^^^^^^^^^^^^^
//! ```
//!
//! ## Versions
//!
//! ```text
//! ### v0.1.0 -- 2020-01-01
//!     ^^^^^
//! ```
//!
//! The default version format is
//! `MAJOR.MINOR.PATCH(-PRE_RELEASE)?(+BUILD_METADATA)?`, and is
//! based on [Semantic Versioning][semver]. (Pre-release version and build
//! metadata are optional.)
//!
//! This is parsed using the following regular expression:
//!
//! ```text
//! ^\d+\.\d+\.\d+(-[\w\.-]+)?(\+[\w\.-]+)?$
//! ```
//!
//! To customize the version format, use the [`Parser::version_format`] method.
//!
//! # Optional features
//!
//! * **`serde`** — Implements [`serde::Serialize`] trait for parse-changelog types.
//!
//! [`serde::Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html
//! [keepachangelog]: https://keepachangelog.com/en/1.0.0
//! [semver]: https://semver.org/spec/v2.0.0.html

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    single_use_lifetimes,
    unreachable_pub
)]
#![warn(
    clippy::default_trait_access,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::wildcard_imports
)]

#[cfg(test)]
#[path = "gen/assert_impl.rs"]
mod assert_impl;

mod error;

use std::{iter::Peekable, mem};

use indexmap::IndexMap;
use memchr::memmem;
use once_cell::sync::Lazy;
use regex::Regex;

pub use crate::error::Error;
use crate::error::Result;

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
/// [keepachangelog]: https://keepachangelog.com/en/1.0.0
pub type Changelog<'a> = IndexMap<&'a str, Release<'a>>;

/// Parses release notes from the given `text`.
///
/// This function uses the default version and prefix format. If you want to use
/// another version format, use [`Parser::version_format`].
///
/// See crate level documentation for changelog and version format supported
/// by default.
///
/// # Errors
///
/// Returns an error if any of the following:
///
/// - There are multiple release notes for one version.
/// - No release was found. This usually means that the changelog isn't
///   written in the supported format.
pub fn parse(text: &str) -> Result<Changelog<'_>> {
    Parser::new().parse(text)
}

/// An iterator over all release notes in the given `text`.
///
/// Unlike [`parse`] function, this function doesn't error on duplicate release
/// notes or empty changelog.
///
/// This function uses the default version and prefix format. If you want to use
/// another version format, use [`Parser::version_format`].
///
/// See crate level documentation for changelog and version format supported
/// by default.
pub fn parse_iter(text: &str) -> ParseIter<'_, 'static> {
    ParseIter::new(text, None, None)
}

/// A release note for a version.
#[allow(single_use_lifetimes)] // https://github.com/rust-lang/rust/issues/69952
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde_crate::Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_crate"))]
#[non_exhaustive]
pub struct Release<'a> {
    /// The version of this release.
    ///
    /// ```text
    /// ### Version 0.1.0 -- 2020-01-01
    ///            ^^^^^
    /// ```
    ///
    /// This is the same value as the key of the [`Changelog`] type.
    pub version: &'a str,
    /// The title of this release.
    ///
    /// ```text
    /// ### Version 0.1.0 -- 2020-01-01
    ///    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// Note that leading and trailing [whitespaces](char::is_whitespace) have
    /// been removed.
    pub title: &'a str,
    /// The descriptions of this release.
    ///
    /// Note that leading and trailing newlines have been removed.
    pub notes: &'a str,
}

impl Release<'_> {
    fn new() -> Self {
        Self { version: "", title: "", notes: "" }
    }
}

/// A changelog parser.
#[derive(Debug, Default)]
pub struct Parser {
    /// Version format. e.g., "0.1.0" in "# v0.1.0 (2020-01-01)".
    ///
    /// If `None`, `DEFAULT_VERSION_FORMAT` is used.
    version: Option<Regex>,
    /// Prefix format. e.g., "v" in "# v0.1.0 (2020-01-01)", "Version " in
    /// "# Version 0.1.0 (2020-01-01)".
    ///
    /// If `None`, `DEFAULT_PREFIX_FORMAT` is used.
    prefix: Option<Regex>,
}

impl Parser {
    /// Creates a new changelog parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the version format.
    ///
    /// ```text
    /// ### v0.1.0 -- 2020-01-01
    ///     ^^^^^
    /// ```
    ///
    /// The default version format is based on [Semantic Versioning][semver]
    /// and is the following regular expression:
    ///
    /// ```text
    /// ^\d+\.\d+\.\d+(-[\w\.-]+)?(\+[\w\.-]+)?
    /// ```
    ///
    /// **Note**: Most projects that adopt [Semantic Versioning][semver] do not
    /// need to change this.
    ///
    /// To customize the text before the version number (e.g., "v" in "# v0.1.0",
    /// "Version " in "# Version 0.1.0", etc.), use the [`prefix_format`] method
    /// instead of this method.
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
    /// [`parse`]: Self::parse
    /// [`prefix_format`]: Self::prefix_format
    /// [regex]: https://docs.rs/regex
    /// [semver]: https://semver.org/spec/v2.0.0.html
    pub fn version_format(&mut self, version_format: &str) -> Result<&mut Self> {
        if version_format.trim().is_empty() {
            return Err(Error::format("empty or whitespace version format"));
        }
        self.version = Some(Regex::new(version_format).map_err(Error::new)?);
        Ok(self)
    }

    /// Sets the prefix format.
    ///
    /// "Prefix" means the range from the first non-whitespace character after
    /// heading to the character before the version (including whitespace
    /// characters). For example:
    ///
    /// ```text
    /// ### Version 0.1.0 -- 2020-01-01
    ///    ^^^^^^^^
    /// ```
    /// ```text
    /// ### v0.1.0 -- 2020-01-01
    ///    ^
    /// ```
    ///
    /// The default prefix format is the following regular expression:
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
    /// - The specified format is empty or contains only
    ///   [whitespace](char::is_whitespace).
    ///
    /// [`parse`]: Self::parse
    /// [`version_format`]: Self::version_format
    /// [regex]: https://docs.rs/regex
    pub fn prefix_format(&mut self, prefix_format: &str) -> Result<&mut Self> {
        if prefix_format.trim().is_empty() {
            return Err(Error::format("empty or whitespace version format"));
        }
        self.prefix = Some(Regex::new(prefix_format).map_err(Error::new)?);
        Ok(self)
    }

    /// Parses release notes from the given `text`.
    ///
    /// See crate level documentation for changelog and version format supported
    /// by default.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following:
    ///
    /// - There are multiple release notes for one version.
    /// - No release was found. This usually means that the changelog isn't
    ///   written in the supported format, or that the specified format is wrong
    ///   if you specify your own format.
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
            return Err(Error::parse("no release was found"));
        }
        Ok(map)
    }

    /// An iterator over all release notes in the given `text`.
    ///
    /// Unlike [`parse`] method, this method doesn't error on duplicate release
    /// notes or empty changelog.
    ///
    /// See crate level documentation for changelog and version format supported
    /// by default.
    ///
    /// [`parse`]: Self::parse
    pub fn parse_iter<'a, 'b>(&'b self, text: &'a str) -> ParseIter<'a, 'b> {
        ParseIter::new(text, self.version.as_ref(), self.prefix.as_ref())
    }
}

/// An iterator for [`parse_iter`].
#[allow(missing_debug_implementations)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ParseIter<'a, 'b> {
    text: &'a str,
    version_format: &'b Regex,
    prefix_format: &'b Regex,
    line_start: usize,
    lines: Peekable<memchr::Memchr<'a>>,
    cur_release: Release<'a>,
    cur_release_start: usize,
    /// If `true`, we are in a release section.
    on_release: bool,
    /// If `true`, we are in a code block ("```").
    on_code_block: bool,
    /// If `true`, we are in a comment (`<!--` and `-->`).
    on_comment: bool,
    /// The heading level of release sections.
    level: Option<usize>,
    find_open: memmem::Finder<'static>,
    find_close: memmem::Finder<'static>,
}

const OPEN: &[u8] = b"<!--";
const CLOSE: &[u8] = b"-->";

impl<'a, 'b> ParseIter<'a, 'b> {
    fn new(
        text: &'a str,
        version_format: Option<&'b Regex>,
        prefix_format: Option<&'b Regex>,
    ) -> Self {
        static DEFAULT_PREFIX_FORMAT: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(v|Version |Release )?").unwrap());
        static DEFAULT_VERSION_FORMAT: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\d+\.\d+\.\d+(-[\w\.-]+)?(\+[\w\.-]+)?$").unwrap());

        Self {
            text,
            version_format: version_format.unwrap_or(&DEFAULT_VERSION_FORMAT),
            prefix_format: prefix_format.unwrap_or(&DEFAULT_PREFIX_FORMAT),
            line_start: 0,
            lines: memchr::memchr_iter(b'\n', text.as_bytes()).peekable(),
            cur_release: Release::new(),
            cur_release_start: 0,
            on_release: false,
            on_code_block: false,
            on_comment: false,
            level: None,
            find_open: memmem::Finder::new(OPEN),
            find_close: memmem::Finder::new(CLOSE),
        }
    }
}

impl<'a> Iterator for ParseIter<'a, '_> {
    type Item = Release<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_value = None;
        loop {
            let (line, line_end) = match self.lines.next() {
                Some(line_end) => (&self.text[self.line_start..line_end], line_end),
                None => {
                    if let Some(l) = self.text.get(self.line_start..) {
                        (l, self.text.len())
                    } else {
                        break;
                    }
                }
            };
            let heading = heading(self.text, line, line_end, &mut self.lines);
            if heading.is_none() || self.on_code_block || self.on_comment {
                if trim(line).starts_with("```") {
                    self.on_code_block = !self.on_code_block;
                }

                if !self.on_code_block {
                    let mut ll = Some(line);
                    while let Some(l) = ll {
                        match (
                            self.find_open.find(l.as_bytes()),
                            self.find_close.find(l.as_bytes()),
                        ) {
                            (None, None) => {}
                            // <!-- ...
                            (Some(_), None) => self.on_comment = true,
                            // ... -->
                            (None, Some(_)) => self.on_comment = false,
                            (Some(open), Some(close)) => {
                                if open < close {
                                    // <!-- ... -->
                                    self.on_comment = false;
                                    ll = l.get(close + CLOSE.len()..);
                                } else {
                                    // --> ... <!--
                                    self.on_comment = true;
                                    ll = l.get(open + OPEN.len()..);
                                }
                                continue;
                            }
                        }
                        break;
                    }
                }

                // Non-heading lines are always considered part of the current
                // section.
                if line_end == self.text.len() {
                    break;
                }
                self.line_start = line_end + 1;
                continue;
            }
            let heading = heading.unwrap();

            let mut unlinked = unlink(heading.text);
            if let Some(m) = self.prefix_format.find(unlinked) {
                unlinked = &unlinked[m.end()..];
            }
            let unlinked = unlink(unlinked.splitn(2, char::is_whitespace).next().unwrap());
            let version = match self.version_format.find(unlinked) {
                Some(m) => &unlinked[m.start()..m.end()],
                None => {
                    if self.level.map_or(true, |l| heading.level <= l) {
                        // Ignore non-release sections that have the same or higher
                        // heading levels as release sections.
                        self.on_release = false;
                    } else {
                        // Otherwise, it is considered part of the current section.
                    }
                    if line_end == self.text.len() {
                        break;
                    }
                    self.line_start = line_end + 1;
                    continue;
                }
            };

            if mem::replace(&mut self.on_release, true) {
                if self.cur_release_start < self.line_start {
                    // Remove trailing newlines.
                    self.cur_release.notes =
                        self.text[self.cur_release_start..self.line_start - 1].trim_end();
                }
                // end of prev release
                debug_assert!(!self.cur_release.version.is_empty());
                debug_assert!(next_value.is_none());
                next_value = Some(mem::replace(&mut self.cur_release, Release::new()));
            }

            self.cur_release.version = version;
            self.cur_release.title = heading.text;
            self.level.get_or_insert(heading.level);

            self.line_start = line_end + 1;
            if heading.style == HeadingStyle::Setext {
                // Remove an underline after a Setext-style heading.
                if let Some(next) = self.lines.next() {
                    self.line_start = next + 1;
                }
            }
            while let Some(&next) = self.lines.peek() {
                if self.text[self.line_start..next].trim().is_empty() {
                    // Remove newlines after a heading.
                    self.line_start = next + 1;
                    self.lines.next();
                } else {
                    break;
                }
            }
            self.cur_release_start = self.line_start;
            if next_value.is_some() {
                return next_value.take();
            }
            if line_end == self.text.len() {
                break;
            }
        }

        if !self.cur_release.version.is_empty() {
            if self.cur_release_start < self.line_start {
                // Remove trailing newlines.
                self.cur_release.notes = self.text[self.cur_release_start..].trim_end();
            }
            return Some(mem::replace(&mut self.cur_release, Release::new()));
        }

        None
    }
}

struct Heading<'a> {
    text: &'a str,
    level: usize,
    style: HeadingStyle,
}

#[derive(Eq, PartialEq)]
enum HeadingStyle {
    /// Atx-style headings use 1-6 `#` characters at the start of the line,
    /// corresponding to header levels 1-6.
    Atx,
    /// Setext-style headings are “underlined” using equal signs `=` (for
    /// first-level headings) and dashes `-` (for second-level headings).
    Setext,
}

fn heading<'a>(
    text: &'a str,
    line: &'a str,
    line_end: usize,
    lines: &mut Peekable<impl Iterator<Item = usize>>,
) -> Option<Heading<'a>> {
    let line = trim(line);
    if line.starts_with('#') {
        let mut level = 0;
        while line.as_bytes().get(level) == Some(&b'#') {
            level += 1;
        }
        if level <= 6 && line.as_bytes().get(level) == Some(&b' ') {
            Some(Heading { text: line[level..].trim(), level, style: HeadingStyle::Atx })
        } else {
            None
        }
    } else if let Some(&next) = lines.peek() {
        let next = trim(&text[line_end + 1..next]);
        if next.is_empty() {
            None
        } else if next.as_bytes().iter().all(|&b| b == b'=') {
            Some(Heading { text: line, level: 1, style: HeadingStyle::Setext })
        } else if next.as_bytes().iter().all(|&b| b == b'-') {
            Some(Heading { text: line, level: 2, style: HeadingStyle::Setext })
        } else {
            None
        }
    } else {
        None
    }
}

fn trim(s: &str) -> &str {
    let mut count = 0;
    while s.as_bytes().get(count) == Some(&b' ') {
        count += 1;
    }
    // Indents less than 4 are ignored.
    if count < 4 { s[count..].trim_end() } else { s.trim_end() }
}

/// If a leading `[` or trailing `]` exists, returns a string with it removed.
///
/// This is not a full "unlink" on markdown, but this is enough as this crate
/// does not parse a string at the end of headings.
fn unlink(mut s: &str) -> &str {
    s = s.strip_prefix('[').unwrap_or(s);
    s.strip_suffix(']').unwrap_or(s)
}
