// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

#[derive(Parser)]
#[grammar = "parser/retronym.pest"]
pub struct RymParser;

//------------------------------------------------------------------------------

use parser::error::*;
use parser::ast::{ASTNode, ASTResult, MaybeASTResult};
use parser::token::{TokenKind, TokenKindKeyword};
use parser::tokenstream::TokenStreamIterator;

pub struct Parser<'t> {
    /// A `TokenStream` (or any `Iterator<Item = Token>`), from which we'll
    /// read `Token`s and parse into an `AST`.
    tokens: TokenStreamIterator<'t>,
}

impl<'t> Parser<'t> {
    pub fn new(mut tokens: TokenStreamIterator<'t>) -> Parser<'t> {
        // TODO: error with no tokens in tokenstream?
        // read the first token and store as the 'current' token
        let _ = tokens.next();

        Parser { tokens }
    }

    fn root(&mut self) -> MaybeASTResult {
        // what's allowed at root level?
        self.atomdef()
    }

    fn atomdef(&mut self) -> MaybeASTResult {
        if self.tokens.is_eof() {
            return Some(Err(new_parse_error(ParseErrorKind::EndOfFile)));
        };
        match self.tokens.expect_keyword(TokenKindKeyword::Atom) {
            Some(t) => match t.kind {
                TokenKind::Atom(ref atom) => {
                    Some(Ok(ASTNode::new_atomdef(atom)))
                }
                _ => panic!("No Atom following the `atom` keyword."),
            },
            None => panic!("No Atom following the `atom` keyword."),
        }
    }
}

impl<'t> Iterator for Parser<'t> {
    type Item = ASTResult;

    fn next(&mut self) -> MaybeASTResult {
        self.root()
    }
}
