// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::TryFrom_;
use parser::Rule;
use parser::RymParser;
use pest::Parser;
use tokenizer::token::Token;

pub type Tokens = Vec<Token>;

/// A `TokenStream` is a vector of `Token`s; a machine-understandable
/// representation of the source code, split into 'words' ("lexemes"),
/// and marked up with specific types.
pub struct TokenStream {
    /// The collection of tokens; not exposed publicly so that we can
    /// implement the immutable interface with internal cursor
    tokens: Tokens,
    /// The current cursor position within the stream
    index: usize,
}

impl TokenStream {
    fn tokenize(source: String) -> Self {
        let mut tokenstream = TokenStream {
            tokens: Vec::new(),
            index: 0,
        };

        let pairs = RymParser::parse(Rule::rym, &source);
        if pairs.is_err() {
            panic!("error parsing: {:#?}", pairs.err().unwrap());
        }

        // loop over our Pairs
        for pair in pairs.unwrap().flatten() {
            for inner_pair in pair.into_inner() {
                //let inner_span = inner_pair.clone().into_span();
                let token = Token::try_from_(inner_pair).unwrap();
                println!("{:?}", token);
                // a huge thanks to this reddit post that explained that
                // you have to use "...0..." to access the native methods
                // of the vector within the new-type
                // https://www.reddit.com/r/rust/comments/3wgb4e//cxvzpdw/
                tokenstream.tokens.push(token);
            }
        }

        tokenstream
    }

    /// Rewind the `TokenStream` back to the beginning
    pub fn rewind(&mut self) {
        self.index = 0;
    }

    /// Move the cursor forward and return the Token
    pub fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        // if we've hit the end of the `TokenStream` this will return `None`
        self.tokens.get(self.index)
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