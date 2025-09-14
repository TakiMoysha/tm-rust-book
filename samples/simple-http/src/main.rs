use may_minihttp::{HttpServer, HttpService, Request, Response};
use std::{io, time};

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, _req: Request, res: &mut Response) -> io::Result<()> {
        // println!(
        //     "{}: {:#?}",
        //     time::SystemTime::now()
        //         .duration_since(time::UNIX_EPOCH)
        //         .unwrap()
        //         .as_millis(),
        //     _req
        // );
        res.body("Hello, world!");
        Ok(())
    }
}

// Start the server in `main`.
fn main() {
    let server = HttpServer(HelloWorld).start("0.0.0.0:8080").unwrap();
    println!("Listening on http://0.0.0.0:8080");
    server.join().unwrap();
}
