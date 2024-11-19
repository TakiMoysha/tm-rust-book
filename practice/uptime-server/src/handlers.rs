use std::io;

use crate::inmemory_store::{InMemoryStore, Repository};
use rocket::*;
use tokio::{
    task::spawn_blocking,
    time::{sleep, Duration},
};

#[get("/")]
pub async fn index() -> Option<fs::NamedFile> {
    fs::NamedFile::open("index.html").await.ok()
}

#[get("/webservices")]
pub async fn get_webservices(services_store: &State<InMemoryStore>) -> String {
    let all_services = services_store.all().unwrap();

    let service_list = all_services
        .iter()
        .map(|(name, address)| format!("<li>{} [{}]</li>", name, address))
        .collect::<Vec<String>>()
        .join("");

    format!("<h1>Services</h1><ol>{}</ol>", service_list)
}

#[post("/webservices")]
pub async fn new_webservices() {}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("{} seconds delayed", seconds)
}

#[get("/blocking_task")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("README.md"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    Ok(vec)
}


