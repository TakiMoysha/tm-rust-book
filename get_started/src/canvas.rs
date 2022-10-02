
pub fn terminal_write_hello() {
    println!("hello world");
}


pub mod canvas {
    pub(crate) fn terminal_write_hi() {
        println!("Hi!")
    }
}

enum Foo {
    X(pub u32),
    Y { f: f32 }
}

impl Foo {
    pub(super) fn new(x: u32) -> Foo { Foo::X(x) }
}