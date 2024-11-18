use std::sync::Mutex;
use std::collections::HashMap;

use crate::types::{Address, ServiceAlias};

#[derive(Debug)]
pub enum StoreError {
    Unknow,
    NotFound,
    InsertError,
    LockError,
}

pub struct InMemoryStore {
    services: Mutex<HashMap<ServiceAlias, Address>>,
}
impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
        }
    }
}

pub trait Repository: Send + Sync {
    fn all(&self) -> Result<HashMap<ServiceAlias, Address>, StoreError>;

    fn get(&self, key: ServiceAlias) -> Result<Address, StoreError>;

    fn delete(&self, key: ServiceAlias) -> Result<String, StoreError>;

    fn insert(&self, key: ServiceAlias, value: Address) -> Result<(), StoreError>;
}

impl Repository for InMemoryStore {
    fn all(&self) -> Result<HashMap<ServiceAlias, Address>, StoreError> {
        Ok(self.services.lock().unwrap().clone())
    }

    fn get(&self, key: ServiceAlias) -> Result<Address, StoreError> {
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

    fn insert(&self, key: ServiceAlias, value: Address) -> Result<(), StoreError> {
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
        let service_1_alias = "local_server".to_string();
        let service_1_address = "http://192.168.1.1:8000/api/heartbeat".to_string();
        let service_2_alias = "remote_server".to_string();
        let service_2_address = "http://192.168.1.1:8080/api/heartbeat".to_string();

        let store = InMemoryStore::new();
        store.insert(service_1_alias.clone(), service_1_address.clone());
        store.insert(service_2_alias.clone(), service_2_address.clone());

        assert_eq!(service_1_address, store.get(service_1_alias).unwrap());
        assert_eq!(service_2_address, store.get(service_2_alias).unwrap());
    }
}
