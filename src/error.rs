use std::fmt;

/// An error that occurred during parsing changelog or configuring the parser.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
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
    Format(String),

    /// An error that occurred during parsing changelog.
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Regex(e) => fmt::Display::fmt(e, f),
            Error::Format(e) => f.write_str(e),
            Error::Parse(e) => f.write_str(e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Regex(e) => Some(e),
            _ => None,
        }
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}
