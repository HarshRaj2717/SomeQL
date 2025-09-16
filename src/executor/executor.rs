use super::Executor;

use crate::common::*;
use crate::compiler::{Statement, StatementType};

impl Executor {
    pub(crate) fn new() -> Self {
        Executor {}
    }

    pub(crate) fn execute(&self, statement: &Statement) -> () {
        match statement.get_statement_type() {
            StatementType::Create => todo!(),
            StatementType::Drop => todo!(),
            StatementType::Insert => {
                let table_name: &String = statement
                    .get_table_name()
                    .unwrap_or_else(|| panic!("{}", Error::UnreachablePath));
                let row_to_insert: &Vec<String> = statement
                    .get_row_to_insert()
                    .unwrap_or_else(|| panic!("{}", Error::UnreachablePath));
                self.execute_insert(table_name, row_to_insert)
            }
            StatementType::MetaExit => std::process::exit(0),
            StatementType::MetaHelp => {
                println!("{}", constants::META_HELP_TEXT);
            }
            StatementType::MetaPrint => {
                let meta_args = statement.get_meta_args();
                let to_print: &str = match meta_args {
                    Some(s) => s.as_str(),
                    None => "",
                };
                println!("{to_print}");
            }
            StatementType::Select => {
                let table_name = statement
                    .get_table_name()
                    .unwrap_or_else(|| panic!("{}", Error::UnreachablePath));
                self.execute_select(table_name)
            }
            StatementType::Failed => panic!("{}", Error::UnreachablePath),
        }
    }

    fn execute_select(&self, table_name: &String) {}

    fn execute_insert(&self, table_name: &String, row_to_insert: &Vec<String>) {}
}
