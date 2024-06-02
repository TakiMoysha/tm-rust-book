#[derive(Debug)]
pub enum AppErrors {
    InternalError,
    ReadFileError(std::io::Error),
}

impl std::fmt::Display for AppErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppErrors::InternalError => write!(f, "Internal Error"),
            AppErrors::ReadFileError(e) => {
                write!(f, "Could not read the file, by reason: {}", e)
            }
        }
    }
}

impl std::error::Error for AppErrors {}
