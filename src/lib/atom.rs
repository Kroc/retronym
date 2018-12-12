// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! **Atoms** are unique symbols with no value. Used for machine registers,
//! e.g. "`A`", "`X`", "`HL`" etc.

// An Atom definition. Not an Atom invocation instance -- these appear within
// the AST -- but an Atom that an Object file defines and exports.
#[derive(Debug,Clone)]
pub struct Atom {
    /// The name/symol of the Atom.
    pub name: String,
}

impl Atom {
    //==========================================================================
    /// Create a new Atom. Only the name is required.
    pub fn new(name: &str) -> Self {
        //----------------------------------------------------------------------
        Self {
            name: name.to_string(),
        }
    }
}

use std::fmt::{self, *};

impl Display for Atom {
    //==========================================================================
    /// Print the Atom's symbol.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "{}", self.name)
    }
}

use std::collections::HashMap;

/// A collection of defined Atoms. Object files will contain these for export.
pub type Atoms = HashMap<String, Atom>;
