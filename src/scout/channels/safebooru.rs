use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    result::Result,
    scout::{channels::base::BaseChannel, content::ScoutContentItem},
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
    pub width: usize,
    pub height: usize,
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
    async fn fetch(&self, limit: u32, page: u32) -> Result<Vec<ScoutContentItem>> {
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
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<SafebooruApiResponse>>()
            .await?;

        let items = raw_items
            .into_iter()
            .map(|item| ScoutContentItem {
                channel_name: Self::CHANNEL_ID,
                title: None,
                description: None,
                origin_url: item.source,
                media_width: Some(item.width),
                media_height: Some(item.height),
                file_preview: Some(item.preview_url),
                file_sample: Some(item.sample_url),
                file_original: item.file_url,
            })
            .collect::<Vec<_>>();

        info!(channel = Self::CHANNEL_ID, count = items.len(), "Fetched");

        Ok(items)
    }
}
