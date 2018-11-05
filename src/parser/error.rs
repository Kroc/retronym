// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// NB: A lot of hints and direction taken from BurntSushi's code:
// https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs

// `Error` trait from the standard-library
use std::error::Error as StdError;
use std::io;
use std::num::ParseIntError as StdParseIntError;
use std::result;

use std::fmt;

/// Our own wrapping Error-type that can contain a Rust std Error, such as
/// `io::Error`, or our own according to the `ErrorKind` enum
#[derive(Debug)]
pub struct ParseError(Box<ParseErrorKind>);

/// The specific type of an error
#[derive(Debug)]
pub enum ParseErrorKind {
    #[doc(hidden)]
    Test,

    /// "End Of File" error. This occurs when reading tokens and you reach
    /// the end. It's up to the caller to decide if this is "unexpected".
    EndOfFile,

    /// An I/O error that occurred while reading source files
    Io(io::Error),
    /// An error parsing a string into a number
    ParseInt(StdParseIntError),

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

/// A crate private constructor for `Error`.
pub fn new_parse_error(kind: ParseErrorKind) -> ParseError {
    // use `pub(crate)` when it stabilizes
    ParseError(Box::new(kind))
}

pub type ParseResult<T> = result::Result<T, ParseError>;

impl ParseError {
    /// Return the specific type of this error
    pub fn kind(&self) -> &ParseErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type
    pub fn into_kind(self) -> ParseErrorKind {
        *self.0
    }

    /// Returns true if this is an I/O error.
    ///
    /// If this is true, the underlying `ErrorKind` is guaranteed to be
    /// `ErrorKind::Io`.
    pub fn is_io_error(&self) -> bool {
        match *self.0 {
            ParseErrorKind::Io(_) => true,
            _ => false,
        }
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match *self.0 {
            ParseErrorKind::Test => "TEST ERROR",
            ParseErrorKind::EndOfFile => "End Of File",
            ParseErrorKind::Io(ref err) => err.description(),
            ParseErrorKind::ParseInt(ref err) => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self.0 {
            // Test Error does not contain any error-specific data
            ParseErrorKind::Test => None,
            // should this return the file name?
            ParseErrorKind::EndOfFile => None,
            ParseErrorKind::Io(ref err) => Some(err),
            ParseErrorKind::ParseInt(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ParseErrorKind::Test => write!(f, "TEST ERROR"),
            ParseErrorKind::EndOfFile => write!(f, "End Of File"),
            ParseErrorKind::Io(ref err) => err.fmt(f),
            ParseErrorKind::ParseInt(ref err) => err.fmt(f),
            _ => unreachable!(),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        new_parse_error(ParseErrorKind::Io(err))
    }
}
impl From<ParseError> for io::Error {
    fn from(err: ParseError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl From<StdParseIntError> for ParseError {
    fn from(err: StdParseIntError) -> ParseError {
        new_parse_error(ParseErrorKind::ParseInt(err))
    }
}
