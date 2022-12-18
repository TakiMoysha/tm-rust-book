pub fn ends_with(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

/// Return same string without vowels
pub fn disemvowel(s: &str) -> String {
    let vowel = vec![b'a', b'e', b'i', b'o', b'u'];
    let mut w_s = s.as_bytes();

    let mut res: Vec<u8> = Vec::new();

    for (s_index, s_byte) in w_s.iter().enumerate() {
        if !vowel.contains(&s_byte.to_ascii_lowercase()) {
            res.push(*s_byte);
        }
    }
    return String::from_utf8(res).expect("Can't bytes to string");
}

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

/// Returns True if a triangle with such sides can exist
pub fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    (a + b > c) & (a + c > b) & (b + c > a)
}

/// The array is either entirely comprised of odd integers
/// or entirely comprised of even integers except for a single integer N.
pub fn find_outlier(values: &[i32]) -> i32 {
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

pub fn slice_string(s: &str) -> Vec<String> {
    let mut s_bytes: Vec<char> = s.chars().collect();

    if s_bytes.len() % 2 != 0 {
        s_bytes.push('_');
    }

    let mut res: Vec<String> = vec![];
    for i in 0..s_bytes.len() / 2 {
        let m = &s_bytes[i*2..i*2+2];
        res.push(m.iter().collect::<String>());
    }

    return res;
}

pub fn is_valid_ip(ip: &str) -> bool {
    println!("{ip:?}");
    let octets: Vec<&str> = ip.split(".").collect();
    if octets.len() != 4 {
        return false;
    }

    for octet in octets {
        if octet.len() > 1 && octet.starts_with("0") {
            return false;
        }

        match octet.parse::<u8>() {
            Ok(v) => (),
            Err(_) => return false,
        };
    }
    true
}

// pub fn slice_string_pro(s: &str) -> Vec<String> {
//     use itertools::Itertools;
//     let msg: Vec<char> = s
//         .chars()
//         .chunks(2)
//         .into_iter()
//         .map(|v| v)
//         .collect();
//     println!("{:?}", msg);
//     return vec![];
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_slice_string_pro() {
    //     assert_eq!(slice_string_pro("abcdef"), ["ab", "cd", "ef"]);
    //     assert_eq!(slice_string_pro("abcdefg"), ["ab", "cd", "ef", "g_"]);
    //     assert_eq!(slice_string_pro(""), [] as [&str; 0]);
    // }

    #[test]
    fn test_is_valid_ip() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));

        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
    }

    #[test]
    fn test_slice_string() {
        assert_eq!(slice_string("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(slice_string("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(slice_string(""), [] as [&str; 0]);
    }

    #[test]
    fn if_word_ending() {
        assert_eq!(true, ends_with("word", "rd"));
    }

    #[test]
    fn if_word_not_ending() {
        assert_eq!(false, ends_with("word", "ending"));
    }

    #[test]
    fn it_should_return_the_same_string_but_without_vowel() {
        let input_str = String::from("This website is for losers LOL!");
        let should = String::from("Ths wbst s fr lsrs LL!");

        assert_eq!(should, disemvowel(&input_str))
    }

    #[test]
    fn test_divisors_function() {
        assert_eq!(divisors(15), Ok(vec![3, 5]));
        assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
    }

    #[test]
    fn test_is_triangle() {
        assert_eq!(is_triangle(1, 2, 2), true);
        assert_eq!(is_triangle(7, 2, 2), false);
        assert_eq!(is_triangle(1, 2, 3), false);
        assert_eq!(is_triangle(1, 3, 2), false);
        assert_eq!(is_triangle(3, 1, 2), false);
        assert_eq!(is_triangle(5, 1, 2), false);
        assert_eq!(is_triangle(1, 2, 5), false);
        assert_eq!(is_triangle(2, 5, 1), false);
        assert_eq!(is_triangle(4, 2, 3), true);
        assert_eq!(is_triangle(5, 1, 5), true);
        assert_eq!(is_triangle(2, 2, 2), true);
        assert_eq!(is_triangle(-1, 2, 3), false);
        assert_eq!(is_triangle(1, -2, 3), false);
        assert_eq!(is_triangle(1, 2, -3), false);
        assert_eq!(is_triangle(0, 2, 3), false);
    }

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
