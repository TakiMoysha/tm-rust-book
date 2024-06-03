use std;
use clap::Parser;

use cli_practice::{init_program_scope, read_file, stop_app, stop_app_with_error};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[arg(long)]
    msg: Option<String>,
}

fn main() {
    let args = AppArgs::parse();
    let scope = init_program_scope();

    stop_app();
}
