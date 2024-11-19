mod inmemory_store;

mod types {
    pub type ServiceAlias = String;
    pub type Address = String;
}

mod app {
    #[derive(Debug, Clone)]
    struct AppState {}

    impl AppState {
        fn new() -> Self {
            Self {}
        }
    }
}

mod handlers {
    use std::{collections::HashMap, io, sync::Arc};

    use crate::inmemory_store::{InMemoryStore, Repository};
    use futures::lock::Mutex;
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
        let mut page = String::from("<h1>Services</h1><ol>");
        for (service_name, service_address) in all_services.iter() {
            let line = format!("<li>{} [{}]</li>", service_name, service_address);
            page.push_str(&line);
        }
        page.push_str("</ol>");
        page
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
}

use handlers::*;
use inmemory_store::{InMemoryStore, Repository};
use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    let store = InMemoryStore::new();
    store
        .insert(
            "local_server".to_string(),
            "http://192.168.1.1:8000/api/heartbeat".to_string(),
        )
        .unwrap();

    store
        .insert(
            "remote_server".to_string(),
            "http://192.168.1.1:8000/api/heartbeat".to_string(),
        )
        .unwrap();

    rocket::build()
        .manage(store)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![get_webservices, new_webservices, delay, blocking_task,],
        )
}
