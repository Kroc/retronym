// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use std::io::{self, Write};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

#[derive(Parser)]
#[grammar = "retronym.pest"]
struct RymParser;

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

        io::stdin().read_line(&mut line).unwrap();
        // throw away the carriage-return
        line.pop();

        // parse the line given
        match RymParser::parse(Rule::root, &line) {
            Ok(pairs) => println!("{:?}", pairs),
            Err(e) => println!("\n{}", e)
        };
    }
}
