use std::{env, net::IpAddr, ops::Index};

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

        let pre_args = {};
        args.iter().enumerate().for_each(|(index, value)| {
            value
        });
        let bind_index = args.("-b");

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
