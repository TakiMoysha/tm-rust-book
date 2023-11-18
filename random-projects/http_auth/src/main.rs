use std::env;

use tokio;

#[tokio::main]
async fn main() {
    let env_args = env::args();
    println!("{env_args:?}");
}
