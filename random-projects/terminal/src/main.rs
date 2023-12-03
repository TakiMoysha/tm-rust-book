use std::{io::{self, Write}, thread};
use indicatif;

fn mock_fn() {
    thread::sleep(std::time::Duration::from_millis(50));
}

fn simple_test() {
    println!("Now, println!");
    for i in 0..100000 {
        println!("{}", i);
    }

    println!("Now, stdout -> write");
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 0..100000 {
        writeln!(handle, "{}", i).unwrap();
    }
    handle.flush().unwrap();
}

fn progress_bar() {
    const N: u64 = 75;
    let pb = indicatif::ProgressBar::new(N);
    for i in 0..N {
        mock_fn();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("All Done!");
}

fn main() {
    progress_bar();
}
