use crate::{api::QueryOperator, components::artists::ArtistType};

use super::ArtistGenderQuery;

pub struct ArtistSearch {
    pub alias: Option<String>,
    pub primary_alias: Option<String>,
    pub area: Option<String>,
    pub artist: Option<String>,
    pub artist_accent: Option<String>,
    pub begin: Option<String>,
    pub begin_area: Option<String>,
    pub comment: Option<String>,
    pub country: Option<String>,
    pub end: Option<String>,
    pub end_area: Option<String>,
    pub ended: Option<bool>,
    pub gender: Option<ArtistGenderQuery>,
    pub ipi: Option<String>,
    pub isni: Option<String>,
    pub sort_name: Option<String>,
    pub tag: Option<String>,
    pub artist_type: Option<ArtistType>,
    pub operator: QueryOperator,
}
