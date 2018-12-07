// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Unique symbols with no value. Used for machine registers, e.g.
//! "A", "X", "HL" etc.

// An Atom definition. Not an Atom inovation instance -- these appear within
// the AST -- but an Atom that an Object file defines and exports.
pub struct Atom {
    /// The name/symol of the Atom.
    pub name: String,
}

impl Atom {
    /// Create a new Atom. Only the name is required.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

use std::fmt::{self, *};

impl Display for Atom {
    /// Print the Atom's symbol.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

use std::collections::HashMap;

pub struct Atoms(HashMap<String, Atom>);
