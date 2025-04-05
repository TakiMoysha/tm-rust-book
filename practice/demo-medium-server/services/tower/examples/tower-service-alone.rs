use std::pin::Pin;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    println!("Test");
    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Service;
