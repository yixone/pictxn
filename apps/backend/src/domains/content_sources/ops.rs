use crate::{
    domains::content_sources::{id::ContentSourceId, model::ContentSource},
    result::Result,
};

#[async_trait::async_trait]
pub trait AbstractContentSources: Send + Sync {
    /// Insert a new content source into the database
    async fn insert_content_source(&self, source: &ContentSource) -> Result<()>;

    /// Insert a new content source or return existent row ID
    async fn insert_content_source_or_return_id(
        &self,
        source: &ContentSource,
    ) -> Result<ContentSourceId>;

    /// Get a content source from the database by ID
    async fn get_content_source(&self, id: &ContentSourceId) -> Result<Option<ContentSource>>;

    /// Get a content source from the database by domain
    async fn get_content_source_by_domain(&self, domain: &str) -> Result<Option<ContentSource>>;

    /// Delete a content source from the database by ID
    async fn delete_content_source(&self, id: &ContentSourceId) -> Result<()>;
}
