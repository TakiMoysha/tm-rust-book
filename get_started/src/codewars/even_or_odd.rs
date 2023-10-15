pub fn even_or_odd(number: i32) -> &'static str {
    return if number % 2 == 0 { "Even" } else { "Odd" };
}


#[cfg(test)]
mod sample_tests {
    use super::even_or_odd;
    
    fn do_test(number: i32, expected: &str) {
        let actual = even_or_odd(number);
        assert_eq!(actual, expected, "\nYour result (left) does not match the expected output (right) for the input {number:?}");
    }

    #[test]
    fn test_even_or_odd() {
        do_test(0, "Even");
        do_test(2, "Even");
        do_test(1, "Odd");
        do_test(-2, "Even");
        do_test(-1, "Odd");
    }
}
