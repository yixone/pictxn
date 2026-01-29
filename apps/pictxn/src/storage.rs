use std::path::PathBuf;

use futures::Stream;
use tokio_util::bytes::Bytes;

use crate::types::files::FileWriteResult;

// type Stream<'a> = BoxStream<'a, std::io::Result<Bytes>>;

pub struct FileReadResult<S>
where
    S: Stream<Item = std::io::Result<Bytes>>,
{
    pub stream: S,
}

#[async_trait::async_trait]
pub trait StorageProvider {
    async fn set<S>(&self, key: &str, stream: S) -> std::io::Result<FileWriteResult>
    where
        S: Stream<Item = std::io::Result<Bytes>> + Send + Unpin;

    async fn get(
        &self,
        key: &str,
    ) -> std::io::Result<impl Stream<Item = std::io::Result<Bytes>> + Send>;

    async fn mv(&self, from: &str, to: &str) -> std::io::Result<()>;

    async fn del(&self, key: &str) -> std::io::Result<()>;
}

/// Implementation of file storage that works with the server file system
pub struct FsStorage {
    root: PathBuf,
}

impl FsStorage {
    pub fn init(root: impl Into<PathBuf>) -> std::io::Result<Self> {
        let root = root.into();
        std::fs::create_dir_all(&root)?;

        Ok(FsStorage { root })
    }

    fn generate_path_from_key(&self, key: &str) -> PathBuf {
        self.root.join(&key[..2]).join(&key[2..4]).join(key)
    }
}
