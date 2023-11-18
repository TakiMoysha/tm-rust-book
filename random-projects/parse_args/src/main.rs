use anyhow::{Context, Result};
use clap::Parser;

mod clap_parse;

fn main() -> Result<()> {
    let args: clap_parse::CliArgs = clap_parse::CliArgs::parse();

    let file_content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Failed to read file <{}>", args.path.display()))?;

    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    println!("{args:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[derive(Debug)]
    struct TakiParseDemoError(String);

    #[test]
    #[should_panic]
    fn file_not_found_with_context() {
        let path = "not_exist.txt";
        std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file <{}>", path)).unwrap();
    }

    #[test]
    #[should_panic]
    fn file_not_found_with_custom_error() {
        let path = "not_exist.txt";
        std::fs::read_to_string(path)
            .map_err(|err| TakiParseDemoError(format!("Error reading <{}>: {}", path, err))).unwrap();
    }
}
