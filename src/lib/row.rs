// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! One line of a **Table**.

use crate::field::Field;
use crate::r#struct::Struct;
use std::slice;

/// RowBuilder packs data into a Table Row. Given a Struct, it will accept
/// AST Nodes and place them into Table Cells until the Row is full
/// ("satisfied"). Calling the `finish` method will return the Row.
///
pub(crate) struct RowBuilder<'token> {
    /// The Cells for the Row being built are held here until you've finished
    /// with the RowBuilder; we cannot return a Row instance unless we create
    /// it the final method call.
    cells: Vec<Cell<'token>>,
    /// An Iterator over a record Struct to map the cells in the row to.
    fields: slice::Iter<'token, Field<'token>>,
}

impl<'token> RowBuilder<'token> {
    //==========================================================================
    /// Table Rows are tightly bound to the Record-struct that defines the type
    /// of each column. You cannot create a Row without a Record-struct as this
    /// would allow swapping the Record-struct whilst the Row is using it.
    ///  
    pub fn new(record: &'token Struct<'token>) -> Self {
        Self {
            fields: record.into_iter(),
            // default this, regardless of type
            cells: Default::default(),
        }
    }

    /// Push data into the `Row`,
    /// assigning a new table `Cell` for it.
    ///
    pub fn add_data(&mut self, node: &'token Node<'token>) {
        //----------------------------------------------------------------------
        // get the next record field
        match self.fields.next() {
            // TODO: return error when trying to add to a full row
            None => panic!(),
            Some(field) => {
                // the field gives the data-type and the node gives the data
                let cell = Cell::new(node, field);
                self.cells.push(cell);
            }
        }
    }

    pub fn finish(&mut self) -> Row<'token> {
        //----------------------------------------------------------------------
        Row(self.cells.clone())
    }
}

use crate::cell::Cell;
use crate::node::Node;

/// One row of a data Table.
pub struct Row<'token>(Vec<Cell<'token>>);

use itertools::Itertools;
use std::fmt::{self, *};

impl<'token> Display for Row<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        // IterTools' `join` makes this sane
        f.write_str(&self.0.iter().join(", "))
    }
}
