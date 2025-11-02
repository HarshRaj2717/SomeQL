use super::Executor;

use crate::common::*;
use crate::compiler::Statement;

impl Executor {
    pub(crate) fn new() -> Self {
        Executor {}
    }

    pub(crate) fn execute(&self, statement: &Statement) -> () {
        match &statement {
            &Statement::Create {
                table_name,
                columns,
            } => todo!(),
            &Statement::Drop { table_name } => todo!(),
            &Statement::Insert { table_name, row } => self.execute_insert(&table_name, &row),
            &Statement::MetaExit => std::process::exit(0),
            &Statement::MetaHelp => {
                println!("{}", constants::META_HELP_TEXT);
            }
            &Statement::MetaPrint { text } => {
                println!("{text}");
            }
            &Statement::Select {
                table_name,
                column_names,
            } => self.execute_select(&table_name, &column_names),
            _ => panic!("{}", Error::UnreachablePath),
        }
    }

    fn execute_select(&self, table_name: &String, column_names: &Option<Vec<String>>) {}

    fn execute_insert(&self, table_name: &String, row: &Vec<DataTypeHolders>) {}
}
