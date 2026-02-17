use reqwest::Client;
use tracing::info;

use crate::{
    apis::safebooru::schemas::SafebooruContentItem,
    errors::ScoutError,
    models::file::{ExternalFile, ExternalFileUrls},
    provider::ScoutProvider,
    url::Url,
};

mod schemas;

/// Minimum image width
const MIN_WIDTH: usize = 256;
/// Minimum image height
const MIN_HEIGHT: usize = 256;

/// Content aggregator from `safebooru.org`
pub struct SafebooruProvider {
    http_client: Client,
}

impl SafebooruProvider {
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }

    pub async fn fetch(
        &self,
        limit: usize,
        pid: usize,
    ) -> Result<Vec<SafebooruContentItem>, ScoutError> {
        let limit = limit.to_string();
        let pid = pid.to_string();

        let url = Url::new(
            "https://safebooru.org",
            "index.php",
            vec![
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("json", "1"),
                ("limit", &limit),
                ("pid", &pid),
            ],
        );

        let items = self
            .http_client
            .get(url.to_string())
            .send()
            .await?
            .json::<Vec<SafebooruContentItem>>()
            .await?;
        Ok(items)
    }
}

#[async_trait::async_trait]
impl ScoutProvider for SafebooruProvider {
    async fn fetch_content(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<Vec<ExternalFile>, ScoutError> {
        let raw_items = self.fetch(limit, page).await?;
        let items = raw_items
            .into_iter()
            .filter(|v| v.width >= MIN_WIDTH && v.height >= MIN_HEIGHT)
            .map(|i| ExternalFile {
                id: i.id.to_string(),
                files: ExternalFileUrls {
                    preview: i.preview_url,
                    sample: i.sample_url,
                    source: i.file_url,
                },
                hash: (!i.hash.is_empty()).then_some(i.hash),
                width: Some(i.width),
                height: Some(i.height),
                source: i.source.unwrap_or("https://safebooru.org".to_string()),
            })
            .collect::<Vec<_>>();

        info!("[SCOUT:SAFEBOURU] Fetched: {} items", items.len());

        Ok(items)
    }
}
