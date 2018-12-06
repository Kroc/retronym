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

use crate::error::*;
use crate::node::{ASTResult, Node};

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
        if self.tokens.is_atom() {
            return self.parse_atom();
        }
        if self.tokens.is_macro() {
            return self.parse_macro();
        }
        if self.tokens.is_expr() {
            return self.parse_expr();
        }
        Ok(None)
    }

    /// Parse an atom invocation.
    fn parse_atom(&mut self) -> ASTResult<'token> {
        // if the current token is not an atom,
        // this is not our concern.
        if !self.tokens.is_atom() {
            return Ok(None);
        }

        // build a `Node` for an atom invocation
        ASTResult::from(Node::from(
            // retrieve the current token, containing the atom name
            // (and move the tokenstream to the next automatically)
            self.tokens.consume().unwrap()
        ))
    }

    /// Parse a macro invocation.
    fn parse_macro(&mut self) -> ASTResult<'token> {
        // if the current token is not a macro,
        // this is not our concern.
        if !self.tokens.is_macro() {
            return Ok(None);
        }

        // build a `Node` for a macro invocation
        ASTResult::from(Node::from(
            // retrieve the current token, containing the macro name
            // (and move the tokenstream to the next automatically)
            self.tokens.consume().unwrap()
        ))
    }

    /// Parse an expression, returning an AST node
    /// representing that expression.
    ///
    /// If the current token is not the beginning of an expression returns
    /// `None`; the caller can decide if this is unexpected or not; otherwise
    /// returns an `ASTResult` of either a `Node` of the expression, or the
    /// error encountered.
    fn parse_expr(&mut self) -> ASTResult<'token> {
        // if the current token is not a valid opening for an expression
        // (including if we've reached end-of-file), then return a
        // "unrecognised" state, the caller decides if this is unexpected.
        if !self.tokens.is_expr() {
            return Ok(None);
        }

        // this is the beginning of an expression and we need to read the
        // first value that will form the inner-most (but also left-most)
        // value, e.g. the "1" in `(((1 + 2) + 3) + 4)`
        let left = Node::from(self.tokens.consume().unwrap());

        // is there any token following,
        // is it an operator?
        if !self.tokens.is_oper() {
            // no: this is a single value rather than an expression,
            // we can skip building an expression node and return
            // a value node instead
            return ASTResult::from(left);
        }

        // parse the operator and right-hand-side, passing in
        // the left-hand value we already have
        self.parse_expr_inner(left)
    }

    fn parse_expr_inner(&mut self, left: Node<'token>) -> ASTResult<'token> {
        // save the operator, move to the next token
        let oper = self.tokens.consume().unwrap();

        // is there a token at all, and is it also a valid expression value?
        if !self.tokens.is_expr() {
            // no: we have an operator, but no value following it
            // e.g. "(1 + )"; return an "unexpected token" error
            return ASTResult::from(ParseError::unexpected());
        }

        // get the right hand value
        let right = self.tokens.consume().unwrap();

        //build our expression node:
        let expr = Node::new_expr(
            // left hand side:
            left,
            // op token:
            oper,
            // right hand side:
            Node::from(right),
        );

        // we have managed to parse, for example, the "(1 + 2)" in
        // "((1 + 2) + 3)" but now we need to check if the expression
        // continues further
        if self.tokens.is_oper() {
            // the expression we have just assembled will now
            // form the left-hand-side for the outer expression
            self.parse_expr_inner(expr)
        } else {
            ASTResult::from(expr)
        }
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
                Some(node) => Some(ASTResult::from(node)),
                None => None,
            },
        }
    }
}
