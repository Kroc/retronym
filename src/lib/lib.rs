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
pub mod list;
pub mod node;
pub mod object;
pub mod ops;
pub mod parser;
pub mod record;
pub mod segment;
pub mod table;
pub mod token;
pub mod tokenizer;

use crate::object::Object;

pub fn assemble_str(source: &str) {
    let _object = Object::new_from_str(source);
}
