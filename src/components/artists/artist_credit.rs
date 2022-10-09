use serde::{Deserialize, Serialize};

use super::IncludedArtist;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistCredit {
    #[serde(alias = "joinphrase")]
    pub join_phrase: Option<String>,
    pub artist: Option<IncludedArtist>,
    pub name: String,
}
