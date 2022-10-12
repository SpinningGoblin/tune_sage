use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IncludedRecording {
    pub id: String,
    pub video: Option<bool>,
    pub disambiguation: Option<String>,
    pub title: String,
    pub length: Option<u64>,
}
