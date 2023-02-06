pub fn ends_with(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
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
}