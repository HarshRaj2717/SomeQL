use super::Statement;
use super::lib::{read_next_list, read_next_word};

use crate::common::DataTypeDefiners;
use crate::common::Error;

/// return this when some command is unrecognized
/// this is supposed to be treated as an error in future runtime
#[inline]
pub(super) fn unrecognized_command() -> Statement {
    Statement::Failed {
        error: Error::CompilerUnrecognizedCommand,
    }
}

/// return this when some parsing error occurs
/// this is supposed to be treated as an error in future runtime
#[inline]
fn parse_error() -> Statement {
    Statement::Failed {
        error: Error::CompilerParseError,
    }
}

#[inline]
fn validate_table_name(name: &String) -> bool {
    // Allowing table names starting with alphabets only
    // This also prevents user from giving empty/none table_name
    match name.chars().nth(0) {
        Some(char) => char.is_alphabetic(),
        _ => false,
    }
}

pub(super) fn compile_meta(input: &String) -> Statement {
    let (first_word, end_index) = read_next_word(input, 1);
    match first_word.to_lowercase().as_str() {
        "exit" => Statement::MetaExit,
        "help" => Statement::MetaHelp,
        "print" => Statement::MetaPrint {
            text: input[end_index..].to_string(),
        },
        _ => unrecognized_command(),
    }
}

pub(super) fn compile_statement(input: &String) -> Statement {
    let (first_word, mut end_index) = read_next_word(input, 0);
    let table_name;
    match first_word.to_lowercase().as_str() {
        "create" => {
            (table_name, end_index) = read_next_word(input, end_index);
            if !validate_table_name(&table_name) {
                return parse_error();
            }
            let (success, columns, _) = read_next_list(input, end_index);
            match success {
                true => Statement::Create {
                    table_name,
                    columns: DataTypeDefiners::new_from_list(&columns),
                },
                false => parse_error(),
            }
        }
        "drop" => {
            (table_name, _) = read_next_word(input, end_index);
            match table_name.trim() {
                "" => parse_error(),
                _ => Statement::Drop { table_name },
            }
        }
        "insert" => {
            let second_word;
            (second_word, end_index) = read_next_word(input, end_index);
            if second_word.to_lowercase().as_str() != "into" {
                return parse_error();
            }
            (table_name, end_index) = read_next_word(input, end_index);

            if !validate_table_name(&table_name) {
                return parse_error();
            }

            let (success, row, _) = read_next_list(input, end_index);
            match success {
                true => Statement::Insert { table_name, row },
                false => parse_error(),
            }
        }
        "select" => {
            (table_name, _) = read_next_word(input, end_index);

            if !validate_table_name(&table_name) {
                return parse_error();
            }

            let (success, row, _) = read_next_list(input, end_index);
            match table_name.trim() {
                "" => parse_error(),
                _ => Statement::Select {
                    table_name,
                    column_names: if success { Some(row) } else { None },
                },
            }
        }
        _ => unrecognized_command(),
    }
}
