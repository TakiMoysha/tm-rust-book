use std::sync::Arc;

use crate::repositories::pokemon::InMemoryRepository;

mod api;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
extern crate serde;

fn main() {
    let url = "localhost:8000";
    println!("Server run on: http://{}/;", url);
    let repo = Arc::new(InMemoryRepository::new());
    api::serve(url, repo);
}
