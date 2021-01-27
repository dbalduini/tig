use sha1::Sha1;

pub const NULL_BYTE: char = '\u{0000}';

pub struct Blob {
    data: String,
}

impl Blob {
    pub fn new(data: String) -> Self {
        Self {
            data
        }
    }

    pub fn serialize(self) -> String {
        self.data
    }

    pub fn deserialize(&mut self, data: String) {
        self.data = data;
    }

    pub fn display(&self) -> &str {
        return self.data.as_str();
    }
}

// Return the key-value pair
pub fn hash_object(obj: Blob) -> (String, String) {
    let data = obj.serialize();
    // add header
    let res = format!(
        "{}{}{}{}{}",
        "blob",
        " ", // space separator
        &data.chars().count(),
        NULL_BYTE,
        &data
    );
    // compute sha1 hash
    let result: &[u8] = res.as_bytes();
    (Sha1::from(result).digest().to_string(), res)
}

pub fn parse_object(buff: String) -> Blob {
    // read type
    let x = buff.find(' ').unwrap();
    let fmt = &buff[..x];

    dbg!(fmt);

    // read size
    let y = buff.find(NULL_BYTE).unwrap();
    let size = buff[x + 1..y].parse::<usize>().unwrap();

    if size != buff.chars().count() - y - 1 {
        panic!("Malformed object");
    }
    let data = &buff[y + 1..];

    Blob::new(data.to_string())
}
