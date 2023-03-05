pub fn count_bits(num: i64) -> i64 {
    // println!(num.to_bytes());
    return 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_counting() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(1), 1);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(5), 2);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(9), 2);
        assert_eq!(count_bits(10), 2);
        assert_eq!(count_bits(14), 3);
        assert_eq!(count_bits(254), 7);
    }
}

