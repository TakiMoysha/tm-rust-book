use std::u8;

mod parser;

fn get_data() -> Vec<u8> {
    const DATA: [u8; 5] = [255, 't' as u8, 'e' as u8, 's' as u8, 't' as u8];
    DATA.to_vec()
}

fn main() {
    println!("Hello, from zero-copy-parser!");
    let buffer = get_data();

    let parsed_data = parser::ParsedData::parse(&buffer);
    println!("parsed_data: {:?}", parsed_data);
}
