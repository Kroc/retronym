// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Used-defined **structure** types.

#[derive(Debug)]
pub struct Struct {
    _fields: Vec<Field>,
}

use crate::ptype::PType;

#[derive(Debug)]
pub enum Field {
    /// A native primitive type (on the target system),
    /// e.g. `byte`, `word`, `long` &c.
    Type(PType),
    /// Inception.
    Struct(Box<Struct>),
}

impl Default for Struct {
    fn default() -> Self {
        Self {
            _fields: Vec::new(),
        }
    }
}

use std::fmt::{self, *};

impl Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{}>",
            self._fields
                .iter()
                .fold(String::new(), |acc, field| format!(
                    "{}{}, \n",
                    acc, field
                ))
        )
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Type(p) => write!(f, "{}", p),
            Field::Struct(s) => write!(f, "{}", *s),
        }
    }
}
