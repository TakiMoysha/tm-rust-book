use rand::{thread_rng, Rng};

pub fn rnd_numbers() {
    let mut rng = thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        rnd_numbers();
    }
}