// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::TryFrom_;
use std::error::Error;
use tokenizer::token::{Token, TokenKind};
use tokenizer::tokenstream::{TokenStream, Tokens};

/// The "Abstract Syntax Tree" puts `Token`s together into meaningful
/// relationships. Whilst the `TokenStream` only cares about the type of
/// individual `Token`s, the AST recognises lists, expressions and other
/// such multi-token structures.
pub struct AST {
    nodes: Vec<ASTNode>,
}

/// The AST is made up of a series of interconnected Nodes.
pub struct ASTNode {
    kind: ASTNodeKind,
}

pub enum ASTNodeKind {
    /// An assembler mnemonic, e.g. "lda"
    Mnemonic(ASTMnemonic),
    List,
    /// An experssion -- i.e. a calculation that generates a Value
    Expr,
    /// A single numerical value
    Value,
}

pub enum ASTMnemonic {
    /// Add Memory to Accumulator with Carry
    ADC,
    /// "AND" Memory with Accumulator
    AND,
}

impl TryFrom_<TokenStream> for AST {
    /// Convert a `TokenStream` to an `AST`. Returns a `Result` containing
    /// either a new `AST` object, or an `Error`.
    fn try_from_(tokenstream: TokenStream) -> Result<Self, Box<Error>> {
        // create a blank AST to begin working with
        let ast = AST { nodes: Vec::new() };

        // begin walking the `TokenStream`
        ast.walk_root(tokenstream.tokens, 0);

        Ok(ast)
    }
}

impl AST {
    fn walk_root(&self, tokens: Tokens, index: usize) {
        match tokens[index].kind {
            // keywords & mnemonics
            ref atom @ TokenKind::Atom { .. } => panic!("Unimplemented"),

            // a token that isn't allowed at root-scope
            _ => panic!("Unexpected at this scope!"),
        };
    }
}

impl TryFrom_<Token> for ASTMnemonic {
    fn try_from_(token: Token) -> Result<Self, Box<Error>> {
        match token.kind {
            TokenKind::Atom(s) => match s.to_lowercase().as_ref() {
                "adc" => Ok(ASTMnemonic::ADC),
                "and" => Ok(ASTMnemonic::AND),
                _ => panic!("Not a recognised mnemonic"),
            },
            _ => panic!("Not an assembler mnemonic"),
        }
    }
}
