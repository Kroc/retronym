// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::node::Node;
use crate::ops::Operator;
use crate::token::Token;

/// An Expression; a calculation consisting of a left-hand-side,
/// a right-hand-side and an operator to combine the two.
///
pub struct Expr<'token> {
    pub left: Node<'token>,
    pub oper: Operator,
    pub right: Node<'token>,
}

impl<'token> Expr<'token> {
    //==========================================================================
    pub fn new(
        left: Node<'token>,
        oper: &Token<'token>,
        right: Node<'token>,
    ) -> Self {
        //----------------------------------------------------------------------
        Self {
            // left hand side:
            left,
            // convert op token to op enum:
            oper: Operator::from(oper),
            // right hand side:
            right,
        }
    }
}

use std::fmt::{self, *};

impl<'token> Display for Expr<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "{} {} {}", self.left, self.oper, self.right)
    }
}

impl<'token> Debug for Expr<'token> {
    //==========================================================================
    /// Debug printing an Expression includes the wrapping parentheses.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(f, "({:?} {} {:?})", self.left, self.oper, self.right)
    }
}
