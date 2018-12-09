// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! One line of a `Table`.

use crate::node::Node;

pub struct Record<'token> {
    _data: Vec<&'token Node<'token>>,
}

impl Default for Record<'_> {
    fn default() -> Self {
        Self {
            _data: Vec::new(),
        }
    }
}

impl<'token> Record<'token> {
    pub fn add_data(&self, _node: &'token Node) {

    }
}