// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Lists are one of the fundamental types of Retronym. Source code consists
//! almost entirely of implicit lists and their elements.
//!
//! A simplistic way to describe how Retronym works is to say that it takes
//! lists of numbers and pack them into data tables. A source code file is
//! an implict data table and explicit list of values which to pack.

use crate::node::Node;

struct List<'token> {
    nodes: Vec<Node<'token>>,
}

impl Default for List<'_> {
    //--------------------------------------------------------------------------
    /// Get a default list (no nodes).
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
}

use std::slice;

impl<'token> IntoIterator for &'token List<'token> {
    //--------------------------------------------------------------------------
    type Item = &'token Node<'token>;
    type IntoIter = slice::Iter<'token, Node<'token>>;

    /// Get an iterator over the nodes in the list.
    fn into_iter(self) -> slice::Iter<'token, Node<'token>> {
        self.nodes.iter()
    }
}

impl List<'_> {
    /// Begin a new List.
    fn new() -> Self {
        Self::default()
    }
}
