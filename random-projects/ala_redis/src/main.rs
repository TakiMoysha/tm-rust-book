#![allow(warnings)] // hide warnings

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info};
use std::net::SocketAddr;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::SystemTime;
use tokio::time::{sleep_until, Duration, Instant};
use chrono::DateTime;
use chrono::Local;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
// use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ServerOpts {
    #[clap(short = 'b', long = "bind", default_value = "127.0.0.1:6379")]
    address: Option<String>,
}

// deprecated
struct ServerRuntime {
    listener: TcpListener,
    tx: mpsc::Sender<anyhow::Error>,
    rx: mpsc::Receiver<anyhow::Error>,
    config: ServerOpts,
}

async fn run_server(opts: ServerOpts) -> Result<ServerRuntime> {
    let address = opts.address.as_ref().expect("Address must be provided");
    let (tx, rx) = mpsc::channel::<anyhow::Error>();
    let listener = TcpListener::bind(address)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind server, by reason: {}", e))?;
    let server: ServerRuntime = ServerRuntime {
        listener,
        tx,
        rx,
        config: opts,
    };

    Ok(server)
}

async fn handle_request(mut stream: &mut BufStream<TcpStream>) -> Result<()> {
    let contents = "{\"hello\": \"world\"}";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let server_opts = ServerOpts::parse();
    let server = run_server(server_opts).await?;
    debug!("time:{} Server running on {:?}", 0, server.config);

    println!("Listening on {:?}", server.listener.local_addr().unwrap());
    loop {
        let (socket, _) = server
            .listener
            .accept()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to accept connection, by reason: {}", e))?;
        let mut stream = BufStream::new(socket);

        tokio::spawn(async move {
            let time: DateTime<Local> = SystemTime::now().into();
            info!(
                "[{}] <{}> Connection",
                time, 
                stream.get_ref().peer_addr().unwrap(), 
            );

            handle_request(&mut stream).await.map_err(|e| {
                anyhow::anyhow!("Failed to handle request, by reason: {}", e)
            })
        });
        //
        //     // async move {
        //     //     let mut buffer = [0, 1024];
        //     //     let mut buffer = Vec::new();
        //     //     let mut stream = socket.0;
        //     // let request = stream.read_to_end(&mut buffer);
        //
        //     // let _ = stream.read(&mut buffer).await.unwrap();
        //     // let mut stream = socket.0;
        //     // let mut buf = [0; 512];
        //     // });
    }

    Ok(())
}
