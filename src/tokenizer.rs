// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use pest::iterators::Pairs;
use pest::Parser;
use std::error::Error;
use std::fs;
use token;

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

#[derive(Parser)]
#[grammar = "retronym.pest"]
struct RymParser;

//------------------------------------------------------------------------------

/// Tokeniser takes a file spec and generates a list of `Token`s,
/// representing the original source code in an ordered, machine-understandable
/// way; e.g. numbers in the source are parsed into their scalar values.
pub struct Tokenizer {
    /// A Tokenizer is tied to its source file and cannot be repurposed
    /// for another file (the internal state would be out of sync).
    /// Use `Tokenizer::tokenize_file` to build a new `Tokenizer` from a file.
    pub filepath: String,
    /// The in-memory copy of the source-code used for tokenising and
    /// error-reporting. Note that this is immutable; the tokens produced are
    /// bound to the source representation they came from. If you need to
    /// re-tokenise a file, create a new `Tokenizer`
    //TODO: this is going to copy an already in-memory file;
    //      use a str reference instead?
    pub source: String,

    tokens: Vec<token::Token>,
}

impl Tokenizer {
    /// Generate a new `Tokenizer` and populate it
    /// from the given source-code file.
    pub fn tokenize_file(filepath: &str) -> Result<Tokenizer, Box<Error>> {
        let tokenizer = Tokenizer {
            // copy the given file-name, so as to not keep the string
            // reference alive along with this object
            filepath: filepath.to_string(),
            //TODO: better error handling; pre-check file exists etc?
            source: fs::read_to_string(filepath)?.to_string(),
            tokens: Vec::new(),
        };

        tokenizer.tokenize();
        
        // return the Tokenizer we've created
        Ok(tokenizer)
    }

    /// Generate a new `Tokenizer` and populate it
    /// from the string input given.
    pub fn tokenize_str(input: &str) -> Result<Tokenizer, Box<Error>> {
        let tokenizer = Tokenizer {
            // filename is blank to indicate no file-binding
            filepath: "".to_string(),
            source: input.to_string(),
            tokens: Vec::new(),
        };

        tokenizer.tokenize();

        // return the Tokenizer we've created
        Ok(tokenizer)
    }

    fn tokenize(&self) {
        let pairs = RymParser::parse(Rule::rym, &self.source).unwrap();
        dump_pairs(pairs);
    }
}

// pretty print the contents of Pairs.
// I'm sure we can implement the Debug trait to do this more cleanly,
// but I don't know how to do that just yet
fn dump_pairs(pairs: Pairs<Rule>) {
    // loop over our Pairs
    for pair in pairs.flatten() {
        // a pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            //println!("= {:?}", inner_pair.as_rule());

            let inner_span = inner_pair.clone().into_span();

            println!(
                "= {:16}: \"{}\"",
                format!("{:?}", inner_pair.as_rule()),
                inner_span.as_str()
            );
        }
    }
}

/*
impl<'str> From<&'str str> for Tokenizer {
    /// Create a Tokenizer from any `str` reference
    fn from(input: &'str str) -> Self {
        let tokenizer = Tokenizer {
            // filename is blank to indicate no file-binding
            filepath: "".to_string(),
            source: input.to_string(),
            tokens: Vec::new(),
        };

        // return the Tokenizer we've created
        tokenizer
    }
}

impl<'str> From<&'str String> for Tokenizer {
    fn from(input: &'str String) -> Self {
        let tokenizer = Tokenizer {
            // filename is blank to indicate no file-binding
            filepath: "".to_string(),
            source: input.to_string(),
            tokens: Vec::new(),
        };

        // return the Tokenizer we've created
        tokenizer
    }
}

impl From<String> for Tokenizer {
    fn from(input: String) -> Self {
        let tokenizer = Tokenizer {
            // filename is blank to indicate no file-binding
            filepath: "".to_string(),
            source: input,
            tokens: Vec::new(),
        };

        // return the Tokenizer we've created
        tokenizer
    }
}
*/
