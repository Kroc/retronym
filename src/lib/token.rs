// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Where possible we want to isolate the Pest-specific and grammar-specific
//! interaction from the AST. The `token` module wraps the core parsing output
//! of Pest, hiding the macro-generated `Rule`s from token consumers.

use crate::parser::pest::Rule;
use pest::iterators::Pair;
use pest::Span;

/// A `Token` is a single 'word' ("lexeme") of the source code. It's what's
/// known as a "new type" -- a single tuple struct -- because we cannot
/// implement our own functions on top of Pest's `Pair` as it's from an
/// external crate. We wrap the minimum amount of the interface for our
/// purposes.
#[derive(Clone, Debug)]
pub struct Token<'token>(Rule, Span<'token>);

/// A list of tokens.
pub type Tokens<'token> = Vec<Token<'token>>;

/// An optional Token.
pub type MaybeToken<'token> = Option<Token<'token>>;

impl<'t> From<Pair<'t, Rule>> for Token<'t> {
    // Wrap a Pest `Pair` into our `Token`.
    fn from(pair: Pair<'t, Rule>) -> Self {
        // awating shipping of tuple struct constructors:
        // https://github.com/rust-lang/rust/issues/51994
        Token(pair.as_rule(), pair.as_span())
    }
}

use crate::ptype::PType;

/// Describes the type of the token (and a parsed value, if possible),
/// without having to expose the internal Pest Rule.
pub enum TokenKind {
    /// Token is the "atom" keyword.
    KeywordAtom,
    /// Token is the "macro" keyword.
    KeywordMacro,
    /// Token is a primitive type.
    Type(PType),
    /// Token is an integer literal.
    Int(i64),
    /// Token is a hexadecimal literal.
    Hex(u64),
    /// Token is a binary literal.
    Bin(u64),
    /// Token is a floating-point literal.
    Float(f64),
    /// Token is an atom symbol.
    Atom(String),
    /// Token is a macro symbol.
    Macro(String),
    /// Token is a string literal.
    String(String),
    /// Token is the add "+" operator.
    OpAdd,
    /// Token is the subtract "-" operator.
    OpSub,
    /// Token is the multiply "*" operator.
    OpMul,
    /// Token is the divide "/" operator.
    OpDiv,
    /// Token is the modulo / remainder "//" operator.
    OpMod,
    /// Token is the power / exponention "**" operator.
    OpPow,
    /// Token is the binary xor "^" operator.
    OpXor,
    /// Token is the binary and "&" operator.
    OpAnd,
    /// Token is the binary or "|" operator.
    OpBor,
    /// Token is the shift-left "<<" operator.
    OpShl,
    /// Token is the shift-right ">>" operator.
    OpShr,
}

impl<'token> Token<'token> {
    /// Our lexer/parser, Pest, generates an enum, `Rule`, from the original
    /// grammar file. This method returns the `Rule` discriminant for the
    /// matched production, for example: `Rule::expr` for expressions.
    /// See "retronym.pest" for the grammar and therefore the `Rule` names.
    fn as_rule(&self) -> Rule {
        self.0
    }

    pub fn as_str(&self) -> &'token str {
        self.1.as_str()
    }
}

