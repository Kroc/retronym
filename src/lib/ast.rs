// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use crate::astnode::ASTNode;

/// The "Abstract Syntax Tree" is a machine understandable respresentation of
/// some source code. Because `ASTNode`s can contain a reference back to the
/// original source code (token) for errors, the `'token` lifetime is used
/// so that the source code is not deallocated before the AST.
pub struct AST<'token> {
    nodes: Vec<ASTNode<'token>>,
}

impl Default for AST<'_> {
    /// Gives you an empty AST structure.
    fn default() -> Self {
        AST { nodes: Vec::new() }
    }
}

use std::slice;

impl<'token> IntoIterator for &'token AST<'token> {
    type Item = &'token ASTNode<'token>;
    type IntoIter = slice::Iter<'token, ASTNode<'token>>;

    fn into_iter(self) -> slice::Iter<'token, ASTNode<'token>> {
        self.nodes.iter()
    }
}

//==============================================================================

use crate::parser::RymParser;

impl<'token> AST<'token> {
    pub fn new_from_str(source: &'token str) -> Self {
        // create a parser from the token stream;
        // this will output AST nodes
        let parser = RymParser::from_str(source);

        let mut ast = AST::default();

        // crank the parser and churn out ASTNodes
        for n in parser {
            match n {
                Ok(o) => match o {
                    Some(a) => ast.push(a),
                    None => break,
                },
                Err(e) => {
                    println!("! ERROR: {}", e);
                    // TODO: return an error
                    break;
                },
            }
        }
        
        for n in &ast {
            println!(": {}", n);
        }

        ast
    }

    pub fn push(&mut self, node: ASTNode<'token>) {
        self.nodes.push(node);
    }
}