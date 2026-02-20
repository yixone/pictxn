use std::path::PathBuf;

use result::Result;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{provider::FileStorageProvider, stream::FileStream};

pub struct NativeFsProvider {
    /// Root directory of file storage
    root: PathBuf,
}

impl NativeFsProvider {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        NativeFsProvider { root: root.into() }
    }

    fn os_path_from_key(&self, key: &str) -> PathBuf {
        self.root.join(&key[..2]).join(&key[2..4]).join(key)
    }
}

#[async_trait::async_trait]
impl FileStorageProvider for NativeFsProvider {
    /// Get file by key
    async fn get_file<'a>(&'a self, key: &str) -> Result<FileStream<'a>> {
        let path = self.os_path_from_key(key);
        let file = File::open(path).await?;

        let reader = ReaderStream::new(file);
        let stream = Box::pin(reader);

        Ok(stream)
    }

    /// Save file by stream
    async fn set_from_stream(&self, key: &str, stream: &mut FileStream<'static>) -> Result<()> {
        let path = self.os_path_from_key(key);
        todo!()
    }
    /// Save file by URL
    async fn set_from_url(&self, key: &str, url: &str) -> Result<()> {
        let path = self.os_path_from_key(key);
        todo!()
    }

    /// Delete file by key
    async fn delete(&self, key: &str) -> Result<()> {
        let path = self.os_path_from_key(key);
        todo!()
    }
}
