use std::sync::Arc;

use crate::domain::entities::{PokemonNumber, };
use crate::repositories::pokemon::{Repository, DeleteError, InMemoryRepository};

pub struct Request {
    pub number: u16,
}

pub enum Error {
    BadRequest,
    NotFound,
    Unknown,
}

pub fn execute(req: Request, repo: Arc<dyn Repository>) -> Result<(), Error> {
    match PokemonNumber::try_from(req.number) {
        Ok(number) => match repo.delete(number) {
            Ok(()) => Ok(()),
            Err(DeleteError::NotFound) => Err(Error::NotFound),
            Err(DeleteError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{PokemonName, PokemonTypes, PokemonNumber};

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(req, repo);

        match res {
            Err(Error::Unknown) => {},
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::bad());

        let res = execute(req, repo);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_does_not_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(req, repo);

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_ok_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok();
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(req, repo);

        match res {
            Ok(()) => {},
            _ => unreachable!(),
        };
    }

    impl Request {
        fn new(number: PokemonNumber) -> Self {
            Self {
                number: u16::from(number),
            }
        }
    }
}