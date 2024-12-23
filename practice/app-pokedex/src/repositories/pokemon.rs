use std::sync::Mutex;

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub enum Insert {
    Ok(PokemonNumber),
    Conflict,
    Error,
}

pub struct InMemoryRepository {
    error: bool,
    pokemons: Mutex<Vec<Pokemon>>,
}

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum DeleteError {
    NotFound,
    Unknown,
}

pub enum FetchOneError {
    NotFound,
    Unknown,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let pokemons: Mutex<Vec<Pokemon>> = Mutex::new(vec![]);
        Self {
            error: false,
            pokemons,
        }
    }

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

pub trait Repository: Send + Sync {
    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError>;

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, FetchOneError>;

    fn delete(&self, number: PokemonNumber) -> Result<(), DeleteError>;

    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError>;
}

impl Repository for InMemoryRepository {
    fn delete(&self, number: PokemonNumber) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }

        let mut lock = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(DeleteError::Unknown),
        };

        let index = match lock.iter().position(|p| p.number == number) {
            Some(index) => index,
            None => return Err(DeleteError::NotFound),
        };

        lock.remove(index);
        Ok(())
    }

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, FetchOneError> {
        if self.error {
            return Err(FetchOneError::Unknown);
        }

        let lock = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchOneError::Unknown),
        };

        match lock.iter().find(|p| p.number == number) {
            Some(pokemon) => Ok(pokemon.clone()),
            None => Err(FetchOneError::NotFound),
        }
    }

    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }

        let lock = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown),
        };

        let mut pokemons = lock.to_vec();
        pokemons.sort_by(|a, b| a.number.cmp(&b.number));
        Ok(pokemons)
    }

    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => {
                return Err(InsertError::Unknown);
            }
        };

        if lock.iter().any(|pokemon| pokemon.number == number) {
            return Err(InsertError::Conflict);
        }

        let number_clone = number.clone();
        let pokemon = Pokemon::new(number_clone, name, types);
        lock.push(pokemon.clone());
        Ok(pokemon)
    }
}
