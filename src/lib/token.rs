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

pub enum TokenKind {
    KeywordAtom,
    KeywordMacro,
    Int,
    Hex,
    Bin,
    Float,
    Atom,
    Macro,
    String,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpPow,
    OpXor,
    OpAnd,
    OpBor,
    OpShl,
    OpShr,
}

impl<'t> Token<'t> {
    /// Our lexer/parser, Pest, generates an enum, `Rule`, from the original
    /// grammar file. This method returns the `Rule` discriminant for the
    /// matched production, for example: `Rule::expr` for expressions.
    /// See "retronym.pest" for the grammar and therefore the `Rule` names.
    fn as_rule(&self) -> Rule {
        self.0
    }

    pub fn as_str(&self) -> &'t str {
        self.1.as_str()
    }

    pub fn kind(&self) -> TokenKind {
        match self.as_rule() {
            Rule::keyword_atom => TokenKind::KeywordAtom,
            Rule::keyword_macro => TokenKind::KeywordMacro,
            Rule::int_number => TokenKind::Int,
            Rule::hex_number => TokenKind::Hex,
            Rule::bin_number => TokenKind::Bin,
            Rule::atom => TokenKind::Atom,
            Rule::mac => TokenKind::Macro,
            Rule::string => TokenKind::String,
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
            _ => unimplemented!(),
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
    /// Is this a keyword?
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
            Rule::mac => true,
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
