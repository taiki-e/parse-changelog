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
//! # Format
//!
//! By default, this crate is intended to support markdown-based changelogs
//! that have the title of each release starts with the version format based on
//! [Semantic Versioning][semver]. (e.g., [Keep a Changelog][keepachangelog]'s
//! changelog format.)
//!
//! ### Headings
//!
//! The heading for each release must be Atx-style (1-6 `#`) or
//! Setext-style (`=` or `-` in a line under text), and the heading levels
//! must match with other releases.
//!
//! Atx-style headings:
//!
//! ```markdown
//! # 0.1.0
//! ```
//!
//! ```markdown
//! ## 0.1.0
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
//! ### Titles
//!
//! The title of each release must start with a text or a link text (text with
//! `[` and `]`) that starts with a valid version format. For example:
//!
//! ```markdown
//! # [0.2.0]
//!
//! description...
//!
//! # 0.1.0
//!
//! description...
//! ```
//!
//! You can also include characters before the version as prefix.
//!
//! ```text
//! ## Version 0.1.0
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
//! # 0.1.0 - 2020-01-01
//!        ^^^^^^^^^^^^^
//! ```
//!
//! ### Versions
//!
//! ```text
//! ## v0.1.0 -- 2020-01-01
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
#![warn(clippy::default_trait_access)]

#[cfg(test)]
#[path = "gen/assert_impl.rs"]
mod assert_impl;

mod error;

use std::{iter::Peekable, mem, str::Lines};

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;

pub use crate::error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

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

/// A release note for a version.
#[derive(Debug, Clone)]
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
    /// Note that leading and trailing [whitespaces](char::is_whitespace) have
    /// been removed.
    pub title: &'a str,
    /// The descriptions of this release.
    ///
    /// Note that leading and trailing newlines have been removed.
    pub notes: String,
}

