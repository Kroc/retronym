// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Source code files are parsed into Abstract Syntax Trees which are then
//! compiled into Objects -- essentially a set of instructions for constructing
//! a binary file, given a set of imports (from other Objects) to 'fill in the
//! blanks'.

use parser::tokenstream::TokenStream;
use parser::parser::Parser;
use parser::ast::AST;

pub struct Object {
    _ast: AST
}

impl Object {
    pub fn new_from_str(source: &str) -> Object {
        // scan the source code into a token stream
        let tokenstream = TokenStream::from(source);
        // create a parser from the token stream;
        // this will output AST nodes
        let _parser = Parser::new(tokenstream.iter());

        Object {
            _ast: AST::default(),
        }
    }
}