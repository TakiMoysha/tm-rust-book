pub mod practical_error;
pub mod practical_anyhow;
pub mod my_error;

pub mod practical_thiserror {
    use std;
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum MyError {
        #[error("Something went wrong")]
        Undefined,

        #[error("You need to pass a file name")]
        CommandLineArgs,

        #[error("Could not read the file, by reason: {0}")]
        FileOpenError(std::io::Error),
    }

    pub fn main() -> Result<(), MyError> {
        let file_name = std::env::args().nth(1).ok_or(MyError::CommandLineArgs)?;
        let content = std::fs::read_to_string(&file_name).map_err(MyError::FileOpenError)?;

        Ok(())
    }

    pub fn run_demo() {
        let result = main();

        if let Err(err) = result {
            println!("{:?}", err);
            std::process::exit(1);
        }

    }
}

pub fn main() {
    practical_thiserror::run_demo();
}
