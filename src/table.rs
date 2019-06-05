use crate::pager;
use crate::row;

pub const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 100;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / row::ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
}

pub struct Table {
    pub num_rows: usize,
    pub pager: pager::Pager,
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

    pub fn insert_row(&mut self, row: row::Row) -> Result<(), TableError> {
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

        Ok(())
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
