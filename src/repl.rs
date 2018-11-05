// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use object::Object;
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

        let _object = Object::new_from_str(&line);
    }
}
