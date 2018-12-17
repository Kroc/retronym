// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! A **Table** of data, consisting of a record of fields,
//! populated with data to fit.

use crate::error::*;
use crate::row::{Row, RowBuilder};

pub struct TableBuilder<'token> {
    /// Reference to the Record-struct used to define the columns.
    record: &'token Struct<'token>,
    /// The Rows built for the Table.
    rows: Vec<Row<'token>>,
    /// RowBuilder employed to populate Rows one-by-one.
    builder: RowBuilder<'token>,
    /// Current Row in the Table during building.
    row: usize,
}

impl<'token> TableBuilder<'token> {
    //==========================================================================
    /// Tables are tightly bound to the Record-struct that defines the type of
    /// each column. You cannot create a Table without a Record-struct as this
    /// would allow swapping the Record-struct whilst the Table is using it.
    ///
    pub fn new(record: &'token Struct<'token>) -> Self {
        //----------------------------------------------------------------------
        Self {
            record,
            rows: Default::default(),
            builder: RowBuilder::new(record, 0),
            row: 0,
        }
    }

    /// Add data to the Table by assigning an AST Node to the next Cell in the
    /// current Row. When the record is satsisfied, another row will be started.
    ///
    /// TODO: return satisfied state, exports?
    /// TODO: handle being passed a list (flatten it out?)
    /// TODO: errors for non-data types of nodes
    ///
    pub fn add_data(
        &mut self,
        node: &'token Node<'token>,
    ) -> Option<ParseError> {
        //----------------------------------------------------------------------
        // has the Row already yielded?
        if self.builder.is_satisfied() {
            // yes; start a new Row
            self.builder = RowBuilder::new(self.record, self.row);
        }
        // add the data to the Row builder;
        // it'll return a Row if it has been satisfied
        let row = self.builder.add_data(node).unwrap()?;
        self.rows.push(row);
        self.row += 1;
        // no probalo
        None
    }

    pub fn finish(self) -> ParseResult<Table<'token>> {
        //----------------------------------------------------------------------
        // if finishing the table and the Row is
        // not yet satisified, this is an error!
        if !self.builder.is_satisfied() {
            // TODO: convert to error
            return Err(parse_error(ParseErrorKind::Unsatisfied));
        }

        // TODO: handle remaining Table Row
        Ok(Table {
            record: self.record,
            rows: self.rows,
        })
    }
}

use crate::node::Node;
use crate::r#struct::Struct;

pub struct Table<'token> {
    /// Reference to the Record-struct used to define the columns.
    record: &'token Struct<'token>,
    /// The collection of Rows containing the data.
    rows: Vec<Row<'token>>,
}

use std::fmt::{self, *};

impl<'token> Display for Table<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(
            f,
            "{{\t{}\n{}}}",
            self.record,
            self.rows
                .iter()
                .fold(String::new(), |acc, row| format!("{}\t{}\n", acc, row))
        )
    }
}

impl<'token> Debug for Table<'token> {
    //==========================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //----------------------------------------------------------------------
        write!(
            f,
            "{{\t{:?}\n{}}}",
            self.record,
            self.rows.iter().fold(String::new(), |acc, row| format!(
                "{}\t{:?}\n",
                acc, row
            ))
        )
    }
}
