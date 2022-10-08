use crate::{
    api::{Config, GeneralOptions, Remote},
    components::artists::{Artist, ArtistList},
    errors::ApiError,
};

use super::ArtistQuery;

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
        let text = self.remote.get_body(&url, &self.config.user_agent).await;

        text.and_then(|t| {
            serde_json::from_str(&t).map_err(|error| ApiError::DeserializationError {
                message: error.to_string(),
            })
        })
    }
}
