use urlencoding::encode;

use crate::{api::QueryOperator, components::artists::ArtistType};

use super::ArtistSearch;

#[derive(Clone, Debug)]
pub enum ArtistGenderQuery {
    Male,
    Female,
    Other,
    NotApplicable,
}

pub enum ArtistQuery {
    Term(String),
    Search(Box<ArtistSearch>),
}

impl ArtistQuery {
    pub fn to_query(&self) -> String {
        match self {
            ArtistQuery::Term(term) => term.to_string(),
            ArtistQuery::Search(search) => {
                let mut searches: Vec<String> = Vec::new();

                if let Some(alias) = &search.alias {
                    searches.push(format!("alias:{}", encode(alias)));
                }

                if let Some(area) = &search.area {
                    searches.push(format!("area:{}", encode(area)));
                }

                if let Some(artist) = &search.artist {
                    searches.push(format!("artist:{}", encode(artist)));
                }

                if let Some(artist_accent) = &search.artist_accent {
                    searches.push(format!("artist_accent:{}", encode(artist_accent)));
                }

                if let Some(artist_type) = &search.artist_type {
                    let search_type = match artist_type {
                        ArtistType::Person => "person",
                        ArtistType::Group => "group",
                        ArtistType::Orchestra => "orchestra",
                        ArtistType::Choir => "choir",
                        ArtistType::Character => "character",
                        ArtistType::Other => "other",
                    };
                    searches.push(format!("type:{}", encode(search_type)));
                }

                if let Some(begin) = &search.begin {
                    searches.push(format!("begin:{}", encode(begin)));
                }

                if let Some(begin_area) = &search.begin_area {
                    searches.push(format!("begin_area:{}", encode(begin_area)));
                }

                if let Some(comment) = &search.comment {
                    searches.push(format!("comment:{}", encode(comment)));
                }

                if let Some(country) = &search.country {
                    searches.push(format!("country:{}", encode(country)));
                }

                if let Some(end) = &search.end {
                    searches.push(format!("end:{}", encode(end)));
                }

                if let Some(end_area) = &search.end_area {
                    searches.push(format!("end_area:{}", encode(end_area)));
                }

                if let Some(ended) = &search.ended {
                    searches.push(format!("ended:{}", encode(&ended.to_string())));
                }

                if let Some(gender) = &search.gender {
                    let gender_search = match gender {
                        ArtistGenderQuery::Male => "male",
                        ArtistGenderQuery::Female => "female",
                        ArtistGenderQuery::Other => "other",
                        ArtistGenderQuery::NotApplicable => "not applicable",
                    };
                    searches.push(format!("gender:{}", encode(gender_search)));
                }

                if let Some(ipi) = &search.ipi {
                    searches.push(format!("ipi:{}", encode(ipi)));
                }

                if let Some(isni) = &search.isni {
                    searches.push(format!("isni:{}", encode(isni)));
                }

                if let Some(primary_alias) = &search.primary_alias {
                    searches.push(format!("primary_alias:{}", encode(primary_alias)));
                }

                if let Some(sort_name) = &search.sort_name {
                    searches.push(format!("sortname:{}", encode(sort_name)));
                }

                if let Some(tag) = &search.tag {
                    searches.push(format!("tag:{}", encode(tag)));
                }

                let separator = match search.operator {
                    QueryOperator::And => "%20AND%20",
                    QueryOperator::Or => "%20OR%20",
                };

                searches.join(separator)
            }
        }
    }
}
