// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

/// A `Token` is a machine-understandable representation of one 'word'
/// (or "lexeme") of the original source code.
pub struct Token {
    pub kind: TokenType,
    pub line: u32,
    pub col: u32,
}

pub enum TokenType {
    Atom(String),
    Str(String),
    Num(TokenTypeNumber),
}

pub enum TokenTypeNumber {
    /// An integer number (signed)
    TokenInt(i64),
    /// A floating-point number. All floating-point calculations are done with
    /// 64-bit floats to minimise rounding errors in intermediate calculations
    /// -- the assembly for the target system itself is likely to be Integer
    /// or 32-bit Float at best anyway
    TokenFloat(f64),
    TokenHex(u64),
    TokenBin(u64),
}