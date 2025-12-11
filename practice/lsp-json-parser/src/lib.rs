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
    use tree_sitter::{Parser, Query, QueryCursor};
    use tree_sitter_json;

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

    impl Token {
        pub fn from_char(c: char) -> Token {
            match c {
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                ',' => Token::Comma,
                ':' => Token::Colon,
                _ => todo!("unhandled char: {}", c),
            }
        }
    }

    pub fn tokenize(input: impl AsRef<str>) -> Result<(), String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .expect("Error loading json grammar");

        let tree = parser.parse(input.as_ref(), None).unwrap();

        println!("{:#?}", tree.root_node());
        todo!();
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::*;

        // #[rstest]
        // #[case(",", vec![Token::Comma])]
        // #[case("{}", vec![Token::LeftBrace, Token::RightBrace])]
        // #[case("[]", vec![Token::LeftBracket, Token::RightBracket])]
        // fn should_tokenize_comma(#[case] input: &str, #[case] expected: Vec<Token>) {
        //     let actual = tokenize(input);
        //     assert_eq!(actual, expected);
        // }

        #[rstest]
        #[case("{}")]
        #[case("{ \"nums\": [1,2,3], \"str\": \"hello\", \"bool\": true }")]
        #[case("{ \"human\": { \"name\": \"John\", \"age\": 30 } }")]
        fn target(#[case] input: &str) {
            let tokens = tokenize(input);
            todo!();
        }
    }
}
