// Boundary and cleanup tests for streamed registry bodies. / Test ranh giới và dọn file của body registry.
use std::{io, sync::Arc};

use futures_util::{StreamExt, stream};
use sha2::{Digest, Sha512};
use tokio::sync::Notify;

use super::*;
use crate::native::npm_registry::{LARGE_PKG_THRESHOLD_BYTES, base64_encode};

fn test_directory() -> tempfile::TempDir {
    tempfile::tempdir().expect("temporary test directory must be available")
}

fn bytes_stream(
    chunks: Vec<Vec<u8>>,
) -> impl Stream<Item = std::result::Result<Vec<u8>, io::Error>> + Unpin {
    stream::iter(chunks.into_iter().map(Ok))
}

#[tokio::test]
async fn keeps_small_body_in_memory() {
    let directory = test_directory();
    let destination = directory.path().join("small.tgz");

    let result =
        download_response_body(bytes_stream(vec![b"small".to_vec()]), Some(5), &destination)
            .await
            .unwrap();

    assert!(matches!(result, DownloadedTarball::Bytes(bytes) if bytes == b"small"));
    assert!(!destination.exists());
}

#[tokio::test]
async fn spills_when_length_is_missing_and_observed_body_crosses_threshold() {
    let directory = test_directory();
    let destination = directory.path().join("unknown-length.tgz");
    let prefix = vec![b'a'; LARGE_PKG_THRESHOLD_BYTES as usize];
    let suffix = vec![b'b'];
    let expected = [prefix.as_slice(), suffix.as_slice()].concat();

    let result = download_response_body(bytes_stream(vec![prefix, suffix]), None, &destination)
        .await
        .unwrap();

    let digest = base64_encode(&Sha512::digest(&expected));
    assert!(matches!(
        result,
        DownloadedTarball::Streamed { computed_integrity, bytes_len }
            if computed_integrity == format!("sha512-{digest}")
                && bytes_len == expected.len() as u64
    ));
    assert_eq!(std::fs::read(&destination).unwrap(), expected);
}

#[tokio::test]
async fn spills_when_content_length_understates_observed_body() {
    let directory = test_directory();
    let destination = directory.path().join("understated-length.tgz");
    let body = vec![vec![b'x'; LARGE_PKG_THRESHOLD_BYTES as usize + 1]];

    let result = download_response_body(bytes_stream(body), Some(1), &destination)
        .await
        .unwrap();

    assert!(
        matches!(result, DownloadedTarball::Streamed { bytes_len, .. }
        if bytes_len == LARGE_PKG_THRESHOLD_BYTES + 1)
    );
    assert_eq!(
        std::fs::metadata(destination).unwrap().len(),
        LARGE_PKG_THRESHOLD_BYTES + 1
    );
}

#[tokio::test]
async fn removes_partial_file_when_response_stream_fails() {
    let directory = test_directory();
    let destination = directory.path().join("failed.tgz");
    let stream = stream::iter(vec![
        Ok::<_, io::Error>(vec![b'x'; LARGE_PKG_THRESHOLD_BYTES as usize + 1]),
        Err(io::Error::new(
            io::ErrorKind::ConnectionReset,
            "fixture stream failure",
        )),
    ]);

    assert!(
        download_response_body(stream, None, &destination)
            .await
            .is_err()
    );
    assert!(!destination.exists());
}

#[tokio::test]
async fn removes_staging_file_when_download_future_is_cancelled() {
    let directory = test_directory();
    let destination = directory.path().join("cancelled.tgz");
    let next_poll = Arc::new(Notify::new());
    let stream_poll = Arc::clone(&next_poll);
    let body = stream::unfold(0, move |state| {
        let next_poll = Arc::clone(&stream_poll);
        async move {
            if state == 0 {
                Some((
                    Ok::<_, io::Error>(vec![b'x'; LARGE_PKG_THRESHOLD_BYTES as usize + 1]),
                    1,
                ))
            } else {
                next_poll.notify_one();
                std::future::pending::<Option<(std::result::Result<Vec<u8>, io::Error>, i32)>>()
                    .await
            }
        }
    })
    .boxed();
    let task_destination = destination.clone();
    let task =
        tokio::spawn(async move { download_response_body(body, None, &task_destination).await });

    next_poll.notified().await;
    task.abort();
    let _ = task.await;

    assert!(!destination.exists());
}

#[tokio::test]
async fn does_not_remove_a_pre_existing_destination() {
    let directory = test_directory();
    let destination = directory.path().join("existing.tgz");
    std::fs::write(&destination, b"owned by caller").unwrap();
    let body = vec![vec![b'x'; LARGE_PKG_THRESHOLD_BYTES as usize + 1]];

    assert!(
        download_response_body(
            bytes_stream(body),
            Some(LARGE_PKG_THRESHOLD_BYTES + 1),
            &destination
        )
        .await
        .is_err()
    );
    assert_eq!(std::fs::read(destination).unwrap(), b"owned by caller");
}
