use std;
use clap::Parser;

use cli_practice::{init_program_scope, read_file, stop_app, stop_app_with_error};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(long)]
    slice: Option<String>,
    #[arg(long)]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = AppArgs::parse();
    if args.name.is_some() {
        println!(
            "Hello, {}! You've been greeted {} times!",
            args.name.unwrap_or(String::from("%USERNAME%")),
            args.count
        );
    }

    let scope = init_program_scope();

    if (args.file).is_some() && (args.slice).is_some() {
        let res_content = read_file(args.file.unwrap());       
        if res_content.is_err() {
            stop_app_with_error(res_content.as_ref().unwrap_err());
        }

        let slice = &args.slice.unwrap();
        res_content.unwrap().lines().enumerate().for_each(|(indx, line) | {
            if line.contains(slice) {
                println!("{}: {}", indx, line);
            }
        });

    }

    stop_app();
}
