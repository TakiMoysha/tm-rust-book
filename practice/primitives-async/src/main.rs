async fn read_hosts() -> Result<Vec<u8>, String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("read_hosts done!");

    Ok(Vec::new())
}

async fn bar() {
    let mut a = [0u8; 72];
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    for _ in 0..10 {
        a[0] += 1;
    }

    println!("bar done!");
}

struct HostReader;

impl HostReader {
    pub async fn read_hosts(&self) -> Result<Vec<u8>, String> {
        todo!()
    }
}

trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, String>;
}

#[tokio::main]
async fn main() {}
