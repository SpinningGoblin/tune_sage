use serde::{Deserialize, Serialize};

use crate::components::recordings::IncludedRecording;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Track {
    pub number: Option<String>,
    pub position: Option<u32>,
    pub length: Option<u64>,
    pub id: String,
    pub title: Option<String>,
    pub recording: Option<IncludedRecording>,
}
