// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::{boxed::Box, string::String};
use core::fmt;

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

/// An error that occurred during parsing changelog or configuring the parser.
// TODO: in next breaking release, add PhantomData<Box<dyn Send + Sync>> to make error type !UnwindSafe & !RefUnwindSafe for forward compatibility.
#[derive(Debug)]
pub struct Error(ErrorKind);

// Hiding error variants from a library's public error type to prevent
// dependency updates from becoming breaking changes.
// We can add `is_*` methods that indicate the kind of error if needed, but
// don't expose dependencies' types directly in the public API.
#[derive(Debug)]
pub(crate) enum ErrorKind {
    /// The specified format is not a valid regular expression or supported by
    /// [regex] crate.
    ///
    /// This error only occurs during configuring the parser.
    ///
    /// [regex]: https://docs.rs/regex
    Regex(regex::Error),
    /// The specified format is a valid regular expression but not a format
    /// that accepted by the parser.
    ///
    /// This error only occurs during configuring the parser.
    Format(Box<str>),
    /// An error that occurred during parsing changelog.
    Parse(Box<str>),
}

impl Error {
    pub(crate) fn new(e: impl Into<ErrorKind>) -> Self {
        Self(e.into())
    }

    pub(crate) fn format(e: impl Into<String>) -> Self {
        Self(ErrorKind::Format(e.into().into_boxed_str()))
    }

    pub(crate) fn parse(e: impl Into<String>) -> Self {
        Self(ErrorKind::Parse(e.into().into_boxed_str()))
    }

    /// Returns `true` if this error is that occurred during configuring the parser.
    #[must_use]
    pub fn is_format(&self) -> bool {
        matches!(self.0, ErrorKind::Format(..) | ErrorKind::Regex(..))
    }

    /// Returns `true` if this error is that occurred during parsing changelog.
    #[must_use]
    pub fn is_parse(&self) -> bool {
        matches!(self.0, ErrorKind::Parse(..))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorKind::Regex(e) => fmt::Display::fmt(e, f),
            ErrorKind::Format(e) | ErrorKind::Parse(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self.0 {
            ErrorKind::Regex(e) => Some(e),
            _ => None,
        }
    }
}

impl From<regex::Error> for ErrorKind {
    fn from(e: regex::Error) -> Self {
        Self::Regex(e)
    }
}

// Note: Do not implement From<ThirdPartyErrorType> to prevent dependency
// updates from becoming breaking changes.
// Implementing `From<StdErrorType>` should also be avoided whenever possible,
// as it would be a breaking change to remove the implementation if the
// conversion is no longer needed due to changes in the internal implementation.
