use std::{
    fs::File,
    io::{BufWriter, Write},
    str::Bytes,
};

// use std::iter::Map;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u8,
    address: String,
    phones: Vec<String>,
}

fn get_person() -> Person {
    Person {
        name: "John Doe".to_owned(),
        age: 43,
        address: "unknown".to_owned(),
        phones: vec!["+44 1234567".to_owned(), "+44 2345678".to_owned()],
    }
}

pub mod json {
    use super::Person;
    use serde::Serialize;

    pub fn serialize<T>(obj: &T) -> Result<String, Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(obj).unwrap();
        Ok(serialized)
    }

    pub fn p_deserialize(person: &str) -> Result<Person, Box<dyn std::error::Error>> {
        let deserialized = serde_json::from_str(person).unwrap();
        Ok(deserialized)
    }
}

fn export_data(data: &Vec<Person>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let serialized_persons = data
        .into_iter()
        .map(|person| json::serialize(person))
        .collect::<Result<Vec<String>, Box<dyn std::error::Error>>>()
        .unwrap();

    let output_file = File::create("data/persons.json").unwrap();
    let mut file_buffer = BufWriter::new(output_file);
    file_buffer.write_all(b"[")?;
    file_buffer.write_all(serialized_persons.join("\n").as_bytes())?;
    file_buffer.write_all(b"]")?;
    Ok(serialized_persons)
}

fn main() {
    let person = get_person();
    let person_string = json::serialize(&person).unwrap();
    let person = json::p_deserialize(&person_string).unwrap();

    let data = vec![person];
    export_data(&data).unwrap();
}
