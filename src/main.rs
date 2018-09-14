// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use std::io;
use std::io::Write;

pub mod token;
pub mod tokenizer;
use tokenizer::Tokenizer;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    // display REPL header
    println!("");
    println!("Retronym (C) copryright Kroc Camen 2017, 2018");
    println!("BSD 2-clause licence; see LICENSE.TXT");
    println!("");
    println!("(use ^C to quit)");
    println!("");
    io::stdout().flush().unwrap();

    // start up the REPL
    loop {
        let mut line = String::new();

        // display the prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // get user input
        io::stdin().read_line(&mut line).unwrap();

        // parse the line given
        match Tokenizer::tokenize_str(&line) {
            Ok(_) => (),
            Err(e) => println!("\n{}", e)
        }
    }
}