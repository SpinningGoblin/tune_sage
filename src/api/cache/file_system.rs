use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use tokio::fs;

use crate::errors::CacheError;

use super::Cache;

pub struct FileSystemCache {
    pub folder: String,
    pub max_cache_days: u64,
}

const DEFAULT_CACHE_TIME_DAYS: u64 = 365;

impl FileSystemCache {
    pub fn for_folder(folder: &str) -> Self {
        Self {
            folder: folder.to_string(),
            max_cache_days: DEFAULT_CACHE_TIME_DAYS,
        }
    }

    pub fn for_folder_and_cache_time(folder: &str, days: u64) -> Self {
        Self {
            folder: folder.to_string(),
            max_cache_days: days,
        }
    }
}

#[async_trait::async_trait]
impl Cache for FileSystemCache {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        let path = self.path(key);

        match self.is_file_too_old(&path).await {
            Some(true) | None => return Ok(None),
            Some(_) => {}
        };

        match fs::read_to_string(&path).await {
            Ok(it) => Ok(Some(it)),
            Err(_) => return Ok(None),
        }
    }

    async fn set(&mut self, key: &str, value: &str) -> Result<(), CacheError> {
        let path = self.path(key);

        fs::create_dir_all(self.cache_folder())
            .await
            .map_err(|e| CacheError::WriteError {
                message: e.to_string(),
            })?;

        fs::write(path, value)
            .await
            .map_err(|e| CacheError::WriteError {
                message: e.to_string(),
            })
    }
}

impl FileSystemCache {
    fn path(&self, key: &str) -> String {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        format!("{}/{}.txt", self.cache_folder(), &hasher.finish())
    }

    async fn is_file_too_old(&self, path: &str) -> Option<bool> {
        let created_time = match fs::metadata(&path)
            .await
            .and_then(|metadata| metadata.created())
        {
            Ok(time) => time,
            Err(_) => return None,
        };

        let duration = match created_time.elapsed() {
            Ok(it) => it,
            Err(_) => return None,
        };

        // 86_400 seconds in a day
        Some((duration.as_secs() / 86_400) > self.max_cache_days)
    }

    fn cache_folder(&self) -> String {
        format!("{}/cache", &self.folder)
    }
}
