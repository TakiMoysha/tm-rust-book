use std::sync::Arc;

use crate::repositories::pokemon::Repository;

pub mod create_pokemon;
pub mod health;

enum Status {
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::InternalServerError => 500,
        };
        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None
        }
    }
}

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        router!(req,
            (POST) (/pokemon) => {
                create_pokemon::serve(req, repo.clone())
            },
            (GET) (/health) => {
                health::serve()
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    });
}