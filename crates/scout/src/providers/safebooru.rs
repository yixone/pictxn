use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    ScoutProvider,
    errors::ScoutError,
    models::files::{ExternalFile, ExternalFileUrls},
};

/// Minimum image width
const MIN_WIDTH: usize = 256;
/// Minimum image height
const MIN_HEIGHT: usize = 256;

/// Safebooru API Endpoint
const ENDPOINT: &str = "https://safebooru.org/index.php";

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
    pub(crate) fn new(http_client: Client) -> Self {
        Self { http_client }
    }

    pub(crate) async fn fetch_list(
        &self,
        limit: usize,
        page_id: usize,
    ) -> Result<Vec<SafebooruContentItem>, ScoutError> {
        let items = self
            .http_client
            .get(ENDPOINT)
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

fn filter_content(item: &SafebooruContentItem) -> bool {
    item.width >= MIN_WIDTH && item.height >= MIN_HEIGHT
}

#[async_trait::async_trait]
impl ScoutProvider for SafebooruProvider {
    async fn fetch_content(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<Vec<ExternalFile>, ScoutError> {
        let raw_items = self.fetch_list(limit, page).await?;
        let items_count = raw_items.len();

        let items = raw_items
            .into_iter()
            .filter(filter_content)
            .map(ExternalFile::from)
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
    pub id: usize,
    pub image: String,
    pub owner: String,
    pub source: Option<String>,
}

impl From<SafebooruContentItem> for ExternalFile {
    fn from(item: SafebooruContentItem) -> Self {
        ExternalFile {
            id: item.id.to_string(),
            files: ExternalFileUrls {
                preview: item.preview_url,
                sample: item.sample_url,
                source: item.file_url,
            },
            hash: (!item.hash.is_empty()).then_some(item.hash),
            width: Some(item.width),
            height: Some(item.height),
            source: item.source.unwrap_or_else(|| ENDPOINT.to_string()),
        }
    }
}
