use futures::stream::BoxStream;
use tokio_util::bytes::Bytes;

use crate::types::files::FileWriteResult;

type Stream<'a> = BoxStream<'a, std::io::Result<Bytes>>;

#[async_trait::async_trait]
pub trait StorageProvider {
    async fn set<'a>(&self, key: &str, stream: Stream<'a>) -> std::io::Result<FileWriteResult>;

    async fn get(&self, key: &str) -> std::io::Result<Stream<'static>>;

    async fn mv(&self, from: &str, to: &str) -> std::io::Result<()>;

    async fn del(&self, key: &str) -> std::io::Result<()>;
}
