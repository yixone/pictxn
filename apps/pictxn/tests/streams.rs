use sha2::{Digest, Sha256};
use tokio_util::{bytes::Bytes, io::ReaderStream};

// -- streams::write_in_buff(..) tests --
fn prepare_buffer_and_hash(buffer_size: usize) -> (Bytes, String) {
    let buff = Bytes::from_owner((0..buffer_size).map(|_| 42_u8).collect::<Vec<_>>());

    let mut hasher = Sha256::new();
    hasher.update(&buff);
    let sha256 = hex::encode(hasher.finalize());

    (buff, sha256)
}

#[tokio::test]
async fn write_in_buff() {
    let (buff, sha256) = prepare_buffer_and_hash(64);
    let input_stream = Box::pin(ReaderStream::new(&buff[..]));

    let res = pictxn::streams::write_in_buff(input_stream, 256)
        .await
        .expect("failed to write stream in buffer");

    assert_eq!(
        res.buff, buff,
        "the buffer obtained from the stream must match the original buffer"
    );
    assert_eq!(res.sha256, sha256, "the buffer hashes must match");
}

#[tokio::test]
async fn return_error_if_write_in_buff_limit_exceeded() {
    let (buff, ..) = prepare_buffer_and_hash(128);
    let input_stream = Box::pin(ReaderStream::new(&buff[..]));

    let res = pictxn::streams::write_in_buff(input_stream, 64).await;
    assert!(
        res.is_err(),
        "If the stream size is exceeded, an error should be returned"
    );
}

// -- streams::write_in_str(..) tests --

#[tokio::test]
async fn write_in_str() {
    let string = b"qwerty";
    let input_stream = Box::pin(ReaderStream::new(&string[..]));

    let res = pictxn::streams::write_in_str(input_stream, 256)
        .await
        .expect("failed to write stream in string");

    assert_eq!(res.string, "qwerty");
}

#[tokio::test]
async fn return_error_if_write_in_str_limit_exceeded() {
    let string = b"hello, world!";
    let input_stream = Box::pin(ReaderStream::new(&string[..]));

    let res = pictxn::streams::write_in_str(input_stream, 8).await;

    assert!(
        res.is_err(),
        "If the stream size is exceeded, an error should be returned"
    );
}

#[tokio::test]
async fn return_error_if_write_in_str_input_is_not_utf8() {
    let string = [0xFF, 0xFF];
    let input_stream = Box::pin(ReaderStream::new(&string[..]));

    let res = pictxn::streams::write_in_str(input_stream, 8).await;

    assert!(
        res.is_err(),
        "If the stream is not utf-8, an error should be returned"
    );
}
