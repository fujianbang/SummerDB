#[derive(Debug)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: Option<Row>,
}

impl Statement {
    pub fn new(statement_type: StatementType) -> Self {
        Self {
            statement_type,
            row_to_insert: None,
        }
    }

    pub fn insert_row(mut self, row: Row) -> Self {
        self.row_to_insert = Some(row);
        self
    }
}
