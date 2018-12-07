// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Unique symbols with no value. Used for machine registers, e.g.
//! "A", "X", "HL" etc.

pub struct Atom {
    name: String,
}

use std::fmt::{self, *};

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

use std::collections::HashMap;

pub struct Atoms(HashMap<String, Atom>);

impl Atom {
    /// Create a new Atom. Only the name is required.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
