// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! One line of a **Table**.

use crate::error::*;
use crate::field::Field;
use crate::r#struct::Struct;

/// RowBuilder packs data into a Table Row. Given a Struct, it will accept
/// AST Nodes and place them into Table Cells until the Row is full
/// ("satisfied").
///
pub struct RowBuilder<'token> {
    /// The Cells for the Row being built are held here until you've finished
    /// with the RowBuilder; we cannot return a Row instance unless we create
    /// it the final method call.
    cells: Vec<Cell<'token>>,
    /// An Iterator over a record Struct to map the cells in the row to.
    fields: std::iter::Peekable<std::slice::Iter<'token, Field<'token>>>,
    /// If the Row is satisfied (full) or not. Once the Row is filled,
    /// this will be flipped on and the RowBuilder will accept no more data.
    is_satisfied: bool,
    /// The index of this Row in a Table.
    /// Used to give each Cell a complete reference of its location.
    row: usize,
    /// Current column number in the Row being built.
    /// This is used to provide each Cell with its index number.
    col: usize,
}

use crate::node::Node;

impl<'token> RowBuilder<'token> {
    //==========================================================================
    /// Table Rows are tightly bound to the Record-struct that defines the type
    /// of each column. You cannot create a Row without a Record-struct as this
    /// would allow swapping the Record-struct whilst the Row is using it.
    ///  
    pub fn new(record: &'token Struct<'token>, row: usize) -> Self {
        //----------------------------------------------------------------------
        Self {
            // take an Iterator over the Fields in the Struct;
            // we match each Cell in the Row with its column Field
            fields: record.into_iter().peekable(),
            // default this, regardless of type
            cells: Default::default(),
            // row begins empty
            is_satisfied: false,
            // use the Row index given
            row,
            // columns begin on zero
            col: 0,
        }
    }

    pub fn is_satisfied(&self) -> bool {
        //----------------------------------------------------------------------
        self.is_satisfied
    }

    /// Push data into the `Row`, assigning a new table `Cell` for it.
    ///
    /// Returns a Result:
    /// - `None` if the Row has remaining space
    /// - The finished Row once it is satisfied (full)
    /// - An Error if the Row is full and no more data can be added
    ///   (you can't call `add_data` again once the RowBuilder has yielded)
    ///
    pub fn add_data(
        &mut self,
        node: &'token Node<'token>,
    ) -> ParseResult<Option<Row<'token>>> {
        //----------------------------------------------------------------------
        // get the next record field
        match self.fields.next() {
            // error if the row is already full!
            None => Err(parse_error(ParseErrorKind::RowSatisfied)),
            Some(field) => {
                // the field gives the data-type and the node gives the data
                let cell = Cell::new(node, field, self.row, self.col);
                self.col += 1;

                self.cells.push(cell);
                // are there any fields remaining?
                if self.fields.peek().is_none() {
                    // no: mark the Row as satisifed
                    self.is_satisfied = true;
                    // return the Row we've built
                    Ok(Some(Row(self.cells.clone())))
                } else {
                    // yes: fields remain, return None
                    Ok(None)
                }
            }
        }
    }
}

use crate::cell::Cell;

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

impl<'token> Debug for Row<'token> {
    //==========================================================================
    /// Debug printing a Row places each Cell on its own line.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        f.write_str(&self.0.iter().map(|row| format!("{:?}", row)).join("\n"))
    }
}
