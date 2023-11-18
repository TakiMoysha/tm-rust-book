use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    pub pattern: String,
    pub path: std::path::PathBuf,
    pub num: Option<i32>,
    #[clap(short, long, default_value_t = false)]
    debug: bool,
} 

pub fn parse() -> CliArgs {
    CliArgs::parse()
}

