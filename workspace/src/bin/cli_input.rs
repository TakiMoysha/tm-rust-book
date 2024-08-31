use clap::Parser;

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

fn main() {
    let options = Options::parse();
    let msg = options.message;
    let catmode = options.catmode;
    println!("DEBUG:");
    println!("\t{}", msg);
    println!("\t{:?}", catmode);
    println!("\t{:?}", options.dead);
    println!("\t{:?}", options.catfile);
    println!("\t{:?}", options.stdin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_correct_parsing_input() {
        let options = Options::parse();
    }
}
