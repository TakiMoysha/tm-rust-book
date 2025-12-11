use std::collections::HashMap;

/// JSON supported value types and how to represent them
/// where Null - no associated value
pub enum TValueType {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<TValueType>),
    Object(HashMap<String, TValueType>),
}

mod tokenize {
    #[derive(Debug, PartialEq)]
    pub enum Token {
        LeftBrace,
        RightBrace,
        LeftBracket,
        RightBracket,
        Comma,
        Colon,
        Null,
        False,
        True,
        Number(f64),
        String(String),
    }

    pub fn tokenize(input: impl AsRef<str>) -> Vec<Token> {
        let tokens = Vec::new();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::*;

        #[rstest]
        #[case(",", vec![Token::Comma])]
        #[case("{}", vec![Token::LeftBrace, Token::RightBrace])]
        #[case("[]", vec![Token::LeftBracket, Token::RightBracket])]
        fn should_tokenize_comma(#[case] input: &str, #[case] expected: Vec<Token>) {
            let actual = tokenize(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn target() {
            let test_data = "{ \"nums\": [1,2,3], \"str\": \"hello\", \"bool\": true}";
            let tokens = tokenize(test_data);
            todo!();
        }
    }
}
