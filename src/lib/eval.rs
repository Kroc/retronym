// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::node::{Node,ASTKind,ASTValue};

pub enum ASTFoldResult {
    // Result of the fold is an integer.
    Int(i64),
    // Result of the fold is a float.
    Float(f64),
    //TODO: result of a dyanmic expression should be a relaxtion joint
    //      or some such deferred calculation
}

impl Node<'_> {
    fn _fold(&self) -> ASTFoldResult {
        match &self.kind {
            ASTKind::Value(v) => match v {
                ASTValue::Int(i) => ASTFoldResult::Int(*i),
                ASTValue::Float(d) => ASTFoldResult::Float(*d),
            },
            _ => unimplemented!(),
        }
    }
}

use crate::expr::Expr;

impl Expr<'_> {
    fn _fold(&self) -> ASTFoldResult {
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

        unimplemented!()
    }
}