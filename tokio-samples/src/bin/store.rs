use mini_redis::{Connection, Frame};
use tokio;
use tokio::net::{TcpListener, TcpStream};

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set, Unknown};
    use std::collections::HashMap;

    let mut db = HashMap::new();

    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            Unknown(cmd) => {
                println!(" {:?}", cmd);
                Frame::Null
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        conn.write_frame(&response).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let handle = tokio::spawn(async { process(socket).await });
        handle.await.unwrap();
    }
}
