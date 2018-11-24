// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::parser::Rule;
use pest::iterators::Pair;

/// A `Token` is a single 'word' ("lexeme") of the source code.
pub type Token<'t> = Pair<'t, Rule>;

use parser::ast::ASTData;
use std::convert::From;

impl<'t> From<Token<'t>> for ASTData {
    /// Convert a `Token` to a literal value (if possible) such as stored in
    /// each `ASTNode`.
    fn from(token: Token) -> ASTData {
        match token.as_rule() {
            // parse an integer number:
            Rule::int_number => ASTData::Int(
                // parse the text as an integer number
                token.as_str().parse::<i64>().unwrap(),
            ),
            // parse a hexadecimal number:
            Rule::hex_number => ASTData::Int(
                // note that we have to drop the sigil. limitations in
                // Pest make this difficult to do at the grammar level
                i64::from_str_radix(&token.as_str()[1..], 16).unwrap(),
            ),
            // parse a binary number:
            Rule::bin_number => ASTData::Int(
                i64::from_str_radix(&token.as_str()[1..], 2).unwrap(),
            ),
            _ => ASTData::None,
        }
    }
}
