// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use parser::Rule;
use pest::iterators::Pair;

/// A `Token` is a machine-understandable representation of one 'word'
/// (or "lexeme") of the original source code.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug)]
pub enum TokenKind {
    Atom(String),
    Str(String),
    Num(TokenKindNumber),
    Op(TokenKindOperator),
}

#[derive(Debug)]
pub enum TokenKindNumber {
    /// An integer number (signed)
    Int(i64),
    /// A floating-point number. All floating-point calculations are done with
    /// 64-bit floats to minimise rounding errors in intermediate calculations
    /// -- the assembly for the target system itself is likely to be Integer
    /// or 32-bit Float at best anyway
    Float(f64),
    Hex(u64),
    Bin(u64),
}

#[derive(Debug)]
pub enum TokenKindOperator {
    /// Addition operator "+"
    Add,
    /// Subtraction operator "-"
    Sub,
    /// Multiplication operator "*"
    Mul,
    /// Division operator "/"
    Div,
    /// Modulo operator "\\"
    Mod,
    /// Power/Exponention Operator "**"
    Pow,
    /// Bitwise eXclusive OR operator "^"
    Xor,
    /// Bitwise AND operator "&"
    And,
    /// Bitwise OR operator "|"
    Or,
    /// Bitwise SHift-Left operator "<<"
    Shl,
    /// Bitwise SHift-Right operator ">>"
    Shr,
}

/// Allow the direct conversion of Pest's `Pair`s into our `Token`s.
/// This removes a lot of logic from walking the `Pair`s.
impl<'i> From<Pair<'i, Rule>> for Token {
    fn from(pair: Pair<'i, Rule>) -> Self {
        // get the starting position of the token for line / col number;
        // this will get passed all the way through even the AST so that
        // accurate error information can be given even late into assembling
        let span = pair.clone().into_span();
        let start = span.start_pos();
        //TODO: is this very costly? should we defer this until called?
        let (line, col) = start.line_col();

        Token {
            kind: TokenKind::from(pair),
            line: line as u32,
            col: col as u32,
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for TokenKind {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::atom => TokenKind::Atom(pair.to_string()),
            Rule::int_number => TokenKind::Num(TokenKindNumber::Int(
                pair.as_str().parse::<i64>().unwrap(),
            )),
            Rule::hex_number => TokenKind::Num(TokenKindNumber::Hex(
                // create an unsigned 64-bit Int from a string...
                u64::from_str_radix(
                    // ignore the first character ("$")
                    &pair.as_str()[1..],
                    16, //=hexadecimal
                ).unwrap(),
            )),
            Rule::bin_number => TokenKind::Num(TokenKindNumber::Bin(
                u64::from_str_radix(
                    // ignore the first character ("%")
                    &pair.as_str()[1..],
                    2, //=binary
                ).unwrap(),
            )),
            Rule::op_add => TokenKind::Op(TokenKindOperator::Add),
            Rule::op_sub => TokenKind::Op(TokenKindOperator::Sub),
            Rule::op_mul => TokenKind::Op(TokenKindOperator::Mul),
            Rule::op_div => TokenKind::Op(TokenKindOperator::Div),
            Rule::op_mod => TokenKind::Op(TokenKindOperator::Mod),
            Rule::op_pow => TokenKind::Op(TokenKindOperator::Pow),
            Rule::op_xor => TokenKind::Op(TokenKindOperator::Xor),
            Rule::op_and => TokenKind::Op(TokenKindOperator::And),
            Rule::op_or => TokenKind::Op(TokenKindOperator::Or),
            Rule::op_shl => TokenKind::Op(TokenKindOperator::Shl),
            Rule::op_shr => TokenKind::Op(TokenKindOperator::Shr),
            _ => TokenKind::Atom(pair.to_string()),
        }
    }
}
