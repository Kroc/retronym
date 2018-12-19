// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! **Assembler** orchestrates the assembly process.

#[derive(Default)]
pub struct Assembler<'token> {
    _objects: Vec<Object<'token>>,
}

use crate::ast::AST;
use crate::error::*;
use crate::node::{Node, NodeIter, NodeKind};
use crate::object::Object;
use crate::r#struct::Struct;
use crate::table::TableBuilder;

impl<'token> Assembler<'token> {
    //==========================================================================
    /// Assembles from a string source.
    ///
    pub fn assemble_str(source: &'token str) -> Self {
        //----------------------------------------------------------------------
        //TODO:
        // - evaluate keywords by returning definitions+exports for the object
        //   i.e. macros and atoms need to be defined and exported for use in
        //   other objects
        //
        // - modules! a file can import other modules, requiring these to be
        //   turned into Objects too. Handled by linker only?
        //
        // - macros *must* be expanded before packing -- we can't know how many
        //   list items a macro generates; therefore object files are bound to
        //   the specific choice of macros they import (modules?)
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

        // create an AST from the source code. this walks the whole source
        // code and builds representative nodes but does *not* resolve symbols
        // -- therefore actual structs / records and so on are not built until
        // all referenced files have also been parsed and we have a complete
        // pool of symbol names to draw from

        let asm = Assembler::default();
        let ast = AST::new_from_str(source);
        asm.assemble_ast(&ast);
        asm
    }

    /// Assembles from an `AST`.
    ///
    pub fn assemble_ast(&self, ast: &AST) {
        //----------------------------------------------------------------------
        // create the Object we'll be placing the assembled resources into.
        // during assembly, new objects might be created (module references)
        let mut object = Object::default();

        // create a Node iterator from the AST
        let mut node_iter = ast.into_iter();

        let node = node_iter.next().unwrap();
        self.assemble_root(&mut object, &mut node_iter, node);
    }

    /// Begins assembly at the 'root scope', that is, statements at the
    /// beginning of a source file, before any nesting of statements.
    ///
    /// Results of the assembly are placed into the given `Object`,
    /// this method only returns an error if one occurred.
    ///
    fn assemble_root(
        &self,
        object: &'token mut Object<'token>,
        node_iter: &'token mut NodeIter<'token>,
        node: &'token Node<'token>,
    ) -> MaybeError {
        //----------------------------------------------------------------------
        if node.is_atom_def() {
            // define a new Atom
            return object.new_atom(node);
        }
        if node.is_record() {
            // return error if there was one
            self.assemble_table(object, node_iter, node).err();
        }

        None
    }

    /// Pack data into a Table.
    ///
    fn assemble_table(
        &self,
        _object: &'token mut Object<'token>,
        _node_iter: &'token mut NodeIter<'token>,
        node: &'token Node<'token>,
    ) -> ParseResult<&'token Node<'token>> {
        //----------------------------------------------------------------------
        // the record must come first -- we can't pack data without knowing
        // what the fields are!

        // the record in the AST is stored un-resolved; it's just a List of AST
        // nodes and not a firm structure yet. This is done for the purpose of
        // delaying the resolving of nested structures in the Record; these
        // might be defined in other source code files!
        let list = match &node.kind {
            // from a `Box<List>`, get `&List`
            NodeKind::Record(list) => list.as_ref(),
            _ => panic!(),
        };

        let record = Struct::from(list);

        // start up a TableBuilder with the Record we now have
        let _builder = TableBuilder::new(&record);

        Ok(node)
    }
}
