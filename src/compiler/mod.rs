//! Compiler module
//! Provides functionality to compile queries into an intermediate representation.

mod compiler;
mod lib;

use crate::common::{DataTypeDefiners, DataTypeHolders, Error};
use compiler::*;

/// Internal representation of input for forwarding to executor
pub(crate) enum Statement {
    // SQL commands
    Create {
        table_name: String,
        columns: Vec<DataTypeDefiners>,
    },
    Drop {
        table_name: String,
    },
    Insert {
        table_name: String,
        row: Vec<DataTypeHolders>,
    },
    Select {
        table_name: String,
        column_names: Option<Vec<String>>,
    },

    // Meta commands
    MetaExit,
    MetaHelp,
    MetaPrint {
        text: String,
    },

    // Others
    Failed {
        error: Error,
    },
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
