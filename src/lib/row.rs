// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! One line of a **Table**.

use crate::cell::Cell;
use crate::node::Node;
use crate::r#struct::Struct;

/// One row of a data Table.
#[derive(Clone)]
pub struct Row<'token> {
    /// The record structure to map the rows to.
    record: &'token Struct<'token>,
    /// Table cells in the Row.
    cells: Vec<Cell<'token>>,
    // Current column index in the Row; once the row is satisified,
    // no more data can be added.
    col: usize,
}

use std::convert::From;

impl<'token> From<&'token Struct<'token>> for Row<'token> {
    //==========================================================================
    /// Table Rows are tightly bound to the Record-struct that defines the type
    /// of each column. You cannot create a Row without a Record-struct as this
    /// would allow swapping the Record-struct whilst the Row is using it.
    ///  
    fn from(record: &'token Struct<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            record: record,
            cells: Default::default(),
            col: Default::default(),
        }
    }
}

impl<'token> Row<'token> {
    //==========================================================================
    /// Push data into the row, assigning a new table cell for it.
    ///
    pub fn add_data(&mut self, node: &'token Node<'token>) {
        //----------------------------------------------------------------------
        self.cells.push(Cell::new(node, self.record.field(0)));

        self.col += 1;
    }

    pub fn clear(&mut self) {
        //----------------------------------------------------------------------
        self.cells.clear();
    }
}

use itertools::Itertools;
use std::fmt::{self, *};

impl<'token> Display for Row<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        // IterTools' `join` makes this sane
        f.write_str(&self.cells.iter().join(", "))
    }
}
