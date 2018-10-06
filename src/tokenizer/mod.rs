// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! `tokenizer` takes source code input, splits it into words ("lexemes"),
//! and assigns types and values where possible, recognising each word as
//! either a keyword, a string, a numerical value (written in hex / binary /
//! decimal) or an operator.
//!
//! Only individual 'words' are analysed, not their relation to one another,
//! so tokenisation only implies that no incorrect symbols have been used,
//! not that the grammar is correct.
//!
//! Keywords are *not* checked for existance at this stage because macros
//! can implement new keywords and we can therefore not gaurantee that all
//! keywords are known by looking at the words in a single file.

pub mod token;
pub mod tokenstream;
