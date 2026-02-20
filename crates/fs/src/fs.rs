use std::path::{Path, PathBuf};

use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use result::{AppError, Result};
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
    time::Instant,
};
use tokio_util::io::ReaderStream;
use tracing::error;

use crate::{
    BUFF_WRITER_CAPACITY, MAX_FILE_SIZE,
    hash::FileHash,
    provider::{FileStorageProvider, OutputSetFile},
    stream::FileStream,
};

const TMP_DIR: &str = "tmp";

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

    fn temp_path_from_key(&self, key: &str) -> PathBuf {
        self.root.join(TMP_DIR).join(&key[..2]).join(key)
    }

    async fn write_fallback(path: &Path) {
        if path.is_file()
            && let Err(e) = tokio::fs::remove_file(path).await
        {
            error!(err = ?e, "Failed to execute write fallback");
        }
    }

    async fn create_parent_dir(path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl FileStorageProvider for NativeFsProvider {
    /// Get file by key
    async fn get_file(&self, key: &str) -> Result<FileStream<'static>> {
        let path = self.os_path_from_key(key);
        let file = File::open(path).await?;

        let reader = ReaderStream::new(file);
        let stream = Box::pin(reader);

        Ok(stream)
    }

    /// Save file by stream
    async fn set_from_stream<'a>(
        &self,
        key: &str,
        stream: &mut FileStream<'a>,
    ) -> Result<OutputSetFile> {
        let start_time = Instant::now();

        let mut file_hasher = Sha256::new();
        let mut file_size = 0_u64;

        let temp_path = self.temp_path_from_key(key);
        let temp_file = File::create_new(&temp_path).await?;

        let mut target_writer = BufWriter::with_capacity(BUFF_WRITER_CAPACITY, temp_file);

        let write_result = {
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;

                file_size += chunk.len() as u64;
                if file_size > MAX_FILE_SIZE {
                    Err(result::AppError::TooLargeInput {
                        received: file_size,
                        excepted: MAX_FILE_SIZE,
                    })?;
                }

                file_hasher.update(&chunk);
                target_writer.write_all(&chunk).await?;
            }
            Ok(())
        };
        if let Err(e) = write_result {
            drop(target_writer);
            Self::write_fallback(&temp_path).await;

            return Err(e);
        }

        target_writer.flush().await?;

        let path = self.os_path_from_key(key);
        Self::create_parent_dir(&path).await?;
        tokio::fs::rename(temp_path, path).await?;

        // Casting to u64 because the average file loading time (in ms) should not exceed the u64 size
        let loading_time = start_time.elapsed().as_millis() as u64;
        let file_hash = FileHash::from(file_hasher);
        let timestamp = chrono::Utc::now().timestamp();

        let res = OutputSetFile {
            file_hash,
            file_size,
            timestamp,
            loading_time,
        };
        Ok(res)
    }

    /// Save file by URL
    async fn set_from_url(&self, key: &str, url: &str, client: &Client) -> Result<OutputSetFile> {
        let mut res = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes_stream()
            .map_err(std::io::Error::other)
            .boxed();

        self.set_from_stream(key, &mut res).await
    }

    /// Delete file by key
    async fn delete(&self, key: &str) -> Result<()> {
        let path = self.os_path_from_key(key);
        tokio::fs::remove_file(path).await?;

        Ok(())
    }
}
