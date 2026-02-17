use crate::{errors::ScoutError, models::file::ExternalFile};

// TODO: add pagination
#[async_trait::async_trait]
pub trait ScoutProvider: Send + Sync {
    /// Get a list of files from a self source
    async fn fetch_content(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<Vec<ExternalFile>, ScoutError>;
}
