use std::time::Duration;

use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    result::Result,
    scout::{channels::base::BaseChannel, external_content::model::ExternalContent},
    util,
};

/// Maximum page for random pagination
const MAX_PAGE: u32 = 5000;
/// Safebooru API Endpoint
const SOURCE_ENDPOINT: &str = "https://safebooru.org/index.php";
/// Channel ID
const CHANNEL_ID: &str = "safebooru";

#[derive(Debug)]
pub struct SafebooruChannel {
    client: reqwest::Client,
}

#[derive(Serialize)]
struct FetchQueryParams {
    page: &'static str,
    s: &'static str,
    q: &'static str,
    #[serde(rename = "json")]
    use_json: u8,
    limit: u32,
    #[serde(rename = "pid")]
    page_id: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SafebooruApiResponse {
    pub preview_url: String,
    pub sample_url: String,
    pub file_url: String,
    pub hash: String,
    pub width: u32,
    pub height: u32,
    pub id: i64,
    pub image: String,
    pub owner: String,
    pub source: Option<String>,
}

impl SafebooruChannel {
    pub fn new(client: reqwest::Client) -> Self {
        SafebooruChannel { client }
    }
}

#[async_trait::async_trait]
impl BaseChannel for SafebooruChannel {
    async fn fetch(&self, limit: u32) -> Result<Vec<ExternalContent>> {
        let page_id = rand::rng().random_range(0..MAX_PAGE);

        let raw_items = self
            .client
            .get(SOURCE_ENDPOINT)
            .query(&FetchQueryParams {
                page: "dapi",
                s: "post",
                q: "index",
                use_json: 1,
                limit,
                page_id,
            })
            .timeout(Duration::from_secs(4))
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<SafebooruApiResponse>>()
            .await?;

        let items = raw_items
            .into_iter()
            .map(|item| ExternalContent {
                external_id: item.id.to_string(),
                title: None,
                description: None,
                media_width: Some(item.width),
                media_height: Some(item.height),
                source: item.source.unwrap_or(SOURCE_ENDPOINT.to_string()),
                file_preview_url: Some(item.preview_url),
                file_url: item.file_url,
            })
            .collect::<Vec<_>>();

        info!(channel = CHANNEL_ID, count = items.len(), "Fetched");

        Ok(items)
    }
}
