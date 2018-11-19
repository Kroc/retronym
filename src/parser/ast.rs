// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::error::*;
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

/// The AST is made up of a series of nodes where each node is a top-level
/// "statement" and may contain descendants based on type. In practice,
/// Retronym's top-level statements are either macros or lists.
#[derive(Debug)]
pub struct ASTNode {
    pub kind: ASTKind,
    //TODO: include original source location
}

#[derive(Debug)]
pub enum ASTKind {
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
    left: Box<ASTNode>,
    op: ASTOperator,
    right: Box<ASTNode>,
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
pub type ASTResult = ParseResult<ASTNode>;
pub type MaybeASTResult = Option<ASTResult>;

impl Default for ASTNode {
    fn default() -> Self {
        Self {
            kind: ASTKind::Void,
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
            kind: ASTKind::Value(ASTValue::Int(value)),
        }
    }

    pub fn new_float(value: f64) -> ASTNode {
        Self {
            kind: ASTKind::Value(ASTValue::Float(value)),
        }
    }
}

/*
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
*/

//==============================================================================

use std::fmt::{self, *};

impl Display for ASTNode {
    /// Pretty-prints an ASTNode (and its descendants),
    /// essentially outputting normalised source code
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for ASTKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTKind::Void => write!(f, "<VOID>"),
            ASTKind::Value(v) => write!(f, "{}", v),
            // TODO: this will obviously include sub-elements
            ASTKind::List => write!(f, "()"),
            _ => unimplemented!(
                "ASTNodeKind kind does not have a Display implementation."
            ),
        }
    }
}

impl Display for ASTValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTValue::Int(i) => write!(f, "{}", i),
            ASTValue::Float(d) => write!(f, "{}", d),
            _ => unimplemented!(
                "ASTValue kind does not have a Display implementation."
            ),
        }
    }
}
