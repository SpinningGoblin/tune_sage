mod file_system;
mod in_memory;

pub use file_system::FileSystemCache;
pub use in_memory::InMemoryCache;

use crate::errors::CacheError;

#[async_trait::async_trait]
pub trait Cache {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError>;
    async fn set(&mut self, key: &str, value: &str) -> Result<(), CacheError>;
}
