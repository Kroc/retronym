// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::error::*;
use parser::token::{Token, TokenKind, TokenNumber};
use std::convert::From;

/// The "Abstract Syntax Tree" puts `Token`s together into meaningful
/// relationships. Whilst the `TokenStream` only cares about the type of
/// individual `Token`s, the AST recognises lists, expressions and other
/// such multi-token structures.
pub struct AST {
    _nodes: Vec<ASTNode>,
}

impl Default for AST {
    fn default() -> Self {
        AST { _nodes: Vec::new() }
    }
}

/// The AST is made up of a series of interconnected nodes.
#[derive(Debug)]
pub struct ASTNode {
    pub kind: ASTNodeKind,
}

#[derive(Debug)]
pub enum ASTNodeKind {
    /// An empty node, used for unimplemented node types
    Void,
    /// A single literal value
    Value(ASTValue),
    /// A list of elements
    List,
    /// An experssion -- i.e. a calculation
    Expr(ASTExpr),
}

/// A node that produces a single value. This can be a numeric literal,
/// such as "100" or a calculation.
#[derive(Debug)]
pub enum ASTValue {
    Int(i64),
    Float(f64),
    Expr(Box<ASTExpr>),
}

#[derive(Debug)]
pub struct ASTExpr {
    left: ASTValue,
    right: ASTValue,
}

/// During building of the `AST`, the methods return either a new `ASTNode` to
/// attach to the `AST`, or an `Error`.
pub type ASTResult = ParseResult<ASTNode>;
pub type MaybeASTResult = Option<ASTResult>;

impl Default for ASTNode {
    fn default() -> Self {
        Self {
            kind: ASTNodeKind::Void,
        }
    }
}

impl ASTNode {
    /// Retuns a void AST Node.
    pub fn new_void() -> ASTNode {
        Self::default()
    }

    pub fn new_int(value: i64) -> ASTNode {
        Self {
            kind: ASTNodeKind::Value(ASTValue::Int(value)),
        }
    }

    pub fn new_float(value: f64) -> ASTNode {
        Self {
            kind: ASTNodeKind::Value(ASTValue::Float(value)),
        }
    }
}

// Convert a `Token` into an `ASTNode`
impl<'t> From<&'t Token> for ASTNode {
    fn from(token: &'t Token) -> ASTNode {
        // what kind of token is this?
        match token.kind {
            TokenKind::Num(TokenNumber::Int(i)) => ASTNode::new_int(i),
            TokenKind::Num(TokenNumber::Float(f)) => ASTNode::new_float(f),
            _ => panic!(
                "Token is not of a kind that can be converted into an ASTNode."
            ),
        }
    }
}
