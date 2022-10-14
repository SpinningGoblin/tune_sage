#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Unknown error in cache {message:?}")]
    Unknown { message: String },
    #[error("Read error in cache {message:?}")]
    ReadError { message: String },
    #[error("Write error in cache {message:?}")]
    WriteError { message: String },
}
