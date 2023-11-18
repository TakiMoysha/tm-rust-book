use std::{env, net::IpAddr};

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn parse_args(args: &Vec<String>) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Bad input arguments");
        }

        let f = args[1].clone();

        Ok(())
    }
}
fn parse_args(args: &Vec<String>) -> Result<(), ()> {
    let program = args[0].clone();

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();



    println!("{args:?}");
}
