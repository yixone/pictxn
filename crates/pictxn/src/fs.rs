use futures::stream::BoxStream;
use tokio_util::bytes::Bytes;

use crate::errors::CoreError;

type StreamItem = std::io::Result<Bytes>;

type BoxedStream<'a> = BoxStream<'a, StreamItem>;

#[async_trait::async_trait]
pub trait FSProvider {
    type Error: Into<CoreError>;

    async fn set<'a>(&self, key: &str, value: BoxedStream<'a>) -> Result<(), Self::Error>;
    async fn get(&self, key: &str) -> Result<BoxedStream<'static>, Self::Error>;
    async fn mv(&self, from: &str, to: &str) -> Result<(), Self::Error>;
    async fn del(&self, key: &str) -> Result<(), Self::Error>;
}
