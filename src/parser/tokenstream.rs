// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::TryFrom_;
use parser::parser::Rule;
use parser::parser::RymParser;
use parser::token::{Token, TokenKind, TokenKindKeyword};
use pest::Parser;

pub type Tokens = Vec<Token>;

/// A `TokenStream` is a vector of `Token`s; a machine-understandable
/// representation of the source code, split into 'words' ("lexemes"),
/// and marked up with specific types.
pub struct TokenStream {
    tokens: Tokens,
}

impl Default for TokenStream {
    /// Create an empty `TokenStream`
    fn default() -> Self {
        Self { tokens: Vec::new() }
    }
}

impl TokenStream {
    fn tokenize(source: String) -> Self {
        // create an empty TokenStream to begin with
        let mut tokenstream = Self::default();

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

    // Get an `Iterator` for the `Token`s; this can be used
    // to walk the `TokenStream` contents in standard fashion
    pub fn iter(&self) -> TokenStreamIterator {
        TokenStreamIterator {
            inner: self.tokens.iter(),
            token: None,
        }
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

pub struct TokenStreamIterator<'t> {
    inner: ::std::slice::Iter<'t, Token>,
    pub token: Option<&'t Token>,
}

// an Iterator of Token references:
impl<'a> Iterator for TokenStreamIterator<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token = self.inner.next();
        self.token
    }
}

impl<'t> TokenStreamIterator<'t> {
    pub fn is_eof(&self) -> bool {
        self.token.is_none()
    }

    /// Return the current token, but move to the next
    pub fn consume(&mut self) -> Option<&Token> {
        let token = self.token;
        let _ = self.next();
        token
    }

    /// If the current token is a particular keyword
    /// then return the token, otherwise return None.
    pub fn expect_keyword(&mut self, kind: TokenKindKeyword) -> Option<&Token> {
        // if None, return None
        self.token.and_then(|t| match t.kind {
            // if the kinds match, return the next token
            TokenKind::Keyword(ref k) if k == &kind => self.next(),
            // no match -- return None
            _ => None,
        })
    }

    pub fn expect_atom(&mut self) -> Option<&Token> {
        // if None, return None
        self.token.and_then(move |t| match t.kind {
            // if the kinds match, return the next token
            TokenKind::Atom(_) => self.consume(),
            // no match -- return None
            _ => None,
        })
    }
}
