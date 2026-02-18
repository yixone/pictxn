pub mod providers;

pub mod errors;
pub mod models;

#[async_trait::async_trait]
pub trait ScoutProvider: Send + Sync {
    /// Get a list of files from a self source
    async fn fetch_content(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<Vec<models::files::ExternalFile>, errors::ScoutError>;
}
