pub mod errors;

use errors::AppErrors;

#[derive(Debug)]
pub struct CliScope {
    pub temp_file: Option<std::fs::File>,
}

pub fn read_file(path: std::path::PathBuf) -> Result<String, AppErrors> {
    let res = std::fs::read_to_string(&path);
    match res {
        Ok(content) => Ok(content),
        Err(err) => Err(AppErrors::ReadFileError(err)),
    }
}

const TMP_FILE_PATH: &str = "temp.txt";

pub fn init_program_scope() -> CliScope {
    let temp_file = std::fs::File::create(TMP_FILE_PATH);
    let file = temp_file.unwrap_or_else(|_| {
        // stop_app_with_error(&AppErrors::InternalError);
        eprintln!("Can't create tmp file");
        std::process::exit(1);
    });

    return CliScope {
        temp_file: Some(file)
    }
}

pub fn destruct_program_scope() {
    if std::fs::metadata(TMP_FILE_PATH).is_ok() {
        let _ = std::fs::remove_file(TMP_FILE_PATH).is_err_and(|err| {
            eprintln!("Can't remove tmp file, reason: {}", err);
            true
        });
    }
}

pub fn stop_app_with_error(err: &AppErrors) {
    destruct_program_scope();

    eprintln!("Exit with error: {}", err);
    std::process::exit(1);
}

pub fn stop_app() {
    destruct_program_scope();
    std::process::exit(0);
}
