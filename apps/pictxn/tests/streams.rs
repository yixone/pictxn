use tokio_util::{bytes::Bytes, io::ReaderStream};

// -- streams::write_in_buff(..) tests --
fn prepare_buffer(buffer_size: usize) -> Bytes {
    Bytes::from_owner((0..buffer_size).map(|_| 42_u8).collect::<Vec<_>>())
}

#[tokio::test]
async fn write_in_buff() {
    let buff = prepare_buffer(64);
    let input_stream = Box::pin(ReaderStream::new(&buff[..]));

    let res = pictxn::streams::write_in_buff(input_stream, 256)
        .await
        .expect("failed to write stream in buffer");

    assert_eq!(
        res, buff,
        "the buffer obtained from the stream must match the original buffer"
    );
}

#[tokio::test]
async fn return_error_if_write_in_buff_limit_exceeded() {
    let buff = prepare_buffer(128);
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

    assert_eq!(res, "qwerty");
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
