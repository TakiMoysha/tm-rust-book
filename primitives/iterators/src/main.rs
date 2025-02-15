use std::iter::{ Iterator, IntoIterator };

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_stdlib_iterator_trait() {}
}
