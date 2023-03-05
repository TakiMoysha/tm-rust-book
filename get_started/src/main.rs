mod os;
mod offtop;
mod random;
mod garden;
mod codewars;
mod rust_docs;

#[warn(unused_must_use)]
fn main() {
    let mut num = 1235_i64;

    println!("{:?}", num.to_be_bytes());
}
