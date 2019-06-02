use std::env;
use std::io;
use std::io::Write;

mod cursor;
mod pager;
mod row;
mod statement;
mod table;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please give a filename for the db");
    }

    let table: &mut table::Table = &mut table::Table::db_open(args[1].to_owned());
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
                true => {
                    table.db_close();
                    std::process::exit(0);
                }
                false => println!("Unrecognized command {}", command),
            }
            continue;
        }

        let statement = statement::prepare_statement(&command);

        if statement.is_err() {
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
                Ok(_) => unreachable!(),
            }
            continue;
        }

        let statement = statement.unwrap();

        match statement.statement_type {
            statement::StatementType::Insert => match &statement.row_to_insert {
                None => {
                    println!("Cannot insert empty row!");
                    continue;
                }
                Some(row) => {
                    let row = row.to_owned();
                    let mut cursor = cursor::Cursor::table_end(table);
                    match cursor.add_row(row) {
                        Ok(_) => {}
                        Err(table::TableError::TableFull) => {
                            println!("Table is full.");
                            continue;
                        }
                    }
                }
            },
            statement::StatementType::Select => {
                let mut cursor = cursor::Cursor::table_start(table);
                while !cursor.end_of_table {
                    println!("{:?}", cursor.cursor_value());
                    cursor.advance();
                }
            }
        };

        println!("Executed");
    }
}
