use std::collections::HashMap;
use std::sync::Mutex;

use crate::types::{ServiceAlias, URL};

#[derive(Debug)]
pub enum StoreError {
    Unknow,
    NotFound,
    InsertError,
    LockError,
}

pub struct InMemoryStore {
    services: Mutex<HashMap<ServiceAlias, URL>>,
}
impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
        }
    }
}

pub trait Repository: Send + Sync {
    fn all(&self) -> Result<HashMap<ServiceAlias, URL>, StoreError>;

    fn get(&self, key: ServiceAlias) -> Result<URL, StoreError>;

    fn delete(&self, key: ServiceAlias) -> Result<String, StoreError>;

    fn insert(&self, key: ServiceAlias, value: URL) -> Result<(), StoreError>;
}

impl Repository for InMemoryStore {
    fn all(&self) -> Result<HashMap<ServiceAlias, URL>, StoreError> {
        Ok(self.services.lock().unwrap().clone())
    }

    fn get(&self, key: ServiceAlias) -> Result<URL, StoreError> {
        self.services
            .lock()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or(StoreError::NotFound)
    }

    fn delete(&self, key: ServiceAlias) -> Result<String, StoreError> {
        self.services.lock().unwrap().remove(&key);

        Ok(key)
    }

    fn insert(&self, key: ServiceAlias, value: URL) -> Result<(), StoreError> {
        self.services
            .lock()
            .unwrap()
            .insert(key.clone(), value.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_insert_and_get_services() {
        let s1_alias = "local_server".to_string();
        let s1_url = "http://192.168.1.1:8000/api/heartbeat".to_string();
        let s2_alias = "remote_server".to_string();
        let s2_url = "http://192.168.1.1:8080/api/heartbeat".to_string();

        let store = InMemoryStore::new();
        store.insert(s1_alias.clone(), s1_url.clone()).unwrap();
        store.insert(s2_alias.clone(), s2_url.clone()).unwrap();

        assert_eq!(s1_url, store.get(s1_alias).unwrap());
        assert_eq!(s2_url, store.get(s2_alias).unwrap());
    }
}
