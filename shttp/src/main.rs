use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use shttp::ThreadPool;


#[allow(unused_variables)]
fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    // let request_lines = reader.lines();
    let request_line = reader.lines().next().unwrap().unwrap();
    let request_header =
        request_line.split_whitespace().take(3).collect::<Vec<&str>>();
    let (method, path, http_version) = 
        (request_header[0], request_header[1], request_header[2]);

    println!("{} {} {}", method, path, http_version);
    
    let (status_line, contents) = match &request_header[..] {
        ["GET", "/robots.txt", _] => ("HTTP/1.1 200 OK", fs::read_to_string("./public/robots.txt").unwrap()),
        ["GET", "/", _] => ("HTTP/1.1 200 OK", fs::read_to_string("./public/index.html").unwrap()),
        ["GET", "/long", _] => {
            thread::sleep(Duration::from_secs(3));
            ("HTTP/1.1 200 OK", fs::read_to_string("./public/index.html").unwrap())
        },
        _ => ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("./public/404.html").unwrap())
        
    };

    let length = contents.len();
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(8);
    println!("Listening on {}", listener.local_addr().unwrap());

    // incoming - итератор потоков
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| { 
            handle_connection(stream);
        });
    }
}
