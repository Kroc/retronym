// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use std::io::{self, Write};

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

        // get user input
        io::stdin().read_line(&mut line).unwrap();
        // (throw away the carriage-return)
        line.pop();

        // parse the line given
        match RymParser::parse(Rule::root, &line) {
            Ok(pairs) => dump_pairs(pairs),
            Err(e) => println!("\n{}", e),
        };
    }
}

// pretty print the contents of Pairs.
// I'm sure we can implement the Debug trait to do this more cleanly,
// but I don't know how to do that just yet
fn dump_pairs(pairs: Pairs<Rule>) {
    // loop over our Pairs
    for pair in pairs.flatten() {
        // a pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            println!("= {:?}", inner_pair.as_rule());

            let inner_span = inner_pair.clone().into_span();

            match inner_pair.as_rule() {
                Rule::atom => println!("atom: {}", inner_span.as_str()),
                Rule::int_number => println!("int:  {}", inner_span.as_str()),
                Rule::bin_number => println!("bin:  {}", inner_span.as_str()),
                Rule::hex_number => println!("hex:  {}", inner_span.as_str()),
                _ => println!("  \"{}\"", inner_span.as_str()),
            };
        }
    }
}
