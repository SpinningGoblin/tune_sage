use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::CacheError;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Unknown error occurred {message:?}")]
    Unknown { message: String },
    #[error("Error occurred during deserialization {message:?}")]
    DeserializationError { message: String },
    #[error("Error occurred during deserialization {message:?}")]
    ReadBodyError { message: String },
}

impl From<CacheError> for ApiError {
    fn from(e: CacheError) -> Self {
        Self::Unknown {
            message: e.to_string(),
        }
    }
}
