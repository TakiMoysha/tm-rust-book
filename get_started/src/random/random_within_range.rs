use rand::distr::{Distribution, Uniform};
use rand::Rng;

pub fn random_within_range() {
    let mut rng = rand::rng();

    println!("Integer: {}", rng.random_range(0..10));
    println!("Float: {}", rng.random_range(0.0..10.0));

    let mut rng = rand::rng();
    let die = Uniform::try_from(1..=7).unwrap();

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
