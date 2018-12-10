// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! An Object file is a partially-compiled representation of a source code
//! file. Each Object *always* maps to one source file. Symbols not defined
//! in the file are "imports" to be linked against other Objects.

use crate::ast::AST;
use crate::atom::Atoms;
use crate::table::Tables;

pub struct Object<'token> {
    _ast: AST<'token>,
    _atoms: Atoms,
    _tables: Tables<'token>,
}
