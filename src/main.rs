// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Retronym is a thoroughly modern assembler for retro consoles
//! and computer systems.

//==============================================================================

pub mod repl;

//==============================================================================

use std::io;
use std::io::Write;

fn main() {
    println!();
    println!("Retronym (C) copryright Kroc Camen 2017, 2018");
    println!("BSD 2-clause licence; see LICENSE.TXT");
    println!();
    io::stdout().flush().unwrap();

    repl::repl();
}
