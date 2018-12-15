// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! A **Table** of data, consisting of a record of fields,
//! populated with data to fit.

use crate::node::Node;
use crate::r#struct::Struct;
use crate::row::{RowBuilder,Row};

pub struct Table<'token> {
    /// Reference to the Record-struct used to define the columns.
    record: &'token Struct<'token>,
    /// The collection of Rows containing the data.
    rows: Vec<Row<'token>>,
    /// Current row being packed.
    builder: RowBuilder<'token>,
}

use std::convert::From;

impl<'token> From<&'token Struct<'token>> for Table<'token> {
    //==========================================================================
    /// Tables are tightly bound to the Record-struct that defines the type of
    /// each column. You cannot create a Table without a Record-struct as this
    /// would allow swapping the Record-struct whilst the Table is using it.
    /// 
    fn from(record: &'token Struct<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            record: record,
            rows: Vec::new(),
            builder: RowBuilder::new(record),
        }
    }
}

impl<'token> Table<'token> {
    //==========================================================================
    /// Add data to the Table by assigning an AST Node to the next cell in the
    /// current row. When the record is satsisfied, another row will be started.
    ///
    /// TODO: return satisfied state, exports?
    /// TODO: handle being passed a list (flatten it out?)
    /// TODO: errors for non-data types of nodes
    /// 
    pub fn add_data(&mut self, node: &'token Node<'token>) {
        //----------------------------------------------------------------------
        self.builder.add_data(node);
    }

    pub fn finish(&mut self) {
        //----------------------------------------------------------------------
        let row = self.builder.finish();
        self.rows.push(row);
        self.builder = RowBuilder::new(self.record);
        //self
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
