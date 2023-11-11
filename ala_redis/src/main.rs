#![allow(warnings)] // hide warnings

use anyhow::Result;
use log::{debug, info};
use std::net::SocketAddr;
use std::sync::{mpsc, Mutex, Arc};
use std::thread::sleep;
use std::time::SystemTime;
use tokio::time::{sleep_until, Instant, Duration};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::{signal};
// use tokio::sync::mpsc;

mod runtime;

const ADDR: &str = "127.0.0.1:7878";

struct ServerOpts {
    address: String,
}

struct ServerRuntime {
    listener: TcpListener,
    tx: mpsc::Sender<anyhow::Error>,
    rx: mpsc::Receiver<anyhow::Error>,
    runtime: tokio::runtime::Runtime,
    config: ServerOpts,
}

async fn run_server() -> Result<ServerRuntime> {
    let opts = ServerOpts {
        address: ADDR.to_string(),
    };
    let (tx, rx) = mpsc::channel::<anyhow::Error>();
    let listener = TcpListener::bind(ADDR).await.unwrap();
    let server: ServerRuntime = ServerRuntime {
        listener,
        tx,
        rx,
        runtime: tokio::runtime::Runtime::new().unwrap(),
        config: opts
    };

    Ok(server)
}


async fn stream_process(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read_to_end(&mut buf).unwrap();


}


#[tokio::main]
async fn main() {
    let server = run_server().await.unwrap();
    // sleep 2 seconds
    debug!("time:{} Server running on {}", 0, server.config.address);

    loop {
        let (stream, addr) = server.listener.accept().await.unwrap();
        let mut stream = socket.0;
        
        tokio::spawn(async move {
            stream_process(state, stream, addr).await;
        });

        // async move {
        //     let mut buffer = [0, 1024];
        //     let mut buffer = Vec::new();
        //     let mut stream = socket.0;
            // let request = stream.read_to_end(&mut buffer);

            // let _ = stream.read(&mut buffer).await.unwrap();
            // let mut stream = socket.0;
            // let mut buf = [0; 512]; 
        // });
    }

    // println!("Listening on {}", listener.local_addr().unwrap());
}
