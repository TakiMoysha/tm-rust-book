use std::io;
use std::cmp::Ordering;
use rand::{ thread_rng, Rng };

pub fn guessing_game() {
    let mut rng = thread_rng();
    let secret_number: u8 = rng.gen_range(0..=50);

    // loop {
    //     print!("Enter number: ");
    //     let mut buffer = String::new();

    //     io::stdin()
    //         .read_line(&mut buffer)
    //         .expect("Failed te read line");

    //     if (buffer.eq("quit")) {
    //         break;
    //     }

    //     let buffer: u8 = buffer.trim().parse() {
    //         Ok(num) => println!("{}", num),
    //         Err(_) => continue,
    //     };

    //     //  {
    //     //     Ok(num) => println!("num"),
    //     //     Err(_) => pritn!("ne num"),
    //     // };

    //     match buffer.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("Great! You win!!!");
    //             break;
    //         }
    //     }
    // }
}
