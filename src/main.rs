// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Retronym is a thoroughly modern assembler for retro consoles
//! and computer systems.

// Pest:
//------------------------------------------------------------------------------

extern crate pest;
#[macro_use]
extern crate pest_derive;

//==============================================================================

pub mod error;
pub mod repl;

pub mod object;
pub mod parser;

//==============================================================================

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
