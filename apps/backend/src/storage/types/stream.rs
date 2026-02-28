use futures::stream::BoxStream;
use tokio_util::bytes::Bytes;

pub type StreamInner = std::io::Result<Bytes>;

pub type BoxedStream<'a> = BoxStream<'a, StreamInner>;
