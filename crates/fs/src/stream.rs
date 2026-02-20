use futures::stream::BoxStream;
use tokio_util::bytes::Bytes;

type FileStreamInner = std::io::Result<Bytes>;
pub type FileStream<'a> = BoxStream<'a, FileStreamInner>;
