use std::io;
use std::io::Write;

fn main() {
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

        let mut statement = prepare_statement(&command);

        match statement {
            Err(PrepareError::InsertError) => println!("Insertion error, make sure your name is < 32 chars and email < 255"),
            Err(PrepareError::SyntaxErr) => println!("Syntax error at start of {}", command),
            Err(PrepareError::UnrecognizedStatement) => {
                println!("Unrecognized statement at start of {}", command)
            },
            Ok(statement) => {
                statement.execute_statement(&command);
                println!("Executed");
            },
        }
    }
}

fn do_meta_command(command: &str) -> bool {
    if command == ".exit" {
        std::process::exit(0);
    }

    return false;
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

struct Row {
    id: u32,
    username: String,
    email: String,
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>, // only used by insert
}

enum PrepareError{
    SyntaxErr,
    UnrecognizedStatement,
    InsertError,
}

impl Statement {
    fn execute_statement(&self, command: &str) -> bool {
        match self.statement_type {
            StatementType::Insert => {
                println!("this is where we insert");
            }
            StatementType::Select => {
                println!("this is where we select");
            }
        };

        true
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

        if username.len() > COLUMN_USERNAME_SIZE || email.len() > COLUMN_EMAIL_SIZE {
            return Err(PrepareError::InsertError);
        }

        let row = Row {
            id: v[0].parse().expect("Invalid id"),
            username: username,
            email: email,
        };

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
