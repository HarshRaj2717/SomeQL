use std::collections::HashMap;

pub(crate) struct Database {
    tables: HashMap<String, Table>,
}

pub(crate) struct Table {
    rows: Vec<Row>,
}

enum RowType {
    Boolean(bool),
    Char(char),
    Varchar(String),
    Integer(i32),
    Float(f32),
    BigInt(i64),
    Double(f64),
}

pub(crate) struct Row {
    data: Vec<RowType>,
}

impl Database {
    pub(crate) fn new() -> Self {
        let mut db = Database {
            tables: HashMap::new(),
        };
        db.create_table("test".to_string());

        db
    }

    pub(crate) fn create_table(&mut self, table_name: String) {
        self.tables.insert(table_name, Table::new());
    }

    pub(crate) fn drop_table(&mut self, table_name: &String) {
        self.tables.remove(table_name);
    }

    pub(crate) fn get_table(&self, table_name: &String) -> Option<&Table> {
        self.tables.get(table_name)
    }
}

impl Table {
    pub(crate) fn new() -> Self {
        Table { rows: Vec::new() }
    }

    pub(crate) fn insert_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub(crate) fn get_rows(&self) -> &Vec<Row> {
        &self.rows
    }
}

impl Row {
    pub(crate) fn new(data: Vec<RowType>) -> Self {
        Row { data }
    }

    pub(crate) fn get_data(&self) -> &Vec<RowType> {
        &self.data
    }
}

impl RowType {
    pub(crate) fn print_val(&self) {
        match self {
            RowType::Boolean(val) => println!("{}", val),
            RowType::Char(val) => println!("{}", val),
            RowType::Varchar(val) => println!("{}", val),
            RowType::Integer(val) => println!("{}", val),
            RowType::Float(val) => println!("{}", val),
            RowType::BigInt(val) => println!("{}", val),
            RowType::Double(val) => println!("{}", val),
        }
    }
}