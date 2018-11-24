// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Module `tokenstream` presents Pest's `Pairs` (the result of Pest parsing)
//! into a continuous stream of "tokens", the individual 'words' of the source
//! code.

use parser::parser::Rule;
use parser::parser::RymParser;
use parser::token::Token;

/// A `TokenStream` wraps Pest's `Pairs` struct and presents an interface that
/// is more directly informed of Retronym's grammar.
pub struct TokenStream<'t> {
    tokens: Vec<Token<'t>>,
}

// required for the `parse` method of `RymParser` to be visible here.
use pest::Parser;

impl<'t> TokenStream<'t> {
    /// Creates a `TokenStream` directly from source code.
    pub fn new_from_str(source: &'t str) -> Self {
        // instantiate a TokenStream object,
        let tokenstream = Self {
            // assigning the `tokens` field immediately like this gives
            // better type inference, lest we summon the turbo-fish!
            tokens: RymParser::parse(Rule::rym, &source)
                .expect("error parsing: {:#?}")
                // collect the `Pairs` into a Vector as we want to be able
                // to easily reference the 'current' token regularly
                .collect(),
        };

        for p in tokenstream.tokens.clone() {
            println!("token: {:?}", p);
        }

        tokenstream
    }
}
