use futures::stream::BoxStream;
use sha2::Digest;
use tokio_util::bytes::Bytes;

use crate::errors::CoreError;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Sha256FileHash(pub [u8; 32]);

impl Sha256FileHash {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl From<sha2::Sha256> for Sha256FileHash {
    fn from(value: sha2::Sha256) -> Self {
        let finalized = value.finalize();
        Sha256FileHash(finalized.into())
    }
}

pub struct SetFileResult {
    pub sha256: Sha256FileHash,
    pub filesize: usize,
}

pub type StreamItem = std::io::Result<Bytes>;
pub type BoxedStream<'a> = BoxStream<'a, StreamItem>;

#[async_trait::async_trait]
pub trait FSProvider {
    type Error: Into<CoreError>;

    async fn set<'a>(
        &self,
        key: &str,
        value: BoxedStream<'a>,
    ) -> Result<SetFileResult, Self::Error>;
    async fn get(&self, key: &str) -> Result<BoxedStream<'static>, Self::Error>;
    async fn mv(&self, from: &str, to: &str) -> Result<(), Self::Error>;
    async fn del(&self, key: &str) -> Result<(), Self::Error>;
}
