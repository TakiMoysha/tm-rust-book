use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt.")]
pub struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short="v", long="velocity", default_value="42")]
    speed: f64,
    #[structopt(parse(from_os_str))]
    input: PathBuf,

}

fn main() {
    let _opt = Opt::from_args();
    println!("{:?}", _opt);
}