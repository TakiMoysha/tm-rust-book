use anyhow::Result;
use std::process;
use std::sync::mpsc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
// use tokio::sync::mpsc;

const ADDR: &str = "127.0.0.1:7878";

// loop {
//     match stream.read(buf)? {
//         0 => return Ok(()),
//         c => c,
//     };
//     stream.write_all(b"+PONG\r\n").await?;
// }

async fn request_handler(mut stream: TcpStream) {
    // let mut buf = [0, 1024];
    let mut buf = [0; 512];

    loop {  
        let buf_data = stream.read(&mut buf);
        match buf_data {
            Ok(b"+PONG\r\n") => {
                println!("[INFO] PONG");
            }
            Ok(_) => {
                
            }
        }
    }
}

pub struct ServerOpts {
    address: String,
}

async fn run_server() -> Result<ServerOpts>{
    let opts = ServerOpts {
        address: ADDR.to_string(),
    };
    let (tx, rx) = mpsc::channel::<anyhow::Error>();

    // tokio::spawn(async move { request_handler(tx) });
    // let listener = TcpListener::bind(ADDR).await?;
    // println!("[INFO] Listening on: {}", ADDR);
    // loop {
    //     let err_tx: mpsc::Sender = tx.clone();
    //     let (socket, _) = listener.accept().await?;
    //     tokio::spawn(async move {
    //         if let Err(e) = handle_request(socket).await {
    //             let _err = err_tx.send(e).await;
    //         }
    //     })
    // }
    Ok(opts)
}

#[tokio::main]
async fn main() {
    let server_opts = run_server().await.unwrap();
    println!("[INFO] Server running on {}", server_opts.address);

}
