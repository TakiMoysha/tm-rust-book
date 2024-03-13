// !TODO: https://rohitpaulk.com/articles/redis-4
// !TODO: https://redis.io/docs/reference/protocol-spec/
#![allow(warnings)] // hide warnings

use anyhow::{Context, Result};
use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{debug, error, info, warn, LevelFilter};
use std::net::SocketAddr;
use std::sync::{mpsc, Arc, Mutex};
use std::time::SystemTime;
use tokio::time::sleep;
use tokio::time::{sleep_until, Duration, Instant};

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

enum Exceptions {
    IncompleteRESP,
}

enum RESPCommands {
    PING,
    ECHO,
}
enum RESPDataTypes {
    SIMPLE_STRING,
    BULK_STRING,
    ARRAY,
}

struct RESPDecoder {
    input: Vec<u8>,
    command: RESPCommands,
    data_type: RESPDataTypes,
    // data: Vec<u8>,
}

impl RESPDecoder {
    fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            command: RESPCommands::PING,
            data_type: RESPDataTypes::SIMPLE_STRING,
        }
    }

    fn decode(&mut self) -> Result<()> {
        let buf = self.input;
        Ok(())
    }
}

async fn handle_request(mut stream: &mut BufStream<TcpStream>) -> Result<()> {
    let mut buf = vec![0u8; 1024];
    stream.read(&mut buf).await?;

    // loop {
    //     if buf[0] == b'P' && buf[1] == b'U' && buf[2] == b'G' && buf[3] == b'G' {
    //         break;
    //     }
    //     stream.read(&mut buf).await?;
    // }

    // let contents = "{\"hello\": \"world\"}";
    // let response = format!(
    //     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //     contents.len(),
    //     contents
    // );
    //
    // stream.write_all(response.as_bytes()).await?;
    // stream.flush().await?;
    sleep(Duration::from_secs(2)).await;
    stream.write(b"+PONG\r\n").await?;
    stream.flush().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    Builder::new().filter_level(LevelFilter::Debug).init();
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
            info!("Connection: {}", stream.get_ref().peer_addr().unwrap(),);

            handle_request(&mut stream)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to handle request, by reason: {}", e))
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_ping_request_shoud_return_pong() {
        // assert_eq!("+PONG\r\n", handle_request().await.unwrap());
        todo!()
    }

    /// несколько запросов к серверу
    /// сревер должен обрабатывать все команды от клиента
    /// пока клиент не отключится
    fn sequential_ping_request_shoud_return_pong() {
        todo!()
    }

    fn multiple_client_request_shoud_return_pong() {
        todo!()
    }

    fn echo_command_shoud_return_requested_data() {
        todo!()
    }

    fn decoder_should_return_simple_string() {
        let simple_string = "+HEY)\r\n";
        let decoder = RESPDecoder::new(simple_string.into()).decode();
        todo!()
    }
}
