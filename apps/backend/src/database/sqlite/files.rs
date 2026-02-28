use crate::{
    database::sqlite::db::SqliteDatabase,
    domains::files::{id::FileId, model::File, ops::AbstractFiles},
    result::Result,
};

#[async_trait::async_trait]
impl AbstractFiles for SqliteDatabase {
    /// Insert a new file into the database
    async fn insert_file(&self, file: &File) -> Result<()> {
        sqlx::query(
            "
            INSERT INTO files (
                id, source_id, source_url, created,
                sha256, filename, content_type, size
            )
            VALUES (
                ?, ?, ?, ?,
                ?, ?, ?, ?
            )
            ",
        )
        .bind(file.id)
        .bind(file.source_id)
        .bind(&file.source_url)
        .bind(file.created)
        .bind(&file.sha256)
        .bind(&file.filename)
        .bind(&file.content_type)
        .bind(file.size)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get a file from the database by ID
    async fn get_file(&self, id: &FileId) -> Result<Option<File>> {
        let file = sqlx::query_as(
            "
            SELECT 
                id, file_id, created, title, description
            FROM files
            WHERE id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(file)
    }

    /// Delete a file from the database by ID
    async fn remove_file(&self, id: &FileId) -> Result<()> {
        sqlx::query(
            "
            DELETE FROM files
            WHERE id = ?
            ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
