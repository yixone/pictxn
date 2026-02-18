use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    ScoutProvider,
    errors::ScoutError,
    models::{
        cards::ScoutCard,
        files::{ScoutFile, ScoutFileUrls},
    },
    providers::ProviderType,
};

/// Minimum image width
const MIN_WIDTH: usize = 256;
/// Minimum image height
const MIN_HEIGHT: usize = 256;

/// Safebooru API Endpoint
const SOURCE_ENDPOINT: &str = "https://safebooru.org/index.php";

/// Structure for working with safebooru API
pub struct SafebooruProvider {
    http_client: Client,
}

#[derive(Serialize)]
struct SafebooruFetchQuery<'a> {
    page: &'a str,
    s: &'a str,
    q: &'a str,
    #[serde(rename = "json")]
    use_json: u8,
    limit: usize,
    #[serde(rename = "pid")]
    page_id: usize,
}

impl SafebooruProvider {
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }

    pub(crate) async fn fetch_list(
        &self,
        limit: usize,
        page_id: usize,
    ) -> Result<Vec<SafebooruContentItem>, ScoutError> {
        let items = self
            .http_client
            .get(SOURCE_ENDPOINT)
            .query(&SafebooruFetchQuery {
                page: "dapi",
                s: "post",
                q: "index",
                use_json: 1,
                limit,
                page_id,
            })
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<SafebooruContentItem>>()
            .await?;
        Ok(items)
    }
}

fn filter_content(i: &SafebooruContentItem) -> bool {
    i.width >= MIN_WIDTH && i.height >= MIN_HEIGHT
}

fn map_item(i: SafebooruContentItem) -> ScoutCard {
    ScoutCard {
        provider: ProviderType::Safebooru,
        title: None,
        description: None,
        file: ScoutFile {
            files: ScoutFileUrls {
                preview: Some(i.preview_url),
                sample: Some(i.sample_url),
                original: i.file_url,
            },
            width: Some(i.width),
            height: Some(i.height),
        },
        origin_url: i.source.unwrap_or_else(|| SOURCE_ENDPOINT.to_string()),
    }
}

#[async_trait::async_trait]
impl ScoutProvider for SafebooruProvider {
    async fn fetch_content(&self, limit: usize, page: usize) -> Result<Vec<ScoutCard>, ScoutError> {
        let raw_items = self.fetch_list(limit, page).await?;
        let items_count = raw_items.len();

        let items = raw_items
            .into_iter()
            .filter(filter_content)
            .map(map_item)
            .collect::<Vec<_>>();

        info!(
            api = "safebooru",
            count = items.len(),
            denied = items_count - items.len(),
            "Fetched"
        );

        Ok(items)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SafebooruContentItem {
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
