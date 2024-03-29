use std::sync::Arc;
use std::convert::TryFrom;

use crate::repositories::pokemon::{ Repository, InsertError };

use super::entities::{ Pokemon, PokemonNumber, PokemonName, PokemonTypes };

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(req: Request, repo: Arc<dyn Repository>) -> Result<Response, Error> {
    println!("{} : {} ", req.number, req.name);
    match (
            PokemonNumber::try_from(req.number),
            PokemonName::try_from(req.name),
            PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) =>
            match repo.insert(number, name, types) {
                Ok(
                    Pokemon { number, name, types }) =>
                    Ok(Response {
                        number: u16::from(number),
                        name: String::from(name),
                        types: Vec::<String>::from(types),
                    }
                ),
                Err(InsertError::Conflict) => Err(Error::Conflict),
                Err(InsertError::Unknown) => Err(Error::Unknown),
            }
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::pokemon::InMemoryRepository;

    impl Request {
        fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
            Self {
                number: u16::from(number),
                name: String::from(name),
                types: Vec::<String>::from(types),
            }
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };
        let res = execute(req, repo);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::charmander(),
            PokemonTypes::charmander()
        );

        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::charmander(),
            PokemonTypes::charmander()
        );
        let res = execute(req, repo);

        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::bad(),
            PokemonTypes::pikachu(),
        );

        let res = execute(req, repo);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );

        let res = execute(req, repo);

        match res {
            Ok(res) => {
                assert_eq!(res.number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(res.name, String::from(PokemonName::pikachu()));
                assert_eq!(res.types, Vec::<String>::from(PokemonTypes::pikachu()));
            }
            _ => unreachable!(),
        };
    }
}