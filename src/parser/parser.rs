// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

// build a parser using Pest:

// this will do all the macro work of turning our grammar file into a `parse`
// method on the below structure.
#[derive(Parser)]
#[grammar = "parser/retronym.pest"]
pub struct RymParser<'t> {
    #[allow(dead_code)]
    tokens: TokenStream<'t>,
}

use parser::ast::{ASTResult, MaybeASTResult};
use parser::tokenstream::TokenStream;

impl<'t> RymParser<'t> {
    /// NB: the string reference must live as long as the `RymParser`;
    /// that is, the source string you pass it will not deallocate until
    /// the RymParser does as well.
    pub fn from_str(source: &'t str) -> Self {
        // build a parser struct
        Self {
            tokens: TokenStream::new_from_str(source),
        }
    }

    fn parse_line(&self) -> MaybeASTResult {
        None
    }
}

impl<'t> Iterator for RymParser<'t> {
    type Item = ASTResult;

    /// When you turn the crank on the parser, it spits out AST nodes.
    fn next(&mut self) -> Option<ASTResult> {
        self.parse_line()
    }
}
