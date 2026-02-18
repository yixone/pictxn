use models::files::{File, FileId};

#[async_trait::async_trait]
pub trait AbstractFiles: Send + Sync {
    /// Insert a new file into the database
    async fn insert_file(&self, file: &File) -> Result<(), sqlx::Error>;

    /// Get a file from the database by ID
    async fn get_file(&self, id: &FileId) -> Result<Option<File>, sqlx::Error>;

    /// Delete a file from the database by ID
    async fn remove_file(&self, id: &FileId) -> Result<(), sqlx::Error>;
}
