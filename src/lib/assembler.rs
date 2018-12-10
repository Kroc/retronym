// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! **Assembler** orchestrates the assembly process.

pub struct Assembler {}

use crate::ast::AST;

impl Assembler {
    /// Assembles from a string source.
    pub fn assemble_str(source: &str) {
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

        // walk the AST nodes
        for n in ast.into_iter() {
            match &n.kind {
                /*
                // define an Atom
                NodeKind::DefAtom(atom) => {
                    self.atoms.insert(atom.to_string(), Atom::new(atom));
                }
                */
                _ => println!(": {}", n),
            }
        }
    }
}
