// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("retronym.pest");

// build a parser using Pest:

use crate::tokenstream::TokenStream;

// this will do all the macro work of turning our grammar file into a `parse`
// method on the below structure.
#[derive(Parser)]
#[grammar = "lib/retronym.pest"]
pub struct RymParser<'token> {
    #[allow(dead_code)]
    tokens: TokenStream<'token>,
}

use crate::astnode::{ASTNode, ASTResult};
use crate::error::*;

impl<'token> RymParser<'token> {
    // note that we cannot implement `FromStr` due to the lifetime requirement?
    #[allow(clippy::should_implement_trait)]
    /// NB: the string reference must live as long as the `RymParser`;
    /// that is, the source string you pass it will not deallocate until
    /// the RymParser does as well.
    pub fn from_str(source: &'token str) -> Self {
        let tokens = TokenStream::new_from_str(source);

        // build a parser struct
        Self { tokens }
    }

    fn parse_statement(&mut self) -> ASTResult<'token> {
        if self.tokens.is_macro() {
            return self.parse_macro();
        }
        if self.tokens.is_expr() {
            return self.parse_expr();
        }
        Ok(None)
    }

    /// Parse a macro invocation.
    fn parse_macro(&mut self) -> ASTResult<'token> {
        // well, if there are no tokens, this can't be a macro
        if self.tokens.is_eof() {
            return Ok(None);
        }

        // if the current token is not a macro,
        // this is not our concern.
        if !self.tokens.is_macro() {
            return Ok(None);
        }

        // build an ASTNode for a macro invocation.
        // TODO: messy
        let token = self.tokens.consume().unwrap();
        ASTResult::from(ASTNode::from(token))
    }

    /// Parse an expression, returning an AST node representing that expression.
    ///
    /// If the current token is not the beginning of an expression returns
    /// `None`; the caller can decide if this is unexpected or not; otherwise
    /// returns an `ASTResult` of either an `ASTNode` built from the expression,
    /// or the error encountered.
    fn parse_expr(&mut self) -> ASTResult<'token> {
        // well, if there are no tokens, this can't be an expression
        if self.tokens.is_eof() {
            return Ok(None);
        }

        // if the current token is not valid for the beginning of an
        // expression (values only), then return 'unrecognised'
        if !self.tokens.is_expr() {
            return Ok(None);
        }

        // put aside the current token; we need to check for a following
        // operator that defines an expression. this token is guaranteed
        // to exist (for `unwrap`) because of the `is_eof` check earlier
        let left = self.tokens.consume().unwrap();

        // is there any token following, is it an operator?
        if self.tokens.is_eof() | !self.tokens.is_oper() {
            // no: this is a single value rather than an expression, we can
            // skip the Expr AST node. this brings an end to any recursion;
            // the top most call will receive a single AST node containing
            // descending child nodes
            return ASTResult::from(ASTNode::from(left));
        }

        // save the operator, move to the next token
        let oper = self.tokens.consume().unwrap();

        // the value that follows can itself be part of an expression;
        // e.g. `1 + 2 + 3` is equivilent to `1 + (2 + 3)`. if a terminal
        // follows (i.e. a value) then the return from recursing here will
        // be an AST node containing a single value and not another expression
        //
        // the recursion here returns an `ASTResult` -- a `Result` containing
        // an `Option`; therefore `Err`, `None`, or an `ASTNode`.
        //
        // if retrieving the right-hand-side errored,
        // pass that error up (`and_then` returns `Err` early)
        self.parse_expr().and_then(|option| match option {
            // for a non-error result, check for `None` or `ASTNode`,
            // where `None` means that there was no match -- in this case,
            // we have an operator without a following value! promote this
            // to a hard error:
            None => ASTResult::from(ParseError::end_of_file()),
            // the AST node returned is the right-hand-side of the expression,
            // we still need to combine it with the left-hand-side value and
            // the operator!
            Some(ast_node) => {
                // construct an expression node containing
                // the left & right nodes + the operator:
                ASTResult::from(ASTNode::new_expr(
                    // left hand side:
                    ASTNode::from(left),
                    // op token:
                    oper,
                    // right hand side:
                    ast_node,
                ))
            }
        })
    }
}

impl<'token> Iterator for RymParser<'token> {
    type Item = ASTResult<'token>;

    /// When you turn the crank on the parser, it spits out AST nodes.
    fn next(&mut self) -> Option<ASTResult<'token>> {
        match self.parse_statement() {
            // pass errors through
            Err(e) => Some(Err(e)),
            Ok(option) => match option {
                Some(ast_node) => Some(ASTResult::from(ast_node)),
                None => None,
            },
        }
    }
}
