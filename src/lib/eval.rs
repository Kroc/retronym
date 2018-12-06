// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::node::{Node, NodeKind, Value};

/// The `Eval` trait is used when we want to 'run' the source code --
/// stored in `AST` form in memory -- and want to return concrete values
/// from program statements.
pub trait Eval {
    fn eval(&self) -> Evaluation;
}

pub enum Evaluation {
    // Result of the eval is an integer.
    Int(i64),
    // Result of the eval is a float.
    Float(f64),
    //TODO: result of a dyanmic expression should be a relaxtion joint
    //      or some such deferred calculation
}

use std::fmt::{self, *};

impl Display for Evaluation {
    /// Pretty-print an eval result.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Evaluation::Int(i) => write!(f, "{}", i),
            Evaluation::Float(d) => write!(f, "{}", d),
        }
    }
}

impl Eval for Node<'_> {
    /// Evaluating a node attempts to calculate the 'value' of a node; literal
    /// values (ints, floats) are returned as-is. expressions are calculated
    /// and return a literal value if all elements of the expression are
    /// literals too (e.g. `5 + 5` = `10`).
    fn eval(&self) -> Evaluation {
        match &self.kind {
            // return literal values as-is
            NodeKind::Value(v) => match v {
                Value::Int(i) => Evaluation::Int(*i),
                Value::Float(d) => Evaluation::Float(*d),
            },
            // for expressions, defer to the expression's implementation
            NodeKind::Expr(x) => x.eval(),
            // TODO: dynamic elements
            _ => unimplemented!(),
        }
    }
}

use crate::expr::Expr;
use crate::ops::Operator;

impl Eval for Expr<'_> {
    fn eval(&self) -> Evaluation {
        // we need to check if the expression is static or dynamic:
        //
        // - static expressions require no outside information
        //   and can be flattened into a single value to output
        //
        // - dynamic expressions cannot be calculated without outside
        //   information such as a function call, imported symbol etc.
        //   since we cannot produce a value with these yet, store them
        //   with a reference to their AST node for later calculation
        //
        match &self.oper {
            Operator::Add => self.left.eval() + self.right.eval(),
            Operator::Sub => self.left.eval() - self.right.eval(),
            Operator::Mul => self.left.eval() * self.right.eval(),
            Operator::Div => self.left.eval() / self.right.eval(),
            _ => unimplemented!(),
        }
    }
}
