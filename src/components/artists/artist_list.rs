use serde::{Deserialize, Serialize};

use super::Artist;

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistList {
    pub created: String,
    #[serde(default)]
    pub count: u32,
    #[serde(default)]
    pub offset: u32,
    #[serde(default)]
    pub artists: Vec<Artist>,
}
