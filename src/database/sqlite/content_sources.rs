use crate::{
    database::sqlite::db::SqliteDatabase,
    domains::content_sources::{
        id::ContentSourceId, model::ContentSource, ops::AbstractContentSources,
    },
    result::Result,
};

#[async_trait::async_trait]
impl AbstractContentSources for SqliteDatabase {
    /// Insert a new content source into the database
    async fn insert_content_source(&self, source: &ContentSource) -> Result<()> {
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

    /// Insert a new content source or return existent row ID
    async fn insert_content_source_or_return_id(
        &self,
        source: &ContentSource,
    ) -> Result<ContentSourceId> {
        let id = sqlx::query_scalar(
            "
          INSERT INTO content_sources(
            id, source_domain
          )
          VALUES (
            ?, ?
          )
          ON CONFLICT(source_domain)
            DO UPDATE SET id = id
          RETURNING id
          ",
        )
        .bind(source.id)
        .bind(&source.source_domain)
        .fetch_one(&self.pool)
        .await?;
        Ok(id)
    }

    /// Get a content source from the database by ID
    async fn get_content_source(&self, id: &ContentSourceId) -> Result<Option<ContentSource>> {
        let source = sqlx::query_as(
            "
          SELECT id, source_domain
          FROM content_sources
          WHERE id = ?
        ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(source)
    }

    /// Get a content source from the database by domain
    async fn get_content_source_by_domain(&self, domain: &str) -> Result<Option<ContentSource>> {
        let source = sqlx::query_as(
            "
          SELECT id, source_domain
          FROM content_sources
          WHERE source_domain = ?
        ",
        )
        .bind(domain)
        .fetch_optional(&self.pool)
        .await?;
        Ok(source)
    }

    /// Delete a content source from the database by ID
    async fn delete_content_source(&self, id: &ContentSourceId) -> Result<()> {
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
