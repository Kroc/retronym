// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Used-defined **structure** types.

use crate::field::Field;

#[derive(Debug)]
pub struct Struct {
    fields: Vec<Field>,
}

impl Default for Struct {
    //==========================================================================
    fn default() -> Self {
        //----------------------------------------------------------------------
        Self {
            fields: Vec::new(),
        }
    }
}

use std::fmt::{self, *};

impl Display for Struct {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(
            f,
            "<{}>",
            self.fields
                .iter()
                .fold(String::new(), |acc, field| format!(
                    "{}{}, \n",
                    acc, field
                ))
        )
    }
}

impl Struct {
    //==========================================================================
    /// Add a `Field` to the `Struct`.
    /// 
    pub fn add_field(&mut self, field: Field) {
        //----------------------------------------------------------------------
        self.fields.push(field);
    }
}