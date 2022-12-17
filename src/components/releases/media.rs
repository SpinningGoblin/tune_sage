use serde::{Deserialize, Serialize};

use super::{Format, Track};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Media {
    pub title: Option<String>,
    #[serde(alias = "track-count")]
    pub track_count: Option<u32>,
    pub format: Option<Format>,
    pub tracks: Option<Vec<Track>>,
}

impl Media {
    pub fn tracks(&self) -> Vec<&Track> {
        match &self.tracks {
            Some(tracks) => tracks.iter().collect(),
            None => Vec::new(),
        }
    }
}
