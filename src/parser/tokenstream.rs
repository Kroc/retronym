// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::TryFrom_;
use parser::parser::Rule;
use parser::parser::RymParser;
use parser::token::{Token, TokenKind, TokenKeyword, NULLTOKEN};
use pest::Parser;

pub type Tokens = Vec<Token>;

/// A `TokenStream` is a vector of `Token`s; a machine-understandable
/// representation of the source code, split into 'words' ("lexemes"),
/// and marked up with specific types.
pub struct TokenStream {
    tokens: Tokens
}

impl TokenStream {
    fn tokenize(source: String) -> Self {
        // create an empty TokenStream to begin with
        let mut tokenstream = Self {
            tokens: Vec::new(),
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
                tokenstream.tokens.push(token);
            }
        }

        tokenstream
    }

    // Get an `Iterator` for the `Token`s; this can be used
    // to walk the `TokenStream` contents in standard fashion
    pub fn iter(&self) -> TokenIterator {
        TokenIterator {
            inner: self.tokens.iter(),
            token: &NULLTOKEN,
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

//==============================================================================

pub struct TokenIterator<'t> {
    inner: ::std::slice::Iter<'t, Token>,
    pub token: &'t Token,
}

// an Iterator of Token references:
impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        // is there another token?
        match self.inner.next() {
            // yes;
            Some(ref token) => {
                // set the 'current' token
                self.token = token;
                // and return it
                Some(token)
            }
            // no;
            None => {
                // set the 'current' token to the dummy
                self.token = &NULLTOKEN;
                // but return None as expected from `next`
                None
            }
        }
    }
}

impl<'t> TokenIterator<'t> {
    pub fn current(&self) -> &Token {
        self.token
    }

    /// Is the current token an "End Of File" token?
    pub fn is_eof(&self) -> bool {
        self.token.is_eof()
    }

    /// Returns true if the token is a number (of any kind).
    pub fn is_number(&self) -> bool {
        self.token.is_number()
    }

    /// Return the current token, but move to the next
    pub fn consume(&mut self) -> &Token {
        let token = self.token;
        let _ = self.next();
        token
    }

    /*
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
    */
}
