
pub fn ends_with(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

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


#[cfg(test)]
mod tests {
    use super::*;

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
}
