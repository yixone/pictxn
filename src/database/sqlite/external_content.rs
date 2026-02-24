use chrono::{DateTime, Utc};
use sqlx::QueryBuilder;

use crate::{
    database::sqlite::db::SqliteDatabase,
    result::Result,
    scout::external_content::{
        id::ExternalContentId, model::ExternalContent, ops::AbstractExternalContent,
    },
};

#[async_trait::async_trait]
impl AbstractExternalContent for SqliteDatabase {
    /// Insert multiple ExternalContent
    async fn insert_external_content_many(&self, items: &[ExternalContent]) -> Result<()> {
        let mut qb = QueryBuilder::new(
            "
            INSERT OR IGNORE INTO external_content (
                id, external_id, created,
                title, description,
                source_id,
                media_width, media_height,
                file_preview_url, file_url
            )
            VALUES",
        );
        qb.push_values(items.iter(), |mut qb, i| {
            qb.push_bind(i.id)
                .push_bind(&i.external_id)
                .push_bind(i.created)
                .push_bind(&i.title)
                .push_bind(&i.description)
                .push_bind(i.source_id)
                .push_bind(i.media_width)
                .push_bind(i.media_height)
                .push_bind(&i.file_preview_url)
                .push_bind(&i.file_url);
        });

        qb.build().execute(&self.pool).await?;

        Ok(())
    }

    /// Get ExternalContent by ID
    async fn get_external_content(
        &self,
        id: &ExternalContentId,
    ) -> Result<Option<ExternalContent>> {
        let item = sqlx::query_as(
            "
                SELECT 
                    id, external_id, created,
                    title, description,
                    source_id,
                    media_width, media_height,
                    file_preview_url, file_url
                FROM external_content
                WHERE id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(item)
    }

    /// Get a list of ExternalContent
    async fn list_external_content(&self, limit: u32, offset: u32) -> Result<Vec<ExternalContent>> {
        let items = sqlx::query_as(
            "
                SELECT 
                    id, external_id, created,
                    title, description,
                    source_id,
                    media_width, media_height,
                    file_preview_url, file_url
                FROM external_content
                LIMIT ?
                OFFSET ?
            ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(items)
    }

    /// Get the number of records in the external_content table
    async fn count_external_content(&self) -> Result<u32> {
        let count = sqlx::query_scalar(
            "
                SELECT COUNT(*)
                FROM external_content
            ",
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }

    /// Delete ExternalContent by ID
    async fn remove_external_content(&self, id: &ExternalContentId) -> Result<()> {
        sqlx::query(
            "
            DELETE FROM external_content
            WHERE id = ?
            ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Remove ExternalContent older than the specified date
    async fn remove_old_external_content(&self, date: DateTime<Utc>, limit: u32) -> Result<()> {
        sqlx::query(
            "
            DELETE FROM external_content
            WHERE created < ?
            LIMIT ?
            ",
        )
        .bind(date)
        .bind(limit)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
