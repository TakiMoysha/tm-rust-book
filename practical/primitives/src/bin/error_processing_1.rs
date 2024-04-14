use std::fmt;

#[derive(Debug)]
pub enum MyError {
    CommandLineArgs,
    FileReadError(std::io::Error),
    ParsingError(String),
    Undefined(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::CommandLineArgs => write!(f, "You need to pass a file name"),
            MyError::FileReadError(e) => write!(f, "Could not read the file, by reason: {}", e),
            MyError::ParsingError(e) => write!(f, "Parsing error, by reason: {}", e),
            MyError::Undefined(e) => write!(f, "Undefined error: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

pub fn main() -> Result<(), MyError> {
    let file_name = std::env::args().nth(1).ok_or(MyError::CommandLineArgs)?;
    let content = std::fs::read_to_string(&file_name).map_err(MyError::FileReadError)?;

    for (indx, line) in content.lines().enumerate() {
        if line.starts_with("#error") {
            let err = format!("[InLine:{};Value:{}] Bad line.", indx, line);
            return Err(MyError::ParsingError(err.to_string()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_function() {
        let result = main();

        if result.is_err() {
            println!("{}", result.err().unwrap());
            std::process::exit(1);
        }
    }
}
