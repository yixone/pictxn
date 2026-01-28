use std::path::Path;

use futures::{StreamExt, stream::BoxStream};
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::{bytes::Bytes, io::ReaderStream};

pub struct WriteInFileResult {
    pub size: usize,
    pub sha256: String,
}
pub struct WriteInBuffResult {
    pub buff: Vec<u8>,
    pub sha256: String,
}
pub struct WriteInStrResult {
    pub string: String,
    pub sha256: String,
}

pub async fn write_in_file<P>(
    path: P,
    mut stream: BoxStream<'_, std::io::Result<Bytes>>,
    limit: usize,
) -> std::io::Result<WriteInFileResult>
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
    let sha256 = hex::encode(hasher.finalize());

    Ok(WriteInFileResult { size, sha256 })
}

pub async fn write_in_buff(
    mut stream: BoxStream<'_, std::io::Result<Bytes>>,
    limit: usize,
) -> std::io::Result<WriteInBuffResult> {
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
    let sha256 = hex::encode(hasher.finalize());

    Ok(WriteInBuffResult { buff, sha256 })
}

pub async fn write_in_str(
    stream: BoxStream<'_, std::io::Result<Bytes>>,
    limit: usize,
) -> std::io::Result<WriteInStrResult> {
    let res = write_in_buff(stream, limit).await?;
    let string = String::from_utf8(res.buff)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid utf-8"))?;

    Ok(WriteInStrResult {
        string,
        sha256: res.sha256,
    })
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
