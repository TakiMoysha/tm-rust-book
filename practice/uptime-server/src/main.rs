mod handlers;
mod inmemory_store;
mod middlewares;
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
