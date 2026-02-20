use result::Result;

use crate::stream::FileStream;

#[async_trait::async_trait]
pub trait FileStorageProvider: Send + Sync {
    /// Get file by key
    async fn get_file<'a>(&'a self, key: &str) -> Result<FileStream<'a>>;

    /// Save file by stream
    async fn set_from_stream(&self, key: &str, stream: &mut FileStream<'static>) -> Result<()>;
    /// Save file by URL
    async fn set_from_url(&self, key: &str, url: &str) -> Result<()>;

    /// Delete file by key
    async fn delete(&self, key: &str) -> Result<()>;
}
