// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

// force the Rust compiler to acknowlege external changes to the grammer file
// (this is recommended to do by Pest)
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("retronym.pest");

#[derive(Parser)]
#[grammar = "retronym.pest"]
pub struct RymParser;
