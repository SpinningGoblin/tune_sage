use crate::errors::ApiError;

#[async_trait::async_trait]
pub trait Remote {
    async fn get_body(&self, url: &str, user_agent: &str) -> Result<String, ApiError>;
}

pub struct HttpRemote;

#[async_trait::async_trait]
impl Remote for HttpRemote {
    async fn get_body(&self, url: &str, user_agent: &str) -> Result<String, ApiError> {
        let response = match reqwest::Client::new()
            .get(url)
            .header("User-Agent", user_agent)
            .send()
            .await
        {
            Ok(it) => it,
            Err(e) => {
                return Err(ApiError::Unknown {
                    message: e.to_string(),
                })
            }
        };

        response.text().await.map_err(|e| ApiError::ReadBodyError {
            message: e.to_string(),
        })
    }
}
