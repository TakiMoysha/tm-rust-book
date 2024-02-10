// use anyhow::{Context, Result};
// use clap::Parser;
// use colored::Colorize;
use cursive::traits::Nameable;
use cursive::event::Key;
use cursive::views::{Dialog, TextView, Checkbox, EditView, ListView};
use cursive::Cursive;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Options {
//     #[clap(default_value = "Enter -h or --help. Meow!")]
//     message: String,
//     #[clap(long, default_value = "true")]
//     catmode: Option<bool>,
//     #[clap(short = 'd', long = "dead", default_value = "false")]
//     dead: Option<bool>,
//     #[clap(short = 'f', long = "file")]
//     catfile: Option<std::path::PathBuf>,
//     #[clap(short = 'i', long = "stdin", default_value = "false")]
//     stdin: bool,
// }
//
// fn catsay(msg: &str, catmode: bool, dead: bool, catfile: Option<std::path::PathBuf>) -> Result<()> {
//     if !catmode {
//         println!("{msg}");
//         return Ok(());
//     }
//
//     if msg.to_lowercase() == "woof" {
//         eprintln!("A cat shouldn't bark like a dog.");
//         return Ok(());
//     }
//
//     let eye = if dead { "x" } else { "o" };
//     if catfile.is_some() {
//         let path = &catfile.expect("Failed to read catfile");
//         let cat_template = std::fs::read_to_string(path)
//             .with_context(|| format!("Could not read catfile: {}", path.display()))?;
//         let eye = format!("{}", eye.red().bold());
//         let cat_picture = cat_template.replace("{eye}", &eye);
//         println!("<< {} >>", msg.bright_yellow().underline());
//         println!("{cat_picture}");
//     } else {
//         println!("<< {} >>", msg.bright_yellow().underline());
//         println!("  \\");
//         println!("   \\");
//         println!("    /\\_/\\");
//         println!("   ( {eye} {eye} )");
//         println!("   =( I )=");
//     }
//
//     Ok(())
// }

struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Dead?", Checkbox::new().with_name("dead"))
            )
            .button("Submit", |s| {
                let msg = s.call_on_name("message", |t: &mut EditView| t.get_content()).unwrap();
                let is_dead = s.call_on_name("dead", |t: &mut Checkbox| t.is_checked()).unwrap();
                let options = CatsayOptions { message: &msg, dead: is_dead, };
                result_step(s, &options)
            })
    );
}

fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "<< {msg} >>
        \\
         \\
          /\\_/\\
         ( {eye} {eye} )
         =( I )=",
        msg = options.message, eye = eye
    );

    siv.pop_layer();
    siv.add_layer(
        Dialog::text(cat_text)
            .title("The cat says: ")
            .button("OK", |s| s.quit())
    );
}

fn main() {
    let mut siv = cursive::default();
    input_step(&mut siv);
    siv.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    // #[test]
    // fn run_with_defaults() {
    //     Command::cargo_bin("catsay")
    //         .expect("binary exists")
    //         .assert()
    //         .success()
    //         .stdout(predicate::str::contains("Meow!"));
    // }
    //
    // #[test]
    // fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    //     Command::cargo_bin("catsay")
    //         .expect("binary exists")
    //         .args(&["-f", "no/such/file"])
    //         .assert()
    //         .failure();
    //     Ok(())
    // }
}
