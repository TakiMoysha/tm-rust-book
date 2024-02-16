use std::io::Write;

use clap::{command, Parser, Subcommand};
use env_logger;
use log::{debug, error, info, LevelFilter};
use serde_derive::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpStream};
// use std::{
//     collections::HashMap,
//     sync::{Arc, Mutex},
//     thread,
// };

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct AppArgs {
    #[arg(short, long, default_value = "server")]
    mode: Option<String>,
    #[arg(short, long, default_value = "127.0.0.1:12345")]
    bind: Option<String>,
    #[arg(short, long)]
    server_url: Option<String>,
}

#[derive(Subcommand, Debug)]
enum AppCommands {
    Ping {},
}

mod crypto {
    use aes::cipher::generic_array::GenericArray;
    use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};

    const KEY: &[u8] = "SECRET_KEY_WITH_32_BYTES_8124082".as_bytes();
    const NONCE: &[u8] = "SECRET_NONCE".as_bytes();

    pub fn encrypt_data(data: &[u8]) -> Result<Vec<u8>, String> {
        let key = GenericArray::from_slice(&KEY);
        let nonce = GenericArray::from_slice(&NONCE);
        let cipher = Aes256Gcm::new(key);

        match cipher.encrypt(nonce, data) {
            Ok(encrypted_data) => Ok(encrypted_data),
            Err(_) => Err("Failed to encrypt data".to_string()),
        }
    }

    pub fn decrypt_data(data: &[u8]) -> Result<Vec<u8>, String> {
        let key = GenericArray::from_slice(&KEY);
        let nonce = GenericArray::from_slice(&NONCE);
        let cipher = Aes256Gcm::new(key);

        match cipher.decrypt(nonce, data) {
            Ok(decrypted_data) => Ok(decrypted_data),
            Err(_) => Err("Failed to decrypt data".to_string()),
        }
    }
}

async fn run_server(args: AppArgs) {
    info!("Starting server on {}", args.bind.unwrap());
}

mod client {
    use log::error;
    use std::io::{Error, ErrorKind};

    pub fn setup_network_as_client() -> Result<(), Error> {
        let ip_output = std::process::Command::new("ip")
            .arg("addr")
            .arg("add")
            .arg("10.8.0.2/24")
            .arg("dev")
            .arg("tun0")
            .output()
            .expect("Failed to execute command");

        if !ip_output.status.success() {
            let msg = format!("Failed to setup tun device: {}", ip_output.status);
            error!("{}", msg);
            return Err(Error::new(ErrorKind::Other, msg));
        }

        let link_output = std::process::Command::new("ip")
            .arg("link")
            .arg("set")
            .arg("up")
            .arg("dev")
            .arg("tun0")
            .output()
            .expect("Failed to execute IP LINK command");

        if !link_output.status.success() {
            let msg = format!("Failed to setup IP LINK: {}", link_output.status);
            error!("{}", msg);
            return Err(Error::new(ErrorKind::Other, msg));
        }

        let route_output = std::process::Command::new("ip")
            .arg("route")
            .arg("add")
            .arg("0.0.0.0/24")
            .arg("via")
            .arg("10.8.0.1")
            .arg("dev")
            .arg("tun0")
            .output()
            .expect("Failed to execute IP ROUTE command");

        if !route_output.status.success() {
            let msg = format!("Failed to setup IP ROUTE: {}", route_output.status);
            error!("{}", msg);
            return Err(Error::new(ErrorKind::Other, msg));
        }

        Ok(())
    }

    pub fn cleanup_network() {
        let cleanup_output = std::process::Command::new("sudo")
            .arg("ip")
            .arg("link")
            .arg("delete")
            .arg("tun0")
            .output()
            .expect("Failed to execute CLEANUP command");

        if !cleanup_output.status.success() {
            let msg = format!("Failed to cleanup IP LINK: {}", cleanup_output.status);
            error!("{}", msg);
        }
    }
}

const TUN_INTERFACE_NAME: &str = "simple-vpn";

async fn run_client(args: AppArgs) {
    let server_url = args.server_url.unwrap();
    debug!("Starting client - {}", server_url.clone());
    // client::run(server_url.as_str()).await;

    let mut stream = TcpStream::connect(server_url.clone()).await.unwrap();

    let mut config = tun::Configuration::default();
    config.name(TUN_INTERFACE_NAME);
    let mut tun_device = tun::create(&config).unwrap();

    client::setup_network_as_client().unwrap();

    info!("Connection established with {}", server_url.clone());

    let mut buffer = vec![0; 1024];
    loop {
        match stream.try_read(&mut buffer) {
            Ok(n) => {
                info!("Read {} bytes", n);
                transmit_data_from_client_to_tun(&mut stream, &mut tun_device).await;
            }
            Err(_) => {
                error!("Error while reading from stream");
                return;
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct VpnPackage {
    data: String,
}

async fn transmit_data_from_client_to_tun(client: &mut TcpStream, tun: &mut tun::platform::Device) {
    let mut buffer = vec![0u8; 1500];
    loop {
        match client.try_read(&mut buffer) {
            Ok(n) => {
                let vpn_package: VpnPackage = bincode::deserialize(&buffer[..n]).unwrap();
                let decrypted_data = crypto::decrypt_data(&vpn_package.data.as_bytes()).unwrap();
                debug!("Writing {} bytes", n);
                tun.write(&decrypted_data).unwrap();
            }
            Err(e) => {
                error!("Error while reading from client: {}", e);
                continue;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    let args = AppArgs::parse();

    if args.mode == Some("server".to_string()) {
        run_server(args).await;
    } else if args.mode == Some("client".to_string()) {
        run_client(args).await;
    } else {
        error!("Unknown mode: {}", args.mode.unwrap());
        std::process::exit(1);
    }
}
