pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Serialize, Deserialize)]
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
        let decoded: Option<Row> = bincode::deserialize(&raw_data[..]).unwrap();
        decoded.unwrap()
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

        let (page_index, offset) = self.row_slot(self.num_rows + 1);
        let mut page = self.pages[page_index];

        let bytes = row.serialize();
        println!("Bytes are: {:?}", bytes);

        let mut i = 0;
        for byte in bytes.iter() {
            page.data[offset + i] = *byte;
            i += 1
        }

        self.num_rows += 1;
        return true
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
