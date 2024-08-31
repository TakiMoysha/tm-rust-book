#[derive(Debug)]
pub enum AppErrors {
    InternalError(Option<String>),
    ReadFileError(std::io::Error),
    NotSupportedOS,
}

impl std::fmt::Display for AppErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppErrors::InternalError(e) => {
                write!(f, "{}", e.as_ref().unwrap_or(&"Internal Error".to_string()))
            }
            AppErrors::ReadFileError(e) => {
                write!(f, "Could not read the file, by reason: {}", e)
            }
            AppErrors::NotSupportedOS => {
                write!(f, "Your operation system is not supported")
            }
        }
    }
}

impl std::error::Error for AppErrors {}
