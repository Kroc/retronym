// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Retronym is a thoroughly modern assembler for retro consoles
//! and computer systems.

extern crate pest;
#[macro_use]
extern crate pest_derive;
/// We're using Pest as our parser and don't need to expose this as our own
mod parser;

#[macro_use]
extern crate errln;

pub mod repl;
pub mod token;
pub mod tokenstream;

use std::io;
use std::io::Write;

fn main() {
    println!("");
    println!("Retronym (C) copryright Kroc Camen 2017, 2018");
    println!("BSD 2-clause licence; see LICENSE.TXT");
    println!("");
    io::stdout().flush().unwrap();

    repl::repl();
}
