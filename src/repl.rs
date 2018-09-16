// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use tokenizer::Tokenizer;
use std::io;
use std::io::Write;

pub fn repl() {
    // display REPL header
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
            Err(e) => println!("\n{}", e),
        }
    }
}