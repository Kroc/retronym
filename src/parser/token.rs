// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::*;
use parser::parser::Rule;
use pest::iterators::Pair;

//------------------------------------------------------------------------------

/// A `Token` is a machine-understandable representation of one 'word'
/// (or "lexeme") of the original source code.
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub data: TokenData,
    pub line: u32,
    pub col: u32,
}

impl Default for Token {
    /// Create an empty `Token`
    fn default() -> Self {
        Self {
            kind: TokenKind::None,
            data: TokenData::None,
            line: 0,
            col: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenData {
    None,
    String(String),
    I64(i64),
    U64(u64),
    F64(f64),
}

/// As the source code is broken into `Token`s, we assign what "kind" each is
/// via an enum, so that the assembler can work with strict type information
/// rather than having to continually look at ASCII-codes throughout.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// Internally used for skipping sub-tokens and finding errors.
    None,

    /// A reserved keyword; there are very few because all CPU instruction
    /// mnemonics are implemented as macros.
    Keyword(TokenKindKeyword),

    /// An Atom is a unique word. It has no value and does not expand into
    /// anything (it's a terminal macro). Atoms are used to implement CPU
    /// registers, e.g. `adc A, B`. Atoms are defined by the `atom` keyword.
    /// E.g. `atom A`. Atoms must be upper-case.
    Atom(String),

    /// A Macro is a unique word that gets expanded into another block of code,
    /// defined elsewhere. For flexibility Retronym uses macros to implement
    /// the CPU instruction mnemonics. Macros must be lower-case.
    Macro(String),

    /// A user string, e.g. `"the quick brown fox"`.
    Str(String),

    /// A literal number.
    Num(TokenKindNumber),

    /// An operator.
    Op(TokenKindOperator),
    Deref(TokenKindDeref),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKindKeyword {
    /// The `atom` keyword defines a new Atom. It is a reserved keyword
    /// because you cannot create a macro named "atom" or an atom named "atom".
    Atom,
    /// The `macro` keyword defines a new Macro.
    /// You cannot create a macro named "macro" nor an Atom named "macro".
    Macro,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKindNumber {
    /// An integer number (signed).
    Int(i64),
    /// A floating-point number. All floating-point calculations are done with
    /// 64-bit floats to minimise rounding errors in intermediate calculations
    /// -- the assembly for the target system itself is likely to be Integer
    /// or 32-bit Float at best anyway.
    Float(f64),
    Hex(u64),
    Bin(u64),
}

#[derive(Clone, Debug, PartialEq)]
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
    /// Repeat operator "x"
    Rep,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKindDeref {
    Begin,
    End,
}

//------------------------------------------------------------------------------

/// Allow the direct conversion of Pest's `Pair`s into our `Token`s.
/// This removes a lot of logic from walking the `Pair`s.
impl<'i, 'p> TryFrom_<'p, Pair<'i, Rule>> for Token {
    fn try_from_(pair: Pair<'i, Rule>) -> Result<Self> {
        // get the starting position of the token for line / col number;
        // this will get passed all the way through even the AST so that
        // accurate error information can be given even late into assembling
        let span = pair.clone().into_span();
        let start = span.start_pos();
        // TODO: is this very costly? should we defer this until called?
        let (line, col) = start.line_col();

        Ok(Token {
            kind: TokenKind::try_from_(pair)?,
            data: TokenData::None,
            line: line as u32,
            col: col as u32,
        })
    }
}

impl<'i, 'p> TryFrom_<'p, Pair<'i, Rule>> for i64 {
    fn try_from_(pair: Pair<'i, Rule>) -> Result<Self> {
        match pair.as_str().parse::<i64>() {
            // FIXME: Match parse error specifically?
            Err(e) => Err(new_error(ErrorKind::ParseInt(e))),
            Ok(i) => Ok(i),
        }
    }
}

impl<'i, 'p> TryFrom_<'p, Pair<'i, Rule>> for u64 {
    fn try_from_(pair: Pair<'i, Rule>) -> Result<Self> {
        match pair.as_rule() {
            Rule::int_number => match pair.as_str().parse::<u64>() {
                // FIXME: Match parse error specifically?
                Err(e) => Err(new_error(ErrorKind::ParseInt(e))),
                Ok(i) => Ok(i),
            },
            _ => panic!("It's a lion, get in the car!"),
        }
    }
}

impl<'i, 'p> TryFrom_<'p, Pair<'i, Rule>> for TokenKind {
    fn try_from_(pair: Pair<'i, Rule>) -> Result<Self> {
        let token_kind = match pair.as_rule() {
            // keywords
            Rule::keyword => TokenKind::Keyword(TokenKindKeyword::Atom),
            // an Atom name
            Rule::atom => TokenKind::Atom(pair.to_string()),
            // a Macro name
            Rule::mac => TokenKind::Macro(pair.to_string()),
            // a string, e.g. `"the quick brown fox"`
            Rule::string => TokenKind::Str(pair.to_string()),

            Rule::int_number => TokenKind::Num(TokenKindNumber::Int(
                // attempt to convert the `Pair` to an i64,
                // if it fails, the error will propogate upwards
                i64::try_from_(pair)?,
            )),
            Rule::hex_number => TokenKind::Num(TokenKindNumber::Hex(
                // create an unsigned 64-bit Int from a string...
                u64::from_str_radix(
                    // ignore the first character ("$")
                    &pair.as_str()[1..],
                    16, //=hexadecimal
                )?,
            )),
            Rule::bin_number => {
                TokenKind::Num(TokenKindNumber::Bin(u64::from_str_radix(
                    // ignore the first character ("%")
                    &pair.as_str()[1..],
                    2, //=binary
                )?))
            }

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
            Rule::op_rep => TokenKind::Op(TokenKindOperator::Rep),

            Rule::deref_begin => TokenKind::Deref(TokenKindDeref::Begin),
            Rule::deref_end => TokenKind::Deref(TokenKindDeref::End),

            _ => TokenKind::None,
        };

        Ok(token_kind)
    }
}

impl<'i, 'p> TryFrom_<'p, Pair<'i, Rule>> for TokenKindKeyword {
    fn try_from_(pair: Pair<'i, Rule>) -> Result<Self> {
        match pair.as_str().as_ref() {
            "atom" => Ok(TokenKindKeyword::Atom),
            "macro" => Ok(TokenKindKeyword::Macro),
            _ => unimplemented!(),
        }
    }
}

//------------------------------------------------------------------------------

impl Token {
    /// Returns true if the token represents a keyword (any of them).
    pub fn is_keyword(&self) -> bool {
        match &self.kind {
            TokenKind::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn expect_keyword(&self, kind: TokenKindKeyword) -> Option<&Token> {
        match &self.kind {
            TokenKind::Keyword(k) if k == &kind => Some(self),
            _ => None,
        }
    }

    /// Returns true if the token is the `atom` keyword.
    pub fn is_keyword_atom(&self) -> bool {
        match &self.kind {
            TokenKind::Keyword(TokenKindKeyword::Atom) => true,
            _ => false,
        }
    }

    /// Returns true if the token is the `macro` keyword.
    pub fn is_keyword_macro(&self) -> bool {
        match &self.kind {
            TokenKind::Keyword(TokenKindKeyword::Macro) => true,
            _ => false,
        }
    }

    /// Returns true if the token is an Atom name.
    pub fn is_atom(&self) -> bool {
        match &self.kind {
            TokenKind::Atom(_) => true,
            _ => false,
        }
    }

    /// Returns true if the token is a Macro name.
    pub fn is_macro(&self) -> bool {
        match &self.kind {
            TokenKind::Macro(_) => true,
            _ => false,
        }
    }

    /// Returns true if the token is a string.
    pub fn is_string(&self) -> bool {
        match &self.kind {
            TokenKind::Str(_) => true,
            _ => false,
        }
    }

    /// Returns true if the token is a number (of any kind).
    pub fn is_number(&self) -> bool {
        match &self.kind {
            TokenKind::Num(_) => true,
            _ => false,
        }
    }

    /// Returns true if the token is an operator.
    pub fn is_operator(&self) -> bool {
        match &self.kind {
            TokenKind::Op(_) => true,
            _ => false,
        }
    }
}