use crate::{
    domains::files::{id::FileId, model::File},
    result::Result,
};

#[async_trait::async_trait]
pub trait AbstractFiles: Send + Sync {
    /// Insert a new file into the database
    async fn insert_file(&self, file: &File) -> Result<()>;

    /// Get a file from the database by ID
    async fn get_file(&self, id: &FileId) -> Result<Option<File>>;

    /// Delete a file from the database by ID
    async fn remove_file(&self, id: &FileId) -> Result<()>;
}
