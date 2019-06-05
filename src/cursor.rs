use crate::row;
use crate::table;

pub struct Cursor<'a> {
    pub table: &'a mut table::Table,
    pub row_num: usize,
    pub end_of_table: bool,
    pub num_rows_in_table: usize,
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &mut table::Table) -> Cursor {
        Cursor {
            end_of_table: table.num_rows == 0,
            num_rows_in_table: table.num_rows,
            table: table,
            row_num: 0,
        }
    }

    pub fn table_end(table: &mut table::Table) -> Cursor {
        Cursor {
            row_num: table.num_rows,
            end_of_table: true,
            num_rows_in_table: table.num_rows,
            table: table,
        }
    }

    pub fn advance(&mut self) {
        self.row_num += 1;

        if self.row_num >= self.num_rows_in_table {
            self.end_of_table = true;
        }
    }

    pub fn add_row(&mut self, row: row::Row) -> Result<(), table::TableError> {
        self.table.insert_row(row)?;
        self.advance();
        Ok(())
    }

    pub fn cursor_value(&mut self) -> row::Row {
        let page_idx: usize = self.row_num / table::ROWS_PER_PAGE;
        let page = self.table.pager.get_page(page_idx);

        let row_offset = self.row_num % table::ROWS_PER_PAGE;
        let byte_offset = row_offset * row::ROW_SIZE;

        return row::Row::deserialize_row(
            page.data[byte_offset..byte_offset + row::ROW_SIZE].to_vec(),
        );
    }
}
