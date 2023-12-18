use tokio;

async fn tokio_main() {
    println!("Hello, Tokio!");
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(tokio_main());
}
