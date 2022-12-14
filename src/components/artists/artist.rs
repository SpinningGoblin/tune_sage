use serde::{Deserialize, Serialize};

use crate::components::{
    recordings::IncludedRecording, releases::IncludedRelease, Area, Genre, LifeSpan, Tag,
};

use super::ArtistType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(alias = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(alias = "type")]
    pub artist_type: Option<ArtistType>,
    pub gender: Option<String>,
    pub area: Option<Area>,
    pub begin_area: Option<Area>,
    #[serde(alias = "life-span")]
    pub life_span: Option<LifeSpan>,
    pub disambiguation: Option<String>,
    pub score: Option<u32>,
    pub releases: Option<Vec<IncludedRelease>>,
    pub recordings: Option<Vec<IncludedRecording>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    pub isnis: Option<Vec<String>>,
}
