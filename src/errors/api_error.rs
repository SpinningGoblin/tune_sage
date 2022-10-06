use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Unknown error occurred {message:?}")]
    Unknown { message: String },
    #[error("Error occurred during deserialization {message:?}")]
    DeserializationError { message: String },
    #[error("Error occurred during deserialization {message:?}")]
    ReadBodyError { message: String },
}
