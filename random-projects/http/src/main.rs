use hyper;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo; 
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;
    let host = url.host().expect("no host in url");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {});

    Ok(())
}
