// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use std::boxed::Box;
use std::error::{self, Error};
use std::fmt;

/// Shorthand for `Box<Error>` so as to [try to] be able to pass both Rust
/// standard errors (e.g. `io::Error`) and our own.
pub type BoxError = Box<Error>;

/// Allow a type conversion to return a potential error;
/// this is not on stable Rust yet so we use a similarly named trait.
pub trait TryFrom_<T>: Sized {
    fn try_from_(t: T) -> Result<Self, BoxError>;
}

//------------------------------------------------------------------------------

/// TokenError is an `Error` type that occurs when tokenizing a string to
/// produce a `TokenStream`.
#[derive(Debug, Clone)]
// TODO: Add data for specific token / source location
pub struct TokenError {
    details: String,
}

impl TokenError {
    fn new(msg: &str) -> TokenError {
        TokenError {
            details: msg.to_string(),
        }
    }
}

/// Allow printing of the `TokenError` by implementing the `Display` trait.
impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid Token")
    }
}

impl Error for TokenError {
    fn description(&self) -> &str {
        "invalid Token"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
