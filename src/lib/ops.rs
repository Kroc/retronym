// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Operators.

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
    /// Print the operators. These could be referenced from the source code
    /// so that we don't duplicate these strings with the Pest grammar, but
    /// I don't want to entangle the Pest lifetimes too broadly or deeply.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

use crate::parser::Rule;
use crate::token::Token;

impl From<&Token<'_>> for Operator {
    /// Convert a token into an `Operator` enum.
    /// Panics if using a token that is not an operator!
    fn from(token: &Token<'_>) -> Self {
        match token.as_rule() {
            Rule::op_add => Operator::Add,
            Rule::op_sub => Operator::Sub,
            Rule::op_mul => Operator::Mul,
            Rule::op_div => Operator::Div,
            Rule::op_mod => Operator::Mod,
            Rule::op_pow => Operator::Pow,
            Rule::op_xor => Operator::Xor,
            Rule::op_and => Operator::And,
            Rule::op_bor => Operator::Bor,
            Rule::op_shl => Operator::Shl,
            Rule::op_shr => Operator::Shr,
            _ => panic!("Not an operator token!"),
        }
    }
}

use crate::eval::Evaluation;
use std::ops::Add;

impl Add for Evaluation {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Evaluation::Int(a), Evaluation::Int(b)) => Evaluation::Int(a + b),
            (Evaluation::Int(a), Evaluation::Float(b)) => {
                Evaluation::Float(a as f64 + b)
            }
            (Evaluation::Float(a), Evaluation::Int(b)) => {
                Evaluation::Float(a + b as f64)
            }
            (Evaluation::Float(a), Evaluation::Float(b)) => {
                Evaluation::Float(a + b)
            }
        }
    }
}

use std::ops::Sub;

impl Sub for Evaluation {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Evaluation::Int(a), Evaluation::Int(b)) => Evaluation::Int(a - b),
            (Evaluation::Int(a), Evaluation::Float(b)) => {
                Evaluation::Float(a as f64 - b)
            }
            (Evaluation::Float(a), Evaluation::Int(b)) => {
                Evaluation::Float(a - b as f64)
            }
            (Evaluation::Float(a), Evaluation::Float(b)) => {
                Evaluation::Float(a - b)
            }
        }
    }
}

use std::ops::Mul;

impl Mul for Evaluation {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Evaluation::Int(a), Evaluation::Int(b)) => Evaluation::Int(a * b),
            (Evaluation::Int(a), Evaluation::Float(b)) => {
                Evaluation::Float(a as f64 * b)
            }
            (Evaluation::Float(a), Evaluation::Int(b)) => {
                Evaluation::Float(a * b as f64)
            }
            (Evaluation::Float(a), Evaluation::Float(b)) => {
                Evaluation::Float(a * b)
            }
        }
    }
}

use std::ops::Div;

impl Div for Evaluation {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Evaluation::Int(a), Evaluation::Int(b)) => {
                Evaluation::Float(a as f64 / b as f64)
            }
            (Evaluation::Int(a), Evaluation::Float(b)) => {
                Evaluation::Float(a as f64 / b)
            }
            (Evaluation::Float(a), Evaluation::Int(b)) => {
                Evaluation::Float(a / b as f64)
            }
            (Evaluation::Float(a), Evaluation::Float(b)) => {
                Evaluation::Float(a / b)
            }
        }
    }
}

use std::ops::Rem;

impl Rem for Evaluation {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Evaluation::Int(a), Evaluation::Int(b)) => Evaluation::Int(a % b),
            (Evaluation::Int(a), Evaluation::Float(b)) => {
                Evaluation::Float(a as f64 % b)
            }
            (Evaluation::Float(a), Evaluation::Int(b)) => {
                Evaluation::Float(a % b as f64)
            }
            (Evaluation::Float(a), Evaluation::Float(b)) => {
                Evaluation::Float(a % b)
            }
        }
    }
}
