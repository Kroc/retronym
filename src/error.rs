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

/// A crate private constructor for `Error`.
pub fn new_error(kind: ErrorKind) -> Error {
    // use `pub(crate)` when it stabilizes
    Error(Box::new(kind))
}

pub type Result<T> = result::Result<T, Error>;

/// Allow a type conversion to return a potential error;
/// this is not on stable Rust yet so we use a similarly named trait
pub trait TryFrom_<'t, T>: Sized {
    fn try_from_(t: T) -> Result<Self>;
}

/// Our own wrapping Error-type that can contain a Rust std Error, such as
/// `io::Error`, or our own according to the `ErrorKind` enum
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Return the specific type of this error
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }

    /// Returns true if this is an I/O error.
    ///
    /// If this is true, the underlying `ErrorKind` is guaranteed to be
    /// `ErrorKind::Io`.
    pub fn is_io_error(&self) -> bool {
        match *self.0 {
            ErrorKind::Io(_) => true,
            _ => false,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Test => "TEST ERROR",
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::ParseInt(ref err) => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self.0 {
            // Test Error does not contain any error-specific data
            ErrorKind::Test => None,
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::ParseInt(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.0 {
            ErrorKind::Test => write!(f, "TEST ERROR"),
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::ParseInt(ref err) => err.fmt(f),
            _ => unreachable!(),
        }
    }
}

/// The specific type of an error
#[derive(Debug)]
pub enum ErrorKind {
    #[doc(hidden)]
    Test,

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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        new_error(ErrorKind::Io(err))
    }
}
impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl From<StdParseIntError> for Error {
    fn from(err: StdParseIntError) -> Error {
        new_error(ErrorKind::ParseInt(err))
    }
}
