use std::sync::Arc;

use crate::repositories::pokemon::{Repository, FetchAllError};
use crate::domain::entities::{ PokemonName, PokemonNumber, PokemonTypes };

pub enum Error {
    Unknown,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>) -> Result<Vec<Response>, Error> {
    match repo.fetch_all() {
        Ok(pokemons) =>
            Ok(
                pokemons.into_iter()
                    .map(|p| Response {
                        number: u16::from(p.number),
                        name: String::from(p.name),
                        types: Vec::<String>::from(p.types),
                    })
                    .collect::<Vec<Response>>()
            ),
        Err(FetchAllError::Unknown) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ repositories::pokemon::InMemoryRepository, domain::entities as DomainEntities };

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());

        let res = execute(repo);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_all_the_pokemons_ordered_by_increasing_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());

        repo.insert(
            DomainEntities::PokemonNumber::pikachu(),
            DomainEntities::PokemonName::pikachu(),
            DomainEntities::PokemonTypes::pikachu()
        ).ok();
        repo.insert(
            DomainEntities::PokemonNumber::charmander(),
            DomainEntities::PokemonName::charmander(),
            DomainEntities::PokemonTypes::charmander()
        ).ok();

        let res = execute(repo);

        match res {
            Ok(res) => {
                assert_eq!(res[0].number, u16::from(DomainEntities::PokemonNumber::charmander()));
                assert_eq!(res[0].name, String::from(DomainEntities::PokemonName::charmander()));
                assert_eq!(
                    res[0].types,
                    Vec::<String>::from(DomainEntities::PokemonTypes::charmander())
                );
                assert_eq!(res[1].number, u16::from(DomainEntities::PokemonNumber::pikachu()));
                assert_eq!(res[1].name, String::from(DomainEntities::PokemonName::pikachu()));
                assert_eq!(
                    res[1].types,
                    Vec::<String>::from(DomainEntities::PokemonTypes::pikachu())
                );
            }
            _ => unreachable!(),
        }
    }
}