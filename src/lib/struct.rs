// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Used-defined **structure** types.
//!
//! ## Examples ##
//!
//! The example below defines a new Retronym struct
//! named "point" containing two fields, each a byte wide.
//!
//! ```
//! %point  byte, byte
//! ```
//!

use crate::field::Field;

// This represents a user-defined structure in Retronym, consisting of a
// list of types (`Primitive`s / other `Struct`s). Not to be confused with
// Rust structs.
//
#[derive(Default)]
pub struct Struct<'token> {
    /// The list of Fields in the Struct.
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

    /// Add a `Field` to the `Struct`. [Chainable]
    ///
    pub fn add_field(mut self, field: Field<'token>) -> Self {
        //----------------------------------------------------------------------
        // how many bytes does this add to the stride?
        self.stride += match field.bits() {
            // TODO: bit-packing!
            1 | 4 => 1,
            n => n,
        };
        self.cols += field.cols();
        self.fields.push(field);
        self
    }

    /// Get a Field from the Struct.
    ///
    pub fn field(&'token self, index: usize) -> &'token Field<'token> {
        //----------------------------------------------------------------------
        &self.fields[index]
    }
}

use crate::list::List;

impl<'token> From<&'token List<'token>> for Struct<'token> {
    //==========================================================================
    /// "Resolve" a List of Types into a Record Struct.
    ///
    fn from(list: &'token List<'token>) -> Self {
        //----------------------------------------------------------------------
        // take the List of Nodes,
        list.into_iter()
            // convert each into a Field,
            .map(Field::from)
            // take a Struct and add each Field to it
            .fold(Self::default(), |acc, node| acc.add_field(node))
    }
}
