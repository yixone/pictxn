use std::path::{Path, PathBuf};

use futures::StreamExt;
use pictxn::fs::{BoxedStream, FSProvider, SetFileResult};
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

pub struct NativeFS {
    root: PathBuf,
}

impl NativeFS {
    fn path_from_key(&self, key: &str) -> PathBuf {
        // TODO: add key validation
        self.root.join(&key[..2]).join(&key[2..4]).join(key)
    }
}

async fn create_maybe_parents(p: &Path) -> std::io::Result<()> {
    if let Some(p) = p.parent() {
        tokio::fs::create_dir_all(p).await?;
    }
    Ok(())
}

const MAX_FILE_SIZE: usize = 1024 * 1024 * 1024;

#[async_trait::async_trait]
impl FSProvider for NativeFS {
    type Error = std::io::Error;

    // TODO: add file deletion on error
    async fn set<'a>(
        &self,
        key: &str,
        value: BoxedStream<'a>,
    ) -> Result<SetFileResult, Self::Error> {
        let p = self.path_from_key(key);
        create_maybe_parents(&p).await?;

        let mut size = 0_usize;
        let mut hasher = Sha256::new();
        let mut file = File::create_new(p).await?;
        {
            let mut stream = value;
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                size += chunk.len();

                if size > MAX_FILE_SIZE {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "size limit exceed",
                    ));
                }
                hasher.update(&chunk);
                file.write_all(&chunk).await?;
            }
        }

        Ok(SetFileResult {
            sha256: hasher.into(),
            filesize: size,
        })
    }

    async fn get(&self, key: &str) -> Result<BoxedStream<'static>, Self::Error> {
        let p = self.path_from_key(key);

        let file = File::open(p).await?;
        let reader = ReaderStream::new(file);

        let pinned_stream = Box::pin(reader);
        Ok(pinned_stream)
    }

    async fn mv(&self, from: &str, to: &str) -> Result<(), Self::Error> {
        let f = self.path_from_key(from);
        let t = self.path_from_key(to);

        create_maybe_parents(&t).await?;
        tokio::fs::rename(f, t).await
    }

    async fn del(&self, key: &str) -> Result<(), Self::Error> {
        let p = self.path_from_key(key);
        // TODO: add parent dirs cleaning
        tokio::fs::remove_file(p).await
    }
}
