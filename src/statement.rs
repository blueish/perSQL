use crate::table;
use crate::row;

pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<row::Row>, // only used by insert
}

pub enum PrepareError {
    SyntaxErr,
    UnrecognizedStatement,
    InsertError,
}

impl Statement {
    pub fn execute_statement(&self, table: &mut table::Table) -> bool {
        match self.statement_type {
            StatementType::Insert => {
                return match &self.row_to_insert {
                    None => false,
                    Some(row) => table.insert_row(row),
                };
            }
            StatementType::Select => {
                table.print_rows();
                return true;
            }
        };
    }
}

pub fn prepare_statement<'a>(command: &'a str) -> Result<Statement, PrepareError> {
    if command.starts_with("insert") {
        let v: Vec<String> = command.split(' ').skip(1).map(|x| x.to_string()).collect();

        if v.len() < 3 {
            return Err(PrepareError::SyntaxErr);
        }

        let username = v[1].to_string();
        let email = v[2].to_string();

        if username.len() > row::COLUMN_USERNAME_SIZE || email.len() > row::COLUMN_EMAIL_SIZE {
            return Err(PrepareError::InsertError);
        }

        let row = row::Row::new(v[0].parse().expect("Invalid id"), username, email);

        let statement = Statement {
            statement_type: StatementType::Insert,
            row_to_insert: Some(row),
        };

        return Ok(statement);
    }
    if command.starts_with("select") {
        let statement = Statement {
            statement_type: StatementType::Select,
            row_to_insert: None,
        };

        return Ok(statement);
    }

    return Err(PrepareError::UnrecognizedStatement);
}
