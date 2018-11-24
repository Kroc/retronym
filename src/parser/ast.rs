// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::error::*;

/// The "Abstract Syntax Tree" is a machine understandable respresentation of
/// some source code. 
pub struct AST {
    _nodes: Vec<ASTNode>,
}

impl Default for AST {
    /// Gives you an empty AST structure.
    fn default() -> Self {
        AST { _nodes: Vec::new() }
    }
}

/// The AST is made up of a series of nodes where each node is a top-level
/// "statement" and may contain descendants based on type. In practice,
/// Retronym's top-level statements are either macros or lists.
#[derive(Debug)]
pub struct ASTNode {
    /// The 'type' of the node, e.g. whether this is a literal number,
    /// an expression, a macro invocation etc.
    pub kind: ASTKind,
    /// The attached data for the node.
    pub data: ASTData,
    //TODO: include original source location
}

#[derive(Debug)]
pub enum ASTKind {
    /// An empty node, used for unimplemented node types.
    Void,
    /// A literal value. The `data` field contains the actual value.
    Value,
    /// A list of elements
    List,
    /// An experssion -- i.e. a calculation
    Expr,
}

#[derive(Debug)]
pub enum ASTData {
    /// No data for this node.
    None,
    /// An integer literal value.
    Int(i64),
    /// A floating point literal value.
    Float(f64),
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
            data: ASTData::None,
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
            kind: ASTKind::Value,
            data: ASTData::Int(value),
        }
    }

    pub fn new_float(value: f64) -> ASTNode {
        Self {
            kind: ASTKind::Value,
            data: ASTData::Float(value),
        }
    }
}

//==============================================================================

use std::fmt::{self, *};

impl Display for ASTNode {
    /// Pretty-prints an ASTNode (and its descendants),
    /// essentially outputting normalised source code
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ASTKind::Void => write!(f, "<VOID>"),
            ASTKind::Value => write!(f, "{}", self.data),
            // TODO: this will obviously include sub-elements
            ASTKind::List => write!(f, "()"),
            _ => unimplemented!(
                "ASTKind kind does not have a Display implementation."
            ),
        }
    }
}

impl Display for ASTData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTData::Int(i) => write!(f, "{}", i),
            ASTData::Float(d) => write!(f, "{}", d),
            _ => unimplemented!(
                "ASTData does not have a Display implementation."
            ),
        }
    }
}
