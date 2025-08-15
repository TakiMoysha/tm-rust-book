pub fn find_outlier(values: &[i32]) -> i32 {
    /// The array is either entirely comprised of odd integers
    /// or entirely comprised of even integers except for a single integer N.
    fn is_even(value: i32) -> bool {
        value % 2 == 0
    }

    let mut even_count = 0;

    for i in 0..3 {
        if is_even(values[i]) {
            even_count += 1;
        }
    }
    let mut outlier = 0;

    if even_count >= 2 {
        outlier = *values.iter().find(|&x| x % 2 != 0).unwrap_or(&0);
    } else {
        outlier = *values.iter().find(|&x| x % 2 == 0).unwrap_or(&0);
    }

    outlier
}

pub fn find_outlier_pro(values: &[i32]) -> i32 {
    let sum: i32 = values.iter()
        .take(3)
        .map(|v| v.abs() % 2)
        .sum();

    let m = if sum == 1 || sum == 0 { 1 } else { 0 };

    values
        .iter()
        .find(|v| v.abs() % 2 == m)
        .map(|v| *v)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_outlier() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [206847684,1056521,7,17,1901,21104421,7,1,35521,1,7781];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}

