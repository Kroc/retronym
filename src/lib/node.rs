// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! The AST `Node` is the core representation of source code in the program,
//! it uses a different enum type to differentiate numbers, strings, macros
//! and so on. AST nodes can contain other nodes, such as with expressions.

use crate::error::*;
use crate::token::MaybeToken;

/// The AST is made up of a series of nodes where each node is a top-level
/// "statement" and may contain descendants based on type. In practice,
/// Retronym's top-level statements are either macros or expressions.
#[derive(Debug)]
pub struct Node<'token> {
    /// The 'type' of the node, e.g. whether this is a literal number,
    /// an expression, a macro invocation etc. This can contain nested nodes!
    pub kind: ASTKind<'token>,
    /// An optional reference back to the original source code,
    /// for error messages.
    pub token: MaybeToken<'token>,
    /// A node is static if it, and any descendents, contain only literal
    /// values that can be calculated without outside information.
    pub is_static: bool,
}

pub type MaybeNode<'token> = Option<Node<'token>>;

use crate::expr::Expr;

#[derive(Debug)]
pub enum ASTKind<'token> {
    /// An empty node, used for unimplemented node types.
    Void,
    /// An experssion -- i.e. a calculation
    Expr(Box<Expr<'token>>),
    /// An atom.
    Atom(String),
    /// A macro invocation.
    Macro(String),
    /// A string literal. Since strings are self-contained lists, these are
    /// not treated as expression values.
    Str(String),
    /// A literal value.
    Value(ASTValue),
}

#[derive(Debug)]
pub enum ASTValue {
    /// An integer literal value.
    Int(i64),
    /// A floating point literal value.
    Float(f64),
}

#[derive(Debug)]
pub enum Operator {
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
    Bor,
    /// Bitwise SHift-Left operator "<<"
    Shl,
    /// Bitwise SHift-Right operator ">>"
    Shr,
}

/// During building of the `AST`, the methods return either a new `Node` to
/// attach to the `AST`, or a `ParseError`.
pub type ASTResult<'token> = ParseResult<MaybeNode<'token>>;

impl From<ParseError> for ASTResult<'_> {
    /// For brevity, allow conversion of a ParseError to an ASTResult,
    /// i.e. `Result<Err(ParseError)>`.
    fn from(parse_error: ParseError) -> Self {
        Err(parse_error)
    }
}

impl<'token> From<Node<'token>> for ASTResult<'token> {
    fn from(ast_node: Node<'token>) -> Self {
        Ok(Some(ast_node))
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
            kind: ASTKind::Void,
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
            kind: ASTKind::Expr(Box::new(Expr::new(left, &oper, right))),
            token: Some(oper),
        }
    }
}

//==============================================================================

use crate::parser::Rule;
use crate::token::Token;
use std::convert::From;

impl<'token> From<Token<'token>> for Node<'token> {
    fn from(token: Token<'token>) -> Node<'_> {
        Node {
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
                // an atom is returned as a string
                Rule::atom => ASTKind::Atom(
                    //TODO: messy
                    token.as_str().to_string(),
                ),
                // a macro is returned as a string
                Rule::mac => ASTKind::Macro(
                    //TODO: messy
                    token.as_str().to_string(),
                ),
                _ => panic!(
                    "Not a `Token` that can be converted into an `Node`."
                ),
            },
            // is this a static (literal) value?
            is_static: match token.as_rule() {
                // numbers and strings need no dynamic calculation
                Rule::int_number | Rule::hex_number | Rule::bin_number => true,
                Rule::string => true,
                // atoms & macros require name-resolution
                _ => false,
            },
            // embed the original token with the source-code location.
            // this'll be used if we need to print an error message
            token: Some(token),
        }
    }
}

impl From<&Token<'_>> for Operator {
    /// Convert a token into an `Operator` enum.
    /// Panics if using a token that is not an operator!
    fn from(token: &Token<'_>) -> Self {
        match token.as_rule() {
            Rule::op_add => Operator::Add,
            Rule::op_sub => Operator::Sub,
            Rule::op_mul => Operator::Mul,
            Rule::op_div => Operator::Div,
            Rule::op_mod => Operator::Mod,
            Rule::op_pow => Operator::Pow,
            Rule::op_xor => Operator::Xor,
            Rule::op_and => Operator::And,
            Rule::op_bor => Operator::Bor,
            Rule::op_shl => Operator::Shl,
            Rule::op_shr => Operator::Shr,
            _ => panic!("Not an operator token!"),
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
            ASTKind::Void => write!(f, "<VOID>"),
            ASTKind::Expr(ref x) => write!(f, "{}", x),
            ASTKind::Atom(ref a) => write!(f, "{}", a),
            ASTKind::Macro(ref m) => write!(f, "{}", m),
            ASTKind::Str(ref s) => write!(f, "{}", s),
            ASTKind::Value(ref v) => write!(f, "{}", v),
        }
    }
}

impl Display for ASTValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTValue::Int(i) => write!(f, "{}", i),
            ASTValue::Float(d) => write!(f, "{}", d),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Div => "/",
                Operator::Mod => r"\\",
                Operator::Pow => "**",
                Operator::Xor => "^",
                Operator::And => "&",
                Operator::Bor => "|",
                Operator::Shl => "<<",
                Operator::Shr => ">>",
            }
        )
    }
}