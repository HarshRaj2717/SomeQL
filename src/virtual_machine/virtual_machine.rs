use crate::compiler::compiler::{Statement, StatementType};
use crate::constants;

const UNEXPECTED_ERROR_MESSAGE: &str = "Unexpected Error Encountered";

fn execute_select(table_name: &String) {}

fn execute_insert(table_name: &String, row_to_insert: &Vec<String>) {}

pub(crate) fn execute(statement: &Statement) -> () {
    match statement.get_statement_type() {
        StatementType::Create => todo!(),
        StatementType::Drop => todo!(),
        StatementType::Insert => {
            let table_name: &String = statement.get_table_name().expect(UNEXPECTED_ERROR_MESSAGE);
            let row_to_insert: &Vec<String> = statement
                .get_row_to_insert()
                .expect(UNEXPECTED_ERROR_MESSAGE);
            execute_insert(table_name, row_to_insert)
        }
        StatementType::MetaExit => std::process::exit(0),
        StatementType::MetaHelp => {
            println!("{}", constants::META_HELP_TEXT);
        }
        StatementType::MetaPrint => {
            let to_print: &String = statement.get_meta_args().expect("");
            println!("{to_print}");
        }
        StatementType::Select => {
            let table_name = statement.get_table_name().expect(UNEXPECTED_ERROR_MESSAGE);
            execute_select(table_name)
        }
        StatementType::Undefined => panic!("{UNEXPECTED_ERROR_MESSAGE}"),
    }
}
