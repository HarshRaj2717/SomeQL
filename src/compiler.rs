use crate::utils::{read_next_list, read_next_word};

pub enum StatementResult {
    Success,
    Unrecognized,
    ParseError,
}

pub enum StatementType {
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
pub struct Statement {
    statement_result: StatementResult,
    statement_type: StatementType,
    meta_args: Option<String>,              // for META commands only
    table_name: Option<String>,             // for non-META commands only
    columns_to_create: Option<Vec<String>>, // for create statement only
    row_to_insert: Option<Vec<String>>,     // for insert statement only
}

impl Statement {
    fn new(
        statement_result: StatementResult,
        statement_type: StatementType,
        meta_args: Option<String>,
        table_name: Option<String>,
        columns_to_create: Option<Vec<String>>,
        row_to_insert: Option<Vec<String>>,
    ) -> Self {
        Self {
            statement_result,
            statement_type,
            meta_args,
            table_name,
            columns_to_create,
            row_to_insert,
        }
    }

    pub fn statement_result(&self) -> &StatementResult {
        &self.statement_result
    }

    pub fn statement_type(&self) -> &StatementType {
        &self.statement_type
    }

    pub fn meta_args(&self) -> Option<&String> {
        self.meta_args.as_ref()
    }

    pub fn table_name(&self) -> Option<&String> {
        self.table_name.as_ref()
    }

    // pub fn columns_to_create(&self) -> Option<&Vec<String>> {
    //     self.columns_to_create.as_ref()
    // }

    pub fn row_to_insert(&self) -> Option<&Vec<String>> {
        self.row_to_insert.as_ref()
    }
}

/// return this when some command is unrecognized
/// this is supposed to be treated as an error in future runtime
fn unrecognized_command() -> Statement {
    Statement::new(
        StatementResult::Unrecognized,
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
        StatementResult::ParseError,
        StatementType::Undefined,
        None,
        None,
        None,
        None,
    )
}

fn compile_meta(input: &String) -> Statement {
    let (first_word, end_index) = read_next_word(input, 1);
    let meta_args = input[end_index..].to_string();
    match first_word.to_lowercase().as_str() {
        "exit" => Statement::new(
            StatementResult::Success,
            StatementType::MetaExit,
            Some(meta_args),
            None,
            None,
            None,
        ),
        "help" => Statement::new(
            StatementResult::Success,
            StatementType::MetaHelp,
            Some(meta_args),
            None,
            None,
            None,
        ),
        "print" => Statement::new(
            StatementResult::Success,
            StatementType::MetaPrint,
            Some(meta_args),
            None,
            None,
            None,
        ),
        _ => unrecognized_command(),
    }
}

fn compile_statement(input: &String) -> Statement {
    let (first_word, mut end_index) = read_next_word(input, 0);
    let table_name;
    match first_word.to_lowercase().as_str() {
        "create" => {
            (table_name, end_index) = read_next_word(input, end_index);
            let (success, columns_to_create, _) = read_next_list(input, end_index);
            match success {
                true => Statement::new(
                    StatementResult::Success,
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
                    StatementResult::Success,
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
                    StatementResult::Success,
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
                    StatementResult::Success,
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

/// Parse an input string and return its internal representation
pub fn compile(input: &String) -> Statement {
    if let Some(first_char) = input.chars().next() {
        match first_char {
            '.' => compile_meta(input),
            _ => compile_statement(input),
        }
    } else {
        unrecognized_command()
    }
}