impl<'token> Token<'token> {
    pub fn kind(&self) -> TokenKind {
        match self.as_rule() {
            // keywords:
            Rule::keyword_atom => TokenKind::KeywordAtom,
            Rule::keyword_macro => TokenKind::KeywordMacro,
            // primitive types:
            Rule::type_bool => TokenKind::Type(PType::BOOL),
            Rule::type_nybl => TokenKind::Type(PType::NYBL),
            Rule::type_byte => TokenKind::Type(PType::BYTE),
            Rule::type_word => TokenKind::Type(PType::WORD),
            Rule::type_long => TokenKind::Type(PType::LONG),
            Rule::type_quad => TokenKind::Type(PType::QUAD),
            // literals:
            Rule::int_number => {
                TokenKind::Int(i64::from_str_radix(&self.as_str(), 16).unwrap())
            }
            Rule::hex_number => TokenKind::Hex(
                // note that we have to drop the sigil. limitations in
                // Pest make this difficult to do at the grammar level
                u64::from_str_radix(&self.as_str()[1..], 16).unwrap(),
            ),
            Rule::bin_number => TokenKind::Bin(
                // note that we have to drop the sigil. limitations in
                // Pest make this difficult to do at the grammar level
                u64::from_str_radix(&self.as_str()[1..], 2).unwrap(),
            ),
            Rule::string => TokenKind::String(self.as_str().to_string()),
            Rule::atom => TokenKind::Atom(self.as_str().to_string()),
            Rule::macro_ => TokenKind::Macro(self.as_str().to_string()),
            // operators:
            Rule::op_add => TokenKind::OpAdd,
            Rule::op_sub => TokenKind::OpSub,
            Rule::op_mul => TokenKind::OpMul,
            Rule::op_div => TokenKind::OpDiv,
            Rule::op_mod => TokenKind::OpMod,
            Rule::op_pow => TokenKind::OpPow,
            Rule::op_xor => TokenKind::OpXor,
            Rule::op_and => TokenKind::OpAnd,
            Rule::op_bor => TokenKind::OpBor,
            Rule::op_shl => TokenKind::OpShl,
            Rule::op_shr => TokenKind::OpShr,
            // Pest rules that do not translate to tokens, e.g. `EOI`
            _ => panic!(
                "Token not of a type that could be translated to a TokenKind!"
            ),
        }
    }
}

use std::fmt::{self, *};

impl<'token> Display for Token<'token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'t> Token<'t> {
    /// Is this token a keyword? ("atom", "macro"). This doesn't include the
    /// type-names ("byte", "word", "long" &c.) because those are recognised
    /// separately and not bundled in with keywords.
    pub fn is_keyword(&self) -> bool {
        match self.as_rule() {
            Rule::keyword_atom | Rule::keyword_macro => true,
            _ => false,
        }
    }

    pub fn is_keyword_atom(&self) -> bool {
        match self.as_rule() {
            Rule::keyword_atom => true,
            _ => false,
        }
    }

    pub fn is_keyword_macro(&self) -> bool {
        match self.as_rule() {
            Rule::keyword_macro => true,
            _ => false,
        }
    }

    /// Is this token a type name?
    pub fn is_type(&self) -> bool {
        match self.as_rule() {
            Rule::type_bool
            | Rule::type_nybl
            | Rule::type_byte
            | Rule::type_word
            | Rule::type_long
            | Rule::type_quad => true,
            _ => false,
        }
    }

    /// Is this an Atom?
    pub fn is_atom(&self) -> bool {
        match self.as_rule() {
            Rule::atom => true,
            _ => false,
        }
    }

    /// Is this a Macro?
    pub fn is_macro(&self) -> bool {
        match self.as_rule() {
            Rule::macro_ => true,
            _ => false,
        }
    }

    /// Is this a number literal?
    pub fn is_number(&self) -> bool {
        match self.as_rule() {
            Rule::int_number | Rule::hex_number | Rule::bin_number => true,
            _ => false,
        }
    }

    /// Is this a string literal?
    pub fn is_string(&self) -> bool {
        match self.as_rule() {
            Rule::string => true,
            _ => false,
        }
    }

    /// Is this a valid opening token for an expression? This wouldn't include
    /// operators because an expression cannot begin with an operator.
    pub fn is_expr(&self) -> bool {
        match self.as_rule() {
            Rule::atom
            | Rule::int_number
            | Rule::hex_number
            | Rule::bin_number => true,
            _ => false,
        }
    }

    /// Is this a 'value' -- i.e. a token that can return a value.
    /// This would not include macros as they are statements and not values,
    /// nor strings as they are self-contained lists and not an individual
    /// value.
    pub fn is_value(&self) -> bool {
        match self.as_rule() {
            Rule::int_number | Rule::hex_number | Rule::bin_number => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self.as_rule() {
            Rule::int_number
            | Rule::hex_number
            | Rule::bin_number
            | Rule::string => true,
            _ => false,
        }
    }

    /// Is this an operator?
    pub fn is_oper(&self) -> bool {
        match self.as_rule() {
            Rule::op_pow
            | Rule::op_add
            | Rule::op_sub
            | Rule::op_mul
            | Rule::op_div
            | Rule::op_mod
            | Rule::op_xor
            | Rule::op_and
            | Rule::op_bor
            | Rule::op_shl
            | Rule::op_shr => true,
            _ => false,
        }
    }
}
