#[derive(Debug)]
pub struct ParsedData<'a> {
    pub header: u8,
    pub payload: &'a str,
}

impl ParsedData<'_> {
    pub fn parse(data: &[u8]) -> ParsedData {
        let header = data[0];

        let payload = std::str::from_utf8(&data[1..data.len()]).unwrap();

        ParsedData { header, payload }
    }
}
