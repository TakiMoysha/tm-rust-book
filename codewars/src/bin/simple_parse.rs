pub fn get_age(input: &str) -> u32 {
    let age = input.chars().next().unwrap();
    age.to_digit(10).unwrap() as u32
}

pub fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_age("2 years old"), 2);
        assert_eq!(get_age("4 years old"), 4);
        assert_eq!(get_age("5 years old"), 5);
        assert_eq!(get_age("7 years old"), 7);
    }
}
