use crate::row;
use crate::statement;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / row::ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

struct Page {
    data: Vec<row::Row>,
}

impl Page {
    fn new(v: Vec<row::Row>) -> Page {
        Page {
            data: v
        }
    }
}

#[derive(Debug)]
pub enum TableError {
    TableFull,

}

pub struct Table {
    num_rows: usize,
    pages: Vec<Page>,
}

impl Table {
    pub fn new() -> Table {
        let mut pages = vec![];
        for _ in 0..TABLE_MAX_PAGES {
            let v = vec![];
            pages.push(Page::new(v));
        }

        Table {
            num_rows: 0,
            pages: pages,
        }
    }

    pub fn execute_statement<'c>(&mut self, statement: &'c statement::Statement) -> Result<& mut Table, TableError> {
        match statement.statement_type {
            statement::StatementType::Insert => {
                return match &statement.row_to_insert {
                    None => Ok(self),
                    Some(row) => {
                        let row = row.to_owned();
                        return self.insert_row(row);
                    },
                };
            }
            statement::StatementType::Select => {
                self.print_rows();
                return Ok(self);
            }
        };
    }

    fn insert_row(&mut self, row: row::Row) -> Result<&mut Table, TableError> {
        if self.num_rows >= TABLE_MAX_ROWS {
            return Err(TableError::TableFull);
        }

        let row_num = self.num_rows + 1;

        let page_num: usize = (row_num as usize) / ROWS_PER_PAGE;
        let page: &mut Page = &mut self.pages[page_num];

        page.data.push(row);

        self.num_rows += 1;
        return Ok(self);
    }

    pub fn print_rows(&self) {
        for page_num in 0..TABLE_MAX_PAGES {
            for row in self.pages[page_num].data.iter() {
                println!("{:?}", row);
            }
        }
    }

    fn row_slot(&self, row_num: usize) -> (usize, usize) {
        let page_num: usize = (row_num as usize) / ROWS_PER_PAGE;
        // let page: &Page = &self.pages[page_num];

        // TODO null check?

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * row::ROW_SIZE;

        return (page_num, byte_offset);
    }

    fn page_row_idx(row_num: usize) -> (usize, usize) {
        let page_num: usize = (row_num as usize) / ROWS_PER_PAGE;

        let row_idx = row_num % ROWS_PER_PAGE;

        return (page_num, row_idx - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_row() {
        let row = row::Row::new(1, String::from(""), String::from(""));
        let mut table = Table::new();
        assert!(table.num_rows == 0);

        table.insert_row(&row);
        assert!(table.num_rows == 1);
    }
}
