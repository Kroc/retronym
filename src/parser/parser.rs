// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

// build a parser using Pest:

use parser::tokenstream::TokenStream;

// this will do all the macro work of turning our grammar file into a `parse`
// method on the below structure.
#[derive(Parser)]
#[grammar = "parser/retronym.pest"]
pub struct RymParser<'t> {
    #[allow(dead_code)]
    tokens: TokenStream<'t>,
}

use parser::ast::{ASTNode, ASTResult, MaybeASTResult};

impl<'token> RymParser<'token> {
    /// NB: the string reference must live as long as the `RymParser`;
    /// that is, the source string you pass it will not deallocate until
    /// the RymParser does as well.
    pub fn from_str(source: &'token str) -> Self {
        let tokens = TokenStream::new_from_str(source);

        // build a parser struct
        Self { tokens }
    }

    fn parse_statement(&self) -> MaybeASTResult<'token> {
        // a statement can be either a macro invocation (optionally followed
        // by a list) or a list of expressions.
        self.parse_macro()
    }

    /// Parse a macro invocation.
    fn parse_macro(&self) -> MaybeASTResult<'token> {
        // if the current token is not a macro, this is not our concern.
        if !self.tokens.is_macro() {
            return None;
        }

        let token = self.tokens.token().unwrap();

        // build an ASTNode for a macro invocation.
        // TODO: messy
        Some(Ok(ASTNode::from(token)))
    }
}

impl<'token> Iterator for RymParser<'token> {
    type Item = ASTResult<'token>;

    /// When you turn the crank on the parser, it spits out AST nodes.
    fn next(&mut self) -> Option<ASTResult<'token>> {
        self.parse_statement()
    }
}
