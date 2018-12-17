// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::field::Field;
use crate::node::Node;

/// A **Table Cell**.
#[derive(Clone)]
pub struct Cell<'token> {
    /// Reference to the Record field that this cell aligns with (column).
    /// This is so that the Cell knows what its intended data-width is.
    field: &'token Field<'token>,
    /// AST Node containing the data value for this Cell.
    node: &'token Node<'token>,
    /// The Row number of this Cell,
    /// i.e. its Row index in a Table.
    row: usize, 
    /// The column number of this Cell,
    /// i.e. its index in the Row.
    col: usize,
}

use std::fmt::{self, *};

impl Display for Cell<'_> {
    //==========================================================================
    /// Get a normalized representation of the source code.
    /// 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        f.write_str(&self.node.to_string())
    }
}

impl Debug for Cell<'_> {
    //==========================================================================
    /// For debugging, we want to include the Cell Row & Col co-ords,
    /// and the primitive type of the Cell.
    /// 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "[{}][{}] {:?}: {:?}", self.row, self.col, self.field, self.node)
    }
}

impl<'token> Cell<'token> {
    //==========================================================================
    /// Create a new Table Cell.
    ///
    pub fn new(
        node: &'token Node<'token>,
        field: &'token Field<'token>,
        row: usize,
        col: usize,
    ) -> Self {
        //----------------------------------------------------------------------
        Self { field, node, row, col }
    }

    /// Return the row-index of this Cell.
    /// 
    pub fn row(&self) -> usize {
        //----------------------------------------------------------------------
        self.row
    }

    /// Return the column-index of this Cell.
    /// 
    pub fn col(&self) -> usize {
        //----------------------------------------------------------------------
        self.col
    }
}
