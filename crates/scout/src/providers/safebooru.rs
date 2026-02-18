use reqwest::Client;
use serde::Deserialize;
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

impl SafebooruProvider {
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }

    pub(crate) async fn fetch_list(
        &self,
        limit: usize,
        pid: usize,
    ) -> Result<Vec<SafebooruContentItem>, ScoutError> {
        let limit = limit.to_string();
        let pid = pid.to_string();

        let items = self
            .http_client
            .get(ENDPOINT)
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("json", "1"),
                ("limit", &limit),
                ("pid", &pid),
            ])
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
        let items = raw_items
            .into_iter()
            .filter(filter_content)
            .map(ExternalFile::from)
            .collect::<Vec<_>>();

        info!(api = "safebooru", count = items.len(), "Fetched");

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
