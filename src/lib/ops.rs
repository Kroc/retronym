// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! **Operators**.

#[derive(Debug)]
pub enum Operator {
    /// Addition operator "+"
    Add,
    /// Subtraction operator "-"
    Sub,
    /// Multiplication operator "*"
    Mul,
    /// Division operator "/"
    Div,
    /// Modulo operator "\\"
    Mod,
    /// Power/Exponention Operator "**"
    Pow,
    /// Bitwise eXclusive OR operator "^"
    Xor,
    /// Bitwise AND operator "&"
    And,
    /// Bitwise OR operator "|"
    Bor,
    /// Bitwise SHift-Left operator "<<"
    Shl,
    /// Bitwise SHift-Right operator ">>"
    Shr,
}

use std::fmt::{self, *};

impl Display for Operator {
    //==========================================================================
    /// Print the operators. These could be referenced from the source code
    /// so that we don't duplicate these strings with the Pest grammar, but
    /// I don't want to entangle the Pest lifetimes too broadly or deeply.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Div => "/",
                Operator::Mod => r"\\",
                Operator::Pow => "**",
                Operator::Xor => "^",
                Operator::And => "&",
                Operator::Bor => "|",
                Operator::Shl => "<<",
                Operator::Shr => ">>",
            }
        )
    }
}

use crate::token::{Token, TokenKind};

impl From<&Token<'_>> for Operator {
    //==========================================================================
    /// Convert a token into an `Operator` enum.
    /// Panics if using a token that is not an operator!
    fn from(token: &Token<'_>) -> Self {
        //----------------------------------------------------------------------
        match token.kind() {
            TokenKind::Operator(o) => o,
            _ => panic!("Not an operator token!"),
        }
    }
}
