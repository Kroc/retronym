// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::node::Node;
use crate::ops::Operator;
use crate::token::Token;

#[derive(Debug)]
pub struct Expr<'token> {
    pub left: Node<'token>,
    pub oper: Operator,
    pub right: Node<'token>,
}

impl<'token> Expr<'token> {
    pub fn new(
        left: Node<'token>,
        oper: &Token<'token>,
        right: Node<'token>,
    ) -> Self {
        Self {
            // left hand side:
            left: left,
            // convert op token to op enum:
            oper: Operator::from(oper),
            // right hand side:
            right: right,
        }
    }
}

use std::fmt::{self, *};

impl<'token> Display for Expr<'token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Operator to string
        write!(f, "({} {} {})", self.left, self.oper, self.right)
    }
}
