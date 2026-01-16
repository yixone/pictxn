use std::path::Path;

use futures::{Stream, StreamExt};
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::{io::ReaderStream, sync::CancellationToken};

use crate::{error::StreamError, types::StreamToFileResult};

pub mod error;
pub mod types;

const MEGABYTES: usize = 1024 * 1024;

/// The maximum amount of bytes read from a stream
const MAX_STREAM_SIZE: usize = 1024 * MEGABYTES;

/// The maximum size of the buffer read from the stream
const MAX_STREAM_BUFF_SIZE: usize = 512 * 1024;

fn finalize_sha256_as_hex(hasher: Sha256) -> String {
    let bytes = hasher.finalize();
    hex::encode(bytes)
}

async fn quiet_delete_file(path: &Path) {
    if let Err(e) = tokio::fs::remove_file(path).await {
        tracing::error!(
            err = ?e,
            "Failed to delete temporary file after error"
        );
    }
}

pub async fn write_stream_to_file<T, B, E>(
    stream: &mut T,
    path: impl AsRef<Path>,
    cancel: &CancellationToken,
) -> Result<StreamToFileResult, E>
where
    T: Stream<Item = Result<B, E>> + Unpin,
    B: AsRef<[u8]>,
    E: From<StreamError> + From<std::io::Error>,
{
    let mut bytes_processed = 0;
    let mut hasher = Sha256::new();

    let path = path.as_ref();
    let mut writtable_file = File::create_new(path).await?;
    {
        while let Some(chunk_res) = tokio::select! {
            chunk = stream.next() => chunk,
            _ = cancel.cancelled() => return Err(StreamError::Cancelled.into())
        } {
            let chunk = chunk_res?;
            let chunk_ref = chunk.as_ref();

            bytes_processed += chunk_ref.len();
            if bytes_processed > MAX_STREAM_SIZE {
                quiet_delete_file(path).await;
                return Err(StreamError::StreamSizeLimitExceeded.into());
            }

            hasher.update(chunk_ref);
            writtable_file.write_all(chunk_ref).await?;
        }
    }

    let sha256 = finalize_sha256_as_hex(hasher);

    let processing_result = StreamToFileResult::new(bytes_processed, path.to_path_buf(), sha256);
    Ok(processing_result)
}

pub async fn write_stream_to_buff<T, B, E>(
    stream: &mut T,
    cancel: &CancellationToken,
) -> Result<Vec<u8>, E>
where
    T: Stream<Item = Result<B, E>> + Unpin,
    B: AsRef<[u8]>,
    E: From<StreamError>,
{
    let mut buff = Vec::new();

    while let Some(chunk) = tokio::select! {
        chunk = stream.next() => chunk,
        _ = cancel.cancelled() => return Err(StreamError::Cancelled.into())
    } {
        let chunk = chunk?;
        let bytes = chunk.as_ref();

        if buff.len() + bytes.len() > MAX_STREAM_BUFF_SIZE {
            return Err(StreamError::StreamSizeLimitExceeded.into());
        }

        buff.extend_from_slice(bytes);
    }

    Ok(buff)
}

pub async fn write_stream_to_string<T, B, E>(
    stream: &mut T,
    cancel: &CancellationToken,
) -> Result<String, E>
where
    T: Stream<Item = Result<B, E>> + Unpin,
    B: AsRef<[u8]>,
    E: From<StreamError> + From<std::string::FromUtf8Error>,
{
    let bytes = write_stream_to_buff(stream, cancel).await?;

    let string = String::from_utf8(bytes)?;
    Ok(string)
}

pub async fn read_stream_from_file<E>(
    file_path: impl AsRef<Path>,
) -> Result<ReaderStream<File>, std::io::Error> {
    let file = File::open(file_path.as_ref()).await?;

    let stream = ReaderStream::new(file);

    Ok(stream)
}
