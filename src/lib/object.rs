// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! An Object file is a partially-compiled representation of a source code
//! file. Each Object *always* maps to one source file. Symbols not defined
//! in the file are "imports" to be linked against other Objects.

use crate::atom::Atom;
use crate::r#struct::Struct;
use crate::table::Table;
use std::collections::HashMap;

pub type Structs<'token> = HashMap<String, Struct<'token>>;
pub type Atoms<'token> = HashMap<String, Atom<'token>>;
pub type Tables<'token> = Vec<Table<'token>>;

#[derive(Default)]
pub struct Object<'token> {
    pub atoms: Atoms<'token>,
    _tables: Tables<'token>,
    _structs: Structs<'token>,
}

use crate::error::*;
use crate::node::Node;

impl<'token> Object<'token> {
    //==========================================================================
    /// Define a new Atom, storing it in the Object's symbol pool.
    ///
    /// Takes a reference to an AST `Node`, since the node will remain in the
    /// AST and the Atom will only need the token within.
    ///
    /// Returns `None` if successful, otherwise if attempting to define an
    /// Atom that already exists, returns a `ParseError`.
    ///
    pub fn new_atom(&mut self, node: &'token Node<'token>) -> MaybeError {
        self.atoms
            .insert(node.to_string(), Atom::from(node))
            //TODO: include error details from the `Ok(V)`
            .and_then(|_v| Some(ParseError::duplicate()))
    }
}
