/// Return all divisors of "integer" except 1 and itself
pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut res: Vec<u32> = Vec::new();

    for i in 2..integer {
        if (f32::sqrt(integer as f32) < (i as f32)) & (res.len() == 0) {
            return Err(format!("{integer} is prime"));
        }

        if integer % i == 0 {
            res.push(i);
        }
    }
    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors_function() {
        assert_eq!(divisors(15), Ok(vec![3, 5]));
        assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
    }

}

