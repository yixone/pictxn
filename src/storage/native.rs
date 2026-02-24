use std::path::PathBuf;

use chrono::Utc;
use futures::StreamExt;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
    time::Instant,
};
use tokio_util::io::ReaderStream;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    result::{Result, errors::AppError},
    storage::{
        ops::AbstractFileStorage,
        types::{dto::OutputTempFile, hash::FileHash, stream::BoxedStream},
    },
};

const BUFF_WRITER_CAPACITY: usize = 64 * 1024;
const MAX_FILE_SIZE: u64 = 512 * 1024 * 1024;

pub struct NativeFS {
    root_dir: PathBuf,
    temp_dir: PathBuf,
}

impl NativeFS {
    pub fn new(root_dir: impl Into<PathBuf>, temp_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
            temp_dir: temp_dir.into(),
        }
    }

    pub fn init(&self) -> Result<()> {
        if let Some(parent) = self.root_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if let Some(parent) = self.temp_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    fn real_path_from_key(&self, key: &str) -> PathBuf {
        self.root_dir.join(&key[..2]).join(key)
    }

    fn temp_path_from_key(&self, key: &str) -> PathBuf {
        self.temp_dir.join(key)
    }
}

#[async_trait::async_trait]
impl AbstractFileStorage for NativeFS {
    /// Get file by key
    async fn get_file(&self, key: &str) -> Result<BoxedStream<'static>> {
        let path = self.real_path_from_key(key);
        let file = File::open(path).await?;

        let reader = ReaderStream::new(file);
        let stream = Box::pin(reader);

        Ok(stream)
    }

    /// Save file to a temporary directory
    async fn save_temp<'a>(&self, stream: &mut BoxedStream<'a>) -> Result<OutputTempFile> {
        let start_time = Instant::now();

        let key = Uuid::new_v4().to_string();
        let path = self.temp_path_from_key(&key);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file_hasher = Sha256::new();
        let mut file_size = 0_u64;

        let file = File::create_new(path).await?;
        let mut writer = BufWriter::with_capacity(BUFF_WRITER_CAPACITY, file);

        while let Some(next_res) = stream.next().await {
            let chunk = next_res?;

            file_size += chunk.len() as u64;
            if file_size > MAX_FILE_SIZE {
                Err(AppError::TooLargeInput {
                    received: file_size,
                    excepted: MAX_FILE_SIZE,
                })?;
            }

            file_hasher.update(&chunk);
            writer.write_all(&chunk).await?;
        }
        writer.flush().await?;

        // Casting to u64 because the average file loading time (in ms) should not exceed the u64 size
        let uploading_time = start_time.elapsed().as_millis() as u64;
        let file_hash = FileHash::from(file_hasher);
        let timestamp = Utc::now().timestamp();

        Ok(OutputTempFile {
            key,
            file_hash,
            file_size,
            timestamp,
            uploading_time,
        })
    }

    /// Save a temporary file to permanent storage
    async fn promote(&self, temp: OutputTempFile, key: &str) -> Result<()> {
        let temp_path = self.temp_path_from_key(&temp.key);

        let final_path = self.real_path_from_key(key);
        if let Some(parent) = final_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::rename(temp_path, final_path).await?;

        Ok(())
    }

    /// Delete temporary file
    async fn delete_temp(&self, temp: OutputTempFile) -> Result<()> {
        self.delete(&temp.key).await
    }

    /// Delete file by key
    async fn delete(&self, key: &str) -> Result<()> {
        let path = self.real_path_from_key(key);
        match tokio::fs::remove_file(&path).await {
            Ok(..) => {
                info!(path =?path, "File deleted");
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!(path = ?path, "Attempt to delete a non-existent file");
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
