use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    api::{Cache, Config, GeneralOptions, Remote},
    components::artists::{Artist, ArtistList},
    errors::ApiError,
};

use super::{ArtistIncludeRelation, ArtistQuery};

pub struct ArtistApi {
    pub config: Config,
    pub remote: Arc<dyn Remote>,
    pub cache: Arc<Mutex<dyn Cache>>,
}

impl ArtistApi {
    pub async fn by_id(
        &mut self,
        id: &str,
        included_relations: Option<Vec<ArtistIncludeRelation>>,
    ) -> Result<Artist, ApiError> {
        let query_included_relations: String = included_relations
            .map(|names| names.into_iter().map(|name| name.to_string()).collect())
            .map(|names: Vec<String>| format!("&inc={}", names.join("%20")))
            .unwrap_or_else(|| "".to_string());
        let url = format!(
            "{}/artist/{}?fmt=json{}",
            &self.config.base_url, id, query_included_relations
        );

        let text = self.retrieve(&url).await?;

        serde_json::from_str(&text).map_err(|error| ApiError::DeserializationError {
            message: error.to_string(),
        })
    }

    pub async fn query(
        &mut self,
        artist_query: &ArtistQuery,
        general_options: Option<GeneralOptions>,
    ) -> Result<ArtistList, ApiError> {
        let options = general_options.unwrap_or_default();
        let query = artist_query.to_query();
        let url = format!(
            "{}/artist?query={}&limit={}&offset={}&fmt=json",
            self.config.base_url, query, options.limit, options.offset
        );
        let text = self.retrieve(&url).await?;

        serde_json::from_str(&text).map_err(|error| ApiError::DeserializationError {
            message: error.to_string(),
        })
    }

    async fn retrieve(&mut self, url: &str) -> Result<String, ApiError> {
        let mut cache = self.cache.lock().await;

        let text = match cache.get(url).await {
            Some(it) => return Ok(it.clone()),
            None => self.remote.get_body(url, &self.config.user_agent).await?,
        };

        cache.set(url, &text).await;
        Ok(text.clone())
    }
}
