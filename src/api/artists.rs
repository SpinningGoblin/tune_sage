use crate::{
    components::artists::{Artist, ArtistList},
    errors::ApiError,
};

struct SellPrice(u32);

impl Default for SellPrice {
    fn default() -> Self {
        Self(100)
    }
}

use super::{Config, Remote};

pub enum ArtistQuery {
    Term(String),
    Search(ArtistSearch),
}

pub struct ArtistSearchBuilder {
    term: Option<String>,
}

impl ArtistSearchBuilder {
    pub fn new() -> ArtistSearchBuilder {
        Self { term: None }
    }

    pub fn term(&mut self, term: &str) -> &mut ArtistSearchBuilder {
        self.term = Some(term.to_string());

        self
    }

    pub fn build(&self) -> ArtistSearch {
        ArtistSearch {
            term: self.term.clone(),
        }
    }
}

pub struct ArtistSearch {
    pub term: Option<String>,
}

impl ArtistQuery {
    pub fn to_query(&self) -> String {
        match self {
            ArtistQuery::Term(term) => term.to_string(),
            ArtistQuery::Search(_) => "".to_string(),
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

    pub async fn query(&self, artist_query: &ArtistQuery) -> Result<ArtistList, ApiError> {
        let query = artist_query.to_query();
        let url = format!("{}/artist?query={}&fmt=json", self.config.base_url, query);
        let text = self.remote.get_body(&url, &self.config.user_agent).await;

        text.and_then(|t| {
            serde_json::from_str(&t).map_err(|error| ApiError::DeserializationError {
                message: error.to_string(),
            })
        })
    }
}
