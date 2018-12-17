// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Used-defined **structure** types.

use crate::field::Field;

/// A Struct is a user-defined structure in Retronym, consisting of a
/// list of types (`Primitive`s / other `Struct`s).
/// 
/// ## Examples ##
/// 
/// The example below defines a new struct named "point" containing two
/// fields, each a byte wide.
/// 
/// ```
/// %point  byte, byte
/// ```
/// 
#[derive(Default)]
pub struct Struct<'token> {
    fields: Vec<Field<'token>>,
    /// Width, in bytes, of the structure. Not public as this value is
    /// calculated according to the bit-packing rules.
    stride: usize,
    /// Number of columns in the Struct. Note that this has to account for
    /// nested Structs, so is not necessarily the same as `self.fields.len()`.
    cols: usize,
}

use itertools::Itertools;
use std::fmt::{self, *};

impl Display for Struct<'_> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        // IterTools' `join` makes this sane
        f.write_str(&self.fields.iter().join(", "))
    }
}

impl Debug for Struct<'_> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        // IterTools' `join` makes this sane
        f.write_str(&self.fields.iter().join(", "))
    }
}

use std::slice;

impl<'token> IntoIterator for &'token Struct<'token> {
    //==========================================================================
    type Item = &'token Field<'token>;
    type IntoIter = slice::Iter<'token, Field<'token>>;

    /// We only ever return references from iterating a Struct.
    /// 
    fn into_iter(self) -> slice::Iter<'token, Field<'token>> {
        //----------------------------------------------------------------------
        self.fields.iter()
    }
}

impl<'token> Struct<'token> {
    //==========================================================================
    /// The width of the struct, in bytes.
    ///
    pub fn stride(&self) -> usize {
        //----------------------------------------------------------------------
        self.stride
    }

    /// Number of columns in the Struct
    /// (inlcuding nested Structs).
    ///
    pub fn cols(&self) -> usize {
        //----------------------------------------------------------------------
        self.cols
    }

    /// Add a `Field` to the `Struct`.
    ///
    pub fn add_field(&mut self, field: Field<'token>) {
        //----------------------------------------------------------------------
        // how many bytes does this add to the stride?
        self.stride += match field.bits() {
            // TODO: bit-packing!
            1 | 4 => 1,
            n => n,
        };
        self.cols += field.cols();
        self.fields.push(field);
    }

    /// Get a Field from the Struct.
    ///
    pub fn field(&'token self, index: usize) -> &'token Field<'token> {
        //----------------------------------------------------------------------
        &self.fields[index]
    }
}
