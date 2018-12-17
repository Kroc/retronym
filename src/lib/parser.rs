// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("retronym.pest");

// build a parser using Pest:
pub(crate) mod pest {
    // this will do all the macro work of turning our grammar file into a `parse`
    // method on the below structure.
    #[derive(Parser)]
    #[grammar = "lib/retronym.pest"]
    pub struct RymParser;
}

use crate::node::MaybeNode;

/// During building of the `AST`, the methods return either a new `Node` to
/// attach to the `AST`, or a `ParseError`.
pub type ASTResult<'token> = ParseResult<MaybeNode<'token>>;

impl From<ParseError> for ASTResult<'_> {
    //==========================================================================
    /// For brevity, allow conversion of a `ParseError` to an `ASTResult`,
    /// i.e. `Result<Err(ParseError)>`.
    ///
    fn from(parse_error: ParseError) -> Self {
        //----------------------------------------------------------------------
        Err(parse_error)
    }
}

impl<'token> From<Node<'token>> for ASTResult<'token> {
    //==========================================================================
    fn from(node: Node<'token>) -> Self {
        //----------------------------------------------------------------------
        Ok(Some(node))
    }
}

impl<'token> From<Token<'token>> for ASTResult<'token> {
    //==========================================================================
    fn from(token: Token<'token>) -> Self {
        //----------------------------------------------------------------------
        Self::from(Node::from(token))
    }
}

use crate::tokenizer::Tokenizer;
use std::iter::Peekable;

pub struct Parser<'token> {
    tokens: Peekable<Tokenizer<'token>>,
}

use crate::error::*;
use crate::list::List;
use crate::node::Node;
use crate::token::Token;

impl<'token> Parser<'token> {
    //==========================================================================
    // note that we cannot implement `FromStr` due to the lifetime requirement?
    #[allow(clippy::should_implement_trait)]
    /// NB: the string reference must live as long as the `RymParser`;
    /// that is, the source string you pass it will not deallocate until
    /// the RymParser does as well.
    ///
    pub fn from_str(source: &'token str) -> Self {
        //----------------------------------------------------------------------
        Self {
            tokens: Tokenizer::from_str(source).unwrap().peekable(),
        }
    }

