//! Compiler module
//! Provides functionality to compile queries into an intermediate representation.

mod compiler;
mod lib;

use compiler::*;
use crate::common::Error;

pub(crate) enum StatementType {
    Create,
    Drop,
    Insert,
    Undefined,
    MetaExit,
    MetaHelp,
    MetaPrint,
    Select,
}

/// Internal representation of input for forwarding to virtual machine
pub(crate) struct Statement {
    error: Option<Error>,
    statement_type: StatementType,
    meta_args: Option<String>,              // for META commands only
    table_name: Option<String>,             // for non-META commands only
    columns_to_create: Option<Vec<String>>, // for create statement only
    row_to_insert: Option<Vec<String>>,     // for insert statement only
}

/// Parse an input string and return its internal representation
pub(crate) fn compile(input: &String) -> Statement {
    if let Some(first_char) = input.chars().next() {
        match first_char {
            '.' => compile_meta(input),
            _ => compile_statement(input),
        }
    } else {
        unrecognized_command()
    }
}
