use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[clap(default_value = "Enter -h or --help. Meow!")]
    message: String,
    #[clap(long, default_value = "true")]
    catmode: Option<bool>,
    #[clap(short = 'd', long = "dead", default_value = "false")]
    dead: Option<bool>,
    #[clap(short = 'f', long = "file")]
    catfile: Option<std::path::PathBuf>,
    #[clap(short = 'i', long = "stdin", default_value = "false")]
    stdin: bool,
}

fn catsay(msg: &str, catmode: bool, dead: bool, catfile: Option<std::path::PathBuf>) -> Result<()> {
    if !catmode {
        println!("{msg}");
        return Ok(());
    }

    if msg.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
        return Ok(());
    }

    let eye = if dead { "x" } else { "o" };
    if catfile.is_some() {
        let path = &catfile.expect("Failed to read catfile");
        let cat_template = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read catfile: {}", path.display()))?;
        let eye = format!("{}", eye.red().bold());
        let cat_picture = cat_template.replace("{eye}", &eye);
        println!("<< {} >>", msg.bright_yellow().underline());
        println!("{cat_picture}");
    } else {
        println!("<< {} >>", msg.bright_yellow().underline());
        println!("  \\");
        println!("   \\");
        println!("    /\\_/\\");
        println!("   ( {eye} {eye} )");
        println!("   =( I )=");
    }

    Ok(())
}

fn main() {
    let options = Options::parse();
    let mut msg = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut msg).unwrap();
    } else {
        msg = options.message;
    };

    catsay(
        msg.as_str(),
        options.catmode.unwrap(),
        options.dead.unwrap(),
        options.catfile,
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn run_with_defaults() {
        Command::cargo_bin("catsay")
            .expect("binary exists")
            .assert()
            .success()
            .stdout(predicate::str::contains("Meow!"));
    }

    #[test]
    fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("catsay")
            .expect("binary exists")
            .args(&["-f", "no/such/file"])
            .assert()
            .failure();
        Ok(())
    }
}
