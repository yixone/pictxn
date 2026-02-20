use models::content_source::{ContentSource, SourceId};

use crate::{ops::content_source::AbstractContentSource, sqlite::SqliteDatabaseInner};

#[async_trait::async_trait]
impl AbstractContentSource for SqliteDatabaseInner {
    /// Insert a new content source into the database
    async fn insert_content_source(&self, source: &ContentSource) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
          INSERT INTO content_sources (
            id, source_domain
          )
          VALUES (
            ?, ?
          )
          ",
        )
        .bind(source.id)
        .bind(&source.source_domain)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get a content source from the database by ID
    async fn get_content_source(
        &self,
        id: &SourceId,
    ) -> Result<Option<ContentSource>, sqlx::Error> {
        sqlx::query_as(
            "
          SELECT id, source_domain
          FROM content_sources
          WHERE id = ?
        ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get a content source from the database by domain
    async fn get_content_source_by_domain(
        &self,
        domain: &str,
    ) -> Result<Option<ContentSource>, sqlx::Error> {
        sqlx::query_as(
            "
          SELECT id, source_domain
          FROM content_sources
          WHERE source_domain = ?
        ",
        )
        .bind(domain)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete a content source from the database by ID
    async fn delete_content_source(&self, id: &SourceId) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            DELETE FROM content_sources
            WHERE id = ?
            ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
