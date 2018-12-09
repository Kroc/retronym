// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! An Object file is a partially-compiled representation of a source code
//! file. Each Object *always* maps to one source file. Symbols not defined
//! in the file are "imports" to be linked against other Objects.

use crate::atom::Atoms;
use crate::table::Tables;

pub struct Object<'token> {
    ast: AST<'token>,
    atoms: Atoms,
    _tables: Tables<'token>,
}

use crate::ast::AST;
use crate::atom::Atom;
use crate::node::NodeKind;
use crate::record::Record;
use crate::table::Table;

impl<'token> Object<'token> {
    pub fn new_from_str(source: &'token str) -> Self {
        // create an AST from the source code
        let ast = AST::new_from_str(source);

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

        Self {
            ast: ast,
            atoms: Atoms::default(),
            _tables: Tables::default(),
        }
    }

    //TODO: will need a better name / location for this
    pub fn build(&mut self) {
        // create a data table to begin packing in
        let _table = Table::default();

        // create a first record to pack
        let _record = Record::default();

        // walk the AST nodes
        for n in self.ast.into_iter() {
            match &n.kind {
                // define an Atom
                NodeKind::DefAtom(atom) => {
                    self.atoms.insert(atom.to_string(), Atom::new(atom));
                }
                NodeKind::Value(_) => _record.add_data(n),
                _ => println!(": {}", n),
            }
        }
    }
}
