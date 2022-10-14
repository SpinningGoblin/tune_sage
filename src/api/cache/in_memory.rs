use std::collections::HashMap;

use crate::errors::CacheError;

use super::Cache;

#[derive(Default)]
pub struct InMemoryCache {
    data: HashMap<String, String>,
}

#[async_trait::async_trait]
impl Cache for InMemoryCache {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        Ok(self.data.get(key).cloned())
    }

    async fn set(&mut self, key: &str, value: &str) -> Result<(), CacheError> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }
}
