pub fn slice_string(s: &str) -> Vec<String> {
    let mut s_bytes: Vec<char> = s.chars().collect();

    if s_bytes.len() % 2 != 0 {
        s_bytes.push('_');
    }

    let mut res: Vec<String> = vec![];
    for i in 0..s_bytes.len() / 2 {
        let m = &s_bytes[i * 2..i * 2 + 2];
        res.push(m.iter().collect::<String>());
    }

    return res;
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

    #[test]
    fn test_slice_string() {
        assert_eq!(slice_string("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(slice_string("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(slice_string(""), [] as [&str; 0]);
    }

    // #[test]
    // fn test_slice_string_pro() {
    //     assert_eq!(slice_string_pro("abcdef"), ["ab", "cd", "ef"]);
    //     assert_eq!(slice_string_pro("abcdefg"), ["ab", "cd", "ef", "g_"]);
    //     assert_eq!(slice_string_pro(""), [] as [&str; 0]);
    // }
}