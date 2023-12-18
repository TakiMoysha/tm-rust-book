use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let handle = tokio::spawn(async {
        "return value"
    });
    let out = handle.await.unwrap();
    println!("out: {}", out);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let handle = tokio::spawn(async {
            req_process(socket).await
        });
        println!("spawned, handle: {:?}", handle);
        handle.await.unwrap();
    }
}

async fn req_process(socket: TcpStream) {
    let mut conn = Connection::new(socket);

    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("received {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }
}
