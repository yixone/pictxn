use chrono::{DateTime, Utc};

use crate::{
    result::Result,
    scout::external_content::{id::ExternalContentId, model::ExternalContent},
};

#[async_trait::async_trait]
pub trait AbstractExternalContent: Send + Sync {
    /// Insert multiple ExternalContent
    async fn insert_external_content_many(&self, items: &[ExternalContent]) -> Result<()>;

    /// Get ExternalContent by ID
    async fn get_external_content(&self, id: &ExternalContentId)
    -> Result<Option<ExternalContent>>;

    /// Get a list of ExternalContent
    async fn list_external_content(&self) -> Result<Vec<ExternalContent>>;

    /// Get the number of records in the external_content table
    async fn count_external_content(&self) -> Result<u32>;

    /// Delete ExternalContent by ID
    async fn remove_external_content(&self, id: &ExternalContentId) -> Result<()>;

    /// Remove ExternalContent older than the specified date
    async fn remove_old_external_content(&self, date: DateTime<Utc>) -> Result<()>;
}
