use std::sync::Mutex;

use crate::domain::entities::{ Pokemon, PokemonName, PokemonNumber, PokemonTypes };

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
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes
    ) -> Result<Pokemon, InsertError>;
}

impl Repository for InMemoryRepository {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes
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