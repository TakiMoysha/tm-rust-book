use std::{env, fs};
use anyhow::{anyhow, Context};


pub fn run() -> anyhow::Result<()> {
    let file_name = env::args().nth(1).ok_or_else(|| anyhow!("You need to pass a file name"))?;
    let content = fs::read_to_string(&file_name)
        .with_context(|| format!("Could not read the file: '{}'", &file_name))?;

    Ok(())
}

pub fn main() {
    let result = run();

    if let Err(e) = result {
        println!("{}", e);
        std::process::exit(1);
    }

    println!("Done");
}
