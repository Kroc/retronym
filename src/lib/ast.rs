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

impl<'token> AST<'token> {
    pub fn eval(&self) {
        // create an iterator over the AST nodes;
        // we'll use this to process each statement in the AST
        let nodes = self.into_iter();

        //TODO:
        // - evaluate keywords by returning definitions+exports for the object
        //   i.e. macros and atoms need to be defined and exported for use in
        //   other objects
        //
        // - establish a default segment for relocating once the AST has been
        //   parsed into data tables
        //
        // - establish the default record type
        //
        // - read values. size those values based on the record type and build
        //   records
        //
        // - begin writing the records to the segment, using name resolution.
        //   names that cannot be resolved should be stored as an import for
        //   the segment -- these values can be resolved at linking
        //
        for n in nodes {
            // we need to determine if each statement is static or dynamic:
            //
            // - static statements require no outside information
            //   and can be flattened into a single value to output
            //
            // - dynamic statements cannot be calculated without outside
            //   information such as a macro, function call, imported symbol
            //   etc. since we cannot produce a value with these yet,
            //   store them with a reference to their AST node for later
            //   calculation
            //
            /*if n.is_static {
                // for nodes containing only static information, execute the
                // expression, folding the node (and children) down to a final
                // value
            }*/
            println!(": {}", n);
        }
    }
}
