mod middlewares {
    use std::{
        io::Cursor,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use rocket::{
        fairing::{Fairing, Info, Kind},
        http::{ContentType, Method, Status},
        Data, Request, Response,
    };

    pub struct DemoMiddlewareCounter {
        get: AtomicUsize,
        post: AtomicUsize,
    }

    impl DemoMiddlewareCounter {
        pub fn new() -> Self {
            Self {
                get: AtomicUsize::new(0),
                post: AtomicUsize::new(0),
            }
        }
    }

    #[rocket::async_trait]
    impl Fairing for DemoMiddlewareCounter {
        fn info(&self) -> Info {
            Info {
                name: "GET/POST Counter",
                kind: Kind::Request | Kind::Response,
            }
        }

        // increment the counter for GET | POST requests
        async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
            match request.method() {
                Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
                Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
                _ => return,
            };
        }

        async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
            if response.status() != Status::NotFound {
                return;
            }

            if request.method() == Method::Get && request.uri().path() == "/api/counts" {
                let get_count = self.get.load(Ordering::Relaxed);
                let post_count = self.post.load(Ordering::Relaxed);
                let body = format!("Get: {}\nPost: {}", get_count, post_count);

                response.set_status(Status::Ok);
                response.set_header(ContentType::Plain);
                response.set_sized_body(body.len(), Cursor::new(body));
            }
        }
    }
}

mod handlers;
mod inmemory_store;
mod types;

use handlers::*;
use inmemory_store::{InMemoryStore, Repository};
use middlewares::*;
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
        .attach(DemoMiddlewareCounter::new())
        .manage(store)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                get_webservices,
                new_webservices,
                delay,
                blocking_task,
                get_counter,
                post_coutner
            ],
        )
}
