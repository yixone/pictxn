use crate::provider::ScoutProvider;

/// Pinterest RSS Content Aggregator
///
/// Parses RSS feed and returns content
pub struct PinterestAggregator {
    /// reqwest client for HTTP requests
    pub client: reqwest::Client,

    /// List of PinterestRSS endpoint URLs
    pub rss_endpoints: Vec<String>,
}

impl PinterestAggregator {
    //
}

#[async_trait::async_trait]
impl ScoutProvider for PinterestAggregator {
    type Error = std::io::Error;

    async fn search_content(&self, search_string: &str) -> Result<(), Self::Error> {
        todo!()
    }

    async fn fetch_content(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
