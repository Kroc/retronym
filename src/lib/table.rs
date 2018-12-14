// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! A **Table** of data, consisting of a record of fields,
//! populated with data to fit.

use crate::node::Node;
use crate::r#struct::Struct;
use crate::row::Row;

#[derive(Default)]
pub struct Table<'token> {
    record: Struct<'token>,
    rows: Vec<Row<'token>>,
    /// Current row being packed.
    row: Row<'token>,
}

use std::convert::From;

impl<'token> From<Struct<'token>> for Table<'token> {
    //==========================================================================
    fn from(record: Struct<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            record: record,
            // start with a 256-element buffer
            rows: Vec::with_capacity(256),
            // use the default structure to populate the rest
            ..Default::default()
        }
    }
}

impl<'token> Table<'token> {
    //==========================================================================
    pub fn add_data(&mut self, node: &'token Node<'token>) {
        self.row.add_data(node);
    }

    pub fn end(&mut self) {
        self.rows.push(self.row.clone());
        self.row.clear();
    }
}

use std::fmt::{self, *};

impl<'token> Display for Table<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(
            f,
            "{{\t{}\n{}}}",
            self.record,
            self.rows
                .iter()
                .fold(String::new(), |acc, row| format!("{}\t{}\n", acc, row))
        )
    }
}
