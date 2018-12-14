// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Used-defined **structure** types.

use crate::field::Field;

#[derive(Debug, Default)]
pub struct Struct<'token> {
    fields: Vec<Field<'token>>,
    /// Width, in bytes, of the structure.
    pub stride: usize,
}

use itertools::Itertools;
use std::fmt::{self, *};

impl Display for Struct<'_> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        // IterTools' `join` makes this sane
        write!(f, "{}", self.fields.iter().join(", "),)
    }
}

impl<'token> Struct<'token> {
    //==========================================================================
    /// Add a `Field` to the `Struct`.
    ///
    pub fn add_field(&mut self, field: Field<'token>) {
        //----------------------------------------------------------------------
        // how many bytes does this add to the stride?
        self.stride += match field.bits {
            // TODO: bit-packing!
            1 | 4 => 1,
            n => n,
        };
        self.fields.push(field);
    }
}
