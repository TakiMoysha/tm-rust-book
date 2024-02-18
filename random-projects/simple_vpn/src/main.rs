use clap::{command, error, Parser, Subcommand};
use env_logger;
use log::{debug, error, info, LevelFilter};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const TUN_INTERFACE_NAME: &str = "simple-vpn";

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

mod network {
    use log::error;
    pub fn setup_tun_server_interface() -> Result<(), Box<dyn std::error::Error>> {
        let tun_device_output = std::process::Command::new("sudo")
            .arg("ip")
            .arg("link")
            .arg("set")
            .arg("dev")
            .arg("tun0")
            .arg("up")
            .output()?;

        if !tun_device_output.status.success() {
            return Err(format!(
                "Failed to setup SETUP TUN DEVICE: {:?}",
                tun_device_output.stderr
            )
            .into());
        }

        let tun_ip_output = std::process::Command::new("sudo")
            .arg("ip")
            .arg("addr")
            .arg("add")
            .arg("10.8.0.1/24")
            .arg("dev")
            .arg("tun0")
            .output()?;

        if !tun_ip_output.status.success() {
            return Err(format!("Failed to setup TUN IP: {:?}", tun_ip_output.stderr).into());
        }

        Ok(())
    }

    pub fn cleanup_tun_server_interface() {
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

    pub fn setup_tun_client_interface() -> Result<(), Box<dyn std::error::Error>> {
        let ip_output = std::process::Command::new("ip")
            .arg("addr")
            .arg("add")
            .arg("10.8.0.2/24")
            .arg("dev")
            .arg("tun0")
            .output()?;

        if !ip_output.status.success() {
            return Err(format!("Failed to SETUP IP: {:?}", ip_output.stderr).into());
        }

        let link_output = std::process::Command::new("ip")
            .arg("link")
            .arg("set")
            .arg("up")
            .arg("dev")
            .arg("tun0")
            .output()?;

        if !link_output.status.success() {
            return Err(format!("Failed to SETUP IP LINK: {:?}", link_output.stderr).into());
        }

        let route_output = std::process::Command::new("ip")
            .arg("route")
            .arg("add")
            .arg("0.0.0.0/24")
            .arg("via")
            .arg("10.8.0.1")
            .arg("dev")
            .arg("tun0")
            .output()?;

        if !route_output.status.success() {
            return Err(format!("Failed to SETUP IP ROUTE: {:?}", route_output.stderr).into());
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

#[derive(Serialize, Deserialize)]
struct VpnPackage {
    data: String,
}

async fn transmit_data_from_client_to_tun(client: &mut TcpStream, tun: &mut tun::platform::Device) {
    let mut buffer = vec![0u8];
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

async fn transmit_data_from_tun_to_client(tun: &mut tun::platform::Device, clone: &TcpStream) {
    let mut buffer = vec![0u8];

    loop {
        match tun.read(&mut buffer) {
            Ok(n) => {
                debug!("Read {} bytes", n);

                let encrypted_data = crypto::encrypt_data(&buffer[..n]).unwrap();
                clone
                    .try_write(&encrypted_data)
                    .expect("Failed to write to client");
            }
            Err(e) => {
                error!("Error while reading from tun: {}", e);
            }
        }
    }
}

async fn handle_client(
    client_id: usize,
    mut stream: TcpStream,
    clients: Arc<Mutex<HashMap<usize, TcpStream>>>,
) {
    let mut buffer = vec![0];

    loop {
        match stream.try_read(&mut buffer) {
            Ok(0) => {
                info!("Client {} disconnected", client_id);
                break;
            }
            Ok(n) => {
                info!("Read {} bytes from client {}", n, client_id);
                let clients_guard = clients.lock().unwrap();
                if let Some(client) = clients_guard.get(&client_id) {
                    client
                        .try_write(&buffer[..n])
                        .expect("Failed to write to client");
                }
            }
            Err(e) => {
                error!("Error in handle_client {}: {}", client_id, e);
                break;
            }
        }
    }

    clients.lock().unwrap().remove(&client_id);
    let _ = stream.shutdown().await.unwrap();
}

async fn run_client(args: AppArgs) {
    let server_url = args.server_url.unwrap();
    debug!("Starting client - {}", server_url.clone());

    let mut stream = TcpStream::connect(server_url.clone()).await.unwrap();

    let mut config = tun::Configuration::default();
    config.name(TUN_INTERFACE_NAME);
    let mut tun_device = tun::create(&config).unwrap();

    network::setup_tun_client_interface().unwrap();

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

async fn run_server(args: AppArgs) {
    let listener = TcpListener::bind(args.bind.clone().unwrap()).await.unwrap();
    let clients: Arc<Mutex<HashMap<usize, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut config = tun::Configuration::default();
    config.name("tun0");
    let tun_device = tun::create(&config).unwrap();

    if let Err(e) = network::setup_tun_server_interface() {
        error!("Failed to setup TUN interface: {}", e);
        return;
    }

    let shared_tun = Arc::new(Mutex::new(tun_device));
    let tun_device_clone = shared_tun.clone();

    info!("Starting server on {}", args.bind.clone().unwrap());
    let clients_clone = clients.clone();

    const CLIENT_INDX: usize = 0;

    tokio::spawn(async move {
        // todo!("Implement server with multi-client support");
        let clients_guard = clients_clone.lock().unwrap();

        match clients_guard.get(&CLIENT_INDX) {
            Some(client) => {
                let mut locked_tun = tun_device_clone.lock().unwrap();
                transmit_data_from_tun_to_client(&mut *locked_tun, client).await;
                drop(clients_guard);
            }
            None => {
                error!("Failed to get client");
            }
        }
    });

    let new_conn = listener.accept().await;

    if new_conn.is_err() {
        error!(
            "Failed to accept new connection: {:?}",
            new_conn.unwrap_err().to_string()
        );
        return;
    }

    let (mut stream, socket_addr) = new_conn.unwrap();
    info!("Client connected: {}", socket_addr);
    let p_stream = Arc::new(Mutex::new(stream));Распространенные заблуждения на протяжении всей жизни Rust

    clients.lock().unwrap().insert(CLIENT_INDX, stream);

    let tun_device_clone = shared_tun.clone();

    tokio::spawn(async move {
        let mut locked_tun = tun_device_clone.lock().unwrap();
        transmit_data_from_tun_to_client(&mut *locked_tun, &mut stream).await;
    });

    // let tun_device_clone = shared_tun.clone();
    // let clients_clone = clients.clone();

    // tokio::spawn(async move {
    //     let client_clone = clients_clone.lock().unwrap();
    //     let client_clone = client_clone.get(&CLIENT_INDX).unwrap();
    //     let mut locked_tun = tun_device_clone.lock().unwrap();
    //     transmit_data_from_tun_to_client(&mut *locked_tun, &client_clone).await;
    // });

    // let clients_clone = clients.clone();
    // tokio::spawn(async {
    //     handle_client(CLIENT_INDX, &stream, clients_clone).await;
    // });

    let _ = network::cleanup_tun_server_interface();
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
