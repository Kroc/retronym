// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

// build a parser using Pest:

use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/retronym.pest"]
pub struct RymParser<'p, R>{
    pairs: Pairs<'p, R>
}

use pest::iterators::Pairs;

impl<'p> RymParser<'p, Rule> {
    /// NB: the string reference must live as long as the `RymParser`;
    /// that is, the source string you pass it will not deallocate until
    /// the RymParser does as well.
    pub fn from_str(source: &'p str) -> Self {
        // use Pest to parse the source text
        let pairs = Self::parse(Rule::rym, &source).expect(
            "error parsing: {:#?}"
        );

        Self{
            pairs,
        }
    }
}

use parser::ast::ASTNode;

impl<'p> Iterator for RymParser<'p, Rule> {
    type Item = ASTNode;

    /// When you turn the crank on the parser, it spits out AST nodes.
    fn next(&mut self) -> Option<Self::Item> {
        Some(ASTNode::default())
    }
}