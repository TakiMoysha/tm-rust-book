use std::env;
use std::process;

pub mod lib;
use lib as ai_lib;


pub fn start() {
    let config = ai_lib::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = ai_lib::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}