// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Source code in text is transformed into an Abstract Syntax Tree. This is a
//! structure that does away with the text of the source code and turns it into
//! discrete types that we can quickly recognise without having to read out
//! words letter-by-letter any more.

use crate::node::Node;
use crate::list::List;

/// The "Abstract Syntax Tree" is a machine understandable respresentation of
/// some source code. Because AST nodes can contain a reference back to the
/// original source code (token) for errors, the `'token` lifetime is used
/// so that the source code is not deallocated before the AST.
pub struct AST<'token> {
    nodes: List<'token>,
}

impl Default for AST<'_> {
    /// Gives you an empty AST structure.
    fn default() -> Self {
        AST { nodes: List::default() }
    }
}

use std::slice;

impl<'token> IntoIterator for &'token AST<'token> {
    type Item = &'token Node<'token>;
    type IntoIter = slice::Iter<'token, Node<'token>>;

    fn into_iter(self) -> slice::Iter<'token, Node<'token>> {
        self.nodes.into_iter()
    }
}

//==============================================================================

use crate::parser::Parser;

impl<'token> AST<'token> {
    pub fn new_from_str(source: &'token str) -> Self {
        let p = Parser::from_str(source);

        let mut ast = AST::default();

        for n in p {
            match n {
                Ok(o) => match o {
                    Some(a) => ast.push(a),
                    None => break,
                },
                Err(e) => {
                    println!("! ERROR: {}", e);
                    // TODO: return an error
                    break;
                }
            }
        }

        ast
    }

    fn push(&mut self, node: Node<'token>) {
        self.nodes.push(node);
    }
}
