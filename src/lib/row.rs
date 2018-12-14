// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! One line of a **Table**.

use crate::node::Node;

#[derive(Default, Clone)]
pub struct Row<'token> {
    data: Vec<&'token Node<'token>>,
}

impl<'token> Row<'token> {
    //==========================================================================
    pub fn add_data(&mut self, node: &'token Node<'token>) {
        //----------------------------------------------------------------------
        self.data.push(node);
    }

    pub fn clear(&mut self) {
        //----------------------------------------------------------------------
        self.data.clear();
    }
}

use itertools::Itertools;
use std::fmt::{self, *};

impl<'token> Display for Row<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        // IterTools' `join` makes this sane
        write!(f, "({})", &self.data.iter().join(", "))
    }
}
