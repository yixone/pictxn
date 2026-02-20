use result::Result;

use crate::{hash::FileHash, stream::FileStream};

pub struct OutputSetFile {
    pub file_hash: FileHash,
    pub file_size: u64,

    pub timestamp: i64,
    pub loading_time: u64,
}

#[async_trait::async_trait]
pub trait FileStorageProvider: Send + Sync {
    /// Get file by key
    async fn get_file(&self, key: &str) -> Result<FileStream<'static>>;

    /// Save file by stream
    async fn set_from_stream<'a>(
        &self,
        key: &str,
        stream: &mut FileStream<'a>,
    ) -> Result<OutputSetFile>;
    /// Save file by URL
    async fn set_from_url(&self, key: &str, url: &str) -> Result<OutputSetFile>;

    /// Delete file by key
    async fn delete(&self, key: &str) -> Result<()>;
}
