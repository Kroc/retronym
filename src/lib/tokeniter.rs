// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::parser::pest::Rule;
use crate::parser::pest::RymParser;
use crate::token::Token;
use pest::iterators::Pairs;

pub struct TokenIter<'token> {
    /// A Pest Pairs iterator which will yield each token from the source code.
    pairs: Pairs<'token, Rule>,
}

use std::convert::From;

impl<'token> From<Pairs<'token, Rule>> for TokenIter<'token> {
    /// Build a new token iterator from Pest's Pairs iterator.
    /// (the `parse` method will give you one of these)
    fn from(pairs: Pairs<'token, Rule>) -> Self {
        Self { pairs }
    }
}

use crate::error::ParseError;
// required for the `parse` method of `RymParser` to be visible here.
use pest::Parser;

impl<'token> TokenIter<'token> {
    // note that we cannot implement `FromStr` due to the lifetime requirement?
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &'token str) -> Result<Self, ParseError> {
        // create a parser from the given source code
        let parser = RymParser::parse(Rule::rym, s).expect(
            // TODO: cast `PestError` to `ParseError`
            "Fail to parse source text using Pest.",
        );

        // convert the parse result into an wrapped iterator
        Ok(TokenIter::from(parser))
    }
}

impl<'token> Iterator for TokenIter<'token> {
    type Item = Token<'token>;

    fn next(&mut self) -> Option<Self::Item> {
        // get the next Pest Pair and return it as a Token
        // (we don't want to expose the Pair internals)
        self.pairs.next().map(Token::from)
    }
}
