use crate::row;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: Option<row::Row>, // only used by insert
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrepareError {
    SyntaxErr,
    UnrecognizedStatement,
    InsertError,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_insert() {
        let res = prepare_statement("insert 1 asdf jkl;");

        assert!(res.is_ok());
    }

    #[test]
    fn insert_too_short() {
        let res = prepare_statement("insert");

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), PrepareError::SyntaxErr);
    }

    #[test]
    fn simple_select() {
        let res = prepare_statement("select");

        assert!(res.is_ok());
    }

    #[test]
    fn unrecognized_statement() {
        let res = prepare_statement("asdf");

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), PrepareError::UnrecognizedStatement);
    }
}
