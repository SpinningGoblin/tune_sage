use serde::{Deserialize, Serialize};

use crate::components::{artists::ArtistCredit, releases::IncludedRelease};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Recording {
    pub id: String,
    pub disambiguation: Option<String>,
    pub score: Option<u8>,
    #[serde(alias = "first-release-date")]
    pub first_release_date: Option<String>,
    pub title: String,
    pub video: Option<bool>,
    pub length: Option<u64>,
    pub releases: Option<Vec<IncludedRelease>>,
    #[serde(alias = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

impl Recording {
    pub fn official_releases(&self) -> Vec<IncludedRelease> {
        self.releases()
            .iter()
            .filter(|release| release.is_official())
            .cloned()
            .collect()
    }

    pub fn releases(&self) -> Vec<IncludedRelease> {
        self.releases.clone().unwrap_or_default()
    }
}