impl Release<'_> {
    fn new() -> Self {
        Self { version: "", title: "", notes: String::new() }
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

static DEFAULT_PREFIX_FORMAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(v|Version |Release )?").unwrap());
static DEFAULT_VERSION_FORMAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d+\.\d+\.\d+(-[\w\.-]+)?(\+[\w\.-]+)?$").unwrap());

impl Parser {
    /// Creates a new changelog parser.
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
    /// ## Version 0.1.0 -- 2020-01-01
    ///    ^^^^^^^^
    /// ```
    /// ```text
    /// ## v0.1.0 -- 2020-01-01
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

    fn get_version_format(&self) -> &Regex {
        self.version.as_ref().unwrap_or(&DEFAULT_VERSION_FORMAT)
    }

    fn get_prefix_format(&self) -> &Regex {
        self.prefix.as_ref().unwrap_or(&DEFAULT_PREFIX_FORMAT)
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
        parse_inner(self, text)
    }
}

fn parse_inner<'a>(parser: &Parser, text: &'a str) -> Result<Changelog<'a>> {
    const LN: char = '\n';

    let version_format = parser.get_version_format();
    let prefix_format = parser.get_prefix_format();

    let mut map = IndexMap::new();
    let mut insert_release = |mut cur_release: Release<'a>| {
        debug_assert!(!cur_release.version.is_empty());
        while cur_release.notes.ends_with(LN) {
            // Remove trailing newlines.
            cur_release.notes.pop();
        }
        if let Some(release) = map.insert(cur_release.version, cur_release) {
            return Err(Error::parse(format!("multiple release notes for '{}'", release.version)));
        }
        Ok(())
    };

    let lines = &mut text.lines().peekable();
    let mut cur_release = Release::new();
    // If `true`, we are in a release section.
    let mut on_release = false;
    // If `true`, we are in a code block ("```").
    let mut on_code_block = false;
    // If `true`, we are in a comment (`<!--` and `-->`).
    let mut on_comment = false;
    // The heading level of release sections.
    let mut level = None;

    while let Some(line) = lines.next() {
        let heading = heading(line, lines);
        if heading.is_none() || on_code_block || on_comment {
            if trim(line).starts_with("```") {
                on_code_block = !on_code_block;
            }

            if !on_code_block {
                const OPEN: &str = "<!--";
                const CLOSE: &str = "-->";
                let mut ll = Some(line);
                while let Some(l) = ll {
                    match (l.find(OPEN), l.find(CLOSE)) {
                        (None, None) => {}
                        // <!-- ...
                        (Some(_), None) => on_comment = true,
                        // ... -->
                        (None, Some(_)) => on_comment = false,
                        (Some(open), Some(close)) => {
                            if open < close {
                                // <!-- ... -->
                                on_comment = false;
                                ll = l.get(close + CLOSE.len()..);
                            } else {
                                // --> ... <!--
                                on_comment = true;
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
            if on_release {
                cur_release.notes.push_str(line);
                cur_release.notes.push(LN);
            }
            continue;
        }
        let heading = heading.unwrap();

        let mut unlinked = unlink(heading.text);
        if let Some(m) = prefix_format.find(unlinked) {
            unlinked = &unlinked[m.end()..];
        }
        let unlinked = unlink(unlinked.splitn(2, char::is_whitespace).next().unwrap());
        let version = match version_format.find(unlinked) {
            Some(m) => &unlinked[m.start()..m.end()],
            None => {
                if level.map_or(true, |l| heading.level <= l) {
                    // Ignore non-release sections that have the same or higher
                    // heading levels as release sections.
                    on_release = false;
                } else if on_release {
                    // Otherwise, it is considered part of the current section.
                    cur_release.notes.push_str(line);
                    cur_release.notes.push(LN);
                }
                continue;
            }
        };

        if mem::replace(&mut on_release, true) {
            // end of prev release
            insert_release(mem::replace(&mut cur_release, Release::new()))?;
        }

        cur_release.version = version;
        cur_release.title = heading.text;
        level.get_or_insert(heading.level);

        if heading.style == HeadingStyle::Setext {
            // Remove an underline after a Setext-style heading.
            lines.next();
        }
        while let Some(next) = lines.peek() {
            if next.trim().is_empty() {
                // Remove newlines after a heading.
                lines.next();
            } else {
                break;
            }
        }
    }

    if !cur_release.version.is_empty() {
        insert_release(cur_release)?;
    }

    if map.is_empty() {
        return Err(Error::parse("no release was found"));
    }

    Ok(map)
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

fn heading<'a>(line: &'a str, lines: &mut Peekable<Lines<'_>>) -> Option<Heading<'a>> {
    static ALL_EQUAL_SIGNS: Lazy<Regex> = Lazy::new(|| Regex::new("^=+$").unwrap());
    static ALL_DASHES: Lazy<Regex> = Lazy::new(|| Regex::new("^-+$").unwrap());

    let line = trim(line);
    if line.starts_with('#') {
        let mut level = 0;
        while line[level..].starts_with('#') {
            level += 1;
        }
        if level <= 6 {
            Some(Heading { text: line[level..].trim(), level, style: HeadingStyle::Atx })
        } else {
            None
        }
    } else if let Some(next) = lines.peek() {
        let next = trim(next);
        if ALL_EQUAL_SIGNS.is_match(next) {
            Some(Heading { text: line, level: 1, style: HeadingStyle::Setext })
        } else if ALL_DASHES.is_match(next) {
            Some(Heading { text: line, level: 2, style: HeadingStyle::Setext })
        } else {
            None
        }
    } else {
        None
    }
}

fn trim(s: &str) -> &str {
    let mut cnt = 0;
    while s[cnt..].starts_with(' ') {
        cnt += 1;
    }
    // Indents less than 4 are ignored.
    if cnt < 4 { s[cnt..].trim_end() } else { s.trim_end() }
}

/// If a leading `[` or trailing `]` exists, returns a string with it removed.
///
/// This is not a full "unlink" on markdown, but this is enough as this crate
/// does not parse a string at the end of headings.
fn unlink(mut s: &str) -> &str {
    s = s.strip_prefix('[').unwrap_or(s);
    s.strip_suffix(']').unwrap_or(s)
}
