// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! The **AST Node** is the core representation of source code in the program,
//! it uses an enum to differentiate numbers, strings, macros and so on.
//! AST nodes can contain other nodes, such as with expressions.

use crate::token::MaybeToken;

/// The AST is made up of a series of nodes where each node is a top-level
/// "statement" and may contain descendants based on type. In practice,
/// Retronym's top-level statements are either macros or expressions.
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

use crate::expr::Expr;
use crate::list::List;
use crate::primitive::Primitive;

#[derive(Debug)]
pub enum NodeKind<'token> {
    /// An empty node.
    Void,
    /// An Atom definition. Defines a new Atom and exports it.
    /// When the final linking occurs, all Atoms used must be defined.
    DefAtom(String),
    /// A primitive type.
    /// `byte`, `word`, `long`, for example.
    Primitive(Primitive),
    /// A record type defines the fields for a table. At the AST level,
    /// any nested struct-types are likely undefined until all source files
    /// have been parsed, so the AST stores the record type as a `List` of
    /// `Nodes` and resolves the struct names later to build the actual
    /// record-type.
    Record(Box<List<'token>>),
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
    Int(i32),
    /// An unsigned literal value as used by binary and hexadecimal
    /// numbers to represent raw numbers up to 32-bits.
    UInt(u32),
    /// A floating point literal value.
    Float(f32),
}

impl Display for Value {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match self {
            Value::Int(i) => f.write_str(&i.to_string()),
            Value::UInt(u) => f.write_str(&u.to_string()),
            Value::Float(s) => f.write_str(&s.to_string()),
        }
    }
}

impl Default for Node<'_> {
    //==========================================================================
    fn default() -> Self {
        //----------------------------------------------------------------------
        Self {
            kind: NodeKind::Void,
            token: None,
            is_static: true,
        }
    }
}

impl<'token> Node<'token> {
    //==========================================================================
    pub fn new_expr(
        left: Node<'token>,
        oper: Token<'token>,
        right: Node<'token>,
    ) -> Self {
        //----------------------------------------------------------------------
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
    ///
    pub fn new_atom(atom: Token<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            // create the Atom and embed it in the node
            kind: NodeKind::DefAtom(atom.to_string()),
            // store the reference back to the original source code;
            // this will be at the Atom name, not the "atom" keyword
            token: Some(atom),
            // node is static because it does not require name resolution
            is_static: true,
        }
    }

    pub fn new_record(list: List<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            kind: NodeKind::Record(Box::new(list)),
            // the items in the list will have references to
            // their tokens, the list itself doesn't need one
            token: None,
            // TODO: the list can report if it is static to us
            is_static: false,
        }
    }

    /// Does this Node contain data? That is, a literal value, expression
    /// or a symbol that can be resolved into a value.
    ///
    pub fn is_data(&self) -> bool {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::Value(_) | NodeKind::Expr(_) | NodeKind::Str(_) => true,
            _ => false,
        }
    }

    /// Is this Node a Record?
    ///
    /// The AST only stores Records as a `List` because any struct-nesting
    /// can not be resolved until all structs are defined and these might
    /// be in other modules.
    ///
    pub fn is_record(&self) -> bool {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::Record(_) => true,
            _ => false,
        }
    }

    /// Is this Node an Atom definition?
    ///
    pub fn is_atom_def(&self) -> bool {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::DefAtom(_) => true,
            _ => false,
        }
    }

    /// Is this Node an Atom reference?
    ///
    pub fn is_atom(&self) -> bool {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::Atom(_) => true,
            _ => false,
        }
    }

    /// Is this Node a Macro invocation?
    ///
    pub fn is_macro(&self) -> bool {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::Macro(_) => true,
            _ => false,
        }
    }
}

//==============================================================================

use crate::token::{Token, TokenKind};
use std::convert::From;

impl<'token> From<Token<'token>> for Node<'token> {
    //==========================================================================
    fn from(token: Token<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            kind: match token.kind() {
                TokenKind::Primitive(p) => NodeKind::Primitive(p),
                TokenKind::Int(i) => NodeKind::Value(Value::Int(i)),
                TokenKind::Hex(h) => NodeKind::Value(Value::UInt(h)),
                TokenKind::Bin(b) => NodeKind::Value(Value::UInt(b)),
                TokenKind::Atom(s) => NodeKind::Atom(s),
                TokenKind::Macro(s) => NodeKind::Macro(s),
                _ => {
                    panic!("Not a `Token` that can be converted into a `Node`.")
                }
            },
            // is this a static (literal) value?
            is_static: token.is_static(),
            // embed the original token with the source-code location.
            // this'll be used if we need to print an error message
            token: Some(token),
        }
    }
}

use std::fmt::{self, *};

impl Display for Node<'_> {
    //==========================================================================
    /// Pretty-prints a `Node` (and its descendants),
    /// essentially outputting normalised source code
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::Void => write!(f, "<VOID>"),
            NodeKind::DefAtom(ref a) => write!(f, "atom {}", a),
            NodeKind::Primitive(ref p) => write!(f, "{}", p),
            NodeKind::Record(ref l) => write!(f, "{}", l),
            NodeKind::List(ref l) => write!(f, "{}", l),
            NodeKind::Expr(ref x) => write!(f, "{}", x),
            NodeKind::Atom(ref a) => write!(f, "{}", a),
            NodeKind::Macro(ref m) => write!(f, "{}", m),
            NodeKind::Str(ref s) => write!(f, "{}", s),
            NodeKind::Value(ref v) => write!(f, "{}", v),
        }
    }
}

impl Debug for Node<'_> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        match self.kind {
            NodeKind::Void => write!(f, "<VOID>"),
            NodeKind::DefAtom(ref a) => write!(f, "atom {:?}", a),
            NodeKind::Primitive(ref p) => write!(f, "{:?}", p),
            NodeKind::Record(ref l) => write!(f, "{:?}", l),
            NodeKind::List(ref l) => write!(f, "{:?}", l),
            NodeKind::Expr(ref x) => write!(f, "{:?}", x),
            NodeKind::Atom(ref a) => write!(f, "{:?}", a),
            NodeKind::Macro(ref m) => write!(f, "{:?}", m),
            NodeKind::Str(ref s) => write!(f, "{}", s),
            NodeKind::Value(ref v) => write!(f, "{}", v),
        }
    }
}
