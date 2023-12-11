// for clear
use lex;

// [] - release lua types
//     [] - release lua nil
//     [] - release lua boolean
//     [] - release lua number
//     [] - release lua string
//     [] - release lua userdata
//     [] - release lua function
//     [] - release lua thread
//     [] - release lua table (like a class)
//

pub mod lib {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn load_script_from_file(file_name: &str) -> String {
        let file_path = Path::new(file_name);
        if !file_path.exists() {
            panic!("File {file_name} not found!");
        }

        let mut file = File::open(file_path).unwrap();
        let mut script_content = String::new();
        Read::read_to_string(&mut file, &mut script_content).unwrap();

        script_content
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Location {
        col: i32,
        line: i32,
        index: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TokenKind {
        Identifier,
        Syntax,
        Keyword,
        Number,
        Operator,
    }

    #[derive(Debug, Clone)]
    pub struct Token {
        pub value: String,
        pub kind: TokenKind,
        pub loc: Location,
    }

    impl Location {
        fn increment(&self, newline: bool) -> Location {
            if newline {
                return Location {
                    index: self.index + 1,
                    col: 0,
                    line: self.line + 1,
                };
            }

            Location {
                index: self.index + 1,
                col: self.col + 1,
                line: self.line,
            }
        }

        pub fn debug<S: Into<String>>(&self, raw: &[char], msg: S) -> String {
            let mut line = 0;
            let mut line_str = String::new();

            for c in raw {
                if *c == '\n' {
                    line += 1;
                    if !line_str.is_empty() {
                        break;
                    };
                    continue;
                }

                if self.line == line {
                    line_str.push_str(&c.to_string());
                }
            }

            let space = " ".repeat(self.col as usize);
            format!("{}\n\n{line_str}\n{space}^ here", msg.into())
        }
    }

    pub fn lex(s: &[char]) -> Result<Vec<Token>, String> {
        let mut loc = Location {
            col: 0,
            index: 0,
            line: 0,
        };
        let size = s.len();
        let mut tokens: Vec<Token> = vec![];

        let lexers = [
            lex_keyword,
            lex_identifier,
            lex_number,
            lex_operator,
            lex_syntax,
        ];

        'outer: while loc.index < size {
            loc = eat_whitespace(s, loc);
            if loc.index == size {
                break;
            }

            for lexer in lexers {
                let res = lexer(s, loc);
                if let Some((t, next_loc)) = res {
                    loc = next_loc;
                    tokens.push(t);
                    continue 'outer;
                }
            }

            Ok(tokens)
        }
    }

    fn eat_whitespace(raw: &[char], initial_loc: Location) -> Location {
        let mut c = raw[initial_loc.index];
        let mut next_loc = initial_loc;
        while [' ', '\n', '\r', '\t'].contains(&c) {
            next_loc = next_loc.increment(c == '\n');
            if next_loc.index == raw.len() {
                break;
            }
            c = raw[next_loc.index];
        }
        next_loc
    }

    fn lex_number(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
        let mut ident = String::new();
        let mut next_loc = initial_loc;
        let mut c = raw[initial_loc.index];
        while c.is_digit(10) {
            ident.push_str(&c.to_string());
            next_loc = next_loc.increment(false);
            c = raw[next_loc.index];
        }

        if !ident.is_empty() {
            Some((
                Token {
                    value: ident,
                    loc: initial_loc,
                    kind: TokenKind::Number,
                },
                next_loc,
            ))
        } else {
            None
        }
    }

    fn lex_identifier(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
        let mut ident = String::new();
        let mut next_loc = initial_loc;
        let mut c = raw[initial_loc.index];
        while c.is_alphanumeric() || c == '_' {
            ident.push_str(&c.to_string());
            next_loc = next_loc.increment(false);
            c = raw[next_loc.index];
        }

        if ident.len() > 0 && !ident.chars().next().unwrap().is_digit(10) {
            Some((
                Token {
                    value: ident,
                    loc: initial_loc,
                    kind: TokenKind::Identifier,
                },
                next_loc,
            ))
        } else {
            None
        }
    }

    pub fn lex_keyword(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
        let syntax = ["function", "end", "if", "then", "local", "return"];

        let mut next_loc = initial_loc;
        let mut value = String::new();
        'outer: for possible_syntax in syntax {
            let mut c = raw[initial_loc.index];
            next_loc = initial_loc;
            while c.is_alphanumeric() || c == '_' {
                value.push_str(&c.to_string());
                next_loc = next_loc.increment(false);
                c = raw[next_loc.index];

                let n = next_loc.index - initial_loc.index;
                if value != possible_syntax[..n] {
                    value = String::new();
                    continue 'outer;
                }
            }

            if value.len() < possible_syntax.len() {
                value = String::new();
                continue;
            }

            break;
        }

        if value.is_empty() {
            return None;
        }

        if next_loc.index < raw.len() - 1 {
            let next_c = raw[next_loc.index];
            if next_c.is_alphanumeric() || next_c == '_' {
                return None;
            }
        }

        Some((
            Token {
                value,
                loc: initial_loc,
                kind: TokenKind::Keyword,
            },
            next_loc,
        ))
    }

    fn lex_syntax(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
        let syntax = [";", "=", "(", ")", ","];

        for possible_syntax in syntax {
            let c = raw[initial_loc.index];
            let next_loc = initial_loc.increment(false);

            if c.to_string() == possible_syntax {
                return Some((
                    Token {
                        value: possible_syntax.to_string(),
                        loc: initial_loc,
                        kind: TokenKind::Syntax,
                    },
                    next_loc,
                ));
            }
        }

        None
    }

    fn lex_operator(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
        // let operators = ["+", "-", "*", "/", "%", "^", "==", "~=", "<", ">", "<=", ">="];
        let operators = ["+", "-", "<"];

        for possible_syntax in operators {
            let c = raw[initial_loc.index];
            let next_loc = initial_loc.increment(false);

            if possible_syntax == c.to_string() {
                return Some((
                    Token {
                        value: possible_syntax.to_string(),
                        loc: initial_loc,
                        kind: TokenKind::Operator,
                    },
                    next_loc,
                ));
            }
        }

        None
    }
}

// use lib;
// use proto_interpreter;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lua_content() {
        let file_name = "scripts/hello_world.lua";
        let content = lib::load_script_from_file(file_name);
    }

    #[test]
    fn test_bianry_search() {
        let file_name = "scripts/binary_search.lua";
        let content = lib::load_script_from_file(file_name);
    }

    #[test]
    fn test_fib() {
        let file_name = "scripts/fib.lua";
        let content = lib::load_script_from_file(file_name);
    }
}
