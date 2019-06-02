use std::fmt;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Copy, Clone)]
pub struct Row {
    id: u32,
    username: [u8; USERNAME_SIZE],
    email: [u8; EMAIL_SIZE],
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.id,
            string_from_serialized(self.username.to_vec()),
            string_from_serialized(self.email.to_vec()),
        )
    }
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Row {
        let mut u = [0; USERNAME_SIZE];
        u[..username.bytes().len()].copy_from_slice(username.as_bytes());

        let mut e = [0; EMAIL_SIZE];
        e[..email.bytes().len()].copy_from_slice(email.as_bytes());
        Row {
            id: id,
            username: u,
            email: e,
        }
    }

    pub fn serialize_row(&self) -> Vec<u8> {
        let mut serialized = Vec::new();

        let id_bytes: [u8; ID_SIZE] = u32_to_u8(self.id.to_be());

        serialized.extend_from_slice(&id_bytes);
        serialized.extend_from_slice(&self.username);
        serialized.extend_from_slice(&self.email);

        if serialized.len() != ROW_SIZE {
            panic!(
                "serialized len was {}, expected {}",
                serialized.len(),
                ROW_SIZE
            );
        }

        serialized
    }

    pub fn deserialize_row(v: Vec<u8>) -> Row {
        if v.len() != ROW_SIZE {
            panic!("vec size was {}, expected {}", v.len(), ROW_SIZE);
        }

        let mut id_bytes: [u8; ID_SIZE] = [0; ID_SIZE];

        id_bytes.copy_from_slice(&v[0..ID_SIZE]);
        let id: u32 = u8_to_u32(id_bytes);

        let mut username: [u8; USERNAME_SIZE] = [0; USERNAME_SIZE];
        username.copy_from_slice(&v[USERNAME_OFFSET..EMAIL_OFFSET]);

        let mut email: [u8; EMAIL_SIZE] = [0; EMAIL_SIZE];
        email.copy_from_slice(&v[EMAIL_OFFSET..]);

        Row {
            id,
            username,
            email,
        }
    }
}

fn u8_to_u32(bytes: [u8; 4]) -> u32 {
    let mut acc = 0;
    acc += (bytes[0] as u32) << 24;
    acc += (bytes[1] as u32) << 16;
    acc += (bytes[2] as u32) << 8;
    acc += bytes[3] as u32;

    acc
}

fn u32_to_u8(int: u32) -> [u8; 4] {
    let mut res: [u8; 4] = [0; 4];
    let first = ((int & 0xff000000) >> 24) as u8;
    let second = ((int & 0x00ff0000) >> 16) as u8;
    let third = ((int & 0x0000ff00) >> 8) as u8;
    let fourth = (int & 0x000000ff) as u8;

    res[0] = first;
    res[1] = second;
    res[2] = third;
    res[3] = fourth;

    res
}

fn string_from_serialized(bytes: Vec<u8>) -> String {
    let mut without_nulls = vec![];

    for byte in bytes.iter() {
        if *byte != 0 {
            without_nulls.push(*byte);
        } else {
            return String::from_utf8(without_nulls).expect("Corrupted string from serialized row");
        }
    }

    return String::from_utf8(without_nulls).expect("Corrupted string from serialized row");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_to_u8_lower() {
        let mut expected: [u8; 4] = [0; ID_SIZE];
        expected[3] = 1;

        assert_eq!(u32_to_u8(1), expected);

        expected[2] = 1;

        assert_eq!(u32_to_u8(0x101), expected);

        expected[3] = 0;

        assert_eq!(u32_to_u8(std::u8::MAX as u32 + 1), expected);
    }

    #[test]
    fn u32_to_u8_higher() {
        let mut expected: [u8; 4] = [std::u8::MAX; ID_SIZE];

        assert_eq!(u32_to_u8(std::u32::MAX), expected);
        expected[0] = 0;
        expected[2] = 0;
        expected[3] = 0;

        assert_eq!(u32_to_u8(0x00ff0000), expected);
    }

    #[test]
    fn u32_to_u8_and_back() {
        assert_eq!(1, u8_to_u32(u32_to_u8(1)));
        assert_eq!(255, u8_to_u32(u32_to_u8(255)));
        assert_eq!(256, u8_to_u32(u32_to_u8(256)));
        assert_eq!(0xff000000, u8_to_u32(u32_to_u8(0xff000000)));
        assert_eq!(0xffffffff, u8_to_u32(u32_to_u8(0xffffffff)));
        assert_eq!(0, u8_to_u32(u32_to_u8(0)));
    }

    #[test]
    fn serialize() {
        let r = Row::new(1, String::from("ab"), String::from("cd"));

        let mut expected = vec![0; ROW_SIZE];
        expected[0] = 1; // the id to be 1

        expected[USERNAME_OFFSET] = 97; // the first chars to be 97, 98
        expected[USERNAME_OFFSET + 1] = 98;

        expected[EMAIL_OFFSET] = 99; // the first chars to be 97, 98
        expected[EMAIL_OFFSET + 1] = 100;

        assert_eq!(r.serialize_row(), expected);
    }

    #[test]
    fn deserialize() {
        let r = Row::new(1, String::from("ab"), String::from("cd"));

        let mut expected = vec![0; ROW_SIZE];
        expected[3] = 1; // the id to be 1

        expected[USERNAME_OFFSET] = 97; // the first chars to be 97, 98
        expected[USERNAME_OFFSET + 1] = 98;

        expected[EMAIL_OFFSET] = 99; // the first chars to be 97, 98
        expected[EMAIL_OFFSET + 1] = 100;

        assert_eq!(
            format!("{:?}", Row::deserialize_row(expected)),
            format!("{:?}", r)
        );
    }

    #[test]
    fn simple_string_from_serialized() {
        let bytes: Vec<u8> = vec![97, 98];
        assert_eq!("ab", string_from_serialized(bytes));
    }

    #[test]
    fn string_with_nulls_from_serialized() {
        let bytes: Vec<u8> = vec![99, 98, 0, 0];
        assert_eq!("cb", string_from_serialized(bytes));
    }
}
