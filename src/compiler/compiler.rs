use super::lib::{read_next_list, read_next_word};
use super::{Statement, StatementType};

use crate::common::Error;

impl Statement {
    fn new(
        error: Option<Error>,
        statement_type: StatementType,
        meta_args: Option<String>,
        table_name: Option<String>,
        columns_to_create: Option<Vec<String>>,
        row_to_insert: Option<Vec<String>>,
    ) -> Self {
        Self {
            error,
            statement_type,
            meta_args,
            table_name,
            columns_to_create,
            row_to_insert,
        }
    }

    pub(crate) fn get_error(&self) -> &Option<Error> {
        &self.error
    }

    pub(crate) fn get_statement_type(&self) -> &StatementType {
        &self.statement_type
    }

    pub(crate) fn get_meta_args(&self) -> Option<&String> {
        self.meta_args.as_ref()
    }

    pub(crate) fn get_table_name(&self) -> Option<&String> {
        self.table_name.as_ref()
    }

    // pub(crate) fn columns_to_create(&self) -> Option<&Vec<String>> {
    //     self.columns_to_create.as_ref()
    // }

    pub(crate) fn get_row_to_insert(&self) -> Option<&Vec<String>> {
        self.row_to_insert.as_ref()
    }
}

/// return this when some command is unrecognized
/// this is supposed to be treated as an error in future runtime
pub(super) fn unrecognized_command() -> Statement {
    Statement::new(
        Some(Error::CompilerUnrecognizedCommand),
        StatementType::Undefined,
        None,
        None,
        None,
        None,
    )
}

/// return this when some parsing error occurs
/// this is supposed to be treated as an error in future runtime
fn parse_error() -> Statement {
    Statement::new(
        Some(Error::CompilerParseError),
        StatementType::Undefined,
        None,
        None,
        None,
        None,
    )
}

pub(super) fn compile_meta(input: &String) -> Statement {
    let (first_word, end_index) = read_next_word(input, 1);
    let meta_args = input[end_index..].to_string();
    match first_word.to_lowercase().as_str() {
        "exit" => Statement::new(
            None,
            StatementType::MetaExit,
            Some(meta_args),
            None,
            None,
            None,
        ),
        "help" => Statement::new(
            None,
            StatementType::MetaHelp,
            Some(meta_args),
            None,
            None,
            None,
        ),
        "print" => Statement::new(
            None,
            StatementType::MetaPrint,
            Some(meta_args),
            None,
            None,
            None,
        ),
        _ => unrecognized_command(),
    }
}

pub(super) fn compile_statement(input: &String) -> Statement {
    let (first_word, mut end_index) = read_next_word(input, 0);
    let table_name;
    match first_word.to_lowercase().as_str() {
        "create" => {
            (table_name, end_index) = read_next_word(input, end_index);
            let (success, columns_to_create, _) = read_next_list(input, end_index);
            match success {
                true => Statement::new(
                    None,
                    StatementType::Create,
                    None,
                    Some(table_name),
                    Some(columns_to_create),
                    None,
                ),
                false => parse_error(),
            }
        }
        "drop" => {
            (table_name, _) = read_next_word(input, end_index);
            match table_name.trim() {
                "" => parse_error(),
                _ => Statement::new(
                    None,
                    StatementType::Drop,
                    None,
                    Some(table_name),
                    None,
                    None,
                ),
            }
        }
        "insert" => {
            let second_word;
            (second_word, end_index) = read_next_word(input, end_index);
            if second_word.to_lowercase().as_str() != "into" {
                return parse_error();
            }
            (table_name, end_index) = read_next_word(input, end_index);

            // Allowing table names starting with alphabets only
            // This also prevents user from giving empty/none table_name
            if let Some(table_name_first_char) = table_name.chars().next() {
                if !table_name_first_char.is_alphabetic() {
                    return parse_error();
                }
            } else {
                return parse_error();
            }

            let (success, row_to_insert, _) = read_next_list(input, end_index);
            match success {
                true => Statement::new(
                    None,
                    StatementType::Insert,
                    None,
                    Some(table_name),
                    None,
                    Some(row_to_insert),
                ),
                false => parse_error(),
            }
        }
        "select" => {
            (table_name, _) = read_next_word(input, end_index);
            match table_name.trim() {
                "" => parse_error(),
                _ => Statement::new(
                    None,
                    StatementType::Select,
                    None,
                    Some(table_name),
                    None,
                    None,
                ),
            }
        }
        _ => unrecognized_command(),
    }
}
