use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize)]
pub struct SampleRow {
    pub id: i32,
    pub name: String,
    pub value: String,
}

pub struct CoreClient {
    base_url: String,
    client: Client,
}

impl CoreClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_row(&self, id: i32, token: &str) -> Result<SampleRow, CoreError> {
        let resp = self
            .client
            .get(format!("{}/rows/{}", self.base_url, id))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp)
    }
}
