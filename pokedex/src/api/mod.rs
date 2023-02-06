use std::{ sync::Arc, time::SystemTime, time::UNIX_EPOCH };

use crate::repositories::pokemon::Repository;

mod fetch_pokemon;

pub mod fetch_all_pokemons;
pub mod create_pokemon;
pub mod delete_pokemon;
pub mod health;

enum Status {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::Ok => 200,
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::InternalServerError => 500,
        };
        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None,
        }
    }
}

fn print_init_request_msg(request: &rouille::Request) {
    let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!(
        "{:?} - {:?} {:?} - {}",
        time_now,
        request.remote_addr(),
        request.method(),
        request.url()
    );
}

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        print_init_request_msg(&req);
        router!(req,
            (GET) (/health) => { health::serve() },
            (GET) (/{number: u16}) => { fetch_pokemon::serve(repo.clone(), number) },
            (DELETE) (/{number: u16}) => { delete_pokemon::serve(repo.clone(), number) },
            (GET) (/) => { fetch_all_pokemons::serve(repo.clone()) },
            (POST) (/) => { create_pokemon::serve(req, repo.clone()) },
            _ => rouille::Response::from(Status::NotFound)
        )
    });
}