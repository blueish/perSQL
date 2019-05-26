pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Row {
        Row {
            id,
            username,
            email,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let encoded = bincode::serialize(self).unwrap();

        encoded
    }

    pub fn deserialize(raw_data: Vec<u8>) -> Row {
        println!("{:?}",raw_data.to_vec());
        let decoded: Result<Option<Row>, bincode::Error> = bincode::deserialize(&raw_data[..]);
        // let decoded: Option<Row> = bincode::deserialize(&raw_data[..]).unwrap();
        dbg!(&decoded);
        decoded.unwrap().unwrap()
    }
}

// Page stuff

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    num_rows: usize,
    pages: [Page; TABLE_MAX_PAGES],
}

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: [Page {
                data: [0; PAGE_SIZE],
            }; TABLE_MAX_PAGES],
        }
    }

    pub fn insert_row(&mut self, row: &Row) -> bool {
        if self.num_rows >= TABLE_MAX_ROWS {
            return false
        }

        let (page_index, offset) = self.row_slot(self.num_rows);
        let mut page = self.pages[page_index];

        let bytes = row.serialize();
        dbg!(bytes.len());

        let mut i = 0;
        for byte in bytes.iter() {
            page.data[offset + i] = *byte;
            i += 1
        }

        // Since we modified a copy of the page, we need to slot it in
        self.pages[page_index] = page;
        self.num_rows += 1;
        return true
    }

    pub fn print_rows(&self) {
        dbg!(self.num_rows);
        for row_num in 0..self.num_rows {
            dbg!(row_num);
            let (page_index, offset) = self.row_slot(row_num);
            dbg!(page_index);
            dbg!(offset);

            let page = self.pages[page_index];

            let bytes = page.data[offset..offset + ROW_SIZE].to_vec();
            dbg!(bytes.len());

            let row = Row::deserialize(bytes);
            println!("{:?}", row);
        }
    }


    fn row_slot(&self, row_num: usize) -> (usize, usize) {
        let page_num: usize = (row_num as usize) / ROWS_PER_PAGE;
        // let page: &Page = &self.pages[page_num];

        // TODO null check?

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        return (page_num, byte_offset);
    }
}

#[derive(Copy, Clone)]
struct Page {
    data: [u8; PAGE_SIZE],
}
