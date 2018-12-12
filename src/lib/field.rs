// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::primitive::Primitive;
use crate::r#struct::Struct;

#[derive(Debug)]
pub enum Field {
    /// A native primitive type (on the target system),
    /// e.g. `byte`, `word`, `long` &c.
    Primitive(Primitive),
    /// Inception.
    Struct(Box<Struct>),
}

use std::fmt::{self, *};

impl Display for Field {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match self {
            Field::Primitive(p) => write!(f, "{}", p),
            Field::Struct(s) => write!(f, "{}", *s),
        }
    }
}

impl Field {
    //==========================================================================
    pub fn new_primitive(primitive: Primitive) -> Self {
        Field::Primitive(primitive)
    }
}

use std::convert::From;

impl From<Primitive> for Field {
    //==========================================================================
    fn from(primitive: Primitive) -> Self {
        //----------------------------------------------------------------------
        Field::Primitive(primitive)
    }
}
