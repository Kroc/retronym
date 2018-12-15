// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! **Assembler** orchestrates the assembly process.

pub struct Assembler<'token> {
    _objects: Vec<Object<'token>>,
}

use crate::ast::AST;
use crate::field::Field;
use crate::object::Object;
use crate::primitive::Primitive;
use crate::r#struct::Struct;
use crate::table::Table;

impl<'token> Assembler<'token> {
    //==========================================================================
    /// Assembles from a string source.
    ///
    pub fn assemble_str(source: &str) {
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
        // TODO: error handling
        Self::assemble_ast(AST::new_from_str(source));
    }

    /// Assembles from an `AST`.
    ///
    pub fn assemble_ast(ast: AST) {
        //----------------------------------------------------------------------
        // create the Object we'll be placing the assembled resources into.
        // during assembly, new objects might be created (module references)
        let mut object = Object::default();

        // create the initial record type;
        // this dictates how data will be packed into tables
        let mut record = Struct::default();
        // the default record-type is a single byte (for now)
        record.add_field(Field::from(Primitive::BYTE));

        // create the initial data table,
        // and apply the record-type to it
        let mut table = Table::from(&record);

        // TODO: macro expansion pass? we still need to think about how macros
        // will consume elements ahead of themselves

        // walk the AST nodes
        for n in ast.into_iter() {
            match () {
                // define a new atom:
                _ if n.is_atom_def() => object.new_atom(n),
                // TODO: a record-list needs to be compiled into a record-type
                // data to be packed:
                _ if n.is_data() => table.add_data(n),
                // unhandled!
                _ => println!(": {}", n),
            }
        }

        table.end();

        println!("{}", table);
    }
}
