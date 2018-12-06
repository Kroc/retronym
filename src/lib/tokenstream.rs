// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Module `tokenstream` presents Pest's `Pairs` (the result of Pest parsing)
//! into a continuous stream of "tokens", the individual 'words' of the source
//! code.

use crate::parser::Rule;
use crate::parser::RymParser;
use crate::token::{Token, Tokens};

/// A `TokenStream` wraps Pest's `Pairs` struct and presents an interface that
/// is more directly informed of Retronym's grammar.
pub struct TokenStream<'token> {
    tokens: Tokens<'token>,
    /// Index of 'current' token in the stream.
    index: usize,
}

// required for the `parse` method of `RymParser` to be visible here.
use pest::Parser;

use crate::token::MaybeToken;

impl<'token> TokenStream<'token> {
    /// Creates a `TokenStream` directly from source code.
    pub fn new_from_str(source: &'token str) -> Self {
        // instantiate a TokenStream object,
        Self {
            // assigning the `tokens` field immediately like this gives
            // better type inference, lest we summon the turbo-fish!
            tokens: RymParser::parse(Rule::rym, &source)
                .expect("error parsing: {:#?}")
                // convert Pest `Pairs` into `Token`s
                .map(Token::from)
                // collect the `Token`s into a Vector as we want to be able
                // to easily reference the 'current' token regularly
                .collect(),

            // begin at the beginning
            index: 0,
        }
    }

    /// Moves to the next token.
    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn is_eof(&self) -> bool {
        self.tokens.get(self.index).is_none()
    }

    /// Returns the 'current' token.
    pub fn token(&self) -> MaybeToken<'token> {
        match self.tokens.get(self.index) {
            None => None,
            Some(t) => Some(t.clone()),
        }
    }

    pub fn is_atom(&self) -> bool {
        match self.tokens.get(self.index) {
            Some(t) => t.is_atom(),
            None => false,
        }
    }

    pub fn is_macro(&self) -> bool {
        match self.tokens.get(self.index) {
            Some(t) => t.is_macro(),
            None => false,
        }
    }

    pub fn is_expr(&self) -> bool {
        match self.tokens.get(self.index) {
            Some(t) => t.is_expr(),
            None => false,
        }
    }

    pub fn is_oper(&self) -> bool {
        match self.tokens.get(self.index) {
            Some(t) => t.is_oper(),
            None => false,
        }
    }

    /// Returns the current token, and (internally) moves to the next.
    pub fn consume(&mut self) -> Option<Token<'token>> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        match token {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
}
