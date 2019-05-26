extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::io;
use std::io::Write;

mod row;

fn main() {
    let table: &mut row::Table = &mut row::Table::new();
    loop {
        print!("persql> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim();

        if command.starts_with(".") {
            match do_meta_command(&command) {
                true => println!("meta command success"),
                false => println!("Unrecognized command {}", command),
            }
            continue;
        }

        let statement = prepare_statement(&command);

        match statement {
            Err(PrepareError::InsertError) => {
                println!("Insertion error, make sure your name is < 32 chars and email < 255")
            }
            Err(PrepareError::SyntaxErr) => println!("Syntax error at start of {}", command),
            Err(PrepareError::UnrecognizedStatement) => {
                println!("Unrecognized statement at start of {}", command)
            }
            Ok(statement) => {
                statement.execute_statement(table);
                println!("Executed");
            }
        }
    }
}

fn do_meta_command(command: &str) -> bool {
    if command == ".exit" {
        std::process::exit(0);
    }

    return false;
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<row::Row>, // only used by insert
}

enum PrepareError {
    SyntaxErr,
    UnrecognizedStatement,
    InsertError,
}

impl Statement {
    fn execute_statement(&self, table: &mut row::Table) -> bool {
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

fn prepare_statement<'a>(command: &'a str) -> Result<Statement, PrepareError> {
    if command.starts_with("insert") {
        let v: Vec<String> = command.split(' ').skip(1).map(|x| x.to_string()).collect();

        if v.len() < 3 {
            return Err(PrepareError::InsertError);
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
