use super::Person;

pub fn serialize(person: &Person) -> Result<String, Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string(person).unwrap();
    println!("Serialized: {}", serialized);
    Ok(serialized)
}
pub fn deserialize(person: &str) -> Result<Person, Box<dyn std::error::Error>> {
    let deserialized = serde_json::from_str(person).unwrap();
    println!("Deserialized: {:?}", deserialized);
    Ok(deserialized)
}

