mod errors;

use errors::AppErrors;

pub fn read_file(path: std::path::PathBuf) -> Result<String, AppErrors> {
    let res = std::fs::read_to_string(&path);
    match res {
        Ok(content) => Ok(content),
        Err(err) => Err(AppErrors::ReadFileError(err)),
    }
}

const TMP_FILE_PATH: &str = "temp.txt";

pub fn init_program_scope() {
    let temp_file = std::fs::File::create(TMP_FILE_PATH);

    if temp_file.is_err() {
        stop_app_with_error(&AppErrors::ReadFileError(temp_file.unwrap_err()));
    }
}

pub fn destruct_program_scope() {
    if std::fs::metadata(TMP_FILE_PATH).is_ok() {
        let _ = std::fs::remove_file(TMP_FILE_PATH).is_err_and(|err| {
            println!("Can't remove tmp file, reason: {}", err);
            true
        });
    }
}

pub fn stop_app_with_error(err: &AppErrors) {
    destruct_program_scope();

    match err {
        AppErrors::InternalError => {
            println!("Internal Error");
        }
        AppErrors::ReadFileError(e) => {
            println!("Could not read the file, by reason: {}", e);
        }
    }

    std::process::exit(1);
}

pub fn stop_app() {
    destruct_program_scope();
    std::process::exit(0);
}
