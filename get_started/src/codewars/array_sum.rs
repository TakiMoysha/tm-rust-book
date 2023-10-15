pub fn slice_plus_slice(slice1: &[i32], slice2: &[i32]) -> i32 {
    // slice1.iter().sum::<i32>() + slice2.iter().sum::<i32>()
    slice1.iter().chain(slice2).sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(slice_plus_slice(&vec![1], &vec![4]), 5);
        assert_eq!(slice_plus_slice(&vec![1, 2, 3], &vec![4, 5, 6]), 21);
        assert_eq!(slice_plus_slice(&vec![-1, -2, -3], &vec![-4, -5, -6]), -21);
    }
}
