use serde::{Deserialize, Serialize};

use super::ArtistType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludedArtist {
    pub id: String,
    #[serde(alias = "type")]
    pub artist_type: Option<ArtistType>,
    pub disambiguation: Option<String>,
    pub name: String,
    #[serde(alias = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(alias = "type-id")]
    pub type_id: Option<String>,
}
