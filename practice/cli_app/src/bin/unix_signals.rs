use std;
use std::env;
use clap::Parser;

use cli_practice::errors::AppErrors;
use cli_practice::{init_program_scope, stop_app, stop_app_with_error};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[arg(long)]
    msg: Option<String>,
}

fn check_work_condition() -> bool {
    if ["macos", "linux"].contains(&env::consts::OS) {
        return false;
    };

    true
}

fn main() {
    if check_work_condition() {
        stop_app_with_error(&AppErrors::NotSupportedOS);
    }

    let args = AppArgs::parse();
    let scope = init_program_scope();

    println!("Hello, {:?}!", args.msg.unwrap_or("World".to_string()));
    println!("Scope of this program is: {:?}", scope);
    stop_app();
}
