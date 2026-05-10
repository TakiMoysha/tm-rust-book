use std::{env, fs, path::Path};

fn main() {
    let test_path = env::var("TEST_DIR").expect("TEST_DIR must be set");

    let data_dir = Path::new(&test_path).join("Data");

    println!("TEST_DIR {}: {}", data_dir.exists(), data_dir.display());

    // loading data-files
    let data_files: Vec<_> = fs::read_dir(data_dir)
        .expect("failed to read data dir")
        .filter_map(|entry| {
            if let Ok(e) = entry {
                if e.path().is_file() && e.path().extension().is_some_and(|ext| ext == "sbc") {
                    Some(e)
                } else {
                    None
                }
            } else {
                eprintln!("failed to read entry: {}", entry.unwrap_err());
                None
            }
        })
        .collect();

    println!("files: {:?}", data_files);
}
