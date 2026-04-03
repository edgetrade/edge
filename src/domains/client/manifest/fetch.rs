use std::time::Duration;

use crate::domains::client::manifest::types::McpManifest;
use crate::messages;

const FETCH_TIMEOUT: Duration = Duration::from_secs(180); // 3 minutes
const INITIAL_DELAY: Duration = Duration::from_secs(1);
const MAX_DELAY: Duration = Duration::from_secs(30);
const MAX_RETRIES: u32 = 10;

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Timeout after 3 minutes")]
    Timeout,
    #[error("Unauthorized - check API key")]
    Unauthorized,
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

pub async fn fetch_manifest(url: &str, api_key: &str) -> Result<McpManifest, FetchError> {
    let client = reqwest::Client::new();
    let deadline = tokio::time::Instant::now() + FETCH_TIMEOUT;
    let mut delay = INITIAL_DELAY;
    let mut retries = 0;

    loop {
        match client.get(url).bearer_auth(api_key).send().await {
            Ok(r) if r.status().is_success() => {
                return r.json::<McpManifest>().await.map_err(FetchError::from);
            }
            Ok(r) if r.status() == 401 => {
                return Err(FetchError::Unauthorized);
            }
            Ok(r) => {
                messages::error::fetch_http_error(r.status().as_u16());
            }
            Err(e) => {
                messages::error::fetch_error(&e.to_string());
            }
        }

        if tokio::time::Instant::now() + delay > deadline || retries >= MAX_RETRIES {
            return Err(FetchError::Timeout);
        }

        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(MAX_DELAY);
        retries += 1;
    }
}

pub async fn fetch_manifest_raw(url: &str, api_key: &str) -> Result<Vec<u8>, FetchError> {
    let client = reqwest::Client::new();
    client
        .get(url)
        .bearer_auth(api_key)
        .send()
        .await?
        .bytes()
        .await
        .map(|b| b.to_vec())
        .map_err(FetchError::from)
}
