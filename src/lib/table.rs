// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! A data table, consisting of records and fields, populated with data to fit.

use crate::record::Record;

pub struct Table<'token> {
    _records: Vec<Record<'token>>,
}

impl Default for Table<'_> {
    //==========================================================================
    fn default() -> Self {
        //----------------------------------------------------------------------
        Self {
            _records: Vec::new(),
        }
    }
}

pub type Tables<'token> = Vec<Table<'token>>;
