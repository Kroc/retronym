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

use parser::ast::{ASTNode, ASTResult, MaybeASTResult};
use parser::tokenstream::TokenIterator;

pub struct Parser<'t> {
    tokens: TokenIterator<'t>,
}

impl<'t> Parser<'t> {
    pub fn new(mut tokens: TokenIterator<'t>) -> Parser<'t> {
        // TODO: error with no tokens in tokenstream?
        // read the first token and store as the 'current' token
        let _ = tokens.next();

        Parser { tokens }
    }

    fn root(&mut self) -> MaybeASTResult {
        // what's allowed at root level?
        self.parse_number()
    }

    fn parse_number(&mut self) -> MaybeASTResult {
        // is the current token relevant to us?
        if !self.tokens.is_number() {
            // if not, return None to indicate that this is not our
            // responsibility; it's up to the caller to decide if that's
            // unexpected or not
            return None;
        };

        // TODO: return an AST node of a literal number
        Some(Ok(
            ASTNode::from( self.tokens.consume() )
        ))
    }
}

impl<'t> Iterator for Parser<'t> {
    type Item = ASTResult;

    fn next(&mut self) -> MaybeASTResult {
        self.root()
    }
}
