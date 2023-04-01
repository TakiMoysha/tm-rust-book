fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(adder(2, 2), 4);
    }

    #[test]
    fn test_panic() {
        panic!("should panic");
    }
}