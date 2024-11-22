use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

use reqwest::StatusCode;

use crate::types::{ServiceAlias, URL};

#[derive(Debug)]
pub enum StoreError {
    Unknow,
    NotFound,
    LockError,
    InsertError,
}

#[derive(Debug, Clone)]
pub struct HealthPoint {
    alias: ServiceAlias,
    time_point: SystemTime,
    status_code: StatusCode,
    response_text: String,
}

#[derive(Debug)]
pub struct InMemoryStore {
    services: Mutex<HashMap<ServiceAlias, URL>>,
    health_points: Mutex<Vec<HealthPoint>>,
}
impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
            health_points: Mutex::new(Vec::new()),
        }
    }
}

pub trait Repository: Send + Sync {
    fn all(&self) -> Result<HashMap<ServiceAlias, URL>, StoreError>;

    fn get(&self, key: ServiceAlias) -> Result<URL, StoreError>;

    fn delete(&self, key: ServiceAlias) -> Result<String, StoreError>;

    fn insert(&self, key: ServiceAlias, value: URL) -> Result<(), StoreError>;

    fn get_health_points_by_alias(&self, key: ServiceAlias)
        -> Result<Vec<HealthPoint>, StoreError>;

    fn save_health_point(
        &self,
        key: ServiceAlias,
        time_point: std::time::SystemTime,
        status_code: reqwest::StatusCode,
        response_text: String,
    ) -> Result<(), StoreError>;
}

impl Repository for InMemoryStore {
    fn all(&self) -> Result<HashMap<ServiceAlias, URL>, StoreError> {
        let lock = match self.services.lock() {
            Ok(lock) => lock,
            _ => return Err(StoreError::LockError),
        };

        Ok(lock.clone())
    }

    fn get(&self, key: ServiceAlias) -> Result<URL, StoreError> {
        let lock = match self.services.lock() {
            Ok(lock) => lock,
            _ => return Err(StoreError::LockError),
        };

        lock.get(&key).cloned().ok_or(StoreError::NotFound)
    }

    fn delete(&self, key: ServiceAlias) -> Result<String, StoreError> {
        let mut lock = match self.services.lock() {
            Ok(lock) => lock,
            _ => return Err(StoreError::LockError),
        };

        lock.remove(&key).ok_or(StoreError::NotFound)
    }

    fn insert(&self, key: ServiceAlias, value: URL) -> Result<(), StoreError> {
        let mut lock = match self.services.lock() {
            Ok(lock) => lock,
            _ => return Err(StoreError::LockError),
        };

        lock.insert(key.clone(), value.clone());

        Ok(())
    }

    fn get_health_points_by_alias(
        &self,
        key: ServiceAlias,
    ) -> Result<Vec<HealthPoint>, StoreError> {
        let lock = match self.health_points.lock() {
            Ok(lock) => lock,
            _ => return Err(StoreError::LockError),
        };

        let server_points = lock
            .iter()
            .filter(|record| record.alias == key)
            .cloned()
            .collect();

        Ok(server_points)
    }

    fn save_health_point(
        &self,
        key: ServiceAlias,
        time_point: std::time::SystemTime,
        status_code: reqwest::StatusCode,
        response_text: String,
    ) -> Result<(), StoreError> {
        let mut lock = match self.health_points.lock() {
            Ok(lock) => lock,
            _ => return Err(StoreError::LockError),
        };

        lock.push(HealthPoint {
            alias: key,
            time_point,
            status_code,
            response_text,
        });

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
