use serde::{Deserialize, Serialize};

use crate::components::{Area, LifeSpan};

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
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ArtistType {
    Person,
    Group,
    Orchestra,
    Choir,
    Character,
    Other,
}

impl Default for ArtistType {
    fn default() -> Self {
        Self::Other
    }
}
