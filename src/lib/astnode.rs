// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::error::*;
use crate::token::MaybeToken;

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
    /// A node is static if it, and any descendents, contain only literal
    /// values that can be calculated without outside information.
    pub is_static: bool,
}

pub type MaybeASTNode<'token> = Option<ASTNode<'token>>;

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
pub struct ASTExpr<'token> {
    pub left: ASTNode<'token>,
    pub oper: ASTOperator,
    pub right: ASTNode<'token>,
}

impl<'token> ASTExpr<'token> {
    fn new(
        left: ASTNode<'token>,
        oper: &Token<'token>,
        right: ASTNode<'token>,
    ) -> Self {
        ASTExpr {
            // left hand side:
            left: left,
            // convert op token to op enum:
            oper: ASTOperator::from(oper),
            // right hand side:
            right: right,
        }
    }
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
}

/// During building of the `AST`, the methods return either a new `ASTNode` to
/// attach to the `AST`, or a `ParseError`.
pub type ASTResult<'token> = ParseResult<MaybeASTNode<'token>>;

impl From<ParseError> for ASTResult<'_> {
    /// For brevity, allow conversion of a ParseError to an ASTResult,
    /// i.e. `Result<Err(ParseError)>`.
    fn from(parse_error: ParseError) -> Self {
        Err(parse_error)
    }
}

impl<'token> From<ASTNode<'token>> for ASTResult<'token> {
    fn from(ast_node: ASTNode<'token>) -> Self {
        Ok(Some(ast_node))
    }
}

impl<'token> From<Token<'token>> for ASTResult<'token> {
    fn from(token: Token<'token>) -> Self {
        Self::from(ASTNode::from(token))
    }
}

//------------------------------------------------------------------------------

impl Default for ASTNode<'_> {
    fn default() -> Self {
        Self {
            kind: ASTKind::Void,
            token: None,
            is_static: true,
        }
    }
}

impl<'token> ASTNode<'token> {
    pub fn new_expr(
        left: ASTNode<'token>,
        oper: Token<'token>,
        right: ASTNode<'token>,
    ) -> Self {
        Self {
            // the expression can only be static if *both* sides
            // of the expression are also static
            is_static: left.is_static && right.is_static,
            kind: ASTKind::Expr(Box::new(ASTExpr::new(left, &oper, right))),
            token: Some(oper),
        }
    }
}

//==============================================================================

use crate::parser::Rule;
use crate::token::Token;
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
                    "Not a `Token` that can be converted into an `ASTNode`."
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

impl From<&Token<'_>> for ASTOperator {
    /// Convert a token into an `ASTOperator` enum.
    /// Panics if using a token that is not an operator!
    fn from(token: &Token<'_>) -> Self {
        match token.as_rule() {
            Rule::op_add => ASTOperator::Add,
            Rule::op_sub => ASTOperator::Sub,
            Rule::op_mul => ASTOperator::Mul,
            Rule::op_div => ASTOperator::Div,
            Rule::op_mod => ASTOperator::Mod,
            Rule::op_pow => ASTOperator::Pow,
            Rule::op_xor => ASTOperator::Xor,
            Rule::op_and => ASTOperator::And,
            Rule::op_or => ASTOperator::Or,
            Rule::op_shl => ASTOperator::Shl,
            Rule::op_shr => ASTOperator::Shr,
            _ => panic!("Not an operator token!"),
        }
    }
}

//==============================================================================

use std::fmt::{self, *};

impl Display for ASTNode<'_> {
    /// Pretty-prints an ASTNode (and its descendants),
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

impl<'token> Display for ASTExpr<'token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: ASTOperator to string
        write!(f, "({} {} {})", self.left, self.oper, self.right)
    }
}

impl Display for ASTOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ASTOperator::Add => "+",
                ASTOperator::Sub => "-",
                ASTOperator::Mul => "*",
                ASTOperator::Div => "/",
                ASTOperator::Mod => r"\\",
                ASTOperator::Pow => "**",
                ASTOperator::Xor => "^",
                ASTOperator::And => "&",
                ASTOperator::Or => "|",
                ASTOperator::Shl => "<<",
                ASTOperator::Shr => ">>",
            }
        )
    }
}

//==============================================================================

pub enum ASTFoldResult {
    // Result of the fold is an integer.
    Int(i64),
    // Result of the fold is a float.
    Float(f64),
    //TODO: result of a dyanmic expression should be a relaxtion joint
    //      or some such deferred calculation
}

impl ASTNode<'_> {
    fn _fold(&self) -> ASTFoldResult {
        match &self.kind {
            ASTKind::Value( v ) => match v {
                ASTValue::Int( i ) => ASTFoldResult::Int(*i),
                ASTValue::Float( d ) => ASTFoldResult::Float(*d),
            }
            _ => unimplemented!()
        }
    }
}

impl ASTExpr<'_> {
    fn _fold(&self) -> ASTFoldResult {
        // we need to check if the expression is static or dynamic:
        //
        // - static expressions require no outside information
        //   and can be flattened into a single value to output
        //
        // - dynamic expressions cannot be calculated without outside
        //   information such as a function call, imported symbol etc.
        //   since we cannot produce a value with these yet, store them
        //   with a reference to their AST node for later calculation
        //

        unimplemented!()
    }
}