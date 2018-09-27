// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::Rule;
use parser::RymParser;
use pest::Parser;
use token;

/// A `TokenStream` is a vector of `Token`s; a machine-understandable
/// representation of the source code, split into 'words' ("lexemes"),
/// and marked up with specific types.
pub struct TokenStream(Vec<token::Token>);

impl TokenStream {
    fn tokenize(source: String) -> Self {
        let mut tokens = TokenStream(Vec::new());

        let pairs = RymParser::parse(Rule::rym, &source);
        if pairs.is_err() {
            panic!("error parsing: {:#?}", pairs.err().unwrap());
        }

        // loop over our Pairs
        for pair in pairs.unwrap().flatten() {
            for inner_pair in pair.into_inner() {
                //let inner_span = inner_pair.clone().into_span();
                let token = token::Token::from(inner_pair);
                println!("{:?}", token);
                // a huge thanks to this reddit post that explained that
                // you have to use "...0..." to access the native methods
                // of the vector within the new-type
                // https://www.reddit.com/r/rust/comments/3wgb4e//cxvzpdw/
                tokens.0.push(token);
            }
        }

        tokens
    }
}

/// Create a `TokenStream` from a string of source-code; this works for
/// anything that implements the `ToString` trait, which is automatic for
/// anything that implements the `Display` trait.
impl<T: ToString> From<T> for TokenStream {
    fn from(from: T) -> Self {
        TokenStream::tokenize(from.to_string())
    }
}
