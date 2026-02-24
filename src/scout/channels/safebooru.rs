use std::time::Duration;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    result::Result,
    scout::{
        channels::base::BaseChannel,
        external_content::{id::ExternalContentId, model::ExternalContent},
    },
};

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
    /// Safebooru API Endpoint
    const SOURCE_ENDPOINT: &str = "https://safebooru.org/index.php";

    /// Channel ID
    const CHANNEL_ID: &str = "safebooru";

    pub fn new(client: reqwest::Client) -> Self {
        SafebooruChannel { client }
    }
}

#[async_trait::async_trait]
impl BaseChannel for SafebooruChannel {
    async fn fetch(&self, limit: u32, page: u32) -> Result<Vec<ExternalContent>> {
        let raw_items = self
            .client
            .get(Self::SOURCE_ENDPOINT)
            .query(&FetchQueryParams {
                page: "dapi",
                s: "post",
                q: "index",
                use_json: 1,
                limit,
                page_id: page,
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
                id: ExternalContentId::generate(),
                external_id: item.id.to_string(),
                created: Utc::now(),
                title: None,
                description: None,
                media_width: Some(item.width),
                media_height: Some(item.height),
                source: item.source.unwrap_or(Self::SOURCE_ENDPOINT.to_string()),
                file_preview_url: Some(item.preview_url),
                file_url: item.file_url,
            })
            .collect::<Vec<_>>();

        info!(channel = Self::CHANNEL_ID, count = items.len(), "Fetched");

        Ok(items)
    }
}
