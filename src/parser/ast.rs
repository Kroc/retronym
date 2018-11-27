// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::parser::error::*;

/// The "Abstract Syntax Tree" is a machine understandable respresentation of
/// some source code. Because `ASTNode`s can contain a reference back to the
/// original source code (token) for errors, the `'token` lifetime is used
/// so that the source code is not deallocated before the AST.
pub struct AST<'token> {
    _nodes: Vec<ASTNode<'token>>,
}

impl<'token> Default for AST<'token> {
    /// Gives you an empty AST structure.
    fn default() -> Self {
        AST { _nodes: Vec::new() }
    }
}

use crate::parser::token::MaybeToken;

/// The AST is made up of a series of nodes where each node is a top-level
/// "statement" and may contain descendants based on type. In practice,
/// Retronym's top-level statements are either macros or expressions.
#[derive(Debug)]
pub struct ASTNode<'token> {
    /// The 'type' of the node, e.g. whether this is a literal number,
    /// an expression, a macro invocation etc. This can contain nested nodes!
    pub kind: ASTKind<'token>,
    /// An optional reference back to the original source code,
    /// for error messages.
    pub token: MaybeToken<'token>,
}

#[derive(Debug)]
pub enum ASTKind<'token> {
    /// An empty node, used for unimplemented node types.
    Void,
    /// An experssion -- i.e. a calculation
    Expr(Box<ASTExpr<'token>>),
    /// An atom.
    Atom(String),
    /// A macro invocation.
    Macro(String),
    /// A literal value.
    Value(ASTValue),
}

#[derive(Debug)]
pub enum ASTValue {
    /// An integer literal value.
    Int(i64),
    /// A floating point literal value.
    Float(f64),
    /// A string literal.
    Str(String),
}

#[derive(Debug)]
pub struct ASTExpr<'token> {
    pub left: ASTNode<'token>,
    pub op: ASTOperator,
    pub right: ASTNode<'token>,
}

#[derive(Debug)]
pub enum ASTOperator {
    /// Addition operator "+"
    Add,
    /// Subtraction operator "-"
    Sub,
    /// Multiplication operator "*"
    Mul,
    /// Division operator "/"
    Div,
    /// Modulo operator "\\"
    Mod,
    /// Power/Exponention Operator "**"
    Pow,
    /// Bitwise eXclusive OR operator "^"
    Xor,
    /// Bitwise AND operator "&"
    And,
    /// Bitwise OR operator "|"
    Or,
    /// Bitwise SHift-Left operator "<<"
    Shl,
    /// Bitwise SHift-Right operator ">>"
    Shr,
    /// Repeat operator "x"
    Rep,
}

/// During building of the `AST`, the methods return either a new `ASTNode` to
/// attach to the `AST`, or an `Error`.
pub type ASTResult<'token> = ParseResult<ASTNode<'token>>;
pub type MaybeASTResult<'token> = Option<ASTResult<'token>>;

impl<'token> Default for ASTNode<'token> {
    fn default() -> Self {
        Self {
            kind: ASTKind::Void,
            token: None,
        }
    }
}

//==============================================================================

use crate::parser::parser::Rule;
use crate::parser::token::Token;
use std::convert::From;

impl<'token> From<Token<'token>> for ASTNode<'token> {
    fn from(token: Token<'token>) -> ASTNode<'_> {
        ASTNode {
            kind: match token.as_rule() {
                // parse an integer number:
                Rule::int_number => ASTKind::Value(ASTValue::Int(
                    // parse the text as an integer number
                    token.as_str().parse::<i64>().unwrap(),
                )),
                // parse a hexadecimal number:
                Rule::hex_number => ASTKind::Value(ASTValue::Int(
                    // note that we have to drop the sigil. limitations in
                    // Pest make this difficult to do at the grammar level
                    i64::from_str_radix(&token.as_str()[1..], 16).unwrap(),
                )),
                // parse a binary number:
                Rule::bin_number => ASTKind::Value(ASTValue::Int(
                    i64::from_str_radix(&token.as_str()[1..], 2).unwrap(),
                )),
                Rule::atom => ASTKind::Atom(
                    //TODO: messy
                    token.as_str().to_string(),
                ),
                // a macro is returned as a string
                Rule::mac => ASTKind::Macro(
                    //TODO: messy
                    token.as_str().to_string(),
                ),
                _ => ASTKind::Void,
            },
            // embed the original token with the source-code location.
            // this'll be used if we need to print an error message.
            token: Some(token),
        }
    }
}

//==============================================================================

use std::fmt::{self, *};

impl<'token> Display for ASTNode<'token> {
    /// Pretty-prints an ASTNode (and its descendants),
    /// essentially outputting normalised source code
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ASTKind::Void => write!(f, "<VOID>"),
            ASTKind::Value(_) => write!(f, "{}", self.kind),
            _ => unimplemented!(
                "ASTKind kind does not have a Display implementation."
            ),
        }
    }
}

impl<'token> Display for ASTKind<'token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTKind::Value(v) => match v {
                ASTValue::Int(i) => write!(f, "{}", i),
                ASTValue::Float(d) => write!(f, "{}", d),
                ASTValue::Str(s) => write!(f, "{}", s),
            },
            _ => unimplemented!(
                "ASTData does not have a Display implementation."
            ),
        }
    }
}
