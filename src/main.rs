extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::io;
use std::io::Write;

mod row;
mod statement;
mod table;
mod util;

fn main() {
    let table: &mut table::Table = &mut table::Table::new();
    loop {
        print!("persql> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim();

        if command.starts_with(".") {
            match util::do_meta_command(&command) {
                true => println!("meta command success"),
                false => println!("Unrecognized command {}", command),
            }
            continue;
        }

        let statement = statement::prepare_statement(&command);

        match statement {
            Err(statement::PrepareError::InsertError) => {
                println!("Insertion error, make sure your name is < 32 chars and email < 255")
            }
            Err(statement::PrepareError::SyntaxErr) => {
                println!("Syntax error at start of {}", command)
            }
            Err(statement::PrepareError::UnrecognizedStatement) => {
                println!("Unrecognized statement at start of {}", command)
            }
            Ok(statement) => {
                statement.execute_statement(table);
                println!("Executed");
            }
        }
    }
}
