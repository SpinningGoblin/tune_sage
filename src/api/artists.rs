use serde::{Deserialize, Serialize};
use urlencoding::encode;

use crate::{
    components::artists::{Artist, ArtistList, ArtistType},
    errors::ApiError,
};

use super::{query_operator::QueryOperator, Config, GeneralOptions, Remote};

pub enum ArtistQuery {
    Term(String),
    Search(Box<ArtistSearch>),
}

#[derive(Clone, Debug)]
pub enum ArtistGenderQuery {
    Male,
    Female,
    Other,
    NotApplicable,
}

#[derive(Default)]
pub struct ArtistSearchBuilder {
    alias: Option<String>,
    primary_alias: Option<String>,
    area: Option<String>,
    artist: Option<String>,
    artist_accent: Option<String>,
    begin: Option<String>,
    begin_area: Option<String>,
    comment: Option<String>,
    country: Option<String>,
    end: Option<String>,
    end_area: Option<String>,
    ended: Option<bool>,
    gender: Option<ArtistGenderQuery>,
    ipi: Option<String>,
    isni: Option<String>,
    sort_name: Option<String>,
    tag: Option<String>,
    artist_type: Option<ArtistType>,
    operator: Option<QueryOperator>,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ArtistSearchBuilderError {
    #[error("Not enough options given for search")]
    NotEnoughOptionsGiven,
}

impl ArtistSearchBuilder {
    pub fn new() -> ArtistSearchBuilder {
        Self::default()
    }

    pub fn alias(&mut self, alias: &str) -> &mut ArtistSearchBuilder {
        self.alias = Some(alias.to_string());

        self
    }

    pub fn primary_alias(&mut self, primary_alias: &str) -> &mut ArtistSearchBuilder {
        self.primary_alias = Some(primary_alias.to_string());

        self
    }

    pub fn artist(&mut self, artist: &str) -> &mut ArtistSearchBuilder {
        self.artist = Some(artist.to_string());

        self
    }

    pub fn artist_accent(&mut self, artist_accent: &str) -> &mut ArtistSearchBuilder {
        self.artist_accent = Some(artist_accent.to_string());

        self
    }

    pub fn begin(&mut self, begin: &str) -> &mut ArtistSearchBuilder {
        self.begin = Some(begin.to_string());

        self
    }

    pub fn begin_area(&mut self, begin_area: &str) -> &mut ArtistSearchBuilder {
        self.begin_area = Some(begin_area.to_string());

        self
    }

    pub fn comment(&mut self, comment: &str) -> &mut ArtistSearchBuilder {
        self.comment = Some(comment.to_string());

        self
    }

    pub fn country(&mut self, country: &str) -> &mut ArtistSearchBuilder {
        self.country = Some(country.to_string());

        self
    }

    pub fn end(&mut self, end: &str) -> &mut ArtistSearchBuilder {
        self.end = Some(end.to_string());

        self
    }

    pub fn end_area(&mut self, end_area: &str) -> &mut ArtistSearchBuilder {
        self.end_area = Some(end_area.to_string());

        self
    }

    pub fn ended(&mut self, ended: bool) -> &mut ArtistSearchBuilder {
        self.ended = Some(ended);

        self
    }

    pub fn gender(&mut self, gender: ArtistGenderQuery) -> &mut ArtistSearchBuilder {
        self.gender = Some(gender);

        self
    }

    pub fn ipi(&mut self, ipi: &str) -> &mut ArtistSearchBuilder {
        self.ipi = Some(ipi.to_string());

        self
    }

    pub fn isni(&mut self, isni: &str) -> &mut ArtistSearchBuilder {
        self.isni = Some(isni.to_string());

        self
    }

    pub fn sort_name(&mut self, sort_name: &str) -> &mut ArtistSearchBuilder {
        self.sort_name = Some(sort_name.to_string());

        self
    }

    pub fn tag(&mut self, tag: &str) -> &mut ArtistSearchBuilder {
        self.tag = Some(tag.to_string());

        self
    }

    pub fn artist_type(&mut self, artist_type: ArtistType) -> &mut ArtistSearchBuilder {
        self.artist_type = Some(artist_type);

        self
    }

    fn invalid_builder_options(&self) -> bool {
        self.alias.is_none()
            && self.primary_alias.is_none()
            && self.area.is_none()
            && self.artist.is_none()
            && self.artist_accent.is_none()
            && self.begin.is_none()
            && self.begin_area.is_none()
            && self.comment.is_none()
            && self.country.is_none()
            && self.end.is_none()
            && self.end_area.is_none()
            && self.ended.is_none()
            && self.gender.is_none()
            && self.ipi.is_none()
            && self.isni.is_none()
            && self.sort_name.is_none()
            && self.tag.is_none()
            && self.artist_type.is_none()
    }

    pub fn build(&self) -> Result<ArtistSearch, ArtistSearchBuilderError> {
        if self.invalid_builder_options() {
            return Err(ArtistSearchBuilderError::NotEnoughOptionsGiven);
        }

        Ok(ArtistSearch {
            alias: self.alias.clone(),
            primary_alias: self.primary_alias.clone(),
            area: self.area.clone(),
            artist: self.artist.clone(),
            artist_accent: self.artist_accent.clone(),
            begin: self.begin.clone(),
            begin_area: self.begin_area.clone(),
            comment: self.comment.clone(),
            country: self.country.clone(),
            end: self.end.clone(),
            end_area: self.end_area.clone(),
            ended: self.ended,
            gender: self.gender.clone(),
            ipi: self.ipi.clone(),
            isni: self.isni.clone(),
            sort_name: self.sort_name.clone(),
            tag: self.tag.clone(),
            artist_type: self.artist_type.clone(),
            operator: self.operator.clone().unwrap_or_default(),
        })
    }
}

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
                    searches.push(format!("artistaccent:{}", encode(artist_accent)));
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
                    searches.push(format!("beginarea:{}", encode(begin_area)));
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
                    searches.push(format!("endarea:{}", encode(end_area)));
                }

                if let Some(ended) = &search.ended {
                    searches.push(format!("endarea:{}", encode(&ended.to_string())));
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

pub struct ArtistApi {
    pub config: Config,
    pub remote: Box<dyn Remote>,
}

impl ArtistApi {
    pub async fn by_id(&self, id: &str) -> Result<Artist, ApiError> {
        let url = format!("{}/artist/{}?fmt=json", &self.config.base_url, id);
        let text = self.remote.get_body(&url, &self.config.user_agent).await;

        text.and_then(|t| {
            serde_json::from_str(&t).map_err(|error| ApiError::DeserializationError {
                message: error.to_string(),
            })
        })
    }

    pub async fn query(
        &self,
        artist_query: &ArtistQuery,
        general_options: Option<GeneralOptions>,
    ) -> Result<ArtistList, ApiError> {
        let options = general_options.unwrap_or_default();
        let query = artist_query.to_query();
        let url = format!(
            "{}/artist?query={}&limit={}&offset={}&fmt=json",
            self.config.base_url, query, options.limit, options.offset
        );
        println!("{}", &url);
        let text = self.remote.get_body(&url, &self.config.user_agent).await;

        text.and_then(|t| {
            serde_json::from_str(&t).map_err(|error| ApiError::DeserializationError {
                message: error.to_string(),
            })
        })
    }
}
