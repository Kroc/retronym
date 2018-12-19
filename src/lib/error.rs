// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Anything that can go wrong, will go wrong.

// NB: A lot of hints and direction taken from BurntSushi's code:
// https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs

// `Error` trait from the standard-library
use std::error::Error as StdError;
use std::io;
use std::num::ParseIntError as StdParseIntError;
use std::result;

use std::fmt;

/// Our own wrapping Error-type that can contain a Rust std Error, such as
/// `io::Error`, or our own according to the `ParseErrorKind` enum
#[derive(Debug)]
pub struct ParseError(Box<ParseErrorKind>);

/// The specific type of an error:
#[derive(Debug)]
pub enum ParseErrorKind {
    /// "End Of File" error. This occurs when processing the AST statements
    /// and the end of file is reached; it's up to the caller to decide if
    /// this is "unexpected".
    EndOfFile,

    Unexpected,

    /// Duplicate defintion.
    Duplicate,

    /// Cannot pack data without specifying a Record first.
    NoRecord,

    /// Cannot add any more data to a Table Row when it is already full.
    RowSatisfied,
    /// Cannot leave a record unsatisfied.
    Unsatisfied,

    #[doc(hidden)]
    Unimplemented,

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

/// A crate-private constructor for `ParseError`.
/// Consumers of this library should never make their own ParseError!
///
pub(crate) fn parse_error(kind: ParseErrorKind) -> ParseError {
    //--------------------------------------------------------------------------
    ParseError(Box::new(kind))
}

/// When you only need to return a potential Error.
pub type MaybeError = Option<ParseError>;

/// The `Result` type that includes `ParseError`.
pub type ParseResult<T> = result::Result<T, ParseError>;

// "I AM ERROR!"
impl ParseError {
    //==========================================================================
    /// Create an `EndOfFile` error.
    ///
    #[allow(dead_code)]
    pub(crate) fn end_of_file() -> Self {
        //----------------------------------------------------------------------
        ParseError(Box::new(ParseErrorKind::EndOfFile))
    }

    /// Create an `Unexpected` error.
    ///
    #[allow(dead_code)]
    pub(crate) fn unexpected() -> Self {
        //----------------------------------------------------------------------
        ParseError(Box::new(ParseErrorKind::Unexpected))
    }

    /// Create a `Duplicate` error.
    ///
    #[allow(dead_code)]
    pub(crate) fn duplicate() -> Self {
        //----------------------------------------------------------------------
        ParseError(Box::new(ParseErrorKind::Duplicate))
    }

    /// Create a `NoRecord` error.
    ///
    #[allow(dead_code)]
    pub(crate) fn no_record() -> Self {
        //----------------------------------------------------------------------
        ParseError(Box::new(ParseErrorKind::NoRecord))
    }

    /// Return the specific type of this error.
    ///
    pub fn kind(&self) -> &ParseErrorKind {
        //----------------------------------------------------------------------
        // return the embedded error
        &self.0
    }

    /// Unwrap this error into its underlying type.
    ///
    pub fn into_kind(self) -> ParseErrorKind {
        //----------------------------------------------------------------------
        // dereference the embedded error
        *self.0
    }

    pub fn is_endoffile(&self) -> bool {
        //----------------------------------------------------------------------
        match *self.0 {
            ParseErrorKind::EndOfFile => true,
            _ => false,
        }
    }

    /// Returns true if this is an I/O error.
    ///
    /// If this is true, the underlying `ParseErrorKind`
    /// is guaranteed to be `ParseErrorKind::Io`.
    ///
    pub fn is_io_error(&self) -> bool {
        //----------------------------------------------------------------------
        match *self.0 {
            ParseErrorKind::Io(_) => true,
            _ => false,
        }
    }
}

// "I'm a real boy!"
impl StdError for ParseError {
    //==========================================================================
    fn description(&self) -> &str {
        //----------------------------------------------------------------------
        match *self.0 {
            ParseErrorKind::Unimplemented => "Unimplemented",
            ParseErrorKind::EndOfFile => "End Of File",
            ParseErrorKind::Unexpected => "Unexpected",
            ParseErrorKind::Duplicate => "Duplicate",
            ParseErrorKind::NoRecord => "No Record",
            ParseErrorKind::RowSatisfied => "Table row is full",
            ParseErrorKind::Unsatisfied => "Record Unsatisfied",
            ParseErrorKind::Io(ref err) => err.description(),
            ParseErrorKind::ParseInt(ref err) => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        //----------------------------------------------------------------------
        match *self.0 {
            ParseErrorKind::Unimplemented
            | ParseErrorKind::EndOfFile
            | ParseErrorKind::Unexpected
            | ParseErrorKind::NoRecord
            | ParseErrorKind::RowSatisfied
            | ParseErrorKind::Unsatisfied => None,
            ParseErrorKind::Io(ref err) => Some(err),
            ParseErrorKind::ParseInt(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ParseError {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match *self.0 {
            ParseErrorKind::Unimplemented => write!(f, "Unimplemented"),
            ParseErrorKind::EndOfFile => write!(f, "End Of File"),
            ParseErrorKind::Unexpected => write!(f, "Unexpected"),
            ParseErrorKind::Duplicate => write!(f, "Duplicate"),
            ParseErrorKind::NoRecord => write!(f, "No Record"),
            ParseErrorKind::RowSatisfied => write!(f, "Table row is full"),
            ParseErrorKind::Unsatisfied => write!(f, "Record Unsatisfied"),
            ParseErrorKind::Io(ref err) => err.fmt(f),
            ParseErrorKind::ParseInt(ref err) => err.fmt(f),
            _ => unreachable!(),
        }
    }
}

impl From<io::Error> for ParseError {
    //==========================================================================
    fn from(err: io::Error) -> ParseError {
        //----------------------------------------------------------------------
        parse_error(ParseErrorKind::Io(err))
    }
}
impl From<ParseError> for io::Error {
    //==========================================================================
    fn from(err: ParseError) -> io::Error {
        //----------------------------------------------------------------------
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl From<StdParseIntError> for ParseError {
    //==========================================================================
    fn from(err: StdParseIntError) -> ParseError {
        //----------------------------------------------------------------------
        parse_error(ParseErrorKind::ParseInt(err))
    }
}
