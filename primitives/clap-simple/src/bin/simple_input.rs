use std::path::PathBuf;

use clap::{ArgAction, Command, arg, command, value_parser};
use env_logger::Builder;
use log::{debug, info};

fn main() {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug  "Turn debugging information on"))
        .subcommand(
            Command::new("test")
                .about("test subcommand")
                .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if matches.get_flag("debug") {
        Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    }

    match matches.get_one::<PathBuf>("config") {
        Some(name) => info!("Config file: {}", name.display()),
        None => info!("No config file provided, using default config"),
    }

    debug!("Values from input: {:?}", matches);
}
