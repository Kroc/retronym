// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! The AST `Node` is the core representation of source code in the program,
//! it uses an enum to differentiate numbers, strings, macros and so on.
//! AST nodes can contain other nodes, such as with expressions.

use crate::error::*;
use crate::token::MaybeToken;

/// The AST is made up of a series of nodes where each node is a top-level
/// "statement" and may contain descendants based on type. In practice,
/// Retronym's top-level statements are either macros or expressions.
#[derive(Debug)]
pub struct Node<'token> {
    /// The 'type' of the node, e.g. whether this is a literal number,
    /// an expression, a macro invocation etc. This can contain nested nodes!
    pub kind: NodeKind<'token>,
    /// An optional reference back to the original source code,
    /// for error messages.
    pub token: MaybeToken<'token>,
    /// A node is static if it, and any descendents, contain only literal
    /// values that can be calculated without outside information.
    pub is_static: bool,
}

pub type MaybeNode<'token> = Option<Node<'token>>;

use crate::atom::Atom;
use crate::expr::Expr;
use crate::list::List;

#[derive(Debug)]
pub enum NodeKind<'token> {
    /// An empty node.
    Void,
    /// An Atom definition. Defines a new Atom and exports it. When the final
    /// linking occurs, all atoms used must be defined.
    DefAtom(Atom),
    /// A list.
    List(Box<List<'token>>),
    /// An expression -- i.e. a calculation
    Expr(Box<Expr<'token>>),
    /// An Atom invocation.
    Atom(String),
    /// A Macro invocation.
    Macro(String),
    /// A string literal. Since strings are self-contained lists, these are
    /// not treated as expression values.
    Str(String),
    /// A literal value.
    Value(Value),
}

#[derive(Debug)]
pub enum Value {
    /// An integer literal value.
    Int(i64),
    /// An unsigned literal value as used by binary and hexadecimal
    /// numbers to represent raw numbers up to 64-bits.
    UInt(u64),
    /// A floating point literal value.
    Float(f64),
}

/// During building of the `AST`, the methods return either a new `Node` to
/// attach to the `AST`, or a `ParseError`.
pub type ASTResult<'token> = ParseResult<MaybeNode<'token>>;

impl From<ParseError> for ASTResult<'_> {
    /// For brevity, allow conversion of a `ParseError` to an `ASTResult`,
    /// i.e. `Result<Err(ParseError)>`.
    fn from(parse_error: ParseError) -> Self {
        Err(parse_error)
    }
}

impl<'token> From<Node<'token>> for ASTResult<'token> {
    fn from(node: Node<'token>) -> Self {
        Ok(Some(node))
    }
}

impl<'token> From<Token<'token>> for ASTResult<'token> {
    fn from(token: Token<'token>) -> Self {
        Self::from(Node::from(token))
    }
}

//------------------------------------------------------------------------------

impl Default for Node<'_> {
    fn default() -> Self {
        Self {
            kind: NodeKind::Void,
            token: None,
            is_static: true,
        }
    }
}

impl<'token> Node<'token> {
    pub fn new_expr(
        left: Node<'token>,
        oper: Token<'token>,
        right: Node<'token>,
    ) -> Self {
        Self {
            // the expression can only be static if *both* sides
            // of the expression are also static
            is_static: left.is_static && right.is_static,
            kind: NodeKind::Expr(Box::new(Expr::new(left, &oper, right))),
            token: Some(oper),
        }
    }

    /// Returns a node that defines a new Atom. There is no single token that
    /// does this because the use of a keyword and then Atom (e.g. "atom A"),
    /// meaning that you cannot just convert the token into a node like with
    /// the literals, e.g. `Node::from(token)`.
    pub fn new_atom(atom: Token<'token>) -> Self {
        Self {
            // create the Atom and embed it in the node
            kind: NodeKind::DefAtom(Atom::new(atom.as_str())),
            // store the reference back to the original source code;
            // this will be at the Atom name, not the "atom" keyword
            token: Some(atom),
            // node is static because it does not require name resolution
            is_static: true,
        }
    }
}

//==============================================================================

use crate::token::{Token, TokenKind};
use std::convert::From;

impl<'token> From<Token<'token>> for Node<'token> {
    fn from(token: Token<'token>) -> Node<'_> {
        Node {
            kind: match token.kind() {
                TokenKind::Int(i) => NodeKind::Value(Value::Int(i)),
                TokenKind::Hex(h) => NodeKind::Value(Value::UInt(h)),
                TokenKind::Bin(b) => NodeKind::Value(Value::UInt(b)),
                TokenKind::Atom(s) => NodeKind::Atom(s),
                TokenKind::Macro(s) => NodeKind::Macro(s),
                _ => panic!(
                    "Not a `Token` that can be converted into an `Node`."
                ),
            },
            // is this a static (literal) value?
            is_static: token.is_literal(),
            // embed the original token with the source-code location.
            // this'll be used if we need to print an error message
            token: Some(token),
        }
    }
}

//==============================================================================

use std::fmt::{self, *};

impl Display for Node<'_> {
    /// Pretty-prints a `Node` (and its descendants),
    /// essentially outputting normalised source code
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            NodeKind::Void => write!(f, "<VOID>"),
            NodeKind::DefAtom(ref a) => write!(f, "atom {}", a),
            NodeKind::List(ref l) => write!(f, "{}", l),
            NodeKind::Expr(ref x) => write!(f, "{}", x),
            NodeKind::Atom(ref a) => write!(f, "{}", a),
            NodeKind::Macro(ref m) => write!(f, "{}", m),
            NodeKind::Str(ref s) => write!(f, "{}", s),
            NodeKind::Value(ref v) => write!(f, "{}", v),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::UInt(u) => write!(f, "{}", u),
            Value::Float(d) => write!(f, "{}", d),
        }
    }
}
