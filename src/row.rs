pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = 1;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug, Clone)]
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
}
