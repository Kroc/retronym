// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Where possible we want to isolate the Pest-specific and grammar-specific
//! interaction from the AST. The `token` module wraps the core parsing output
//! of Pest, hiding the macro-generated `Rule`s from token consumers.

use crate::parser::Rule;
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

impl<'t> Token<'t> {
    // Our lexer/parser, Pest, generates an enum, `Rule`, from the original
    // grammar file. This method returns the `Rule` discriminant for the
    // matched production, for example: `Rule::expr` for expressions.
    // See "retronym.pest" for the grammar and therefore the `Rule` names.
    pub fn as_rule(&self) -> Rule {
        self.0
    }

    pub fn as_str(&self) -> &'t str {
        self.1.as_str()
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
