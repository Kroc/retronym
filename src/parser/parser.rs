// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("retronym.pest");

// build a parser using Pest:

use crate::parser::tokenstream::TokenStream;

// this will do all the macro work of turning our grammar file into a `parse`
// method on the below structure.
#[derive(Parser)]
#[grammar = "parser/retronym.pest"]
pub struct RymParser<'t> {
    #[allow(dead_code)]
    tokens: TokenStream<'t>,
}

use crate::parser::ast::{ASTExpr, ASTKind, ASTOperator};
use crate::parser::ast::{ASTNode, ASTResult, MaybeASTResult};

impl<'token> RymParser<'token> {
    // note that we cannot implement `FromStr` due to the lifetime requirement.
    #[allow(clippy::should_implement_trait)]
    /// NB: the string reference must live as long as the `RymParser`;
    /// that is, the source string you pass it will not deallocate until
    /// the RymParser does as well.
    pub fn from_str(source: &'token str) -> Self {
        let tokens = TokenStream::new_from_str(source);

        // build a parser struct
        Self { tokens }
    }

    fn parse_statement(&mut self) -> MaybeASTResult<'token> {
        self.parse_macro().or_else(|| self.parse_expr())
    }

    /// Parse a macro invocation.
    fn parse_macro(&mut self) -> MaybeASTResult<'token> {
        // if the current token is not a macro, this is not our concern.
        if !self.tokens.is_macro() {
            return None;
        }

        // build an ASTNode for a macro invocation.
        // TODO: messy
        let token = self.tokens.consume().unwrap();
        Some(Ok(ASTNode::from(token)))
    }

    /// Checks if the current token is the beginning of an expression, if not
    /// returns `None`; the caller can decide if this is unexpected or not;
    /// otherwise returns an `Option` containing an `ASTResult` of either
    /// an `ASTNode` built from the expression, or the error encountered.
    fn parse_expr(&mut self) -> MaybeASTResult<'token> {
        // if the current token is not valid for the beginning of an expression
        // (values only), then return `None` as our 'unrecognised' response
        if !self.tokens.is_expr() {
            return None;
        }

        // put aside the current token, we need to check for a following
        // operator that defines an expression
        let left = self.tokens.consume().unwrap();

        // is there an operator?
        if !self.tokens.is_oper() {
            // no: this is a single value rather than an expression, we can
            // skip the Expr AST node. this brings an end to any recursion,
            // the top most call will receive a single AST node containing
            // the child nodes
            return Some(Ok(ASTNode::from(left)));
        }

        // save the operator, move to the next token
        let _oper = self.tokens.consume().unwrap();

        // the value that follows can itself be part of an expression;
        // e.g. `1 + 2 + 3` is equivilent to `1 + (2 + 3)`. if a terminal
        // follows (i.e. a value) then the return from recursing here will
        // be an AST node containing a single value and not another expression
        let right = self.parse_expr();

        match right {
            None => panic!("non-expression following operator!"),
            Some(ast_result) => match ast_result {
                Err(e) => Some(Err(e)),
                Ok(ast_node) => Some(Ok(ASTNode {
                    kind: ASTKind::Expr(Box::new(ASTExpr {
                        left: ASTNode::from(left),
                        op: ASTOperator::Add,
                        right: ast_node,
                    })),
                    token: None,
                })),
            },
        }
    }
}

impl<'token> Iterator for RymParser<'token> {
    type Item = ASTResult<'token>;

    /// When you turn the crank on the parser, it spits out AST nodes.
    fn next(&mut self) -> Option<ASTResult<'token>> {
        self.parse_statement()
    }
}
