// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::primitive::Primitive;
use crate::r#struct::Struct;
use crate::token::Token;

#[derive(Debug)]
pub struct Field<'token> {
    /// Provide a reference back to the source code where the field was
    /// defined, for error messages when a value does not fit into a field.
    token: Option<Token<'token>>,
    /// The data-type of the field, which can be a nested struct.
    pub kind: FieldKind<'token>,
    /// Width, **in bits**, of the field. Bit-fields are possible,
    /// but structs are aligned to the byte.
    pub bits: usize,
}

#[derive(Debug)]
pub enum FieldKind<'token> {
    /// A native primitive type (on the target system),
    /// e.g. `byte`, `word`, `long` &c.
    Primitive(Primitive),
    /// Inception.
    Struct(Box<Struct<'token>>),
}

use std::fmt::{self, *};

impl<'token> Display for Field<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match &self.kind {
            FieldKind::Primitive(p) => write!(f, "{}", p),
            FieldKind::Struct(s) => write!(f, "{}", *s),
        }
    }
}

use std::convert::From;

impl From<Primitive> for Field<'_> {
    //==========================================================================
    fn from(primitive: Primitive) -> Self {
        //----------------------------------------------------------------------
        Self {
            token: None,
            kind: FieldKind::Primitive(primitive),
            /// the Primitive enum equals the number of bits.
            bits: primitive as usize,
        }
    }
}
