use std::path::PathBuf;

use futures::Stream;
use tokio::fs::File;
use tokio_util::{bytes::Bytes, io::ReaderStream};

use crate::types::files::FileWriteResult;

// type Stream<'a> = BoxStream<'a, std::io::Result<Bytes>>;

const MAX_FILE_SIZE: usize = 256 * crate::MEGABYTES;

#[async_trait::async_trait]
pub trait StorageProvider {
    async fn set<S>(&self, key: &str, stream: S) -> std::io::Result<FileWriteResult>
    where
        S: Stream<Item = std::io::Result<Bytes>> + Send + Unpin;

    type FileGetResult: Stream<Item = std::io::Result<Bytes>> + Send + 'static;

    async fn get(&self, key: &str) -> std::io::Result<Self::FileGetResult>;

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

#[async_trait::async_trait]
impl StorageProvider for FsStorage {
    async fn set<S>(&self, key: &str, stream: S) -> std::io::Result<FileWriteResult>
    where
        S: Stream<Item = std::io::Result<Bytes>> + Send + Unpin,
    {
        let path = self.generate_path_from_key(key);
        crate::helpers::fs::create_all_parents(&path).await?;

        match crate::streams::write_in_file(&path, stream, MAX_FILE_SIZE).await {
            Ok(r) => Ok(r),
            Err(e) => {
                crate::helpers::fs::try_delete_file(&path).await;
                Err(e)
            }
        }
    }

    type FileGetResult = ReaderStream<File>;

    async fn get(&self, key: &str) -> std::io::Result<Self::FileGetResult> {
        let path = self.generate_path_from_key(key);
        let stream = crate::streams::read_from_file(path).await?;

        Ok(stream)
    }

    async fn mv(&self, from: &str, to: &str) -> std::io::Result<()> {
        let from = self.generate_path_from_key(from);
        let to = self.generate_path_from_key(to);

        tokio::fs::rename(from, to).await
    }

    async fn del(&self, key: &str) -> std::io::Result<()> {
        let path = self.generate_path_from_key(key);
        tokio::fs::remove_file(path).await
    }
}
