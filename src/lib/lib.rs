// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Retronym is a thoroughly modern assembler for retro consoles and computer
//! systems. Write apps and games for 8-bit systems with the flexibility of
//! a real module system (no `include` heirarchies!).

// Parsing provided by Pest: https://pest.rs/
#[macro_use]
extern crate pest_derive;

pub mod assembler;
pub mod ast;
pub mod atom;
pub mod error;
pub mod expr;
pub mod field;
pub mod list;
pub mod node;
pub mod object;
pub mod ops;
pub mod parser;
pub mod primitive;
pub mod record;
pub mod segment;
pub mod r#struct;
pub mod table;
pub mod token;
pub mod tokenizer;

use crate::assembler::Assembler;

pub fn assemble_str(source: &str) {
    //--------------------------------------------------------------------------
    Assembler::assemble_str(source);
}
