// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! **Atoms** are unique symbols with no value. Used for machine registers,
//! e.g. "`A`", "`X`", "`HL`" etc.

use crate::token::Token;

/// An Atom definition. Not an Atom reference -- these appear within
/// the AST -- but an Atom that an Object file defines and exports.
#[derive(Debug, Clone)]
pub struct Atom<'token> {
    /// The name/symol of the Atom.
    pub name: String,
    /// A reference back to the original source code where the atom was
    /// defined, in case of error.
    token: Token<'token>,
}

use std::fmt::{self, *};

impl Display for Atom<'_> {
    //==========================================================================
    /// Print the Atom's symbol.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "{}", self.name)
    }
}

use std::convert::From;

impl<'token> From<&'token Token<'token>> for Atom<'token> {
    //==========================================================================
    /// Create an `Atom` from a `Token`; a `Token` is required as the `Atom`
    /// contains a reference to the source code where it was defined, in case
    /// of error. Does *not* check to see if the token is an actual atom;
    /// it's assumed this sort of logical test has been done by the caller.
    /// the string representation of the token is used as the atom name.
    ///
    fn from(token: &'token Token<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            name: token.to_string(),
            token: token.clone(),
        }
    }
}

use crate::node::Node;

// you can also likewise create one from an AST Node
impl<'token> From<&'token Node<'token>> for Atom<'token> {
    //==========================================================================
    /// Create an `Atom` from an AST `Node`. See the description for
    /// `From<Token> for Atom` for details, this method just passes
    /// the Node's internal token along. 
    ///
    fn from(node: &'token Node<'token>) -> Self {
        //----------------------------------------------------------------------
        // AST Nodes aren't forced to have an original source reference
        match &node.token {
            None => panic!(
                "Cannot convert Node to Atom where the Node does not
                 contain a Token reference to the original source code."
            ),
            // use the already existing `Token` -> `Atom` conversion
            Some(t) => Self::from(t),
        }
    }
}
