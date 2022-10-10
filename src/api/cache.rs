use std::collections::HashMap;

#[async_trait::async_trait]
pub trait Cache {
    async fn get(&self, key: &str) -> Option<&String>;
    async fn set(&mut self, key: &str, value: &str);
}

#[derive(Default)]
pub struct InMemoryCache {
    data: HashMap<String, String>,
}

#[async_trait::async_trait]
impl Cache for InMemoryCache {
    async fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    async fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}
