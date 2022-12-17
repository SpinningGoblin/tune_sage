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

impl RecordingList {
    pub fn recordings_scored_above(&self, score: u8) -> Vec<&Recording> {
        self.recordings
            .iter()
            .filter(|recording| recording.score.map(|s| s >= score).unwrap_or(true))
            .collect()
    }
}
