// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::*;
use tokenizer::tokenstream::TokenStream;

/// The "Abstract Syntax Tree" puts `Token`s together into meaningful
/// relationships. Whilst the `TokenStream` only cares about the type of
/// individual `Token`s, the AST recognises lists, expressions and other
/// such multi-token structures.
pub struct AST {
    _nodes: Vec<ASTNode>,
}

/// The AST is made up of a series of interconnected Nodes.
pub struct ASTNode {
    _kind: ASTNodeKind,
}

pub enum ASTNodeKind {
    /// An Atom definition, e.g. `atom A`
    AtomDef,
    /// A Macro definition, e.g. `macro lda { ... }`
    MacroDef,
    /// A list of elements
    List,
    /// An experssion -- i.e. a calculation that generates a Value
    Expr,
    /// A single numerical value
    Value,
}

impl TryFrom_<TokenStream> for AST {
    /// Convert a `TokenStream` to an `AST`. Returns a `Result` containing
    /// either a new `AST` object, or an `Error`.
    fn try_from_(mut tokenstream: TokenStream) -> Result<Self> {
        // create a blank AST to begin working with
        let ast = AST { _nodes: Vec::new() };

        // ensure the `TokenStream` is at the start
        tokenstream.rewind();

        // begin walking the `TokenStream`
        ast.walk_root(tokenstream);

        Ok(ast)
    }
}

impl AST {
    fn walk_root(&self, _tokenstream: TokenStream) {}
}
