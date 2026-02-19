use models::files::{File, FileId};

use crate::{ops::files::AbstractFiles, sqlite::SqliteDatabaseInner};

#[async_trait::async_trait]
impl AbstractFiles for SqliteDatabaseInner {
    /// Insert a new file into the database
    async fn insert_file(&self, file: &File) -> Result<(), sqlx::Error> {
        todo!()
    }

    /// Get a file from the database by ID
    async fn get_file(&self, id: &FileId) -> Result<Option<File>, sqlx::Error> {
        todo!()
    }

    /// Delete a file from the database by ID
    async fn remove_file(&self, id: &FileId) -> Result<(), sqlx::Error> {
        todo!()
    }
}
