// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::error::*;

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

/// The AST is made up of a series of interconnected nNodes.
pub struct ASTNode {
    pub kind: ASTNodeKind,
}

pub enum ASTNodeKind {
    /// An empty node, used for unimplemented node types
    Void,
    /// An Atom definition, e.g. `atom A`
    AtomDef(String),
    /// A Macro definition, e.g. `macro lda { ... }`
    MacroDef,
    /// A list of elements
    List,
    /// An experssion -- i.e. a calculation that generates a Value
    Expr,
    /// A single numerical value
    Value,
}

/// During building of the `AST`, the methods return either a new `ASTNode` to
/// attach to the `AST`, or an `Error`.
pub type ASTResult = ParseResult<ASTNode>;
pub type MaybeASTResult = Option<ASTResult>;

impl ASTNode {
    /// Retuns a void AST Node.
    pub fn new_void() -> ASTNode {
        ASTNode {
            kind: ASTNodeKind::Void,
        }
    }

    /// Returns an AST node that defines a new Atom.
    pub fn new_atomdef(atom: &str) -> ASTNode {
        ASTNode {
            kind: ASTNodeKind::AtomDef(atom.to_string()),
        }
    }
}
