
use async_std;
use async_std::net::TcpStream;
use async_std::prelude::*;
use error_chain::error_chain;

use hyper;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Http(hyper::Error);
    }
}

#[async_std::main]
async fn main() {
    let url = "https://httpbin.org/ip".parse::<hyper::Uri>().unwrap();
    let host = url.host().expect("url has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{host}:{port}");

    let client = hyper::Client::new();
    let response = client.get(url).await;

    let mut stream = TcpStream::connect(address).await.unwrap();
    let peer = stream.peer_addr().unwrap();
    println!("Connected to {peer}");

    // match response.unwrap() {
    //     hyperOk(response) => {
    //         let status = response.status();
    //         let headers = response.headers();
    //         let body = response.body();
    //         println!("{status:?}, {headers:?}, {body:?}");
    //     },
    //     _ => {
    //         eprintln!("Failed to connect to {address}");
    //     }
    // }
}
