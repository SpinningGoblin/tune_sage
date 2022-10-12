use serde::{Deserialize, Serialize};

use super::Recording;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordingList {
    pub created: String,
    #[serde(default)]
    pub count: u32,
    #[serde(default)]
    pub offset: u32,
    #[serde(default)]
    pub recordings: Vec<Recording>,
}
