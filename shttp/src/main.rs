use std::fs;
use std::net::TcpListener;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

struct App {}

fn app() {}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./public/index.html").unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    // incoming - итератор потоков
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
