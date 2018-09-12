// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

/// A `Token` is a machine-understandable representation of one 'word'
/// (or "lexeme") of the original source code. A `Token` doesn't store the
/// original text, other than in the case of strings.
pub enum Token {
    TokenInt,
    TokenFloat,
    TokenHex,
    TokenBin,
    TokenString,
}

/// A `Token` for an integer number (signed)
pub struct TokenInt {
    value: i64
}
/// A `Token` for a floating-point number. All floating-point calculations
/// are done with 64-bit floats to minimise rounding errors in intermediate
/// calculations -- the assembly for the target system itself is likely to
/// to be Integer or 32-bit Float at best anyway
pub struct TokenFloat {
    value: f64
}
pub struct TokenHex {
    value: u64
}
pub struct TokenBin {
    value: u64
}
pub struct TokenString<'str> {
    value: &'str str
}
pub struct TokenAtom {
    value: i32
}