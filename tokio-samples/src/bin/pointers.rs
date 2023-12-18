use std::rc::Rc;
use tokio;
use tokio::task::yield_now;

async fn foo() {
    {
        let rc = Rc::new("hello");
        println!("foo: {}", rc);
    }
    yield_now().await;
}

async fn bar() {
    let rc = Rc::new("world");
    // rc is used after '.await'
    yield_now().await;
    println!("bar: {}", rc);
}

#[tokio::main]
async fn main() {
    tokio::spawn(foo());
    // tokio::spawn(bar());
}
