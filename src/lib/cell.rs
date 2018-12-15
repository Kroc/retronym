// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::field::Field;
use crate::node::Node;

/// A **Table Cell**.
#[derive(Debug, Clone)]
pub struct Cell<'token> {
    /// Reference to the Record field that this cell aligns with (column).
    /// This is so that the Cell knows what its intended data-width is.
    field: &'token Field<'token>,
    /// AST Node containing the data value for this Cell.
    node: &'token Node<'token>,
}

use std::fmt::{self, *};

impl Display for Cell<'_> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "{}: {}", self.field, self.node)
    }
}

impl<'token> Cell<'token> {
    //==========================================================================
    /// Create a new Table Cell.
    ///
    pub fn new(
        node: &'token Node<'token>,
        field: &'token Field<'token>,
    ) -> Self {
        //----------------------------------------------------------------------
        Self { field, node }
    }
}
