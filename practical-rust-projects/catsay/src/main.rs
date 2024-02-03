use clap::Parser;
use colored::Colorize;

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
}

fn catsay(msg: &str, catmode: bool, dead: bool, catfile: Option<std::path::PathBuf>) {
    if !catmode {
        println!("{msg}");
        return;
    }

    if msg.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
        return;
    }

    let eye = if dead { "x" } else { "o" };
    if catfile.is_some() {
        let path = &catfile.expect("Failed to read catfile");
        let cat_template =
            std::fs::read_to_string(path).expect(&format!("Failed to read {path:?}"));
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
}

fn main() {
    let options = Options::parse();
    let use_catfile = options.catfile.is_some();
    catsay(
        options.message.as_str(),
        options.catmode.unwrap(),
        options.dead.unwrap(),
        options.catfile,
    );
}
