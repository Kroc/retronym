// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Source code files are parsed into Abstract Syntax Trees which are then
//! compiled into Objects -- essentially a set of instructions for constructing
//! a binary file, given a set of imports (from other Objects) to 'fill in the
//! blanks'.

pub struct Object {
    /// An object will contain a number of 'blocks' of data that fit together
    /// to generate a final binary. Some will be raw binary data and others
    /// will be some kind of yet-unknown data such as calculations and external
    /// symbols.
    _blocks: Vec<Block>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            _blocks: Vec::new(),
        }
    }
}

/// An object will contain a number of 'blocks' of data that fit together to
/// generate a final binary. Some will be raw binary data and others will be
/// some kind of yet-unknown data such as calculations and external symbols.
pub enum Block {
    /// A 'Data' block contains just raw binary data that needs no modification
    /// before being output. Once we've decided where its going, it can be
    /// copied as-is.
    Data(Box<Data>),
}

/// A 'Data' block contains just raw binary data that needs no modification
/// before being output. Once we've decided where its going, it can be
/// copied as-is.
pub struct Data {
    _bytes: Vec<u8>,
}

//==============================================================================

use crate::ast::AST;

impl Object {
    pub fn new_from_str(source: &str) -> Self {
        // create an AST from the source code
        let ast = AST::new_from_str(source);

        // TODO: 'execute' the AST producing an object file?
        // - we need to locate the placeholders (imports) and their size
        //   (size will be set by the record type)
        // - exports are created as we populate the object

        ast.eval();

        Self::default()
    }
}
