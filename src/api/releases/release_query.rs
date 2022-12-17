use urlencoding::encode;

use crate::{api::QueryOperator, components::releases::PrimaryGroupType};

use super::ReleaseSearch;

pub enum ReleaseQuery {
    Term(String),
    Search(Box<ReleaseSearch>),
}

impl ReleaseQuery {
    pub fn to_query(&self) -> String {
        match self {
            ReleaseQuery::Term(term) => term.to_string(),
            ReleaseQuery::Search(search) => {
                let mut searches: Vec<String> = Vec::new();

                if let Some(alias) = &search.alias {
                    searches.push(format!("alias:{}", encode(alias)));
                }

                if let Some(area) = &search.arid {
                    searches.push(format!("arid:{}", encode(area)));
                }

                if let Some(artist) = &search.artist {
                    searches.push(format!("artist:{}", encode(artist)));
                }

                if let Some(artist_accent) = &search.artist_name {
                    searches.push(format!("artist_name:{}", encode(artist_accent)));
                }

                if let Some(primary_type) = &search.primary_type {
                    let search_type = match primary_type {
                        PrimaryGroupType::Album => "album",
                        PrimaryGroupType::Single => "single",
                        PrimaryGroupType::EP => "EP",
                        PrimaryGroupType::Broadcast => "broadcast",
                        PrimaryGroupType::Other => "other",
                    };
                    searches.push(format!("primary_type:{}", encode(search_type)));
                }

                if let Some(comment) = &search.comment {
                    searches.push(format!("comment:{}", encode(comment)));
                }

                if let Some(country) = &search.country {
                    searches.push(format!("country:{}", encode(country)));
                }

                if let Some(ipi) = &search.ipi {
                    searches.push(format!("ipi:{}", encode(ipi)));
                }

                if let Some(release_name) = &search.release {
                    searches.push(format!("release:{}", encode(release_name)));
                }

                if let Some(release_name) = &search.release_accent {
                    searches.push(format!("release_accent:{}", encode(release_name)));
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
