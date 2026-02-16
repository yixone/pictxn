// TODO: add pagination
#[async_trait::async_trait]
pub trait ScoutProvider: Send + Sync {
    /// Provider error
    type Error;

    /// Searching for content from a self source
    async fn search_content(&self, search_string: &str) -> Result<(), Self::Error>;

    /// Get a list of files from a self source
    async fn fetch_content(&self) -> Result<(), Self::Error>;
}
