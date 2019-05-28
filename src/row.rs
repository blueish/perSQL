pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = 1;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

// const ID_OFFSET: usize = 0;
// const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
// const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    id: u8,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u8, username: String, email: String) -> Row {
        Row {
            id: id,
            username: username,
            email: email,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        // Here we'll pad the extra chars to make rows identical

        let encoded = bincode::serialize(self).unwrap();
        dbg!(encoded.len());

        encoded
    }

    pub fn deserialize(raw_data: Vec<u8>) -> Row {
        dbg!(raw_data.len());
        let decoded: Result<Option<Row>, bincode::Error> = bincode::deserialize(&raw_data[..]);
        dbg!(&decoded);
        decoded.unwrap().unwrap()
    }
}
