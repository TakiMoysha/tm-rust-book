use csv;
use std::fs::File;
use std::path::Path;
use std::error::Error;

const FILES: [&'static str; 2] = ["faithful.csv", "snakes_count_100.csv"];

struct RuntimeOpts {
    file: String,
}


fn parse_input_args() -> RuntimeOpts {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Bad arguments");
    }

    RuntimeOpts {
        file: args[1].clone(),
    }
}

fn file_handler(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut file_reader = csv::Reader::from_reader(file);
    println!("{:?}", file_reader.headers()?);
    for res in file_reader.records() {
        println!("{:?}", res?);
    }
    Ok(())
}

fn main() {
    let opts = parse_input_args();
    let file_path = Path::new(&opts.file);
    if !file_path.exists() {
        panic!("File does not exist");
    }
    println!("File: {}", file_path.display());
    let file_content = file_handler(file_path).unwrap();
}
