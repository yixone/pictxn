use models::content_source::{ContentSource, SourceId};

#[async_trait::async_trait]
pub trait AbstractContentSource: Send + Sync {
    /// Insert a new content source into the database
    async fn insert_content_source(&self, source: &ContentSource) -> Result<(), sqlx::Error>;

    /// Get a content source from the database by ID
    async fn get_content_source(&self, id: &SourceId)
    -> Result<Option<ContentSource>, sqlx::Error>;

    /// Get a content source from the database by domain
    async fn get_content_source_by_domain(
        &self,
        domain: &str,
    ) -> Result<Option<ContentSource>, sqlx::Error>;

    /// Delete a content source from the database by ID
    async fn delete_content_source(&self, id: &SourceId) -> Result<(), sqlx::Error>;
}
