use std::env;

#[derive(Debug)]
pub struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}

pub fn parse() -> Result<CliArgs, &'static str> {
    let pattern = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();

    let args = CliArgs {
        pattern,
        path: std::path::PathBuf::try_from(path).unwrap(),
    };

    Ok(args)
}

