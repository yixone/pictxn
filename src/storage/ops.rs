use crate::{
    result::Result,
    storage::types::{dto::OutputTempFile, stream::BoxedStream},
};

#[async_trait::async_trait]
pub trait AbstractFileStorage: Send + Sync {
    /// Get file by key
    async fn get_file(&self, key: &str) -> Result<BoxedStream<'static>>;

    /// Save file to a temporary directory
    async fn save_temp<'a>(&self, stream: &mut BoxedStream<'a>) -> Result<OutputTempFile>;

    /// Save a temporary file to permanent storage
    async fn promote(&self, temp: OutputTempFile, key: &str) -> Result<()>;

    /// Delete temporary file
    async fn delete_temp(&self, temp: OutputTempFile) -> Result<()>;

    /// Delete file by key
    async fn delete(&self, key: &str) -> Result<()>;
}