    /// A statement is one computable action.
    ///
    fn parse_statement(&mut self, token: Token<'token>) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        if token.is_keyword() {
            return self.parse_keyword(token);
        }
        if token.is_type() {
            return self.parse_record_type(token);
        }
        if token.is_macro() {
            return self.parse_macro(token);
        }
        if token.is_expr() {
            return self.parse_expr(token);
        }
        Ok(None)
    }

    /// Parse a keyword; for defining new Atoms and Macros.
    ///
    fn parse_keyword(&mut self, token: Token<'token>) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        if !token.is_keyword() {
            return Ok(None);
        }

        // which keyword?
        if token.is_keyword_atom() {
            return self.parse_keyword_atom(token);
        }
        if token.is_keyword_macro() {
            unimplemented!();
        }

        Ok(None)
    }

    /// A record type is an ad-hoc structure, setting the layout of
    /// data-packing for whatever data follows. When building an AST, nested
    /// structure names (e.g. "%vector", cannot be resovled yet (they might
    /// be defined in other modules) so we build a `List`. The assembly
    /// process transforms this list into a true struct, ensuring that any
    /// nested structs are resolved.
    fn parse_record_type(
        &mut self,
        mut token: Token<'token>,
    ) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        // if the current token is not a type,
        // this is not our concern.
        if !token.is_type() {
            return Ok(None);
        }

        // start a new List of Nodes to hold the record type
        let mut list = List::default();

        // add contiguous types to the struct:
        loop {
            // add the type to the record structure
            list.push(Node::from(token));
            // peek at the next token
            // TODO: specifically error on nested lists?
            match self.tokens.peek() {
                // if it's also a type, move to it and loop
                Some(t) if t.is_type() => token = self.tokens.next().unwrap(),
                // if it's not a type, or the file ends,
                // stop building the list
                None | Some(_) => break,
            };
        }

        ASTResult::from(
            // return an AST node containing the record-type
            Node::new_record(list),
        )
    }

    /// Parse an Atom definition.
    ///
    #[allow(clippy::needless_pass_by_value)]
    fn parse_keyword_atom(
        &mut self,
        token: Token<'token>,
    ) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        if !token.is_keyword_atom() {
            return Ok(None);
        }

        // "atom" keyword is present, skip over it
        let token = self.tokens.next().unwrap();

        // the next token *must* be an Atom name and not any kind
        // of expression or macro that might return an Atom
        if !token.is_atom() {
            return ASTResult::from(ParseError::unexpected());
        }

        // build an atom definition node
        ASTResult::from(Node::new_atom(token))
    }

    /// Parse a macro invocation.
    ///
    fn parse_macro(&mut self, token: Token<'token>) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        // if the current token is not a macro,
        // this is not our concern.
        if !token.is_macro() {
            return Ok(None);
        }

        // build a `Node` for a macro invocation
        ASTResult::from(Node::from(token))
    }

    /// Parse an expression, returning an AST node
    /// representing that expression.
    ///
    /// If the current token is not the beginning of an expression returns
    /// `None`; the caller can decide if this is unexpected or not; otherwise
    /// returns an `ASTResult` of either a `Node` of the expression, or the
    /// error encountered.
    ///
    fn parse_expr(&mut self, token: Token<'token>) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        // if the current token is not a valid opening for an expression
        // (including if we've reached end-of-file), then return an
        // "unrecognised" state, the caller decides if this is unexpected.
        if !token.is_expr() {
            return Ok(None);
        }

        // this is the beginning of an expression and we need to read the
        // first value that will form the inner-most (but also left-most)
        // value, e.g. the "1" in `(((1 + 2) + 3) + 4)`
        let left = Node::from(token);

        // is there any token following?
        match self.tokens.peek() {
            // is it an operator?
            // yes: parse the operator and right-hand-side,
            // passing in the left-hand value we already have
            Some(t) if t.is_oper() => self.parse_expr_inner(left),
            // no: this is a single value rather than an expression,
            // we can skip building an expression node and return
            // a value node instead
            None | Some(_) => ASTResult::from(left),
        }
    }

    fn parse_expr_inner(&mut self, left: Node<'token>) -> ASTResult<'token> {
        //----------------------------------------------------------------------
        // save the operator, move to the next token
        let oper = self.tokens.next().unwrap();
        let token = self.tokens.next().unwrap();

        // is there a token at all, and is it also a valid expression value?
        if !token.is_expr() {
            // no: we have an operator, but no value following it
            // e.g. "(1 + )"; return an "unexpected token" error
            return ASTResult::from(ParseError::unexpected());
        }

        // get the right hand value
        let right = token;
        let token = self.tokens.peek().unwrap();

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
        if token.is_oper() {
            // the expression we have just assembled will now
            // form the left-hand-side for the outer expression
            self.parse_expr_inner(expr)
        } else {
            ASTResult::from(expr)
        }
    }
}

impl<'token> Iterator for Parser<'token> {
    //==========================================================================
    type Item = ASTResult<'token>;

    /// When you turn the crank on the parser,
    /// it spits out AST nodes.
    ///
    fn next(&mut self) -> Option<ASTResult<'token>> {
        //----------------------------------------------------------------------
        // pull a token from the source code
        match self.tokens.next() {
            // if there are no more tokens,
            // return no more nodes
            None => None,
            Some(token) => match self.parse_statement(token) {
                // pass errors through
                Err(e) => Some(Err(e)),
                Ok(option) => match option {
                    Some(node) => Some(ASTResult::from(node)),
                    // if the token was consumed but produced
                    // no node, then return no more nodes
                    None => None,
                },
            },
        }
    }
}
