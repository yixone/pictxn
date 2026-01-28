use std::path::Path;

use futures::{StreamExt, stream::BoxStream};
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::{bytes::Bytes, io::ReaderStream};

use crate::types::files::{FileWriteResult, Sha256Hash};

pub async fn write_in_file<P>(
    path: P,
    mut stream: BoxStream<'_, std::io::Result<Bytes>>,
    limit: usize,
) -> std::io::Result<FileWriteResult>
where
    P: AsRef<Path>,
{
    let mut size = 0;
    let mut hasher = Sha256::new();

    let mut file = File::create_new(path.as_ref()).await?;
    {
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;

            size += chunk.len();
            if size > limit {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "size limit exceed",
                ));
            }

            hasher.update(&chunk);
            file.write_all(&chunk).await?;
        }
    }

    Ok(FileWriteResult {
        size,
        sha256: Sha256Hash::from_hasher(hasher),
    })
}

pub async fn write_in_buff(
    mut stream: BoxStream<'_, std::io::Result<Bytes>>,
    limit: usize,
) -> std::io::Result<Bytes> {
    let mut size = 0;
    let mut hasher = Sha256::new();

    let mut buff = Vec::new();
    {
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;

            size += chunk.len();
            if size > limit {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "size limit exceed",
                ));
            }

            hasher.update(&chunk);
            buff.extend_from_slice(&chunk);
        }
    }

    Ok(Bytes::from_owner(buff))
}

pub async fn write_in_str(
    stream: BoxStream<'_, std::io::Result<Bytes>>,
    limit: usize,
) -> std::io::Result<String> {
    let res = write_in_buff(stream, limit).await?;
    let string = String::from_utf8(res.to_vec())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid utf-8"))?;

    Ok(string)
}

pub async fn read_from_file<P>(
    file: P,
) -> std::io::Result<BoxStream<'static, std::io::Result<Bytes>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file.as_ref()).await?;
    let strm = ReaderStream::new(file);

    Ok(Box::pin(strm))
}
