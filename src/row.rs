pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

// const ID_OFFSET: usize = 0;
// const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
// const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

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
        println!("{:?}", raw_data.to_vec());
        let decoded: Result<Option<Row>, bincode::Error> = bincode::deserialize(&raw_data[..]);
        // let decoded: Option<Row> = bincode::deserialize(&raw_data[..]).unwrap();
        dbg!(&decoded);
        decoded.unwrap().unwrap()
    }
}
