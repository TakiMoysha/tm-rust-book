use std::iter::FromIterator;
use std::iter::IntoIterator;

struct IterLen(usize);

impl FromIterator<T> for IterLen {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        IterLen(9)
    }
}

pub fn run() { 
    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    let doubled = a.iter().map(|&x| x * 2).collect::<Vec<i32>>();
    let doubled = a.iter().map(|&x| x * 2).collect::<Vec<_>>();

    let c: IterLen = a.into_iter().collect();
    println!("My IterLen {:?}", c);
}
