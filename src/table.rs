use crate::row;
use crate::statement;
use crate::pager;

pub const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / row::ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
}

pub struct Table {
    num_rows: usize,
    pager: pager::Pager,
}

#[derive(Debug)]
pub enum TableError {
    TableFull,
}

impl Table {
    pub fn db_open(filename: String) -> Table {
        let pager = pager::Pager::new(filename);
        let num_rows = pager.num_rows();

        Table {
            num_rows: num_rows,
            pager: pager,
        }
    }

    pub fn db_close(&mut self) {
        self.pager.close();
    }

    pub fn execute_statement<'c>(
        &mut self,
        statement: &'c statement::Statement,
    ) -> Result<&mut Table, TableError> {
        match statement.statement_type {
            statement::StatementType::Insert => {
                return match &statement.row_to_insert {
                    None => Ok(self),
                    Some(row) => {
                        let row = row.to_owned();
                        return self.insert_row(row);
                    }
                };
            }
            statement::StatementType::Select => {
                self.print_rows();
                return Ok(self);
            }
        };
    }

    fn insert_row(&mut self, row: row::Row) -> Result<&mut Table, TableError> {
        if self.num_rows >= TABLE_MAX_ROWS - 1 {
            return Err(TableError::TableFull);
        }

        let row_num = self.num_rows;
        let (page_idx, offset) = Table::row_slot(row_num);

        let mut page = self.pager.get_page(page_idx);

        let bytes = row.serialize_row();

        let mut i = 0;
        for byte in bytes.iter() {
            page.data[offset + i] = *byte;
            i += 1
        }

        self.num_rows += 1;
        return Ok(self);
    }

    pub fn print_rows(&mut self) {
        for row_num in 0..self.num_rows {
            let (page_index, offset) = Table::row_slot(row_num);

            let page = self.pager.get_page(page_index);

            let bytes = page.data[offset..offset + row::ROW_SIZE].to_vec();

            let row = row::Row::deserialize_row(bytes);
            println!("{:?}", row);
        }
    }

    /// returns the <page_idx, offset_start> for a given row
    /// # Example
    ///
    ///
    /// ```
    /// let row_number = 1;
    /// assert_eq!(row_slot(row_number), (0, 1))
    ///
    /// let row_number = ROWS_PER_PAGE;
    /// assert_eq!(row_slot(row_number), (1, 0))
    /// ```
    fn row_slot(row_num: usize) -> (usize, usize) {
        let page_num: usize = (row_num as usize) / ROWS_PER_PAGE;

        // TODO null check?

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * row::ROW_SIZE;

        return (page_num, byte_offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_row() {
        let row = row::Row::new(1, String::from(""), String::from(""));
        let mut table = Table::db_open(String::from("test.db"));
        assert!(table.num_rows == 0);

        assert!(table.insert_row(row).is_ok());
        assert!(table.num_rows == 1);
    }
}
