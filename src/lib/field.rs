// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::primitive::Primitive;
use crate::r#struct::Struct;
use crate::token::Token;

pub struct Field<'token> {
    /// Provide a reference back to the source code where the field was
    /// defined, for error messages when a value does not fit into a field.
    _token: Option<Token<'token>>,
    /// Width, **in bits**, of the field. Bit-fields are possible,
    /// but structs are aligned to the byte.
    bits: usize,
    /// The data-type of the field, which can be a nested struct.
    pub kind: FieldKind<'token>,
}

pub enum FieldKind<'token> {
    /// A native primitive type (on the target system),
    /// e.g. `byte`, `word`, `long` &c.
    Primitive(Primitive),
    /// Inception.
    Struct(Box<Struct<'token>>),
}

impl<'token> Display for FieldKind<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match self {
            FieldKind::Primitive(p) => f.write_str(&p.to_string()),
            FieldKind::Struct(s) => write!(f, "{}", *s),
        }
    }
}

impl<'token> Debug for FieldKind<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match self {
            FieldKind::Primitive(p) => f.write_str(&p.to_string()),
            FieldKind::Struct(s) => write!(f, "{:?}", *s),
        }
    }
}

use std::fmt::{self, *};

impl<'token> Display for Field<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        f.write_str(&self.kind.to_string())
    }
}

impl<'token> Debug for Field<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "{:?}", self.kind)
    }
}

use std::convert::From;

impl From<Primitive> for Field<'_> {
    //==========================================================================
    fn from(primitive: Primitive) -> Self {
        //----------------------------------------------------------------------
        Self {
            _token: None,
            kind: FieldKind::Primitive(primitive),
            /// the Primitive enum equals the number of bits.
            bits: primitive as usize,
        }
    }
}

impl Field<'_> {
    //==========================================================================
    pub fn cols(&self) -> usize {
        //----------------------------------------------------------------------
        match &self.kind {
            FieldKind::Primitive(_) => 1,
            FieldKind::Struct(s) => s.cols(),
        }
    }

    pub fn bits(&self) -> usize {
        //----------------------------------------------------------------------
        self.bits
    }
}
