// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! Built-in **primitive types** used in Retronym source code files.
//!
//! Retronym has a small number of built-in primitives,
//! defined by the number of bits they occupy:
//! 
//! * `bool` = 1 bit
//! * `nybl` (nybble) = 2 bits
//! * `byte` = 8 bits (1 byte)
//! * `word` = 16 bits (2 bytes)
//! * `long` = 32 bits (4 bytes)
//! 
//! Note that these are specified for the target-system (6502, Z80 etc.) and
//! not Retronym's own internal calculations, which use 32-bit numbers and
//! only constrain to lower sizes at the point of assembling binary output;
//! that is, expressions can work with numbers larger than the chosen output
//! type so long as the final result fits.
//! 

/// Native, target-system, primitive types;
/// described in number of bits.
#[derive(Debug)]
pub enum PType {
    /// A single bit. Cannot be named `bit` due to conflict with the `bit`
    /// instruction on Z80 cpus.
    BOOL = 1,
    /// A nybble, 4 bits.
    NYBL = 4,
    /// A byte, 8 bits.
    BYTE = 8,
    /// A word. 16 bits on retro systems! Not to be confused with the modern
    /// meaning of the term which is the native CPU stride (32/64-bits)
    WORD = 16,
    /// A "long"; 32-bits on retro systems. Also known as a "double-word"
    /// or "double" in the past, not to be confused with the double-precision
    /// float.
    LONG = 32,
}

use std::fmt::{self, *};

impl Display for PType {
    /// Give the normalised string representation of a primitive type.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PType::BOOL => "bool",
            PType::NYBL => "nybl",
            PType::BYTE => "byte",
            PType::WORD => "word",
            PType::LONG => "long",
        })
    }
}