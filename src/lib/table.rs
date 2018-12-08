// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! A data table, consisting of records and fields, populated with data to fit.

use crate::record::Record;

pub struct Table {
    _records: Vec<Record>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            _records: Vec::new(),
        }
    }
}

pub type Tables = Vec<Table>;
